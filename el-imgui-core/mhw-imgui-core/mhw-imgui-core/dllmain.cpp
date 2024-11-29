// dllmain.cpp : 定义 DLL 应用程序的入口点。
#include <windows.h>
#include <dxgi1_4.h>

#include "eigeen_loader/API.hpp"
#include "safetyhook/safetyhook.hpp"
#include "TextureManager.h"

#include "dti_types.h"

#include "imgui_impl_dx12.h"
#include "imgui_impl_dx11.h"
#include "imgui_impl_win32.h"

using namespace elapi;

template<typename T> using ComPtr = Microsoft::WRL::ComPtr<T>;

struct FrameContext {
	ComPtr<ID3D12CommandAllocator> CommandAllocator = nullptr;
	ComPtr<ID3D12Resource> RenderTarget = nullptr;
	D3D12_CPU_DESCRIPTOR_HANDLE RenderTargetDescriptor = { 0 };
};

struct CustomFont {
	const char* Path;
	const char* Name;
	float Size;
	ImFontConfig* Config;
	const ImWchar* GlyphRanges;
	ImFont* Font;
};

static bool m_is_d3d12 = true;

bool m_is_initialized = false;
bool m_is_inside_present = false;
bool m_fonts_loaded = false;

safetyhook::InlineHook m_title_menu_ready_hook;

//safetyhook::InlineHook m_d3d_present_hook;
//safetyhook::InlineHook m_d3d_execute_command_lists_hook;
safetyhook::InlineHook m_d3d_signal_hook;
safetyhook::InlineHook m_d3d_resize_buffers_hook;

safetyhook::MidHook m_d3d_present_hook_alt;

std::unique_ptr<TextureManager> m_texture_manager;

#pragma region D3D12

ID3D12Device* m_d3d12_device = nullptr;
ComPtr<ID3D12DescriptorHeap> m_d3d12_back_buffers = nullptr;
ComPtr<ID3D12DescriptorHeap> m_d3d12_srv_heap = nullptr;
ComPtr<ID3D12GraphicsCommandList> m_d3d12_command_list = nullptr;
ID3D12CommandQueue* m_d3d12_command_queue = nullptr;
ID3D12Fence* m_d3d12_fence = nullptr;
UINT64 m_d3d12_fence_value = 0;
UINT32 m_d3d12_buffer_count = 0;
std::vector<FrameContext> m_d3d12_frame_contexts;

static constexpr u32 D3D12_DESCRIPTOR_HEAP_SIZE = TextureManager::DESCRIPTOR_HEAP_SIZE;

#pragma endregion

#pragma region D3D11

ID3D11Device* m_d3d11_device = nullptr;
ID3D11DeviceContext* m_d3d11_device_context = nullptr;
IDXGISwapChain* m_d3d11_swap_chain = nullptr;

#pragma endregion

HMODULE m_d3d12_module = nullptr;
//HMODULE m_d3d11_module = nullptr;

HWND m_game_window = nullptr;
HMODULE m_game_module = nullptr;
WNDPROC m_game_window_proc = nullptr;

HWND m_temp_window = nullptr;
WNDCLASSEX* m_temp_window_class = nullptr;

//ImGuiContext* (*m_core_initialize_imgui)(MtSize viewport_size, MtSize window_size, bool d3d12, const char* menu_key) = nullptr;
ImDrawData* (*m_core_imgui_render)() = nullptr;
//void(*m_core_render)() = nullptr;
//int(*m_core_get_custom_fonts)(CustomFont** out_fonts) = nullptr;
//void(*m_core_resolve_custom_fonts)() = nullptr;
//void* (*m_get_singleton)(const char* name) = nullptr;


// DirectXTK12 References SerializeRootSignature so we need to link this
#pragma comment(lib, "d3d12.lib")

void common_initialize();

static void title_menu_ready_hook(void* gui) {
	Logger::debug("in title_menu_ready_hook");
	// 初始化d3d
	std::thread t(common_initialize);
	m_title_menu_ready_hook.call(gui);

	t.join();

	m_title_menu_ready_hook = {};

	Logger::debug("end title_menu_ready_hook");
}

void imgui_load_fonts() {
	//if (m_fonts_loaded) {
	//	return;
	//}

	//const auto& io = *igGetIO();
	//ImFontAtlas_Clear(io.Fonts);

	//CustomFont* custom_fonts;
	//const int custom_font_count = m_core_get_custom_fonts(&custom_fonts);

	//const auto& chunk_module = NativePluginFramework::get_module<ChunkModule>();
	//const auto& default_chunk = chunk_module->request_chunk("Default");
	//const auto& roboto = default_chunk->get_file("/Resources/Roboto-Medium.ttf");
	//const auto& noto_sans_jp = default_chunk->get_file("/Resources/NotoSansJP-Regular.ttf");
	//const auto& fa6 = default_chunk->get_file("/Resources/fa-solid-900.ttf");

	//ImFontConfig* font_cfg = ImFontConfig_ImFontConfig();
	//font_cfg->FontDataOwnedByAtlas = false;
	//font_cfg->MergeMode = false;

	//ImFontAtlas_AddFontFromMemoryTTF(io.Fonts, roboto->Contents.data(), (i32)roboto->size(), 16.0f, font_cfg, nullptr);
	//font_cfg->MergeMode = true;
	//ImFontAtlas_AddFontFromMemoryTTF(io.Fonts, noto_sans_jp->Contents.data(), (i32)noto_sans_jp->size(), 18.0f, font_cfg, s_japanese_glyph_ranges);
	//ImFontAtlas_AddFontFromMemoryTTF(io.Fonts, fa6->Contents.data(), (i32)fa6->size(), 16.0f, font_cfg, icons_ranges);

	//for (int i = 0; i < custom_font_count; ++i) {
	//	auto& font = custom_fonts[i];
	//	font.Font = ImFontAtlas_AddFontFromFileTTF(io.Fonts, font.Path, font.Size, font.Config, font.GlyphRanges);
	//	Logger::debug("Loaded custom font: {} - {}", font.Name, font.Path);
	//}

	//ImFontAtlas_Build(io.Fonts);

	//ImFontConfig_destroy(font_cfg);

	//m_core_resolve_custom_fonts();

	m_fonts_loaded = true;
}

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

LRESULT my_window_proc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam) {
	if (m_is_initialized) {
		ImGui_ImplWin32_WndProcHandler(hwnd, msg, wparam, lparam);
	}
	return CallWindowProc(m_game_window_proc, hwnd, msg, wparam, lparam);
}

void SetupImGuiStyle() {
	auto style = igGetStyle();

	igStyleColorsDark(style);

	auto& colors = style->Colors;

	// Window BG
	colors[ImGuiCol_WindowBg] = ImVec4{ 0.1f, 0.105f, 0.11f, 1.0f };

	// Navigatation highlight
	colors[ImGuiCol_NavHighlight] = ImVec4{ 0.3f, 0.305f, 0.31f, 1.0f };

	// Progress Bar
	colors[ImGuiCol_PlotHistogram] = ImVec4{ 0.3f, 0.305f, 0.31f, 1.0f };

	// Headers
	colors[ImGuiCol_Header] = ImVec4{ 0.2f, 0.205f, 0.21f, 1.0f };
	colors[ImGuiCol_HeaderHovered] = ImVec4{ 0.3f, 0.305f, 0.31f, 1.0f };
	colors[ImGuiCol_HeaderActive] = ImVec4{ 0.55f, 0.5505f, 0.551f, 1.0f };

	// Buttons
	colors[ImGuiCol_Button] = ImVec4{ 0.2f, 0.205f, 0.21f, 1.0f };
	colors[ImGuiCol_ButtonHovered] = ImVec4{ 0.3f, 0.305f, 0.31f, 1.0f };
	colors[ImGuiCol_ButtonActive] = ImVec4{ 0.55f, 0.5505f, 0.551f, 1.0f };

	// Checkbox
	colors[ImGuiCol_CheckMark] = ImVec4(0.55f, 0.5505f, 0.551f, 1.0f);

	// Frame BG
	colors[ImGuiCol_FrameBg] = ImVec4{ 0.211f, 0.210f, 0.25f, 1.0f };
	colors[ImGuiCol_FrameBgHovered] = ImVec4{ 0.3f, 0.305f, 0.31f, 1.0f };
	colors[ImGuiCol_FrameBgActive] = ImVec4{ 0.55f, 0.5505f, 0.551f, 1.0f };

	// Tabs
	colors[ImGuiCol_Tab] = ImVec4{ 0.25f, 0.2505f, 0.251f, 1.0f };
	colors[ImGuiCol_TabHovered] = ImVec4{ 0.38f, 0.3805f, 0.381f, 1.0f };
	colors[ImGuiCol_TabActive] = ImVec4{ 0.28f, 0.2805f, 0.281f, 1.0f };
	colors[ImGuiCol_TabUnfocused] = ImVec4{ 0.25f, 0.2505f, 0.251f, 1.0f };
	colors[ImGuiCol_TabUnfocusedActive] = ImVec4{ 0.8f, 0.805f, 0.81f, 1.0f };

	// Resize Grip
	colors[ImGuiCol_ResizeGrip] = ImVec4{ 0.2f, 0.205f, 0.21f, 0.0f };
	colors[ImGuiCol_ResizeGripHovered] = ImVec4{ 0.3f, 0.305f, 0.31f, 1.0f };
	colors[ImGuiCol_ResizeGripActive] = ImVec4{ 0.55f, 0.5505f, 0.551f, 1.0f };

	// Title
	colors[ImGuiCol_TitleBg] = ImVec4{ 0.25f, 0.2505f, 0.251f, 1.0f };
	colors[ImGuiCol_TitleBgActive] = ImVec4{ 0.55f, 0.5505f, 0.551f, 1.0f };
	colors[ImGuiCol_TitleBgCollapsed] = ImVec4{ 0.25f, 0.2505f, 0.251f, 1.0f };

	// set font size
}

ImGuiContext* core_initialize_imgui(MtSize viewport_size, MtSize window_size, bool d3d12) {
	if (igGetCurrentContext() != nullptr) {
		return igGetCurrentContext();
	}

	ImGuiContext* context = igCreateContext(nullptr);

	ImGuiIO* io = igGetIO();

	io->ConfigFlags |= ImGuiConfigFlags_DockingEnable;

	SetupImGuiStyle();

	Logger::debug("Renderer.Initialize");

	return context;
}

//ImDrawData* core_imgui_render() {
//	igNewFrame();
//
//	// 创建一个窗口
//	igBegin("Hello, World!", NULL, 0);
//
//	igText("Hello, this is a basic ImGui window!");
//
//	if (igButton("Click Me", ImVec2(0, 0))) {
//		printf("Button clicked!\n");
//	}
//
//	// 结束窗口
//	igEnd();
//
//	igEndFrame();
//
//	igRender();
//
//	return igGetDrawData();
//}

void d3d12_initialize_imgui(IDXGISwapChain* swap_chain) {
	Logger::debug("In initialize d3d12 imgui");

	if (FAILED(swap_chain->GetDevice(IID_PPV_ARGS(&m_d3d12_device)))) {
		Logger::error("Failed to get D3D12 device in present hook");
		return;
	}

	DXGI_SWAP_CHAIN_DESC desc;
	if (FAILED(swap_chain->GetDesc(&desc))) {
		Logger::error("Failed to get DXGI swap chain description");
		return;
	}

	RECT client_rect;
	GetClientRect(desc.OutputWindow, &client_rect);

	const MtSize viewport_size = { desc.BufferDesc.Width, desc.BufferDesc.Height };
	const MtSize window_size = {
		(u32)(client_rect.right - client_rect.left),
		(u32)(client_rect.bottom - client_rect.top)
	};

	const auto context = core_initialize_imgui(viewport_size, window_size, true);

	igSetCurrentContext(context);

	imgui_load_fonts();

	CreateEvent(nullptr, FALSE, FALSE, nullptr);

	desc.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
	m_game_window = desc.OutputWindow;
	desc.Windowed = GetWindowLongPtr(desc.OutputWindow, GWL_STYLE) & WS_POPUP ? FALSE : TRUE;

	m_d3d12_buffer_count = desc.BufferCount;
	m_d3d12_frame_contexts.resize(desc.BufferCount, FrameContext{});

	constexpr D3D12_DESCRIPTOR_HEAP_DESC dp_imgui_desc = {
		.Type = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
		.NumDescriptors = D3D12_DESCRIPTOR_HEAP_SIZE,
		.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
		.NodeMask = 0
	};

	if (FAILED(m_d3d12_device->CreateDescriptorHeap(&dp_imgui_desc, IID_PPV_ARGS(m_d3d12_srv_heap.GetAddressOf())))) {
		Logger::error("Failed to create D3D12 descriptor heap for back buffers");
		return;
	}

	ComPtr<ID3D12CommandAllocator> command_allocator;
	if (FAILED(m_d3d12_device->CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT, IID_PPV_ARGS(command_allocator.GetAddressOf())))) {
		Logger::error("Failed to create D3D12 command allocator");
		return;
	}

	for (auto i = 0u; i < desc.BufferCount; ++i) {
		m_d3d12_frame_contexts[i].CommandAllocator = command_allocator;
	}

	if (FAILED(m_d3d12_device->CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT,
		command_allocator.Get(), nullptr, IID_PPV_ARGS(m_d3d12_command_list.GetAddressOf())))) {
		Logger::error("Failed to create D3D12 command list");
		return;
	}

	if (FAILED(m_d3d12_command_list->Close())) {
		Logger::error("Failed to close D3D12 command list");
		return;
	}

	const D3D12_DESCRIPTOR_HEAP_DESC back_buffer_desc = {
		.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
		.NumDescriptors = desc.BufferCount,
		.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
		.NodeMask = 1
	};

	if (FAILED(m_d3d12_device->CreateDescriptorHeap(&back_buffer_desc, IID_PPV_ARGS(m_d3d12_back_buffers.GetAddressOf())))) {
		Logger::error("Failed to create D3D12 descriptor heap for back buffers");
		return;
	}

	const auto rtv_descriptor_size = m_d3d12_device->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
	D3D12_CPU_DESCRIPTOR_HANDLE rtv_handle = m_d3d12_back_buffers->GetCPUDescriptorHandleForHeapStart();

	for (auto i = 0u; i < desc.BufferCount; ++i) {
		ComPtr<ID3D12Resource> back_buffer;
		if (FAILED(swap_chain->GetBuffer(i, IID_PPV_ARGS(back_buffer.GetAddressOf())))) {
			Logger::error("Failed to get DXGI swap chain buffer");
			return;
		}

		const auto buffer_desc = back_buffer->GetDesc();
		Logger::debug("Creating RTV for back buffer {}, with size {}x{}", i, buffer_desc.Width, buffer_desc.Height);

		m_d3d12_device->CreateRenderTargetView(back_buffer.Get(), nullptr, rtv_handle);
		m_d3d12_frame_contexts[i].RenderTargetDescriptor = rtv_handle;
		m_d3d12_frame_contexts[i].RenderTarget = back_buffer;

		rtv_handle.ptr += rtv_descriptor_size;
	}

	if (!ImGui_ImplWin32_Init(m_game_window)) {
		Logger::error("Failed to initialize ImGui Win32");
		return;
	}

	ImGui_ImplWin32_EnableDpiAwareness();

	if (!ImGui_ImplDX12_Init(m_d3d12_device, desc.BufferCount,
		DXGI_FORMAT_R8G8B8A8_UNORM, m_d3d12_srv_heap.Get(),
		m_d3d12_srv_heap->GetCPUDescriptorHandleForHeapStart(),
		m_d3d12_srv_heap->GetGPUDescriptorHandleForHeapStart())) {
		Logger::error("Failed to initialize ImGui D3D12");
		return;
	}

	if (!ImGui_ImplDX12_CreateDeviceObjects()) {
		Logger::error("Failed to create ImGui D3D12 device objects");
		return;
	}

	if (GetWindowLongPtr(m_game_window, GWLP_WNDPROC) != (LONG_PTR)my_window_proc) {
		m_game_window_proc = (WNDPROC)SetWindowLongPtr(m_game_window, GWLP_WNDPROC, (LONG_PTR)my_window_proc);
	}

	m_is_initialized = true;

	Logger::debug("Initialized D3D12");

}

void d3d12_deinitialize_imgui() {
	Logger::debug("Uninitializing D3D12 ImGui");

	ImGui_ImplDX12_Shutdown();
	ImGui_ImplWin32_Shutdown();
	m_d3d12_frame_contexts.clear();
	m_d3d12_back_buffers = nullptr;
	m_d3d12_srv_heap = nullptr;
	m_d3d12_command_list = nullptr;
	m_d3d12_command_queue = nullptr;
	m_d3d12_fence = nullptr;
	m_d3d12_fence_value = 0;
	m_d3d12_buffer_count = 0;
}

static HRESULT d3d_resize_buffers_hook(IDXGISwapChain* swap_chain, UINT buffer_count, UINT w, UINT h, DXGI_FORMAT format, UINT flags) {
	Logger::debug("ResizeBuffers called, resetting...");

	if (m_is_initialized) {
		m_is_initialized = false;
		if (m_is_d3d12) {
			d3d12_deinitialize_imgui();
		}
		//else {
		//	d3d11_deinitialize_imgui();
		//}
	}

	return m_d3d_resize_buffers_hook.call<HRESULT>(swap_chain, buffer_count, w, h, format, flags);
}

static UINT64 d3d12_signal_hook(ID3D12CommandQueue* command_queue, ID3D12Fence* fence, UINT64 value) {
	if (m_d3d12_command_queue == command_queue) {
		m_d3d12_fence = fence;
		m_d3d12_fence_value = value;
	}

	return m_d3d_signal_hook.call<UINT64>(command_queue, fence, value);
}

static void d3d12_present_hook_core(IDXGISwapChain* swap_chain) {
	const auto swap_chain3 = (IDXGISwapChain3*)swap_chain;

	// Start new frame
	ImGui_ImplDX12_NewFrame();
	ImGui_ImplWin32_NewFrame();

	ImDrawData* draw_data = m_core_imgui_render();

	const FrameContext& frame_ctx = m_d3d12_frame_contexts[swap_chain3->GetCurrentBackBufferIndex()];
	frame_ctx.CommandAllocator->Reset();

	D3D12_RESOURCE_BARRIER barrier = {
		.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
		.Flags = D3D12_RESOURCE_BARRIER_FLAG_NONE,
		.Transition = {
			.pResource = frame_ctx.RenderTarget.Get(),
			.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
			.StateBefore = D3D12_RESOURCE_STATE_PRESENT,
			.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET
		}
	};

	m_d3d12_command_list->Reset(frame_ctx.CommandAllocator.Get(), nullptr);
	m_d3d12_command_list->ResourceBarrier(1, &barrier);
	m_d3d12_command_list->OMSetRenderTargets(1, &frame_ctx.RenderTargetDescriptor, FALSE, nullptr);
	m_d3d12_command_list->SetDescriptorHeaps(1, m_d3d12_srv_heap.GetAddressOf());

	ImGui_ImplDX12_RenderDrawData(draw_data, m_d3d12_command_list.Get());

	barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
	barrier.Transition.StateAfter = D3D12_RESOURCE_STATE_PRESENT;

	m_d3d12_command_list->ResourceBarrier(1, &barrier);
	m_d3d12_command_list->Close();

	m_d3d12_command_queue->ExecuteCommandLists(1, (ID3D12CommandList* const*)m_d3d12_command_list.GetAddressOf());

	if (igGetIO()->ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
	{
		igUpdatePlatformWindows();
		igRenderPlatformWindowsDefault(nullptr, m_d3d12_command_list.Get());
	}
}

void initialize_for_d3d12_alt() {
	Logger::debug("In initialize_for_d3d12_alt");
	// D3DRender12:SwapChainPresentCall
	const uintptr_t present_call = 0x1423AA1EA;

	if ((m_d3d12_module = GetModuleHandleA("d3d12.dll")) == nullptr) {
		Logger::error("Failed to find d3d12.dll");
		return;
	}

	m_d3d_present_hook_alt = safetyhook::create_mid(present_call, [](safetyhook::Context& ctx) {
		const auto swap_chain = (IDXGISwapChain*)ctx.rcx;

		if (m_is_inside_present) {
			return;
		}

		m_is_inside_present = true;

		if (!m_is_initialized) {
			d3d12_initialize_imgui(swap_chain);

			if (!m_texture_manager) {
				m_texture_manager = std::make_unique<TextureManager>(
					m_d3d12_device,
					m_d3d12_command_queue,
					m_d3d12_srv_heap
				);
			}
		}

		if (!m_d3d12_command_queue) {
			m_is_inside_present = false;
			return;
		}

		d3d12_present_hook_core(swap_chain);
		m_is_inside_present = false;
	});

	//const auto render_singleton = (uintptr_t)m_get_singleton("sMhRender");
	const uintptr_t render_singleton = *(uintptr_t*)0x1451C4480;
	const auto renderer = *(uintptr_t*)(render_singleton + 0x78);

	m_d3d12_command_queue = *(ID3D12CommandQueue**)(renderer + 0x20);
	const auto swap_chain = *(IDXGISwapChain3**)(renderer + 0x1470);
	const auto swap_chain_vft = *(void***)swap_chain;
	const auto cmd_queue_vft = *(void***)m_d3d12_command_queue;

	const auto resize_buffers = swap_chain_vft[13];
	const auto signal = cmd_queue_vft[14];

	Logger::debug("D3D12 Command Queue found at {:p}", (void*)m_d3d12_command_queue);
	Logger::debug("resize_buffers found at {:p}", (void*)resize_buffers);
	Logger::debug("signal found at {:p}", (void*)signal);

	m_d3d_resize_buffers_hook = safetyhook::create_inline(resize_buffers, d3d_resize_buffers_hook);
	m_d3d_signal_hook = safetyhook::create_inline(signal, d3d12_signal_hook);

	Logger::debug("end initialize_for_d3d12_alt");
}

void common_initialize() {
	m_is_d3d12 = true;

	Logger::debug("D3D12 OK");

	const auto game_window_name = std::format("MONSTER HUNTER: WORLD({})", 421810);
	Logger::debug("Looking for game window: {}", game_window_name);

	m_game_window = FindWindowA(nullptr, game_window_name.c_str());
	if (!m_game_window) {
		Logger::error("Failed to find game window ({})", GetLastError());
		return;
	}

	const auto window_class = new WNDCLASSEX;
	window_class->cbSize = sizeof(WNDCLASSEX);
	window_class->style = CS_HREDRAW | CS_VREDRAW;
	window_class->lpfnWndProc = DefWindowProc;
	window_class->cbClsExtra = 0;
	window_class->cbWndExtra = 0;
	window_class->hInstance = GetModuleHandle(nullptr);
	window_class->hIcon = nullptr;
	window_class->hCursor = nullptr;
	window_class->hbrBackground = nullptr;
	window_class->lpszMenuName = nullptr;
	window_class->lpszClassName = TEXT("SharpPluginLoader");
	window_class->hIconSm = nullptr;

	if (!RegisterClassEx(window_class)) {
		Logger::error("Failed to register window class ({})", GetLastError());
		return;
	}

	m_temp_window_class = window_class;

	m_temp_window = CreateWindow(
		window_class->lpszClassName,
		TEXT("SharpPluginLoader DX Hook"),
		WS_OVERLAPPEDWINDOW,
		CW_USEDEFAULT, CW_USEDEFAULT,
		100, 100,
		nullptr,
		nullptr,
		window_class->hInstance,
		nullptr
	);

	if (!m_temp_window) {
		Logger::error("Failed to create temporary window ({})", GetLastError());
		return;
	}

	if (m_is_d3d12) {
		initialize_for_d3d12_alt();
	}
	/*else {
		initialize_for_d3d11_alt();
	}*/

	DestroyWindow(m_temp_window);
	UnregisterClass(window_class->lpszClassName, window_class->hInstance);

	Logger::debug("end common_initialize");
}

static void main_entry() {
	// GUITitle:Play
	const auto GUI_TITLE_PLAY = (void*)0x141EFFDF0;
	m_title_menu_ready_hook = safetyhook::create_inline(GUI_TITLE_PLAY, title_menu_ready_hook);

	Logger::debug("Created GUI_TITLE_PLAY hook: {:p}", GUI_TITLE_PLAY);
}

//BOOL APIENTRY DllMain(HMODULE hModule,
//	DWORD  ul_reason_for_call,
//	LPVOID lpReserved
//)
//{
//	switch (ul_reason_for_call)
//	{
//	case DLL_PROCESS_ATTACH: {
//		main_entry();
//	}
//	case DLL_THREAD_ATTACH:
//	case DLL_THREAD_DETACH:
//	case DLL_PROCESS_DETACH:
//		break;
//	}
//	return TRUE;
//}

extern "C" EL_API void LoaderVersion(Version& version) {
	Logger::debug("in LoaderVersion");
	version.major = LOADER_VERSION_MAJOR;
	version.minor = LOADER_VERSION_MINOR;
	version.patch = LOADER_VERSION_PATCH;
}

extern "C" EL_API int32_t CoreInitialize(const CoreParam& params) {
	m_core_imgui_render = params.get_method<ImDrawData * ()>("Render::core_imgui_render");
	if (!m_core_imgui_render) {
		Logger::error("Failed to get Render::core_imgui_render");
		return 1;
	}

	// do initialize
	main_entry();
}
