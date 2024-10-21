#include <windows.h>
#include <vector>
#include <iostream>
#include <wil/stl.h>
#include <wil/win32_helpers.h>

typedef BOOL (*InitializeFunc)();

const auto MODULE_NAME = L"eigeen_loader.dll";
const auto INITIALIZE_NAME = "Initialize";
const auto EXPECT_EXE_NAME = L"MonsterHunterWorld.exe";

static std::vector<std::wstring> getCurrentProcessModules()
{
    std::vector<std::wstring> moduleList;
    HANDLE hProcess = GetCurrentProcess();
    wil::unique_handle processHandle(hProcess);

    DWORD cbNeeded;
    HMODULE hMods[1024];

    if (EnumProcessModules(processHandle.get(), hMods, sizeof(hMods), &cbNeeded))
    {
        for (unsigned int i = 0; i < (cbNeeded / sizeof(HMODULE)); ++i)
        {
            wchar_t moduleName[MAX_PATH];
            if (GetModuleFileNameExW(processHandle.get(), hMods[i], moduleName, sizeof(moduleName) / sizeof(wchar_t)))
            {
                moduleList.emplace_back(moduleName);
            }
        }
    }

    return moduleList;
}

static bool containsModule(const std::vector<std::wstring> &modules, const std::wstring &moduleName)
{
    return std::find(modules.begin(), modules.end(), moduleName) != modules.end();
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved)
{
    switch (fdwReason)
    {
    case DLL_PROCESS_ATTACH:
    {
        std::vector<std::wstring> modules = getCurrentProcessModules();

        // Check if loaded by game
        if (wil::GetModuleFileNameW<std::wstring>().find(EXPECT_EXE_NAME) == std::wstring::npos)
        {
            return TRUE;
        }

        // Check if already loaded
        if (containsModule(modules, MODULE_NAME))
        {
            return TRUE;
        }

        // // Check if dinput8.dll loaded
        // // ensure loader loaded after dinput8.dll loaded
        // // for compatibility with stracker's loader
        // // if some plugins depends on stracker's loader: loader.dll
        // if (!containsModule(modules, L"dinput8.dll"))
        // {
        //     return TRUE;
        // }

        HMODULE hCore = LoadLibraryW(MODULE_NAME);
        if (!hCore)
        {
            MessageBoxW(NULL, L"Failed to load eigeen_loader.dll", L"EigeenLoader", MB_ICONERROR);
            return TRUE;
        }

        InitializeFunc Initialize = (InitializeFunc)GetProcAddress(hCore, INITIALIZE_NAME);
        if (!Initialize)
        {
            MessageBoxW(NULL, L"Failed to get Initialize function", L"EigeenLoader", MB_ICONERROR);
            FreeLibrary(hCore);
            return TRUE;
        }

        if (!Initialize())
        {
            MessageBoxW(NULL, L"Initialize failed", L"EigeenLoader", MB_ICONERROR);
            // no free for blocking multiple loads
            // FreeLibrary(hCore);
            return TRUE;
        }

        break;
    }
    case DLL_THREAD_ATTACH:
        break;
    case DLL_THREAD_DETACH:
        break;
    case DLL_PROCESS_DETACH:
        if (lpvReserved != nullptr)
        {
            break;
        }
        break;
    }

    return TRUE;
}

#pragma region d3d11 forward

#pragma comment(linker, "/export:D3D11CreateDeviceForD3D12=\"C:\\Windows\\System32\\d3d11.D3D11CreateDeviceForD3D12\"")
#pragma comment(linker, "/export:D3DKMTCloseAdapter=\"C:\\Windows\\System32\\d3d11.D3DKMTCloseAdapter\"")
#pragma comment(linker, "/export:D3DKMTDestroyAllocation=\"C:\\Windows\\System32\\d3d11.D3DKMTDestroyAllocation\"")
#pragma comment(linker, "/export:D3DKMTDestroyContext=\"C:\\Windows\\System32\\d3d11.D3DKMTDestroyContext\"")
#pragma comment(linker, "/export:D3DKMTDestroyDevice=\"C:\\Windows\\System32\\d3d11.D3DKMTDestroyDevice\"")
#pragma comment(linker, "/export:D3DKMTDestroySynchronizationObject=\"C:\\Windows\\System32\\d3d11.D3DKMTDestroySynchronizationObject\"")
#pragma comment(linker, "/export:D3DKMTPresent=\"C:\\Windows\\System32\\d3d11.D3DKMTPresent\"")
#pragma comment(linker, "/export:D3DKMTQueryAdapterInfo=\"C:\\Windows\\System32\\d3d11.D3DKMTQueryAdapterInfo\"")
#pragma comment(linker, "/export:D3DKMTSetDisplayPrivateDriverFormat=\"C:\\Windows\\System32\\d3d11.D3DKMTSetDisplayPrivateDriverFormat\"")
#pragma comment(linker, "/export:D3DKMTSignalSynchronizationObject=\"C:\\Windows\\System32\\d3d11.D3DKMTSignalSynchronizationObject\"")
#pragma comment(linker, "/export:D3DKMTUnlock=\"C:\\Windows\\System32\\d3d11.D3DKMTUnlock\"")
#pragma comment(linker, "/export:D3DKMTWaitForSynchronizationObject=\"C:\\Windows\\System32\\d3d11.D3DKMTWaitForSynchronizationObject\"")
#pragma comment(linker, "/export:EnableFeatureLevelUpgrade=\"C:\\Windows\\System32\\d3d11.EnableFeatureLevelUpgrade\"")
#pragma comment(linker, "/export:OpenAdapter10=\"C:\\Windows\\System32\\d3d11.OpenAdapter10\"")
#pragma comment(linker, "/export:OpenAdapter10_2=\"C:\\Windows\\System32\\d3d11.OpenAdapter10_2\"")
#pragma comment(linker, "/export:CreateDirect3D11DeviceFromDXGIDevice=\"C:\\Windows\\System32\\d3d11.CreateDirect3D11DeviceFromDXGIDevice\"")
#pragma comment(linker, "/export:CreateDirect3D11SurfaceFromDXGISurface=\"C:\\Windows\\System32\\d3d11.CreateDirect3D11SurfaceFromDXGISurface\"")
#pragma comment(linker, "/export:D3D11CoreCreateDevice=\"C:\\Windows\\System32\\d3d11.D3D11CoreCreateDevice\"")
#pragma comment(linker, "/export:D3D11CoreCreateLayeredDevice=\"C:\\Windows\\System32\\d3d11.D3D11CoreCreateLayeredDevice\"")
#pragma comment(linker, "/export:D3D11CoreGetLayeredDeviceSize=\"C:\\Windows\\System32\\d3d11.D3D11CoreGetLayeredDeviceSize\"")
#pragma comment(linker, "/export:D3D11CoreRegisterLayers=\"C:\\Windows\\System32\\d3d11.D3D11CoreRegisterLayers\"")
#pragma comment(linker, "/export:D3D11CreateDevice=\"C:\\Windows\\System32\\d3d11.D3D11CreateDevice\"")
#pragma comment(linker, "/export:D3D11CreateDeviceAndSwapChain=\"C:\\Windows\\System32\\d3d11.D3D11CreateDeviceAndSwapChain\"")
#pragma comment(linker, "/export:D3D11On12CreateDevice=\"C:\\Windows\\System32\\d3d11.D3D11On12CreateDevice\"")
#pragma comment(linker, "/export:D3DKMTCreateAllocation=\"C:\\Windows\\System32\\d3d11.D3DKMTCreateAllocation\"")
#pragma comment(linker, "/export:D3DKMTCreateContext=\"C:\\Windows\\System32\\d3d11.D3DKMTCreateContext\"")
#pragma comment(linker, "/export:D3DKMTCreateDevice=\"C:\\Windows\\System32\\d3d11.D3DKMTCreateDevice\"")
#pragma comment(linker, "/export:D3DKMTCreateSynchronizationObject=\"C:\\Windows\\System32\\d3d11.D3DKMTCreateSynchronizationObject\"")
#pragma comment(linker, "/export:D3DKMTEscape=\"C:\\Windows\\System32\\d3d11.D3DKMTEscape\"")
#pragma comment(linker, "/export:D3DKMTGetContextSchedulingPriority=\"C:\\Windows\\System32\\d3d11.D3DKMTGetContextSchedulingPriority\"")
#pragma comment(linker, "/export:D3DKMTGetDeviceState=\"C:\\Windows\\System32\\d3d11.D3DKMTGetDeviceState\"")
#pragma comment(linker, "/export:D3DKMTGetDisplayModeList=\"C:\\Windows\\System32\\d3d11.D3DKMTGetDisplayModeList\"")
#pragma comment(linker, "/export:D3DKMTGetMultisampleMethodList=\"C:\\Windows\\System32\\d3d11.D3DKMTGetMultisampleMethodList\"")
#pragma comment(linker, "/export:D3DKMTGetRuntimeData=\"C:\\Windows\\System32\\d3d11.D3DKMTGetRuntimeData\"")
#pragma comment(linker, "/export:D3DKMTGetSharedPrimaryHandle=\"C:\\Windows\\System32\\d3d11.D3DKMTGetSharedPrimaryHandle\"")
#pragma comment(linker, "/export:D3DKMTLock=\"C:\\Windows\\System32\\d3d11.D3DKMTLock\"")
#pragma comment(linker, "/export:D3DKMTOpenAdapterFromHdc=\"C:\\Windows\\System32\\d3d11.D3DKMTOpenAdapterFromHdc\"")
#pragma comment(linker, "/export:D3DKMTOpenResource=\"C:\\Windows\\System32\\d3d11.D3DKMTOpenResource\"")
#pragma comment(linker, "/export:D3DKMTQueryAllocationResidency=\"C:\\Windows\\System32\\d3d11.D3DKMTQueryAllocationResidency\"")
#pragma comment(linker, "/export:D3DKMTQueryResourceInfo=\"C:\\Windows\\System32\\d3d11.D3DKMTQueryResourceInfo\"")
#pragma comment(linker, "/export:D3DKMTRender=\"C:\\Windows\\System32\\d3d11.D3DKMTRender\"")
#pragma comment(linker, "/export:D3DKMTSetAllocationPriority=\"C:\\Windows\\System32\\d3d11.D3DKMTSetAllocationPriority\"")
#pragma comment(linker, "/export:D3DKMTSetContextSchedulingPriority=\"C:\\Windows\\System32\\d3d11.D3DKMTSetContextSchedulingPriority\"")
#pragma comment(linker, "/export:D3DKMTSetDisplayMode=\"C:\\Windows\\System32\\d3d11.D3DKMTSetDisplayMode\"")
#pragma comment(linker, "/export:D3DKMTSetGammaRamp=\"C:\\Windows\\System32\\d3d11.D3DKMTSetGammaRamp\"")
#pragma comment(linker, "/export:D3DKMTSetVidPnSourceOwner=\"C:\\Windows\\System32\\d3d11.D3DKMTSetVidPnSourceOwner\"")
#pragma comment(linker, "/export:D3DKMTWaitForVerticalBlankEvent=\"C:\\Windows\\System32\\d3d11.D3DKMTWaitForVerticalBlankEvent\"")
#pragma comment(linker, "/export:D3DPerformance_BeginEvent=\"C:\\Windows\\System32\\d3d11.D3DPerformance_BeginEvent\"")
#pragma comment(linker, "/export:D3DPerformance_EndEvent=\"C:\\Windows\\System32\\d3d11.D3DPerformance_EndEvent\"")
#pragma comment(linker, "/export:D3DPerformance_GetStatus=\"C:\\Windows\\System32\\d3d11.D3DPerformance_GetStatus\"")
#pragma comment(linker, "/export:D3DPerformance_SetMarker=\"C:\\Windows\\System32\\d3d11.D3DPerformance_SetMarker\"")

#pragma endregion