#ifndef API_HPP
#define API_HPP
#endif

#include <string>
#include <cstring>
#include <vector>
#include <format>

namespace elapi {

    extern "C"
    {
        void Log(const uint8_t* msg, size_t len, uint8_t level);

        int32_t GetAddress(const uint8_t* name, size_t len, uintptr_t* result);
        int32_t PatternScanFirst(const uint8_t* pattern, size_t len, uintptr_t* result);
        int32_t PatternScanAll(const uint8_t* pattern, size_t len, uintptr_t* results, size_t results_cap, size_t* results_count);
        int32_t GetSingleton(const uint8_t* name, size_t len, uintptr_t* result);

        void ShowSystemMessage(const uint8_t* msg, size_t len);
    }

#define EL_API __declspec(dllexport)

#define LOADER_VERSION_MAJOR 1
#define LOADER_VERSION_MINOR 0
#define LOADER_VERSION_PATCH 0

    struct Version
    {
        int32_t major;
        int32_t minor;
        int32_t patch;
    };

    class Logger
    {
    public:
        enum Level : uint8_t
        {
            Error = 1,
            Warning = 2,
            Info = 3,
            Debug = 4,
            Trace = 5
        };

        template <typename... Args>
        static void debug(const std::format_string<Args...>& fmt, Args&&... args)
        {
            log(std::vformat(fmt.get(), std::make_format_args(args...)), Level::Debug);
        }

        template <typename... Args>
        static void info(const std::format_string<Args...>& fmt, Args&&... args)
        {
            log(std::vformat(fmt.get(), std::make_format_args(args...)), Level::Info);
        }

        template <typename... Args>
        static void warn(const std::format_string<Args...>& fmt, Args&&... args)
        {
            log(std::vformat(fmt.get(), std::make_format_args(args...)), Level::Warning);
        }

        template <typename... Args>
        static void error(const std::format_string<Args...>& fmt, Args&&... args)
        {
            log(std::vformat(fmt.get(), std::make_format_args(args...)), Level::Error);
        }

        template <typename... Args>
        static void trace(const std::format_string<Args...>& fmt, Args&&... args)
        {
            log(std::vformat(fmt.get(), std::make_format_args(args...)), Level::Trace);
        }

    private:
        static void log(const std::string& msg, Level level)
        {
            Log(reinterpret_cast<const uint8_t*>(msg.c_str()), msg.size(), static_cast<uint8_t>(level));
        }
    };

    class Memory
    {
    public:
        /// @brief Get a loader managed address by its defined name.
        /// @param name Address name.
        /// @return Target address or 0 if not found.
        static uintptr_t get_address(const std::string& name)
        {
            uintptr_t result;

            int32_t status = GetAddress(reinterpret_cast<const uint8_t*>(name.c_str()), name.size(), &result);
            if (status != 0)
            {
                return 0;
            }

            return result;
        }

        /// @brief Scan for the first occurrence of a pattern in memory.
        /// @param pattern Pattern string. Space seperated hex bytes. E.g. "48 8B 05 ?? ?? ?? ?? 48 8B 40 10". Supported wildcards: `?` `??` `*` `**`
        /// @return Target address or 0 if not found.
        /// @note Scan range: base ~ base + size of the first module.
        static uintptr_t pattern_scan_first(const std::string& pattern)
        {
            uintptr_t result;

            int32_t status = PatternScanFirst(reinterpret_cast<const uint8_t*>(pattern.c_str()), pattern.size(), &result);
            if (status != 0)
            {
                return 0;
            }

            return result;
        }

        /// @brief Scan all occurrences of a pattern in memory.
        /// @param pattern Pattern string. Space seperated hex bytes. E.g. "48 8B 05 ?? ?? ?? ?? 48 8B 40 10". Supported wildcards: `?` `??` `*` `**`
        /// @return Target addresses or empty vector if not found.
        /// @note Scan range: base ~ base + size of the first module.
        static std::vector<uintptr_t> pattern_scan_all(const std::string& pattern)
        {
            std::vector<uintptr_t> results;
            size_t results_count = 0;

            size_t results_cap = 128;
            results.reserve(results_cap);

            int32_t status = PatternScanAll(reinterpret_cast<const uint8_t*>(pattern.c_str()), pattern.size(),
                results.data(), results.capacity(), &results_count);
            if (status != 0)
            {
                return results;
            }

            // if results_count > results_cap, resize the vector to fit the actual count
            // but we don't do it here
            // 128 is enough for most cases

            return results;
        }

        /// @brief Get a game engine managed singleton by its name.
        /// @param name Singleton name. E.g. "sPlayer"
        /// @return Address of the singleton or 0 if not found.
        static uintptr_t get_singleton(const std::string& name)
        {
            uintptr_t result;

            int32_t status = GetSingleton(reinterpret_cast<const uint8_t*>(name.c_str()), name.size(), &result);
            if (status != 0)
            {
                return 0;
            }

            return result;
        }
    };

    class Game {
    public:
        static void show_system_message(const std::string& msg)
        {
            ShowSystemMessage(reinterpret_cast<const uint8_t*>(msg.c_str()), msg.size());
        }
    };

    typedef void (*AddCoreFunctionPtr)(const char* name, uint32_t len, const void* func);
    typedef const void* (*GetCoreFunctionPtr)(const char* name, uint32_t len);

    struct CoreParam {
        AddCoreFunctionPtr add_core_function;
        GetCoreFunctionPtr get_core_function;

        template<typename TFunc>
        TFunc* get_method(std::string_view method) const {
            return static_cast<TFunc*>(get_core_function(method.data(), 0));
        }

        template<typename TFunc>
        void add_method(std::string_view method, TFunc* func) const {
            return static_cast<TFunc*>(add_core_function(method.data(), 0, static_cast<void*>(func)));
        }
    };
}
