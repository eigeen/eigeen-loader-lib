use std::ffi::c_void;

pub unsafe fn is_player_in_scene(s_wwise_bgm_manager: *const c_void) -> bool {
    if s_wwise_bgm_manager.is_null() {
        return false;
    }

    if *(s_wwise_bgm_manager.byte_add(0x50) as *const usize) == 0 {
        return false;
    }

    true
}
