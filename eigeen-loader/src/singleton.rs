use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ffi::c_void,
    sync::{LazyLock, Mutex},
};

use log::{trace, warn};
use safetyhook::InlineHook;
use shared::{
    export::AddressName,
    game::mt_type::{EmptyGameObject, GameObject, GameObjectExt},
};

use crate::address::AddressRepository;
use crate::error::Result;

static HOOK: Mutex<Option<InlineHook>> = Mutex::new(None);
static SINGLETONS: LazyLock<Mutex<HashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static mut SINGLETONS_TEMP: LazyLock<RefCell<HashSet<usize>>> =
    LazyLock::new(|| RefCell::new(HashSet::new()));

pub struct SingletonManager {}

impl SingletonManager {
    pub fn initialize() -> Result<()> {
        // 获取 csystem 构造函数地址
        let target_ptr: *mut u8 = AddressRepository::get_ptr(&AddressName::C_SYSTEM_CTOR)?;

        let hook = unsafe {
            InlineHook::builder()
                .target(target_ptr as *mut _)
                .destination(csystem_ctor_hooked as _)
                .enable_after_setup(true)
                .create()?
        };
        HOOK.lock().unwrap().replace(hook);

        Ok(())
    }

    /// Parse all singletons registered before.
    ///
    /// Run it after mhMain ctor.
    pub fn parse_singletons() {
        let mut singletons = SINGLETONS.lock().unwrap();
        let mut temp_singletons = unsafe { SINGLETONS_TEMP.borrow_mut() };

        for addr in temp_singletons.iter().cloned() {
            let mt_obj = EmptyGameObject::from_ptr(addr as *mut _);

            let Some(dti) = mt_obj.get_dti() else {
                warn!("Singleton with no DTI found: 0x{:x}", addr);
                continue;
            };

            let Some(name) = dti.name() else {
                warn!("Singleton DTI with no readable name found: 0x{:x}", addr);
                continue;
            };

            trace!("Found singleton: {} at 0x{:x}", name, addr);

            singletons.insert(name.to_string(), addr);
        }

        temp_singletons.clear();
        temp_singletons.shrink_to_fit();
    }

    pub fn get_address_by_name(name: &str) -> Option<usize> {
        SINGLETONS.lock().unwrap().get(name).cloned()
    }

    #[allow(dead_code)]
    pub fn get_ptr_by_name<T>(name: &str) -> Option<*mut T> {
        Self::get_address_by_name(name).map(|addr| addr as *mut T)
    }
}

type FuncType = extern "C" fn(*const c_void) -> *const c_void;

unsafe extern "C" fn csystem_ctor_hooked(instance: *const c_void) -> *const c_void {
    trace!("Creating singleton: {:p}", instance);

    SINGLETONS_TEMP.borrow_mut().insert(instance as usize);

    let original: FuncType = std::mem::transmute(HOOK.lock().unwrap().as_ref().unwrap().original());
    original(instance)
}
