mod address;
pub mod core_extension;

pub use address::{AddressName, Code as AddressCode, SingletonName};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LoaderVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl std::fmt::Display for LoaderVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
