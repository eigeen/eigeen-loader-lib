//! Core plugin for Eigeen Loader.
//!
//! Core plugin designed not to be safely unloaded.
//! It provides some functions for other plugins or core module to use.

use std::{collections::HashMap, ffi::c_void, path::Path};

use log::{debug, error, info, warn};
use shared::export::{core_extension::CoreAPIParam, LoaderVersion};
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
    extensions: Vec<CoreExtension>,
    functions: HashMap<String, *const c_void>,
}

unsafe impl Send for CoreAPI {}

impl CoreAPI {
    const CORE_EXT_DIR: &str = "eigeen_loader/core_extensions/";

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

    pub fn load_core_exts(&mut self) -> Result<(usize, usize)> {
        if !Path::new(Self::CORE_EXT_DIR).exists() {
            info!("Core extensions directory not found, skipping.");
            return Ok((0, 0));
        }

        let mut stat = (0, 0);

        for entry in std::fs::read_dir(Self::CORE_EXT_DIR)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext == "dll" {
                    stat.0 += 1;

                    match Self::init_core_extension(&path) {
                        Ok(extension) => {
                            info!("Core extension loaded: {}", extension.name);
                            self.extensions.push(extension);

                            stat.1 += 1;
                        }
                        Err(e) => error!(
                            "Failed to load core extension {}: {}",
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

    fn init_core_extension<P: AsRef<Path>>(path: P) -> Result<CoreExtension> {
        let path_w =
            crate::utility::string::to_wstring_bytes_with_nul(path.as_ref().to_str().unwrap());

        // load module
        let hmodule = unsafe { LoadLibraryW(PCWSTR::from_raw(path_w.as_ptr()))? };

        // run initialize function
        unsafe {
            let init_func = GetProcAddress(hmodule, s!("CoreInitialize"));
            if let Some(init_func) = init_func {
                let init_func: InitializeFunc = std::mem::transmute(init_func);

                let param = new_core_api_param();
                let code = init_func(&param);
                if code != 0 {
                    return Err(Error::InitCoreExtension(code));
                }
            } else {
                warn!("Core extension has no CoreInitialize function. Is it a valid extension?");
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

        Ok(CoreExtension {
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
struct CoreExtension {
    name: String,
    handle: HMODULE,
}

fn new_core_api_param() -> CoreAPIParam {
    CoreAPIParam {
        add_core_function: add_core_function as *const c_void,
        get_core_function: get_core_function as *const c_void,
    }
}

extern "C" fn add_core_function(name: *const u8, len: u32, func: *const c_void) {
    if len == 0 {
        // try to initialize c-string
        let c_name = unsafe { std::ffi::CStr::from_ptr(name as *const i8) };
        let name = c_name.to_str().unwrap_or_default();

        debug!("Core extension function added: {}", name);
        CoreAPI::instance().register_function(name, func);
        return;
    }

    let name_slice = unsafe { std::slice::from_raw_parts(name, len as usize) };
    let name = std::str::from_utf8(name_slice).unwrap_or_default();

    debug!("Core extension function added: {}", name);
    CoreAPI::instance().register_function(name, func);
}

extern "C" fn get_core_function(name: *const u8, len: u32) -> *const c_void {
    if len == 0 {
        // try to initialize c-string
        let c_name = unsafe { std::ffi::CStr::from_ptr(name as *const i8) };
        let name = c_name.to_str().unwrap_or_default();

        debug!("Core extension function get: {}", name);
        return CoreAPI::instance()
            .get_function(name)
            .unwrap_or(std::ptr::null());
    }

    let name_slice = unsafe { std::slice::from_raw_parts(name, len as usize) };
    let name = std::str::from_utf8(name_slice).unwrap_or_default();

    debug!("Core extension function get: {}", name);
    CoreAPI::instance()
        .get_function(name)
        .unwrap_or(std::ptr::null())
}
