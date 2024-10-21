use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use log::info;
use windows::Win32::Foundation::{BOOL, TRUE};

mod address;
mod error;
mod export;
mod hook;
mod logger;
mod plugin;
mod singleton;
mod utility;

// exports
pub use export::*;

fn panic_hook(info: &std::panic::PanicInfo) {
    utility::windows::message_box_fatal(&format!("EigeenLoader panic! {}", info));
}

static PLUGIN_LOADER: Mutex<Option<plugin::PluginLoader>> = Mutex::new(None);
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C-unwind" fn Initialize() -> BOOL {
    if INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        return TRUE;
    }

    // setup panic hook
    std::panic::set_hook(Box::new(panic_hook));

    // initialize logging
    let _ = logger::initialize_logging();

    INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);

    // initialize address module
    let address_file_path = Path::new("./eigeen_loader/address/address_records.json");
    if !address_file_path.exists() {
        log::error!("Address file not found: {}", address_file_path.display());
        log::error!("Some plugins may not work correctly.");
    } else if let Err(e) = address::AddressRepository::initialize(address_file_path) {
        log::error!("Failed to initialize address repository: {}", e);
        log::error!("Some plugins may not work correctly.");
    }

    // setup hooks
    let result = hook::mh_main::hook_after_mh_main_ctor(|_mh_main_addr| {
        info!("After MhMainCtor");
        singleton::SingletonManager::parse_singletons();
    });
    if let Err(e) = result {
        log::error!("Fatal error: Failed to hook MhMainCtor: {}", e);
        return TRUE;
    }

    // initialize game singletons
    if let Err(e) = singleton::SingletonManager::initialize() {
        log::error!("Failed to initialize game singletons: {}", e);
        log::error!("Some plugins may not work correctly.");
    }

    info!("EigeenLoader initialized. Loading plugins...");

    // create plugin loader
    let mut loader = plugin::PluginLoader::new();
    // load plugins
    let result = loader.auto_load_plugins();
    match result {
        Ok((total, success)) => info!(
            "Loaded {} plugins ({} total, {} failed).",
            success,
            total,
            total - success
        ),
        Err(e) => log::error!("Failed to load any plugins: {}", e),
    }
    PLUGIN_LOADER.lock().unwrap().replace(loader);

    log::info!("EigeenLoader initialized.");

    TRUE
}
