use std::sync::Mutex;

use safetyhook::MidHook;
use shared::export::AddressName;

use crate::{address::AddressRepository, error::Result};

static HOOK: Mutex<Option<MidHook>> = Mutex::new(None);
static mut CALLBACK: Option<Box<dyn Fn(usize) + Send + 'static>> = None;

pub fn hook_after_mh_main_ctor<F>(callback: F) -> Result<()>
where
    F: Fn(usize) + Send + 'static,
{
    let target = AddressRepository::get_ptr(&AddressName::CORE_MH_MAIN_CTOR)?;

    unsafe { CALLBACK.replace(Box::new(callback)) };

    let hook = unsafe { safetyhook::create_mid(target, mh_main_ctor_hooked as _) }?;
    HOOK.lock().unwrap().replace(hook);

    Ok(())
}

unsafe extern "C" fn mh_main_ctor_hooked(ctx: &mut safetyhook::mid_hook::Context) {
    let mh_main_addr = ctx.rax;

    if let Some(callback) = CALLBACK.as_ref() {
        callback(mh_main_addr);
    }
}
