pub mod mt_type;

#[macro_export]
macro_rules! derive_game_object {
    ($name:ident) => {
        impl $crate::game::mt_type::GameObject for $name {
            fn from_ptr(ptr: *mut c_void) -> Self {
                Self(ptr)
            }

            fn as_ptr(&self) -> *mut c_void {
                self.0
            }
        }
    };
}
