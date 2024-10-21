extern "C" {
    fn GetAddress(name: *const u8, len: usize, result: &mut usize) -> i32;
    fn PatternScanFirst(pattern: *const u8, len: usize, result: &mut usize) -> i32;
    fn PatternScanAll(
        pattern: *const u8,
        len: usize,
        results: *mut usize,
        results_cap: usize,
        results_count: &mut usize,
    ) -> i32;

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

/// Scan first address by pattern.
///
/// Search range: the first module process space.
pub fn pattern_scan_first(pattern: &[u8]) -> Option<usize> {
    let mut result = 0;

    let code = unsafe { PatternScanFirst(pattern.as_ptr(), pattern.len(), &mut result) };

    if code != AddressCode::Ok as i32 {
        return None;
    }

    Some(result)
}

/// Scan all addresses by pattern.
///
/// Search range: the first module process space.
pub fn pattern_scan_all(pattern: &[u8]) -> Vec<usize> {
    // 为结果集分配默认大小
    let mut results_cap = 32;
    let mut results = vec![0; results_cap];

    // real result count after scan
    let mut result_count = 0;

    let code = unsafe {
        PatternScanAll(
            pattern.as_ptr(),
            pattern.len(),
            results.as_mut_ptr(),
            results_cap,
            &mut result_count,
        )
    };

    if code != AddressCode::Ok as i32 {
        return vec![];
    }

    if result_count > results_cap {
        // 如果结果集容量不够，扩展并重新获取
        results_cap = result_count;
        results.resize(results_cap, 0);

        let code = unsafe {
            PatternScanAll(
                pattern.as_ptr(),
                pattern.len(),
                results.as_mut_ptr(),
                results_cap,
                &mut result_count,
            )
        };

        if code != AddressCode::Ok as i32 {
            return vec![];
        }
    }

    results
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
