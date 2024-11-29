#pragma once
#include "eigeen_loader/API.hpp"

#include <intsafe.h>

using namespace elapi;

class HResultHandler {
public:
    static inline void Handle(HRESULT hr, const char* file, int line) {
        if (FAILED(hr)) {
            Logger::error("HRESULT failed: 0x{:X} at {}:{}", hr, file, line);
            std::terminate();
        }
    }

    static inline void Handle(HRESULT hr, const char* file, int line, const char* msg) {
        if (FAILED(hr)) {
            Logger::error("HRESULT failed: 0x{:X} at {}:{}", hr, file, line, msg);
            Logger::error("Message: {}", msg);
            std::terminate();
        }
    }
};

#define HandleResult(hr) HResultHandler::Handle(hr, __FILE__, __LINE__)
#define HandleResultMsg(hr, msg) HResultHandler::Handle(hr, __FILE__, __LINE__, msg)
