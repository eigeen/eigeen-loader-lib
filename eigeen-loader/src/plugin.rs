use std::path::Path;

use log::{error, info, warn};
use shared::export::LoaderVersion;
use windows::{
    core::{s, PCWSTR},
    Win32::{
        Foundation::{FreeLibrary, HMODULE},
        System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    },
};

use crate::{
    error::{Error, Result},
    utility,
};

pub struct PluginLoader {
    plugins: Vec<Plugin>,
}

pub struct Plugin {
    name: String,
    handle: HMODULE,
}

unsafe impl Send for Plugin {}

impl Drop for Plugin {
    fn drop(&mut self) {
        unsafe {
            let _ = FreeLibrary(self.handle);
        }
    }
}

impl PluginLoader {
    pub fn new() -> Self {
        PluginLoader {
            plugins: Vec::new(),
        }
    }

    /// Load all plugins in default plugins directory. `./eigeen_loader/plugins/`
    ///
    /// returns `(all_count, success_count)`
    pub fn auto_load_plugins(&mut self) -> Result<(usize, usize)> {
        let scan_path = "./eigeen_loader/plugins/";

        if !Path::new(scan_path).exists() {
            info!("No plugin directory found, skipping plugin auto-load.");
            return Ok((0, 0));
        }

        let mut stat = (0, 0);

        for entry in std::fs::read_dir(scan_path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext == "dll" {
                    stat.0 += 1;

                    match Self::init_plugin(&path) {
                        Ok(plugin) => {
                            info!("Plugin loaded: {}", plugin.name);
                            self.plugins.push(plugin);

                            stat.1 += 1;
                        }
                        Err(e) => error!(
                            "Failed to load plugin {}: {}",
                            path.file_stem()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default(),
                            e
                        ),
                    }
                }
            }
        }

        Ok(stat)
    }

    fn init_plugin<P: AsRef<Path>>(path: P) -> Result<Plugin> {
        let path_w = utility::string::to_wstring_bytes_with_nul(path.as_ref().to_str().unwrap());

        // load module
        let hmodule = unsafe { LoadLibraryW(PCWSTR::from_raw(path_w.as_ptr()))? };

        // run optional initialize function
        unsafe {
            let init_func = GetProcAddress(hmodule, s!("Initialize"));
            if let Some(init_func) = init_func {
                let init_func: InitializeFunc = std::mem::transmute(init_func);

                let code = init_func();
                if code != 0 {
                    return Err(Error::InitPlugin(code));
                }
            }
        }

        // run optional version function
        let mut version = LoaderVersion::default();

        unsafe {
            let version_func = GetProcAddress(hmodule, s!("LoaderVersion"));
            if let Some(version_func) = version_func {
                let version_func: VersionFunc = std::mem::transmute(version_func);

                version_func(&mut version);
            }
        }

        // loader version compatibility check
        if version.major != 1 {
            if version == LoaderVersion::default() {
                let file_name = path.as_ref().file_name().unwrap().to_str().unwrap();
                warn!("[Loading:{file_name}] Function LoaderVersion(&mut LoaderVersion) not found, or version is not set. If it is a compatible plugin, ignore this warning.");
            } else {
                return Err(Error::IncompatiblePluginRequiredVersion(version));
            }
        }

        Ok(Plugin {
            name: path
                .as_ref()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            handle: hmodule,
        })
    }
}

type InitializeFunc = unsafe extern "C" fn() -> i32;
type VersionFunc = unsafe extern "C" fn(&mut LoaderVersion);
