// If we're running with debug assertions, we probably want a console auto-allocated for us too.
// In that case, only compile for the windows subsystem if debug assertions are disabled (e.g. in
// the release profile).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use triangle_from_scratch::{utf16_null, win32::*};

const WINDOW_CLASS: &str = "Sample Window Class";
const WINDOW_CLASS_WN: [u16; 20] = utf16_null!("Sample Window Class");
const WINDOW_NAME: &str = "Sample Window Name";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hinstance = get_process_handle();

    let wc = WNDCLASSW {
        lpfnWndProc: Some(window_procedure),
        hInstance: hinstance,
        lpszClassName: WINDOW_CLASS_WN.as_ptr(),
        hCursor: load_predefined_cursor(IDCursor::Arrow)?,
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        ..Default::default()
    };

    let _atom = unsafe { register_class(&wc) }?;

    // Set up our request for what we want the window's pixel format to be.
    // let pfd = PIXELFORMATDESCRIPTOR {
    //     dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
    //     iPixelType: PFD_TYPE_RGBA,
    //     cColorBits: 32,
    //     cDepthBits: 24,
    //     cStencilBits: 8,
    //     iLayerType: PFD_MAIN_PLANE,
    //     ..Default::default()
    // };

    // Get some basic WGL functions to use for context creation and vsync setting and multisampling
    // and so on
    let (wgl_extensions, wgl_choose_pixel_format, wgl_create_context_attribs, wgl_swap_interval) =
        get_wgl_basics()?;

    println!("WGL Extensions: {:?}", wgl_extensions);
    println!("> wglChoosePixelFormatARB --> {:?}", unsafe {
        core::mem::transmute::<wglChoosePixelFormatARB_t, *mut core::ffi::c_void>(
            wgl_choose_pixel_format,
        )
    });
    println!("> wglCreateContextAttribsARB --> {:?}", unsafe {
        core::mem::transmute::<wglCreateContextAttribsARB_t, *mut core::ffi::c_void>(
            wgl_create_context_attribs,
        )
    });
    println!("> wglSwapIntervalEXT --> {:?}", unsafe {
        core::mem::transmute::<wglSwapIntervalEXT_t, *mut core::ffi::c_void>(wgl_swap_interval)
    });

    // This is data to pass to the window, which the window procedure can handle in its WM_CREATE
    // or WM_NCCREATE message handlers.
    // Note that we intentionally Box::leak the data - it should be cleaned up by the window procedure
    // in is WM_DESTROY message handler.
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));

    let hwnd =
        unsafe { create_app_window(WINDOW_CLASS, WINDOW_NAME, None, [600, 400], lparam.cast())? };

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    loop {
        match get_any_message() {
            Ok(msg) => {
                if msg.message == WM_QUIT {
                    std::process::exit(msg.wParam as i32);
                }

                translate_message(&msg);

                unsafe {
                    DispatchMessageW(&msg);
                }
            }

            Err(e) => panic!("Error when fetching from message queue: {e}"),
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

            let ptr = (*createstruct).lpCreateParams as *mut i32;
            return set_window_userdata(hwnd, ptr).is_ok() as LRESULT;
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
            match get_window_userdata::<i32>(hwnd) {
                Ok(ptr) if !ptr.is_null() => {
                    println!("Current ptr: {}", *ptr);
                    *ptr += 1;
                }

                Ok(_) => {
                    println!("GWLP_USERDATA pointer is null.");
                }

                Err(e) => {
                    println!("Error while getting the GWLP_USERDATA pointer: {e}");
                }
            }

            do_some_painting_with(hwnd, |hdc, _erase_bg, target_rect| {
                // Just fill the background with the default window color
                fill_rect_with_sys_color(hdc, &target_rect, SysColor::Window)?;
                Ok(())
            })
            .unwrap_or_else(|e| println!("Error during painting: {e}"));
        }

        // Destroy the window class when told to close.
        WM_CLOSE => {
            DestroyWindow(hwnd);
        }
        // Tell the system the application quit upon window class destruction.
        WM_DESTROY => {
            // Remember to clean up application state upon destruction!
            match get_window_userdata::<i32>(hwnd) {
                Ok(ptr) if !ptr.is_null() => {
                    Box::from_raw(ptr); // by not saving the box, it immediately gets dropped and the
                                        // application state deallocated.
                    println!("Deallocated application state!");
                }

                Ok(_) => {
                    println!(
                        "GWLP_USERDATA pointer is null, so no application state cleanup required."
                    );
                }

                Err(e) => {
                    println!("Error while getting the GWLP_USERDATA pointer to clean up application state: {e}");
                }
            }

            post_quit_message(0);
        }

        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }

    0
}
