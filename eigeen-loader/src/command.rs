use log::info;

use crate::utility;

/// A very simple command handler.
///
/// Now only works for handling loader internal commands.
pub struct CommandHandler {}

impl CommandHandler {
    pub fn on_message(msg: &str) {
        if !msg.starts_with("~") {
            return;
        }

        let parts: Vec<&str> = msg.split_whitespace().collect();

        let program = parts[0].trim_start_matches("~");
        match program {
            "ping" => {
                info!("Pong!");
                utility::game::show_system_message("Pong!");
            }
            "load" => {
                let Some(name) = parts.get(1) else {
                    return;
                };

                do_load(name);
            }
            "unload" => {
                let Some(name) = parts.get(1) else {
                    return;
                };

                do_unload(name);
            }
            _ => {}
        }
    }
}

fn do_load(name: &str) {
    if let Some(loader) = crate::PLUGIN_LOADER.lock().unwrap().as_mut() {
        if let Err(e) = loader.load(name) {
            let msg = format!("Failed to load plugin: {}", e);
            log::error!("{}", msg);
            utility::game::show_system_message(&msg);
            return;
        }
    }

    let msg = format!("Loaded plugin: {}", name);
    info!("{}", msg);
    utility::game::show_system_message(&msg);
}

fn do_unload(name: &str) {
    if let Some(loader) = crate::PLUGIN_LOADER.lock().unwrap().as_mut() {
        if let Err(e) = loader.unload(name) {
            let msg = format!("Failed to unload plugin: {}", e);
            log::error!("{}", msg);
            utility::game::show_system_message(&msg);
            return;
        }
    }

    let msg = format!("Unloaded plugin: {}", name);
    info!("{}", msg);
    utility::game::show_system_message(&msg);
}
