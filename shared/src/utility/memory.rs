/// 获取某个地址经过多级偏移后指向的值（的副本） \
/// 该函数与CE中多级偏移取值算法一致
///
/// addr: 裸指针 \
/// offsets: 多级偏移量（单位：byte） \
/// return: 若多级偏移时出现空指针，则返回None，否则返回Some(T)
///
/// 注意：只能检查多次取值时出现的空指针问题。 \
/// 若应用程序出现野指针可能触发异常
pub fn get_value_with_offset<T>(base_addr: *const T, offsets: &[isize]) -> Option<T>
where
    T: Copy,
{
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *const *const T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        // 最后一级取值作为真实值返回
        Some(*addr)
    }
}

/// 获取某个地址经过多级偏移后指向的值的引用 \
/// 该函数与CE中多级偏移取值算法一致
///
/// addr: 裸指针 \
/// offsets: 多级偏移量（单位：byte） \
/// return: 若多级偏移时出现空指针，则返回None，否则返回Some(T)
pub fn get_ref_with_offset<T>(base_addr: *const T, offsets: &[isize]) -> Option<&'static T> {
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *const *const T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        Some(addr.as_ref().unwrap())
    }
}

pub fn get_mut_with_offset<T>(base_addr: *mut T, offsets: &[isize]) -> Option<&'static mut T> {
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *mut *mut T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        Some(addr.as_mut().unwrap())
    }
}

/// 获取某个地址经过多级偏移后的地址 \
/// 该函数与CE中多级偏移取值算法一致
///
/// addr: 裸指针 \
/// offsets: 多级偏移量（单位：byte） \
/// return: 若多级偏移时出现空指针，则返回None，否则返回Some(*const T)
///
/// 注意：只能检查多次取值时出现的空指针问题。 \
/// 若应用程序出现野指针可能触发异常
pub fn get_ptr_with_offset<T>(base_addr: *const T, offsets: &[isize]) -> Option<*const T> {
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *const *const T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        // 返回最后一级指针
        Some(addr)
    }
}
