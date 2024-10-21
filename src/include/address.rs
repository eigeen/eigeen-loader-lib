extern "C" {
    fn GetAddress(name: *const u8, len: usize, result: &mut usize) -> i32;

    fn GetSingleton(name: *const u8, len: usize, result: &mut usize) -> i32;
}

use shared::export::{AddressCode, AddressName, SingletonName};

/// Get address record by name.
pub fn get_address(name: AddressName) -> Option<usize> {
    let mut result = 0;

    let code = unsafe { GetAddress(name.as_ptr(), name.len(), &mut result) };

    if code != AddressCode::Ok as i32 {
        return None;
    }

    Some(result)
}

/// Get address record as pointer by name.
pub fn get_ptr<T>(name: AddressName) -> Option<*mut T> {
    get_address(name).map(|addr| addr as *mut T)
}

/// Get a game managed singleton address by name.
pub fn get_singleton_address(name: SingletonName) -> Option<usize> {
    let mut result = 0;

    let code = unsafe { GetSingleton(name.as_ptr(), name.len(), &mut result) };

    if code != AddressCode::Ok as i32 {
        return None;
    }

    Some(result)
}

/// Get a game managed singleton pointer by name.
pub fn get_singleton_ptr<T>(name: SingletonName) -> Option<*mut T> {
    get_singleton_address(name).map(|addr| addr as *mut T)
}
