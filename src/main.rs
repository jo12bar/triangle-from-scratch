// If we're running with debug assertions, we probably want a console auto-allocated for us too.
// In that case, only compile for the windows subsystem if debug assertions are disabled (e.g. in
// the release profile).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr;
use triangle_from_scratch::{wide_null, win32::*};

fn main() {
    let hinstance = get_process_handle();
    let sample_window_class_wn = wide_null("Sample Window Class");

    let wc = WNDCLASSW {
        lpfnWndProc: Some(window_procedure),
        hInstance: hinstance,
        lpszClassName: sample_window_class_wn.as_ptr(),
        hCursor: unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
        ..Default::default()
    };

    let atom = unsafe { RegisterClassW(&wc) };

    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code {last_error:#x}",);
    }

    let sample_window_name_wn = wide_null("Sample Window Name");

    // This is data to pass to the window, which the window procedure can handle in its WM_CREATE
    // or WM_NCCREATE message handlers.
    // Note that we intentionally Box::leak the data - it should be cleaned up by the window procedure
    // in is WM_DESTROY message handler.
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            hinstance,
            lparam.cast(),
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG::default();
    loop {
        let message_return = unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) };

        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageW`, error code: {last_error:#x}");
        }

        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

/// The main window procedure.
///
/// # Safety
///
/// - Runs in a different thread (potentially)
/// - Uses weird calling conventions
/// - Hates you
pub unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        // This message is sent *just* before the WM_CREATE message, when the window is first created.
        // If window creation should procede, return TRUE (1). Otherwise, return FALSE (0).
        WM_NCCREATE => {
            println!("NC Create");

            // We expect to receive window creation data in l_param when processing this message. If
            // we don't receive it, disallow window creation and exit.
            let createstruct: LPCREATESTRUCTW = lparam as _;
            if createstruct.is_null() {
                return 0;
            }

            let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams.cast();
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);

            return 1;
        }

        // The window is being created. Application state should be setup here.
        //
        // Per MSDN:
        //
        // > If an application processes this message, it should return zero to continue creation of
        // > the window. If the application returns –1, the window is destroyed and the
        // > CreateWindowEx or CreateWindow function returns a NULL handle.
        WM_CREATE => {
            println!("Create");
        }

        // Paint the window's client area.
        WM_PAINT => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;
            println!("Current ptr: {}", *ptr);
            *ptr += 1;

            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            // Just fill the background with the default window color
            let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);

            EndPaint(hwnd, &ps);
        }

        // Destroy the window class when told to close.
        WM_CLOSE => {
            DestroyWindow(hwnd);
        }
        // Tell the system the application quit upon window class destruction.
        WM_DESTROY => {
            // Remember to clean up application state upon destruction!
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;

            Box::from_raw(ptr); // by not saving the box, it immediately gets dropped and the
                                // application state deallocated.
            println!("Deallocated application state!");

            PostQuitMessage(0);
        }

        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }

    0
}
