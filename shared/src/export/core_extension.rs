use std::ffi::{c_void, CString};

/// Core plugin initialize function param.
#[repr(C)]
pub struct CoreAPIParam {
    pub add_core_function: *const c_void,
    pub get_core_function: *const c_void,
}

impl CoreAPIParam {
    pub fn add_method(&self, name: &str, method: *const c_void) {
        let func: AddCoreFunctionFn = unsafe { std::mem::transmute(self.add_core_function) };

        let c_name = CString::new(name).unwrap();
        let c_name_ptr = c_name.as_ptr() as *const u8;
        let c_name_len = c_name.as_bytes().len() as u32;

        func(c_name_ptr, c_name_len, method);
    }

    pub fn get_method(&self, name: &str) -> Option<extern "C" fn()> {
        let func: GetCoreFunctionFn = unsafe { std::mem::transmute(self.get_core_function) };

        let c_name = CString::new(name).unwrap();
        let c_name_ptr = c_name.as_ptr() as *const u8;
        let c_name_len = c_name.as_bytes().len() as u32;

        let result = func(c_name_ptr, c_name_len);
        if result.is_null() {
            None
        } else {
            Some(unsafe { std::mem::transmute::<*const c_void, extern "C" fn()>(result) })
        }
    }
}

pub type AddCoreFunctionFn = extern "C" fn(name: *const u8, len: u32, func: *const c_void);
pub type GetCoreFunctionFn = extern "C" fn(name: *const u8, len: u32) -> *const c_void;
