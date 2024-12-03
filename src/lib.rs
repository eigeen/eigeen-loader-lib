//! The wrapper of EigeenLoader exports and provides utilities for plugins.

pub mod include;
pub mod logger;

pub use shared::export::*;
pub use shared::game::*;

/// Get the version of the EigeenLoader library.
pub fn get_version() -> LoaderVersion {
    let mut splitted = env!("CARGO_PKG_VERSION").split(".");

    LoaderVersion {
        major: splitted.next().unwrap().parse().unwrap(),
        minor: splitted.next().unwrap().parse().unwrap(),
        patch: splitted.next().unwrap().parse().unwrap(),
    }
}

pub mod prelude {
    pub use shared::export::*;

    pub use shared::game::mt_type::*;

    pub use crate::include::address as el_address;
    pub use crate::include::game as el_game;
}
