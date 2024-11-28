//! Core plugin for Eigeen Loader.
//!
//! Core plugin designed not to be safely unloaded.
//! It provides some functions for other plugins or core module to use.

use std::{collections::HashMap, ffi::c_void, path::Path};

use log::{error, info};
use shared::export::LoaderVersion;
use windows::{
    core::{s, PCWSTR},
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    },
};

use crate::error::{Error, Result};

/// Core API for dynamic registration and usage.
///
/// Non-thread-safe.
#[derive(Debug, Default)]
pub struct CoreAPI {
    plugins: Vec<CorePlugin>,
    functions: HashMap<String, *const c_void>,
}

unsafe impl Send for CoreAPI {}

impl CoreAPI {
    const CORE_PLUGIN_DIR: &str = "eigeen_loader/core_plugins/";

    pub fn instance() -> &'static mut CoreAPI {
        static mut INSTANCE: Option<CoreAPI> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(CoreAPI::default());
            }

            INSTANCE.as_mut().unwrap()
        }
    }

    pub fn register_function(&mut self, name: &str, function: *const c_void) {
        self.functions.insert(name.to_string(), function);
    }

    pub fn get_function(&self, name: &str) -> Option<*const c_void> {
        self.functions.get(name).copied()
    }

    pub fn load_core_plugins(&mut self) -> Result<(usize, usize)> {
        if !Path::new(Self::CORE_PLUGIN_DIR).exists() {
            info!("Core plugin directory not found, skipping.");
            return Ok((0, 0));
        }

        let mut stat = (0, 0);

        for entry in std::fs::read_dir(Self::CORE_PLUGIN_DIR)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext == "dll" {
                    stat.0 += 1;

                    match Self::init_core_plugin(&path) {
                        Ok(plugin) => {
                            info!("Core plugin loaded: {}", plugin.name);
                            self.plugins.push(plugin);

                            stat.1 += 1;
                        }
                        Err(e) => error!(
                            "Failed to load core plugin {}: {}",
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

    fn init_core_plugin<P: AsRef<Path>>(path: P) -> Result<CorePlugin> {
        let path_w =
            crate::utility::string::to_wstring_bytes_with_nul(path.as_ref().to_str().unwrap());

        // load module
        let hmodule = unsafe { LoadLibraryW(PCWSTR::from_raw(path_w.as_ptr()))? };

        // run initialize function
        unsafe {
            let init_func = GetProcAddress(hmodule, s!("CoreInitialize"));
            if let Some(init_func) = init_func {
                let init_func: InitializeFunc = std::mem::transmute(init_func);

                let param = CoreAPIParam::new();
                let code = init_func(&param);
                if code != 0 {
                    return Err(Error::InitCorePlugin(code));
                }
            }
        }

        // run version function
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
            return Err(Error::IncompatiblePluginRequiredVersion(version));
        }

        Ok(CorePlugin {
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

type InitializeFunc = extern "C" fn(&CoreAPIParam) -> i32;
type VersionFunc = unsafe extern "C" fn(&mut LoaderVersion);

#[derive(Debug)]
struct CorePlugin {
    name: String,
    handle: HMODULE,
}

/// Core plugin initialize function param.
#[repr(C)]
struct CoreAPIParam {
    add_core_function: *const c_void,
    get_core_function: *const c_void,
}

impl CoreAPIParam {
    fn new() -> Self {
        Self {
            add_core_function: add_core_function as *const c_void,
            get_core_function: get_core_function as *const c_void,
        }
    }
}

extern "C" fn add_core_function(name: *const u8, len: u32, func: *const c_void) {
    let name_slice = unsafe { std::slice::from_raw_parts(name, len as usize) };
    let name = std::str::from_utf8(name_slice).unwrap_or_default();

    CoreAPI::instance().register_function(name, func);
}

extern "C" fn get_core_function(name: *const u8, len: u32) -> *const c_void {
    let name_slice = unsafe { std::slice::from_raw_parts(name, len as usize) };
    let name = std::str::from_utf8(name_slice).unwrap_or_default();

    CoreAPI::instance()
        .get_function(name)
        .unwrap_or(std::ptr::null())
}
