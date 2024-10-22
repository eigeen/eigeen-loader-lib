use std::ffi::{c_void, CString};

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
