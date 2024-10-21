use std::ffi::c_void;

use crate::game::mt_type::GameObjectExt;

pub struct Quest {
    ptr: *mut c_void,
}
unsafe impl Send for Quest {}

crate::derive_game_object!(Quest);

impl Quest {
    pub fn quest_state(&self) -> i32 {
        self.get_value_copy(0x38)
    }

    pub fn quest_state_mut(&self) -> &mut i32 {
        self.get_value_mut(0x38)
    }

    pub fn quest_timer_max(&self) -> f32 {
        self.get_value_copy(0x13198 + 0x0C)
    }

    pub fn quest_timer_mut(&self) -> &mut f32 {
        self.get_value_mut(0x13198 + 0x08)
    }

    pub fn ensurance_state_mut(&self) -> &mut i8 {
        self.get_value_mut(0x17384)
    }
}
