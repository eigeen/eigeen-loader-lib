use std::{
    ffi::{c_void, CStr, CString},
    sync::Mutex,
};

use shared::export::{AddressName, SingletonName};

use crate::{address::AddressRepository, singleton::SingletonManager};

pub fn show_system_message(message: &str) {
    // 为了防止panic，通过检查玩家基址是否为空判断是否进入游戏场景
    // 可能存在不稳定性，请勿依赖该保险措施
    // if memory::get_ptr_with_offset(game_export::PLAYER_BASE, game_export::PLAYER_OFFSET)
    //     .map_or(true, |ptr| ptr.is_null())
    // {
    //     return;
    // };
    let func_addr = match AddressRepository::get_address(&AddressName::CHAT_SYSTEM_MESSAGE) {
        Ok(addr) => addr,
        Err(e) => {
            log::error!("Show system message failed: {}", e);
            return;
        }
    };

    let show_message: extern "C" fn(*const c_void, *const i8, i32, i32, i8) =
        unsafe { std::mem::transmute(func_addr) };
    let message_cstring = CString::new(message).unwrap_or_default();

    let Some(s_chat) = SingletonManager::get_ptr_by_name(&SingletonName::CHAT) else {
        return;
    };

    show_message(
        s_chat,
        message_cstring.as_ptr(),
        message.len() as i32,
        -1,
        0,
    )
}

static GAME_REVISION: Mutex<Option<String>> = Mutex::new(None);

/// Get game revision string.
///
/// e.g. "421810"
pub fn get_game_revision() -> Option<String> {
    if let Some(revision) = GAME_REVISION.lock().unwrap().as_ref() {
        return Some(revision.clone());
    }

    let func_ptr: *const u8 = match AddressRepository::get_ptr(&AddressName::CORE_GAME_REVISION) {
        Ok(ptr) => ptr,
        Err(e) => {
            log::error!("Get game revision failed: {}", e);
            return None;
        }
    };

    unsafe {
        // validate if it is MOV op
        let mov_start = func_ptr.add(4);
        let op = std::slice::from_raw_parts(mov_start, 2);
        if op != [0x48, 0x8B] {
            log::error!("Get game revision failed: invalid MOV op");
            return None;
        }

        let const_addr_rel: i32 = *(mov_start.add(3) as *const i32);
        let mov_end = mov_start.add(7);

        let const_ptr = *(mov_end.offset(const_addr_rel as isize) as *const *const i8);

        let game_revision_cstr = CStr::from_ptr(const_ptr);
        let game_revision_str = game_revision_cstr.to_string_lossy().to_string();

        GAME_REVISION
            .lock()
            .unwrap()
            .replace(game_revision_str.clone());

        Some(game_revision_str)
    }
}

/// Get game revision integer.
///
/// e.g. 421810
pub fn get_game_revision_int() -> Option<i32> {
    let game_revision = get_game_revision()?;

    game_revision.parse().ok()
}

// pub fn show_system_message_primary(message: &str) {
//     let show_message: extern "C" fn(*const c_void, *const i8, i32, i32, i8) =
//         unsafe { std::mem::transmute(0x141A53400_i64) };
//     let message_cstring = CString::new(message).unwrap_or_default();

//     let Some(s_chat) = SingletonManager::get_ptr_by_name(&SingletonName::CHAT) else {
//         return;
//     };

//     show_message(
//         s_chat,
//         message_cstring.as_ptr(),
//         message.len() as i32,
//         -1,
//         1,
//     )
// }
