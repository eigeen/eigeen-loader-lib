use std::{cell::RefCell, sync::LazyLock};

use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{FALSE, HMODULE, HWND},
        System::{
            ProcessStatus::{EnumProcessModules, GetModuleInformation, MODULEINFO},
            Threading::{GetCurrentProcess, GetCurrentProcessId},
        },
        UI::WindowsAndMessaging::{
            FindWindowW, GetForegroundWindow, GetWindowThreadProcessId, MessageBoxW,
            SetForegroundWindow, MB_ICONERROR,
        },
    },
};

type Result<T, E = WindowsError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum WindowsError {
    #[error("Windows Error: {0}")]
    Raw(#[from] windows::core::Error),
    #[error("{0}: {1}")]
    WithContext(&'static str, #[source] windows::core::Error),
    #[error("{0}")]
    Other(String),
}

/// 获取基模块的空间信息，基地址和大小
///
/// # Safety
///
/// 调用 Windows API
pub unsafe fn get_base_module_space() -> Result<(usize, usize)> {
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

static mut MHW_MAIN_WINDOW: LazyLock<RefCell<HWND>> =
    LazyLock::new(|| RefCell::new(HWND::default()));

/// 获取游戏主窗口句柄
pub fn get_mhw_main_window() -> Result<HWND> {
    unsafe {
        if MHW_MAIN_WINDOW.borrow().is_invalid() {
            let Some(revision) = super::game::get_game_revision() else {
                return Err(WindowsError::Other(
                    "Failed to get game revision".to_string(),
                ));
            };

            let window_name = format!("MONSTER HUNTER: WORLD({})", revision);
            let window_name_w = super::string::to_wstring_bytes_with_nul(&window_name);

            const MHW_WINDOW_CLASS: &str = "MT FRAMEWORK";
            let window_class_w = super::string::to_wstring_bytes_with_nul(MHW_WINDOW_CLASS);

            log::trace!(
                "trying to find window: class: {}, title: {}",
                MHW_WINDOW_CLASS,
                window_name
            );

            let hwnd = FindWindowW(
                PCWSTR::from_raw(window_class_w.as_ptr()),
                PCWSTR::from_raw(window_name_w.as_ptr()),
            )
            .map_err(|e| WindowsError::WithContext("FindWindowW failed", e))?;
            *MHW_MAIN_WINDOW.borrow_mut() = hwnd;
        }

        let hwnd = *MHW_MAIN_WINDOW.borrow();
        if hwnd.is_invalid() {
            return Err(WindowsError::Other(
                "Failed to find MHW main window".to_string(),
            ));
        }

        Ok(hwnd)
    }
}

/// 尝试将焦点转移到游戏窗口
pub fn focus_mhw_main_window() -> Result<()> {
    let hwnd = get_mhw_main_window()?;

    let status = unsafe { SetForegroundWindow(hwnd) };
    if status == FALSE {
        return Err(WindowsError::Other(
            "SetForegroundWindow failed".to_string(),
        ));
    }

    Ok(())
}
