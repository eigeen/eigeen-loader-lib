use std::ffi::c_void;

use crate::game::mt_type::GameObjectExt;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Health(pub *mut c_void);

unsafe impl Send for Health {}

crate::derive_game_object!(Health);

impl Health {
    pub fn max(&self) -> f32 {
        self.get_value_copy(0x60)
    }

    pub fn current(&self) -> f32 {
        self.get_value_copy(0x64)
    }

    pub fn max_mut(&self) -> &mut f32 {
        self.get_value_mut(0x60)
    }

    pub fn current_mut(&self) -> &mut f32 {
        self.get_value_mut(0x64)
    }
}
