// If we're running with debug assertions, we probably want a console auto-allocated for us too.
// In that case, only compile for the windows subsystem if debug assertions are disabled (e.g. in
// the release profile).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr;

use c_types::CInt;
use gl::bindings::prelude::*;

use win32::{
    c_str, create_app_window, describe_pixel_format, do_wgl_choose_pixel_format_arb,
    do_wgl_create_context_attribs_arb, get_any_message, get_dc, get_process_handle, get_wgl_basics,
    get_window_userdata, load_library, load_predefined_cursor, post_quit_message, prelude::*,
    register_class, release_dc, set_pixel_format, set_window_userdata, translate_message,
    utf16_null, wgl_delete_context, wgl_make_current,
};

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

    // This is data to pass to the window, which the window procedure can handle in its WM_CREATE
    // or WM_NCCREATE message handlers.
    // Note that we intentionally Box::leak the data - it should be cleaned up by the window procedure
    // in is WM_DESTROY message handler.
    let lparam: *mut WindowData = Box::leak(Box::new(WindowData::default()));

    let hwnd =
        unsafe { create_app_window(WINDOW_CLASS, WINDOW_NAME, None, [800, 600], lparam.cast())? };

    // Bind a handle to the window's device context to the WindowData attached to the window.
    let hdc = unsafe { get_dc(hwnd) }.unwrap();
    unsafe { (*lparam).hdc = hdc };

    // Set the pixel format for the window.
    //
    // First, define some base criteria:
    let mut pf_int_attribs = vec![
        [WGL_DRAW_TO_WINDOW_ARB, true as _],
        [WGL_SUPPORT_OPENGL_ARB, true as _],
        [WGL_DOUBLE_BUFFER_ARB, true as _],
        [WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB],
        [WGL_COLOR_BITS_ARB, 32],
        [WGL_DEPTH_BITS_ARB, 24],
        [WGL_STENCIL_BITS_ARB, 8],
    ];

    // Additional extensions that may or may not exist:
    for ext in wgl_extensions.iter() {
        match ext.as_str() {
            // if sRGB is supported, ask for that
            "WGL_EXT_framebuffer_sRGB" => {
                pf_int_attribs.push([WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT, true as _]);
            }

            // enable multisampling if possible
            "WGL_ARB_multisample" => {
                pf_int_attribs.push([WGL_SAMPLE_BUFFERS_ARB, 1]);
            }

            _ => {}
        }
    }

    // Finalize the list of requested pixel format attributes
    pf_int_attribs.push([0, 0]);

    // Choose the pixel format, get the PIXELFORMATDESCRIPTOR, and set it
    let pix_format = unsafe {
        do_wgl_choose_pixel_format_arb(wgl_choose_pixel_format, hdc, &pf_int_attribs, &[])
    }?;
    let pfd = unsafe { describe_pixel_format(hdc, pix_format) }?;
    unsafe { set_pixel_format(hdc, pix_format, &pfd) }?;

    // Now, create a OpenGL 4.6 Core context, and give it to our window procedure for later use.
    const OPENGL_CONTEXT_FLAGS: CInt = WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB
        | if cfg!(debug_assertions) {
            WGL_CONTEXT_DEBUG_BIT_ARB
        } else {
            0
        };

    let hglrc = unsafe {
        do_wgl_create_context_attribs_arb(
            wgl_create_context_attribs,
            hdc,
            ptr::null_mut(),
            &[
                [WGL_CONTEXT_MAJOR_VERSION_ARB, 4],
                [WGL_CONTEXT_MINOR_VERSION_ARB, 6], // opengl 4.6
                [
                    WGL_CONTEXT_PROFILE_MASK_ARB,
                    WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
                ], // core profile
                [WGL_CONTEXT_FLAGS_ARB, OPENGL_CONTEXT_FLAGS],
                [0, 0],
            ],
        )
    }?;

    unsafe { wgl_make_current(hdc, hglrc) }?;
    unsafe { (*lparam).hglrc = hglrc };

    // Load the OpenGL DLL, and give the window procedure a handle to it.
    let lib_opengl32 = load_library("opengl32.dll")?;
    unsafe { (*lparam).lib_opengl32 = lib_opengl32 };
    unsafe { (*lparam).load_gl_functions() };

    // Enable "adaptive" vsync if possible, otherwise normal vsync
    if wgl_extensions
        .iter()
        .any(|s| s == "WGL_EXT_swap_control_tear")
    {
        unsafe { (wgl_swap_interval.unwrap())(-1) };
    } else {
        unsafe { (wgl_swap_interval.unwrap())(1) };
    }

    // Show the window.
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

/// Data to be stored in the window procedure's state.
struct WindowData {
    hdc: HDC,
    hglrc: HGLRC,
    lib_opengl32: HMODULE,
    gl_clear: glClear_t,
    gl_clear_color: glClearColor_t,
}

impl WindowData {
    /// Get the address of an OpenGL function from [`Self::lib_opengl32`].
    ///
    /// - `name` must be a null-terminated ASCII string. This function will panic if the string is
    ///   not null-terminated.
    pub fn gl_get_proc_address(&self, name: &[u8]) -> *mut core::ffi::c_void {
        assert!(*name.last().unwrap() == 0);

        let p = unsafe { wglGetProcAddress(name.as_ptr().cast()) };

        match p as usize {
            0 | 1 | 2 | 3 | usize::MAX => unsafe {
                GetProcAddress(self.lib_opengl32, name.as_ptr().cast())
            },
            _ => p,
        }
    }

    /// Load handles to various OpenGL functions and store them in this struct.
    ///
    /// ## Safety
    ///
    /// - [`Self::lib_opengl32`] should be a valid handle to a currently-loaded instance of `opengl32.dll`.
    /// - The functions loaded from [`Self::lib_opengl32`] *might* be null pointers. This will be
    ///   represented by [`Option::None`]. Also, if `opengl32.dll` has been modified from its expected
    ///   contents then the functions may be undetectably invalid. Call them at your own risk!
    pub unsafe fn load_gl_functions(&mut self) {
        use core::mem::transmute;

        self.gl_clear = transmute(self.gl_get_proc_address(c_str!("glClear")));
        self.gl_clear_color = transmute(self.gl_get_proc_address(c_str!("glClearColor")));
    }
}

/// Zeros out all members of this structure.
impl Default for WindowData {
    fn default() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
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
        WM_PAINT => match get_window_userdata::<WindowData>(hwnd) {
            Ok(ptr) if !ptr.is_null() => {
                let window_data = ptr.as_mut().unwrap();

                (window_data.gl_clear_color.unwrap())(0.6, 0.7, 0.8, 1.0);
                (window_data.gl_clear.unwrap())(GL_COLOR_BUFFER_BIT);

                // Do all OpenGL drawing before this line:
                SwapBuffers(window_data.hdc);

                // Immediately request a redraw:
                InvalidateRect(hwnd, ptr::null(), 0);
            }

            Ok(_) => {
                println!("GWLP_USERDATA pointer is null.");
            }

            Err(e) => {
                println!("Error while getting the GWLP_USERDATA pointer: {e}");
            }
        },

        // Destroy the window class when told to close.
        WM_CLOSE => {
            DestroyWindow(hwnd);
        }
        // Tell the system the application quit upon window class destruction.
        WM_DESTROY => {
            // Remember to clean up application state upon destruction!
            match get_window_userdata::<WindowData>(hwnd) {
                Ok(ptr) if !ptr.is_null() => {
                    let window_data = Box::from_raw(ptr);

                    FreeLibrary(window_data.lib_opengl32);

                    wgl_delete_context(window_data.hglrc)
                        .unwrap_or_else(|e| eprintln!("GL context deletion error: {e}"));

                    if !release_dc(hwnd, window_data.hdc) {
                        eprintln!("Unable to release device context.");
                    }

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
