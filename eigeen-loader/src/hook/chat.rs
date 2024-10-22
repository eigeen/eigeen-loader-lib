use std::{ffi::CStr, sync::Mutex};

use safetyhook::InlineHook;
use shared::export::AddressName;

use crate::{address::AddressRepository, error::Result};

static HOOK: Mutex<Option<InlineHook>> = Mutex::new(None);
static mut CALLBACK: Option<Box<CallbackFn>> = None;

type CallbackFn = dyn Fn(&str) + Send + 'static;
type ChatSentFn = unsafe extern "C" fn(*const i8) -> i8;

pub fn hook_chat_sent<F>(callback: F) -> Result<()>
where
    F: Fn(&str) + Send + 'static,
{
    let target = AddressRepository::get_ptr(&AddressName::MID_AFTER_MH_MAIN_CTOR)?;

    unsafe { CALLBACK.replace(Box::new(callback)) };

    let hook = unsafe { safetyhook::create_inline(target, chat_sent_hooked as _) }?;
    HOOK.lock().unwrap().replace(hook);

    Ok(())
}

unsafe extern "C" fn chat_sent_hooked(a1: *const i8) -> i8 {
    let inputs_ptr = a1.byte_offset(0x1008);
    let input_cstr = CStr::from_ptr(inputs_ptr);
    let input_str = input_cstr.to_str().unwrap_or_default();

    if let Some(callback) = CALLBACK.as_ref() {
        callback(input_str);
    }

    let original: ChatSentFn =
        std::mem::transmute(HOOK.lock().unwrap().as_ref().unwrap().original());
    original(a1)
}
