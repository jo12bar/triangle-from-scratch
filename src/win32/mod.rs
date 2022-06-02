//! Bindings to Win32 structs, types, and functions.

// Win32 names are very incompatible with Rust and Clippy's default lints, so
// we have to disable some of them.
#![allow(clippy::upper_case_acronyms, non_snake_case, non_camel_case_types)]

use core::ptr;

pub mod c_macros;
pub mod constants;
pub mod extern_bindings;
pub mod prelude;
pub mod structs;
pub mod typedefs;

use prelude::*;

use crate::{
    c_str, gather_null_terminated_bytes, str_util::min_alloc_lossy_into_string, utf16_null,
    wide_null,
};

// Prepares the specified window for painting.
///
/// On success: you get back both the [`HDC`] and [`PAINTSTRUCT`] that you'll need for future
/// painting calls (including [`EndPaint`]).
///
/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
///
/// ## Safety
///
/// `hwnd` must be a valid handle to a window.
pub unsafe fn begin_paint(hwnd: HWND) -> Result<(HDC, PAINTSTRUCT), Win32Error> {
    let mut ps = PAINTSTRUCT::default();
    let hdc = BeginPaint(hwnd, &mut ps);
    if hdc.is_null() {
        Err(get_last_error())
    } else {
        Ok((hdc, ps))
    }
}

/// Creates a window, providing semi-sane defaults.
///
/// ## Safety
///
/// This requires valid pointers in _all_ structs associated with window creation. Use at your own
/// risk!
pub unsafe fn create_app_window(
    class_name: &str,
    window_name: &str,
    position: Option<[i32; 2]>,
    [width, height]: [i32; 2],
    create_param: LPVOID,
) -> Result<HWND, Win32Error> {
    let [x, y] = position.unwrap_or([CW_USEDEFAULT, CW_USEDEFAULT]);

    create_window_ex_w(
        0,
        wide_null(class_name).as_ptr(),
        wide_null(window_name).as_ptr(),
        WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
        x,
        y,
        width,
        height,
        ptr::null_mut(),
        ptr::null_mut(),
        get_process_handle(),
        create_param,
    )
}

/// Creates a window.
///
/// See [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
///
/// ## Safety
///
/// This is basically just a wrapper around `CreateWindowExW`. No attempt is made to validate any
/// arguments. So read MSDN to make sure you're using this function properly!
#[allow(clippy::too_many_arguments)]
pub unsafe fn create_window_ex_w(
    ex_style: DWORD,
    class_name: LPCWSTR,
    window_name: LPCWSTR,
    style: DWORD,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    parent: HWND,
    menu: HMENU,
    instance: HINSTANCE,
    param: LPVOID,
) -> Result<HWND, Win32Error> {
    let hwnd = CreateWindowExW(
        ex_style,
        class_name,
        window_name,
        style,
        x,
        y,
        width,
        height,
        parent,
        menu,
        instance,
        param,
    );
    if hwnd.is_null() {
        Err(get_last_error())
    } else {
        Ok(hwnd)
    }
}

/// Chooses a pixel format for a window. This is part of the process for enabling OpenGL rendering
/// on a window.
///
/// ## Safety
///
/// - `hdc` must be a valid handle to a device context.
/// - If `ppfd` isn't a valid [`PIXELFORMATDESCRIPTOR`], undefined behaviour may happen.
///
/// **See**: [`ChoosePixelFormat()`]
pub unsafe fn choose_pixel_format(
    hdc: HDC,
    ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<c_int, Win32Error> {
    let index = ChoosePixelFormat(hdc, ppfd);
    if index != 0 {
        Ok(index)
    } else {
        Err(get_last_error())
    }
}

/// Gets the pixel format info for a given pixel format index.
///
/// ## Safety
///
/// - `hdc` must be a valid handle to a DC.
/// - `format` must be a valid pixel format index, not exceeding the maximum value returned by
///   [`get_max_pixel_format_index()`].
///
/// **See**: [`DescribePixelFormat()`]
pub unsafe fn describe_pixel_format(
    hdc: HDC,
    format: c_int,
) -> Result<PIXELFORMATDESCRIPTOR, Win32Error> {
    let mut pfd = PIXELFORMATDESCRIPTOR::default();
    let max_index = DescribePixelFormat(
        hdc,
        format,
        core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
        &mut pfd,
    );

    if max_index == 0 {
        Err(get_last_error())
    } else {
        Ok(pfd)
    }
}

/// Destroys a window.
///
/// ## Safety
///
/// - `hwnd` must be a valid handle to a window.
///
/// **See**: [`DestroyWindow`]
pub unsafe fn destroy_window(hwnd: HWND) -> Result<(), Win32Error> {
    let destroyed = DestroyWindow(hwnd);
    if destroyed != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

/// Paint on a device context. This function calls [`begin_paint()`] and [`end_paint()`] around your
/// closure, so you don't have to.
///
/// ## Safety
///
/// `hwnd` must be a valid handle to a window.
pub unsafe fn do_some_painting_with<F, T>(hwnd: HWND, f: F) -> Result<T, Win32Error>
where
    F: FnOnce(HDC, bool, RECT) -> Result<T, Win32Error>,
{
    let (hdc, ps) = begin_paint(hwnd)?;
    let output = f(hdc, ps.fErase != 0, ps.rcPaint);
    end_paint(hwnd, &ps);
    output
}

/// Arranges data for calling a [`wglChoosePixelFormatARB_t`] procedure, and calls it.
///
/// - Inputs are slices of `[key, value]` pairs.
/// - Input slices **can** be empty.
/// - Non-empty slices must have a zero value in the key position of the final pair.
///
/// ## Safety
///
/// - `f` must be a valid nullable pointer to the `wglChoosePixelFormatARB` function.
/// - `hdc` must be a valid handle to a device context.
pub unsafe fn do_wgl_choose_pixel_format_arb(
    f: wglChoosePixelFormatARB_t,
    hdc: HDC,
    int_attrs: &[[c_int; 2]],
    float_attrs: &[[FLOAT; 2]],
) -> Result<c_int, Win32Error> {
    const APP_ERR: Win32Error = Win32Error(Win32Error::APPLICATION_ERROR_BIT);

    let i_ptr = match int_attrs.last() {
        Some([k, _v]) => {
            if *k == 0 {
                int_attrs.as_ptr()
            } else {
                return Err(APP_ERR);
            }
        }

        None => ptr::null(),
    };

    let f_ptr = match float_attrs.last() {
        Some([k, _v]) => {
            if *k == 0.0 {
                float_attrs.as_ptr()
            } else {
                return Err(APP_ERR);
            }
        }

        None => ptr::null(),
    };

    let mut out_format = 0;
    let mut out_format_count = 0;

    let b = (f.ok_or(APP_ERR)?)(
        hdc,
        i_ptr.cast(),
        f_ptr.cast(),
        1,
        &mut out_format,
        &mut out_format_count,
    );

    if b != 0 && out_format_count == 1 {
        Ok(out_format)
    } else {
        Err(get_last_error())
    }
}

/// Arranges data for calling a [`wglCreateContextAttribsARB_t`] procedure, and calls it.
///
/// - The input slice consists of [key, value] pairs.
/// - The input slice **can** be empty.
/// - Any non-empty input must have zero as the key value of the last position.
///
/// ## Safety
///
/// - `f` must be a valid nullable pointer to the `wglChoosePixelFormatARB` function.
/// - `hdc` must be a valid handle to a device context.
/// - `hshare_context` must be a valid handle to a GL context.
///
/// **See**: [`WGL_ARB_create_context`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt)
pub unsafe fn do_wgl_create_context_attribs_arb(
    f: wglCreateContextAttribsARB_t,
    hdc: HDC,
    hshare_context: HGLRC,
    attribList: &[[i32; 2]],
) -> Result<HGLRC, Win32Error> {
    const APP_ERR: Win32Error = Win32Error(Win32Error::APPLICATION_ERROR_BIT);
    let i_ptr = match attribList.last() {
        Some([k, _v]) => {
            if *k == 0 {
                attribList.as_ptr()
            } else {
                return Err(APP_ERR);
            }
        }

        None => ptr::null(),
    };

    let hglrc = (f.ok_or(APP_ERR)?)(hdc, hshare_context, i_ptr.cast());
    if hglrc.is_null() {
        Err(get_last_error())
    } else {
        Ok(hglrc)
    }
}

/// **See:** [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
///
/// ## Safety
///
/// `hwnd` must be a valid handle to a window, and `ps` must be a valid [`PAINTSTRUCT`] obtained
/// from a previous call to, e.g., [`begin_paint()`].
pub unsafe fn end_paint(hwnd: HWND, ps: &PAINTSTRUCT) {
    EndPaint(hwnd, ps);
}

/// Fills a rectangle with the given system color.
///
/// When filling the specified rectangle, this does **not** include the rectangle's right and
/// bottom sides. GDI fills a rectangle up to, but not including, the right column and bottom row,
/// regardless of the current mapping mode.
///
/// **See:** [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
///
/// ## Safety
///
/// `hdc` must be a valid handle to a device context (DC).
pub unsafe fn fill_rect_with_sys_color(
    hdc: HDC,
    rect: &RECT,
    color: SysColor,
) -> Result<(), Win32Error> {
    if FillRect(hdc, rect, (color as u32 + 1) as HBRUSH) != 0 {
        Ok(())
    } else {
        // FillRect doesn't return an actual error code, so we just return Error 0 as a stand-in.
        Err(Win32Error(0))
    }
}

/// Gets a message from the thread's message queue.
///
/// The message can be for any window from this thread, or it can be a non-window message as well.
///
/// See [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
#[inline(always)]
pub fn get_any_message() -> Result<MSG, Win32Error> {
    let mut msg = MSG::default();
    // Safety: This shouldn't crash the program
    let output = unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) };
    if output == -1 {
        Err(get_last_error())
    } else {
        Ok(msg)
    }
}

/// Gets a handle to a window's DC.
///
/// ## Safety
///
/// - `hwnd` must be a valid handle to a window.
///
/// **See**: [`GetDC()`], [`release_dc()`].
pub unsafe fn get_dc(hwnd: HWND) -> Option<HDC> {
    let hdc = GetDC(hwnd);
    if hdc.is_null() {
        None
    } else {
        Some(hdc)
    }
}

/// Gets the thread-local last-error code value.
///
/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
    // Safety: per MSDN, this should always work.
    Win32Error(unsafe { GetLastError() })
}

/// Gets the maximum pixel format index for the HDC.
///
/// Pixel format indexes are 1-based.
///
/// To print out info on all the pixel formats you'd do something like this:
/// ```no_run
/// # use triangle_from_scratch::win32::*;
/// let hdc = todo!("create a window to get an HDC");
/// let max = unsafe { get_max_pixel_format_index(hdc).unwrap() };
/// for index in 1..=max {
///   let pfd = unsafe { describe_pixel_format(hdc, index).unwrap() };
///   todo!("print the pfd info you want to know");
/// }
/// ```
///
/// ## Safety
///
/// - `hdc` must be a valid handle to a DC.
///
/// **See**: [`describe_pixel_format()`]
pub unsafe fn get_max_pixel_format_index(hdc: HDC) -> Result<c_int, Win32Error> {
    let max_index = DescribePixelFormat(
        hdc,
        1,
        core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as _,
        ptr::null_mut(),
    );

    if max_index == 0 {
        Err(get_last_error())
    } else {
        Ok(max_index)
    }
}

/// Returns a handle to the file used to create the calling process (.exe file).
///
/// See [`GetModuleHandleW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew).
pub fn get_process_handle() -> HMODULE {
    // Safety: as per the MSDN docs, passing a nullptr to this function returns the ifle used to
    // create the calling process.
    unsafe { GetModuleHandleW(ptr::null()) }
}

/// Gets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The userdata pointer.
///
/// **See:** [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw).
///
/// ## Safety
///
/// There is no guarantee that the "userdata" pointer is _actually_ of type `T`. As noted in the
/// documentation for [`set_window_userdata()`], consider using a tagged struct to differentiate
/// datatypes at runtime.
pub unsafe fn get_window_userdata<T>(hwnd: HWND) -> Result<*mut T, Win32Error> {
    set_last_error(Win32Error(0));

    let out = GetWindowLongPtrW(hwnd, GWLP_USERDATA);

    if out == 0 {
        // If the output is 0, it's only a _real_ error if the last_error is non-zero.
        let last_error = get_last_error();
        if last_error.0 != 0 {
            Err(last_error)
        } else {
            Ok(out as *mut T)
        }
    } else {
        Ok(out as *mut T)
    }
}

/// Get the basic list of GL extensions and pointers to essential WGL functions that we need to do
/// more complex OpenGL work.
///
/// Creates a fake window with the proper [`PIXELFORMATDESCRIPTOR`] and uses it to create an OpenGL 1.1
/// context. The list of possible extensions is gotten, and then pointers to three essential WGL
/// functions. Then the OpenGL context is destroyed and the window is destroyed.
pub fn get_wgl_basics() -> Result<
    (
        Vec<String>,
        wglChoosePixelFormatARB_t,
        wglCreateContextAttribsARB_t,
        wglSwapIntervalEXT_t,
    ),
    Win32Error,
> {
    const FAKE_WINDOW_CLASS: &str =
        "Fake Window Class That Is Unlikely To Clash 1239429384asdhakjsdh12389eh";
    const FAKE_WINDOW_CLASS_WN: [u16; 72] =
        utf16_null!("Fake Window Class That Is Unlikely To Clash 1239429384asdhakjsdh12389eh");

    let instance = get_process_handle();

    let wc = WNDCLASSW {
        style: CS_OWNDC,
        lpfnWndProc: Some(DefWindowProcW),
        hInstance: get_process_handle(),
        lpszClassName: FAKE_WINDOW_CLASS_WN.as_ptr(),
        ..Default::default()
    };

    let pfd = PIXELFORMATDESCRIPTOR {
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cDepthBits: 24,
        cStencilBits: 8,
        iLayerType: PFD_MAIN_PLANE,
        ..Default::default()
    };

    /// Unregisters the window class on drop
    struct OnDropUnregisterClassW(ATOM, HINSTANCE);
    impl Drop for OnDropUnregisterClassW {
        fn drop(&mut self) {
            let _ = unsafe { unregister_class_by_atom(self.0, self.1) };
        }
    }
    let _atom = OnDropUnregisterClassW(unsafe { register_class(&wc) }?, instance);

    /// Destroys the window on drop
    struct OnDropDestroyWindow(HWND);
    impl Drop for OnDropDestroyWindow {
        fn drop(&mut self) {
            let _ = unsafe { destroy_window(self.0) };
        }
    }
    let hwnd = OnDropDestroyWindow(unsafe {
        create_app_window(
            FAKE_WINDOW_CLASS,
            "Fake Window",
            None,
            [1, 1],
            ptr::null_mut(),
        )
    }?);

    /// Releases the DC on drop
    struct OnDropReleaseDC(HWND, HDC);
    impl Drop for OnDropReleaseDC {
        fn drop(&mut self) {
            let _ = unsafe { release_dc(self.0, self.1) };
        }
    }
    let hdc = OnDropReleaseDC(
        hwnd.0,
        unsafe { get_dc(hwnd.0) }.ok_or(Win32Error(Win32Error::APPLICATION_ERROR_BIT))?,
    );

    // Set the pixel format
    let pf_index = unsafe { choose_pixel_format(hdc.1, &pfd) }?;
    unsafe { set_pixel_format(hdc.1, pf_index, &pfd) }?;

    // Create a fake OpenGL 1.1 context so we can get a better OpenGL context for later use.

    /// Deletes the GL context on drop
    struct OnDropDeleteContext(HGLRC);
    impl Drop for OnDropDeleteContext {
        fn drop(&mut self) {
            let _ = unsafe { wgl_delete_context(self.0) };
        }
    }
    let hglrc = OnDropDeleteContext(unsafe { wgl_create_context(hdc.1) }?);

    unsafe { wgl_make_current(hdc.1, hglrc.0) }?;

    // Get the list of WGL extensions available
    let wgl_extensions: Vec<String> = unsafe { wgl_get_extension_string_arb(hdc.1) }
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();

    // Map some WGL functions
    let choose_pixel_format: wglChoosePixelFormatARB_t =
        unsafe { core::mem::transmute(wgl_get_proc_address(c_str!("wglChoosePixelFormatARB"))?) };
    let create_context_attribs: wglCreateContextAttribsARB_t = unsafe {
        core::mem::transmute(wgl_get_proc_address(c_str!("wglCreateContextAttribsARB"))?)
    };
    let swap_interval: wglSwapIntervalEXT_t =
        unsafe { core::mem::transmute(wgl_get_proc_address(c_str!("wglSwapIntervalEXT"))?) };

    // Unbind the GL context from this thread
    unsafe { wgl_make_current(ptr::null_mut(), ptr::null_mut()) }?;

    Ok((
        wgl_extensions,
        choose_pixel_format,
        create_context_attribs,
        swap_interval,
    ))
}

/// Load a dynamic library.
///
/// Use [`FreeLibrary`] to unload the library, and [`GetProcAddress`] to get the addresses of
/// symbols in the library.
///
/// See [MSDN's documentation for `LoadLibraryW`][msdn-loader-doc] for details of how to specify
/// library names/locations, and how to influence the library search strategy.
///
/// **See**: [`LoadLibraryW`]
///
/// [msdn-loader-doc]: https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw
pub fn load_library(name: &str) -> Result<HMODULE, Win32Error> {
    let name_wn = wide_null(name);

    // Safety: The input pointer is guaranteed to be a null-terminated UTF-16 string
    let hmodule = unsafe { LoadLibraryW(name_wn.as_ptr()) };

    if hmodule.is_null() {
        Err(get_last_error())
    } else {
        Ok(hmodule)
    }
}

/// Load one of the predefined Windows cursors. If loading the cursor fails,
/// `Err(Win32Error)` is returned.
///
/// See [`LoadCursorW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw).
pub fn load_predefined_cursor(cursor: IDCursor) -> Result<HCURSOR, Win32Error> {
    // Safety: The enum only allows cursor values from the predefined list. See MSDN.
    let hcursor = unsafe { LoadCursorW(ptr::null_mut(), MAKEINITRESOURCEW(cursor as WORD)) };
    if hcursor.is_null() {
        Err(get_last_error())
    } else {
        Ok(hcursor)
    }
}

/// Indicates to the system that a thread has made a request to terminate (quit).
///
/// The exit code becomes the `wparam` of the [`WM_QUIT`] message your message loop eventually gets.
///
/// **See:** [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
pub fn post_quit_message(exit_code: c_int) {
    unsafe { PostQuitMessage(exit_code) }
}

/// Registers a window class struct. If registration fails, `Err(Win32Error)` is
/// returned.
///
/// ## Safety
///
/// All pointers in the struct's fields *must* be valid.
///
/// See [`RegisterClassW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw).
pub unsafe fn register_class(window_class: &WNDCLASSW) -> Result<ATOM, Win32Error> {
    let atom = RegisterClassW(window_class);
    if atom == 0 {
        Err(get_last_error())
    } else {
        Ok(atom)
    }
}

/// Releases a handle to a window's DC. Returns the result of attempting to release the handle;
/// `true` if successful, and `false` otherwise.
///
/// ## Safety
///
/// - `hwnd` must be a valid handle to a window.
/// - `hdc` must be a valid handle to a DC owned by the window that `hdc` points to.
///
/// **See**: [`ReleaseDC()`], [`get_dc()`]
#[must_use]
pub unsafe fn release_dc(hwnd: HWND, hdc: HDC) -> bool {
    let was_released = ReleaseDC(hwnd, hdc);
    was_released != 0
}

///
/// See [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
pub fn set_last_error(e: Win32Error) {
    unsafe { SetLastError(e.0) }
}

/// Sets the pixel format of a window and its device context.
///
/// ## Safety
///
/// - `hdc` must be a valid pointer to a device context. If this is a window's DC, then the pixel
///   format of the window will be set.
/// - `format` must be a valid pixel format index generated by, e.g., [`choose_pixel_format()`].
/// - `ppfd` must be a valid [`PIXELFORMATDESCRIPTOR`].
/// - You can't set a window's pixel format more than once, so don't try to do that.
/// - Call this *before* creating an OpenGL context.
/// - OpenGL windows should use [`WS_CLIPCHILDREN`] and [`WS_CLIPSIBLINGS`].
/// - OpenGL windows should _not_ use `CS_PARENTDC`.
///
/// **See**: [`SetPixelFormat()`], [`choose_pixel_format()`].
pub unsafe fn set_pixel_format(
    hdc: HDC,
    format: c_int,
    ppfd: &PIXELFORMATDESCRIPTOR,
) -> Result<(), Win32Error> {
    let success = SetPixelFormat(hdc, format, ppfd);
    if success != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

/// Sets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The previous userdata pointer.
///
/// **See:** [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
///
/// ## Safety
///
/// If successful, this will set the window's "userdata" pointer to arbitrary data of an arbitrary
/// type. Windows doesn't provide a built-in way to detect the type of the data - consider using
/// something like a tagged struct to get around that. Also, be aware that race conditions *could*
/// potentially occur if this function is called simultaneously from different threads.
///
/// Finally, note that the previous userdata data pointer returned by this function might not
/// _actually_ be of type `T`. Again, tagged structs might be a good solution here.
pub unsafe fn set_window_userdata<T>(hwnd: HWND, ptr: *mut T) -> Result<*mut T, Win32Error> {
    set_last_error(Win32Error(0));

    let out = SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR);

    if out == 0 {
        // If the output is 0, it's only a _real_ error if the last_error is non-zero.
        let last_error = get_last_error();
        if last_error.0 != 0 {
            Err(last_error)
        } else {
            Ok(out as *mut T)
        }
    } else {
        Ok(out as *mut T)
    }
}

/// Translates virtual-key messages into character messages.
///
/// The character messages go into your thread's message queue, and you'll see them if you continue
/// to consume messages.
///
/// **Returns:**
///
/// - `true` if the message was `WM_KEYDOWN`, `WM_KEYUP`, `WM_SYSKEYDOWN`, or
///   `WM_SYSKEYUP`.
/// - `true` for any other message type that generated a character message.
/// - otherwise `false`
///
/// See [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage).
pub fn translate_message(msg: &MSG) -> bool {
    // Safety: TranslateMessage can't really go wrong, assuming `msg` is valid
    0 != unsafe { TranslateMessage(msg) }
}

/// Un-registers the window class from the [`HINSTANCE`] given.
///
/// - The name must be the name of a registered window class.
/// - This requires re-encoding the name to a null-terminated UTF-16 string, which allocates.
///   There are some alternatives to this function that avoid allocation:
///   - If you have the atom returned by [`register_class()`], use [`unregister_class_by_atom()`]
///     instead.
///   - If you already have the window class name encoded as a null-terminated UTF-16 string, then
///     use [`unregister_class_by_name_wn()`].
/// - Before calling this function, an application must destroy all windows created with the
///   specified class.
///
/// ## Safety
///
/// - `instance` must be a valid [`HINSTANCE`].
///
/// **See**: [`UnregisterClassW()`]
pub unsafe fn unregister_class_by_name(name: &str, instance: HINSTANCE) -> Result<(), Win32Error> {
    let name_null = wide_null(name);
    unregister_class_by_name_wn(&name_null, instance)
}

/// Un-registers the window class from the [`HINSTANCE`] given.
///
/// - The name must be the name of a registered window class.
/// - Before calling this function, an application must destroy all windows created with the
///   specified class.
///
/// ## Safety
///
/// - `name_wn` must be a null-terminated UTF-16 string.
/// - `instance` must be a valid [`HINSTANCE`].
///
/// **See**: [`UnregisterClassW()`]
pub unsafe fn unregister_class_by_name_wn(
    name_wn: &[u16],
    instance: HINSTANCE,
) -> Result<(), Win32Error> {
    let out = UnregisterClassW(name_wn.as_ptr(), instance);
    if out != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

/// Un-registers the window class from the [`HINSTANCE`] given.
///
/// - The atom must be the atom of a registered window class.
/// - Before calling this function, an application must destroy all windows created with the
///   specified class.
///
/// ## Safety
///
/// - `instance` must be a valid [`HINSTANCE`].
///
/// **See**: [`UnregisterClassW()`]
pub unsafe fn unregister_class_by_atom(a: ATOM, instance: HINSTANCE) -> Result<(), Win32Error> {
    let out = UnregisterClassW(a as LPCWSTR, instance);
    if out != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

/// Create an OpenGL 1.1 context.
///
/// ## Safety
///
/// - `hdc` must be a valid handle to a device context.
///
/// **See**: [`wglCreateContext()`]
pub unsafe fn wgl_create_context(hdc: HDC) -> Result<HGLRC, Win32Error> {
    let hglrc = wglCreateContext(hdc);
    if hglrc.is_null() {
        Err(get_last_error())
    } else {
        Ok(hglrc)
    }
}

/// Deletes an OpenGL context.
///
/// ## Safety
///
/// - You **cannot** use this to delete a context currently in use in another thread.
/// - You **can** use this to delete the current thread's context. The context will be made
///   not-current automatically before it is deleted.
/// - `hglrc` must be a valid handle to an OpenGL 1.1 context.
///
/// **See**: [`wglDeleteContext()`]
pub unsafe fn wgl_delete_context(hglrc: HGLRC) -> Result<(), Win32Error> {
    let success = wglDeleteContext(hglrc);
    if success != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}

/// Gets the WGL extension string for the HDC passed.
///
/// - This relies on [`wgl_get_proc_address`], so you must have a GL context current for it to work.
/// - If [`wgl_get_proc_address`] fails, then an Application Error is generated.
/// - If [`wgl_get_proc_address`] succeeds but the extension string can't be obtained for some other
///   reason, a System Error will be generated.
///
/// The output is a space-seperated list of extensions that are supported.
///
/// ## Safety
///
/// - `hdc` must be a valid handle to a device context.
///
/// **See**:
/// [`wglGetExtensionsStringARB`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_extensions_string.txt)
pub unsafe fn wgl_get_extension_string_arb(hdc: HDC) -> Result<String, Win32Error> {
    let f: wglGetExtensionsStringARB_t =
        core::mem::transmute(wgl_get_proc_address(c_str!("wglGetExtensionsStringARB"))?);

    let p: *const u8 = (f.ok_or(Win32Error(Win32Error::APPLICATION_ERROR_BIT))?)(hdc).cast();

    if p.is_null() {
        Err(get_last_error())
    } else {
        let bytes = gather_null_terminated_bytes(p);
        Ok(min_alloc_lossy_into_string(bytes))
    }
}

/// Gets a OpenGL function address.
///
/// The input should be a null-terminated function name string. Use the [`c_str!`][`super::c_str!`]
/// macro for assistance.
///
/// - You must always have an active GL context for this to work. Otherwise you will always get an
///   error.
/// - The GL function name is case sensitive, and spelling must be exact.
/// - All outputs are context-specific. FUnctions supported in one rendering context are not
///   necessarily supported in another.
/// - The extension function addresses are unique for each pixel format. All rendering contexts of
///   a given pixel format share the same extension function addresses.
///
/// This *will not* return function pointers exported by `OpenGL32.dll`, meaning that it won't
/// return OpenGL 1.1 functions. For those old functions, use [`GetProcAddress`][msdn-getprocaddress].
///
/// ## Safety
///
/// Calling this function is not unneccessarily unsafe. However, using the pointer returned by this
/// function _is_.
///
/// The result of this function is essentially a mutable null pointer pointing at some function
/// in some binary somewhere. The arguments and return value can only be what you expect if the
/// name you provide this function is correct. Remember to use [`core::mem::transmute()`] to case the
/// pointer into a rust function pointer that you can actually call!
///
/// [msdn-getprocaddress]: https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress
pub fn wgl_get_proc_address(func_name: &[u8]) -> Result<PROC, Win32Error> {
    // check that we end the slice with a \0 as expected
    match func_name.last() {
        Some(b'\0') => (),
        _ => return Err(Win32Error(Win32Error::APPLICATION_ERROR_BIT)),
    }

    // Safety: we've already checked that teh end of the slice is null-terminated
    let proc = unsafe { wglGetProcAddress(func_name.as_ptr().cast()) };

    match proc as usize {
        // Some non-zero values can also be errors,
        // https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions#Windows
        0 | 1 | 2 | 3 | usize::MAX => Err(get_last_error()),

        _ => Ok(proc),
    }
}

/// Makes a given [`HGLRC`] current in the thread and targets it at the [`HDC`] given.
///
/// - You can safely pass [`ptr::null_mut()`] for both parameters if you wish to make no context
///   current in the thread.
///
/// ## Safety
///
/// - Unless if both parameters are [`ptr::null_mut()`]:
///   - `hdc` must be a valid handle to a device context
///   - `hglrc` must be a valid handle to a OpenGL 1.1 context
pub unsafe fn wgl_make_current(hdc: HDC, hglrc: HGLRC) -> Result<(), Win32Error> {
    let success = wglMakeCurrent(hdc, hglrc);
    if success != 0 {
        Ok(())
    } else {
        Err(get_last_error())
    }
}
