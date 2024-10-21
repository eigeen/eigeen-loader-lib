extern "C" {
    fn GetAddress(name: *const u8, len: usize, result: &mut usize) -> i32;

    fn GetSingleton(name: *const u8, len: usize, result: &mut *mut c_void) -> i32;
}

use std::ffi::c_void;

use shared::export::AddressCode;

/// Get address record by name.
pub fn get_address(name: &str) -> Option<usize> {
    let mut result = 0;

    let code = unsafe { GetAddress(name.as_ptr(), name.len(), &mut result) };

    if code != AddressCode::Ok as i32 {
        return None;
    }

    Some(result)
}

/// Get address record as pointer by name.
pub fn get_ptr<T>(name: &str) -> Option<*mut T> {
    get_address(name).map(|addr| addr as *mut T)
}

/// Get a game managed singleton by name.
pub fn get_singleton<T>(name: &str) -> Option<*mut T> {
    let mut result = std::ptr::null_mut();

    let code = unsafe { GetSingleton(name.as_ptr(), name.len(), &mut result) };

    if code != AddressCode::Ok as i32 {
        return None;
    }

    Some(result as *mut T)
}
