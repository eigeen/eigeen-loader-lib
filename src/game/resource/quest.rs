use std::ffi::c_void;

use shared::{export::SingletonName, game::mt_type::GameObjectExt};

use crate::include::address;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Quest(*mut c_void);

unsafe impl Send for Quest {}

shared::derive_game_object!(Quest);

impl Quest {
    pub fn from_singleton() -> Option<Self> {
        let ptr = address::get_singleton_ptr(SingletonName::QUEST)?;

        Some(Quest(ptr))
    }

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
