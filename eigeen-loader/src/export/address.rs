use crate::{address::AddressRepository, singleton, utility};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Code {
    Ok = 0,
    InvalidUtf8String = 1,
    NotFound = 2,
    BadPattern = 3,
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
/// pattern: Space seperated hex bytes string.
///
/// Example: "FF 00 ?? 00 ??"
#[no_mangle]
pub extern "C" fn PatternScanFirst(pattern: *const u8, len: usize, result: &mut usize) -> i32 {
    unsafe {
        let buf = std::slice::from_raw_parts(pattern, len);
        let Ok(pattern) = std::str::from_utf8(buf) else {
            return Code::InvalidUtf8String as i32;
        };

        let scan_result = utility::memory::auto_scan_first(pattern);
        match scan_result {
            Ok(addr) => {
                *result = addr;
                Code::Ok as i32
            }
            Err(e) => {
                if let utility::memory::MemoryError::PatternScan(_) = e {
                    Code::BadPattern as i32
                } else {
                    Code::NotFound as i32
                }
            }
        }
    }
}

/// Scan for all pattern match.
///
/// Returns the address of the first match, or null if no match is found.
///
/// pattern: Space seperated hex bytes string.
///
/// Example: "FF 00 ?? 00 ??"
///
/// Wildcards allowed: ? ?? * **
#[no_mangle]
pub extern "C" fn PatternScanAll(
    pattern: *const u8,
    len: usize,
    results: *mut usize,
    results_cap: usize,
    results_count: &mut usize,
) -> i32 {
    unsafe {
        let results = std::slice::from_raw_parts_mut(results, results_cap);

        let buf = std::slice::from_raw_parts(pattern, len);
        let Ok(pattern) = std::str::from_utf8(buf) else {
            return Code::InvalidUtf8String as i32;
        };

        let scan_result = utility::memory::auto_scan_all(pattern);
        match scan_result {
            Ok(addrs) => {
                *results_count = addrs.len();
                // 不可超过返回值的容量
                for (i, addr) in addrs.iter().enumerate() {
                    if i >= results.len() {
                        break;
                    }
                    results[i] = *addr;
                }

                Code::Ok as i32
            }
            Err(e) => {
                if let utility::memory::MemoryError::PatternScan(_) = e {
                    Code::BadPattern as i32
                } else {
                    Code::NotFound as i32
                }
            }
        }
    }
}

/// Get a game managed singleton by name.
#[no_mangle]
pub extern "C" fn GetSingleton(name: *const u8, len: usize, result: &mut usize) -> i32 {
    let name_str = unsafe {
        let buf = std::slice::from_raw_parts(name, len);
        std::str::from_utf8(buf).unwrap_or_default()
    };

    let singleton = singleton::SingletonManager::get_address_by_name(name_str);
    match singleton {
        Some(addr) => *result = addr,
        None => {
            return Code::NotFound as i32;
        }
    }

    Code::Ok as i32
}
