use windows::{
    core::{w, HSTRING},
    Win32::{
        Foundation::{HMODULE, HWND},
        System::{
            ProcessStatus::{EnumProcessModules, GetModuleInformation, MODULEINFO},
            Threading::{GetCurrentProcess, GetCurrentProcessId},
        },
        UI::WindowsAndMessaging::{
            GetForegroundWindow, GetWindowThreadProcessId, MessageBoxW, MB_ICONERROR,
        },
    },
};

/// 获取基模块的空间信息，基地址和大小
///
/// # Safety
///
/// 调用 Windows API
pub unsafe fn get_base_module_space() -> Result<(usize, usize), windows::core::Error> {
    let hprocess = GetCurrentProcess();
    let mut modules: [HMODULE; 1024] = [HMODULE::default(); 1024];
    let mut cb_needed: u32 = 0;

    EnumProcessModules(
        hprocess,
        modules.as_mut_ptr(),
        (modules.len() * std::mem::size_of::<HMODULE>()) as u32,
        &mut cb_needed,
    )?;

    let module_count = cb_needed / std::mem::size_of::<HMODULE>() as u32;
    if module_count > 0 {
        let hmodule = modules[0];
        let mut module_info = MODULEINFO::default();
        GetModuleInformation(
            hprocess,
            hmodule,
            &mut module_info,
            std::mem::size_of::<MODULEINFO>() as u32,
        )?;

        return Ok((
            module_info.lpBaseOfDll as usize,
            module_info.SizeOfImage as usize,
        ));
    }

    Ok((0, 0))
}

/// 显示错误信息对话框
pub fn message_box_fatal(message: &str) {
    let msg_str: HSTRING = message.into();
    unsafe {
        MessageBoxW(
            HWND::default(),
            &msg_str,
            w!("EigeenLoader Fatal Error"),
            MB_ICONERROR,
        )
    };
}

/// 检查当前活动窗口是否为游戏窗口
pub fn is_mhw_foreground() -> bool {
    // 获取当前前台窗口句柄
    let foreground_hwnd = unsafe { GetForegroundWindow() };
    if foreground_hwnd.0.is_null() {
        return false;
    }

    // 获取窗口所属进程ID
    let mut window_pid = 0;
    unsafe {
        GetWindowThreadProcessId(foreground_hwnd, Some(&mut window_pid));
    };
    if window_pid == 0 {
        return false;
    }

    // 获取当前进程ID
    // 伪句柄无需关闭
    let current_pid = unsafe { GetCurrentProcessId() };

    window_pid == current_pid
}
