use std::ffi::c_void;

use crate::{address::AddressRepository, singleton};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Code {
    Ok = 0,
    InvalidUtf8String = 1,
    NotFound = 2,
}

/// Get address record by name.
///
/// To scan a custom address, use [PatternScanFirst] or [PatternScanAll] instead.
#[no_mangle]
pub extern "C" fn GetAddress(name: *const u8, len: usize, result: &mut usize) -> i32 {
    unsafe {
        let buf = std::slice::from_raw_parts(name, len);
        let Ok(name) = std::str::from_utf8(buf) else {
            return Code::InvalidUtf8String as i32;
        };

        let Ok(addr) = AddressRepository::get_address(name) else {
            return Code::NotFound as i32;
        };

        *result = addr;
    }

    Code::Ok as i32
}

/// Scan for the first pattern match.
///
/// Returns the address of the first match, or null if no match is found.
///
/// pattern: Space seperated hex bytes string.
///
/// Example: "FF 00 ** 00 ??"
#[no_mangle]
pub extern "C" fn PatternScanFirst(
    pattern: *const u8,
    len: usize,
    result: &mut *const usize,
    result_len: &mut usize,
) -> i32 {
    unimplemented!()
}

/// Scan for all pattern match.
///
/// Returns the address of the first match, or null if no match is found.
///
/// pattern: Space seperated hex bytes string.
///
/// Example: "FF 00 ** 00 ??"
#[no_mangle]
pub extern "C" fn PatternScanAll(
    pattern: *const u8,
    len: usize,
    count: &mut usize,
) -> *const c_void {
    unimplemented!()
}

/// Get a game managed singleton by name.
#[no_mangle]
pub extern "C" fn GetSingleton(name: *const u8, len: usize, result: &mut *mut c_void) -> i32 {
    let name_str = unsafe {
        let buf = std::slice::from_raw_parts(name, len);
        std::str::from_utf8(buf).unwrap_or_default()
    };

    let singleton = singleton::SingletonManager::get_ptr_by_name::<c_void>(name_str);
    match singleton {
        Some(ptr) => *result = ptr,
        None => {
            return Code::NotFound as i32;
        }
    }

    Code::Ok as i32
}
