#include <API.hpp>

/// @brief Initialize function *required*
/// @return Ok = 0
extern "C" API int32_t Initialize()
{
    // Log something
    // This is a simple way to log messages to the console.
    Logger::debug("This is an debug message.");

    // If has fmt library, you can use formatting
    Logger::info("This is an info message, with {} and {}.", "alice", "bob");

    // We enabled windows console virtual terminal sequences for you.
    // See docs here: https://learn.microsoft.com/windows/console/console-virtual-terminal-sequences
    // You can use control codes to set formatting, colors, etc.

    Logger::error("\033[32mI'm a error message, but I'm green!");

    // Get a singleton instance address, managed by game engine
    uintptr_t player_ptr = Memory::get_singleton("sPlayer");
    if (player_ptr == 0)
    {
        // The singleton will be created later, after the game engine initialized.
        // So you can see error message. Don't worry!
        Logger::error("sPlayer not found.");
    }
    else
    {
        Logger::info("sPlayer found at address: 0x{:X}", player_ptr);
    }

    return 0;
}

/// @brief Required loader version
/// @note For version compatibility checks.
extern "C" API void LoaderVersion(Version *version)
{
    version->major = LOADER_VERSION_MAJOR;
    version->minor = LOADER_VERSION_MINOR;
    version->patch = LOADER_VERSION_PATCH;
}

/// @brief Uninitialize function
/// @note Optional, but recommended.
/// @note You can do nothing here, but if it's not defined, the loader will not unload the plugin.
/// @return Ok = 0
extern "C" API int32_t Uninitialize()
{
    return 0;
}
