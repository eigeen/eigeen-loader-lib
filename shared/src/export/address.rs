#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Code {
    Ok = 0,
    InvalidUtf8String = 1,
    NotFound = 2,
}
