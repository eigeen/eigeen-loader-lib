use std::{borrow::Cow, path::Path};

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
    initialized: bool,
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
    const PLUGIN_DIR: &'static str = "./eigeen_loader/plugins/";

    pub fn new() -> Self {
        PluginLoader {
            plugins: Vec::new(),
        }
    }

    /// Load all plugins in default plugins directory. `./eigeen_loader/plugins/`
    ///
    /// returns `(all_count, success_count)`
    pub fn auto_load_plugins(&mut self) -> Result<(usize, usize)> {
        if !Path::new(Self::PLUGIN_DIR).exists() {
            info!("Plugin directory not found, skipping plugin auto-load.");
            return Ok((0, 0));
        }

        let mut stat = (0, 0);

        for entry in std::fs::read_dir(Self::PLUGIN_DIR)? {
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

    /// Load a specific plugin by name.
    ///
    /// Will search for the plugin in the default plugins directory.
    pub fn load(&mut self, name: &str) -> Result<()> {
        if !Path::new(Self::PLUGIN_DIR).exists() {
            return Err(Error::PluginNotFound(Self::PLUGIN_DIR.to_string()));
        }

        let name = if name.ends_with(".dll") {
            Cow::from(name)
        } else {
            Cow::from(format!("{}.dll", name))
        };

        let plugin_path = Path::new(Self::PLUGIN_DIR).join(name.as_ref());

        if !plugin_path.exists() {
            return Err(Error::PluginNotFound(
                plugin_path.to_string_lossy().to_string(),
            ));
        }

        match Self::init_plugin(&plugin_path) {
            Ok(plugin) => {
                info!("Plugin loaded: {}", plugin.name);
                self.plugins.push(plugin);
                // TODO: show in-game message
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Unload a specific plugin by name.
    pub fn unload(&mut self, name: &str) -> Result<()> {
        let plugin = self.plugins.iter_mut().find(|p| p.name == name);

        let Some(plugin) = plugin else {
            return Err(Error::PluginNotFound(name.to_string()));
        };

        // uninitialize plugin
        if plugin.initialized {
            unsafe {
                let uninit_func = GetProcAddress(plugin.handle, s!("Uninitialize"));

                // We must call the uninitialize function for safety.
                let Some(uninit_func) = uninit_func else {
                    return Err(Error::UnloadPlugin);
                };

                let uninit_func: UninitializeFunc = std::mem::transmute(uninit_func);
                let code = uninit_func();

                if code != 0 {
                    error!("Failed to uninitialize plugin: code = {}", code);
                    return Err(Error::UnloadPlugin);
                }
            }

            plugin.initialized = false;
        }

        // free library
        if let Err(e) = unsafe { FreeLibrary(plugin.handle) } {
            error!("Failed to free library: {}", e);
            return Err(Error::UnloadPlugin);
        }

        // remove from list
        self.plugins.retain(|p| p.name != name);

        Ok(())
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
                warn!("[{file_name}] Function LoaderVersion(&mut LoaderVersion) not found, or version is not set.");
                warn!("[{file_name}] If it is a compatible plugin, ignore this warning.");
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
            initialized: true,
        })
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

type InitializeFunc = unsafe extern "C" fn() -> i32;
type VersionFunc = unsafe extern "C" fn(&mut LoaderVersion);
type UninitializeFunc = unsafe extern "C" fn() -> i32;
