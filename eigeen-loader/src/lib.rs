use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use log::{debug, info};
use windows::Win32::Foundation::{BOOL, TRUE};

mod address;
mod command;
mod error;
mod export;
mod hook;
mod logger;
mod singleton;
mod utility;

mod core_extension;
mod plugin;

// exports
pub use export::*;

fn panic_hook(info: &std::panic::PanicHookInfo) {
    utility::windows::message_box_fatal(&format!("EigeenLoader panic! {}", info));
}

pub static PLUGIN_LOADER: Mutex<Option<plugin::PluginLoader>> = Mutex::new(None);
pub static CORE_PLUGIN_API: Mutex<Option<core_extension::CoreAPI>> = Mutex::new(None);
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
        debug!("After MhMainCtor");
        // parse game singletons
        singleton::SingletonManager::parse_singletons();
        // load core extensions
        let result = core_extension::CoreAPI::instance().load_core_exts();
        match result {
            Ok((total, success)) => {
                info!(
                    "Loaded {} core extensions ({} total, {} failed).",
                    success,
                    total,
                    total - success
                )
            }
            Err(e) => log::error!("Failed to load core extensions: {}", e),
        }
        // initialize d3d core module
        if let Some(init_fn) = core_extension::CoreAPI::instance().get_function("d3d_initialize") {
            let init_fn: extern "C" fn() -> i32 = unsafe { std::mem::transmute(init_fn) };
            let code = init_fn();
            if code != 0 {
                log::error!("Failed to initialize d3d core module: {}", code);
            }
        }

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
    });
    if let Err(e) = result {
        log::error!("Fatal error: Failed to hook MhMainCtor: {}", e);
        return TRUE;
    }

    let result = hook::chat::hook_chat_sent(|msg| {
        command::CommandHandler::on_message(msg);
    });
    if let Err(e) = result {
        log::warn!("Error: Failed to hook ChatSent: {}", e);
        log::warn!("Chat commands would not work correctly.");
    }

    // initialize game singletons
    if let Err(e) = singleton::SingletonManager::initialize() {
        log::error!("Failed to initialize game singletons: {}", e);
        log::error!("Some plugins may not work correctly.");
    }

    // do after initialization
    // recover focus to game window
    if let Err(e) = utility::windows::focus_mhw_main_window() {
        log::warn!("Failed to focus MHW main window: {}", e);
    };

    info!("EigeenLoader initialized.");

    TRUE
}
