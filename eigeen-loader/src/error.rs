use shared::export::LoaderVersion;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Windows error: {0}")]
    Windows(#[from] windows::core::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Inline hook error: {0}")]
    InlineHook(#[from] safetyhook::inline_hook::InlineError),
    #[error("Mid hook error: {0}")]
    MidHook(#[from] safetyhook::mid_hook::MidError),
    #[error("Memory error: {0}")]
    Memory(#[from] crate::utility::memory::MemoryError),

    #[error("Failed to initialize plugin: code {0}")]
    InitPlugin(i32),
    #[error("Failed to initialize core extension: code {0}")]
    InitCoreExtension(i32),
    #[error("Incompatible plugin version required: {0}")]
    IncompatiblePluginRequiredVersion(LoaderVersion),

    #[error("Pattern mismatch: {0}")]
    PatternMismatch(String),
    #[error("Pattern name is not managed by loader: {0}")]
    PatternUnmanaged(String),

    #[error("Plugin not found at path: {0}")]
    PluginNotFound(String),
    #[error("Plugin cannot safely unload.")]
    UnloadPlugin,
}
