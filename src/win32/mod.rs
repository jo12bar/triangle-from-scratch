//! Bindings to Win32 structs, types, and functions.

// Win32 names are very incompatible with Rust and Clippy's default lints, so
// we have to disable some of them.
#![allow(clippy::upper_case_acronyms, non_snake_case, non_camel_case_types)]

use core::ptr;

pub mod c_macros;
pub mod constants;
pub mod extern_bindings;
pub mod structs;
pub mod typedefs;

pub use c_macros::*;
pub use constants::*;
pub use extern_bindings::*;
pub use structs::*;
pub use typedefs::*;

use crate::wide_null;

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
