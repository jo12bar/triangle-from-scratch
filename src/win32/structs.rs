//! Win32 structures, and their associated typedefs (where defined).

use core::{fmt, ptr};

use super::{constants::*, extern_bindings::FormatMessageW, typedefs::*, LocalFree};

/// Implements zero-initialization for C-style structs.
///
/// # Safety
/// This macro uses [`core::mem::MaybeUninit::zeroed()`] and `core::mem::MaybeUninit::assume_init()`
/// to zero-out all of the struct's memory. This might not be a defined value for the struct, so
/// please ensure that the struct can *actually* be zero-initialized before using this macro.
macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
            }
        }
    };
}

/// The [win32 `WNDCLASSW` structure][wndclassw_msdn], used by the [RegisterClass][registerclass_msdn]
/// function to register a new window.
///
/// [wndclassw_msdn]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw
/// [registerclass_msdn]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassa
#[derive(Clone)]
#[repr(C)]
pub struct WNDCLASSW {
    /// The [Class Styles](https://docs.microsoft.com/en-us/windows/desktop/winmsg/about-window-classes)
    /// to apply to this window.
    pub style: UINT,
    /// A pointer to the window procedure.
    pub lpfnWndProc: WNDPROC,
    /// The number of extra bytes to allocate following the window-class structure. The system
    /// initializes the bytes to zero.
    pub cbClsExtra: c_int,
    /// The number of extra bytes to allocate following the window instance. The system initializes
    /// the bytes to zero.
    pub cbWndExtra: c_int,
    /// A handle to the instance that contains the window procedure for the class.
    pub hInstance: HINSTANCE,
    /// A handle to the class icon. This must be a handle to the icon _resource_. If this member
    /// is null, the system provides a default icon.
    pub hIcon: HICON,
    /// A handle to the class cursor. This must be a handle to the cursor _resource_. If this member
    /// is null, an application must explicitly set the cursor shape whenever the mouse moves into
    /// the application's window.
    pub hCursor: HCURSOR,
    /// A handle to the class background brush.
    pub hbrBackground: HBRUSH,
    /// The resource name of the class menu, as the name appears in the resource file.
    pub lpszMenuName: LPCWSTR,
    /// The window class name, with a maximum length of 256.
    pub lpszClassName: LPCWSTR,
}

pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;

unsafe_impl_default_zeroed! { WNDCLASSW }

/// Defines the initialization parameters passed to the window procedure of an application.
/// The members of this struct are identical to the parameters of the
/// [CreateWindowEx][msdn-create-window-ex] function.
///
/// [msdn-create-window-ex]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowexa
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CREATESTRUCTW {
    /// Additional data that may be used to create the window. In particular, this member may contain
    /// data passed to the `lpParam` parameter of the [CreateWindow][msdn-create-window] or
    /// [CreateWindowEx][msdn-create-window-ex] functions.
    ///
    /// [See MSDN for details](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw#members).
    ///
    /// [msdn-create-window]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowa
    /// [msdn-create-window-ex]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowexa
    pub lpCreateParams: LPVOID,
    /// A handle to the module that owns the new window.
    pub hInstance: HINSTANCE,
    /// A handle to the menu to be used by the new window.
    pub hMenu: HMENU,
    /// A handle to the parent/owner window, if the window is a child window or owned. If the window
    /// is neither a child window nor owned, this will be a `NULL` handle.
    pub hwndParent: HWND,
    /// The height of the new window, in pixels.
    pub cy: c_int,
    /// The width of the new window, in pixels.
    pub cx: c_int,
    /// The y-coordinate of the upper left corner of the new window. If the window is a child window,
    /// coordinates are relative to the parent window. Otherwise, coordinates are relative to the
    /// screen origin.
    pub y: c_int,
    /// The x-coordinate of the upper left corner of the new window. If the window is a child window,
    /// coordinates are relative to the parent window. Otherwise, coordinates are relative to the
    /// screen origin.
    pub x: c_int,
    /// The [window style](https://docs.microsoft.com/en-us/windows/desktop/winmsg/window-styles)
    /// for the new window.
    pub style: LONG,
    /// The name of the new window.
    pub lpszName: LPCWSTR,
    /// A pointer to a null-terminated string or atom that specifies the class name of the new
    /// window.
    ///
    /// [MSDN remarks](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw#remarks)
    /// that, since this member can contain a pointer to a local (and thus inacessible) atom, **the
    /// class name shold not be obtained via this member**. The
    /// [`GetClassName`](https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getclassname)
    /// function should be used instead.
    pub lpszClass: LPCWSTR,
    /// The [extended window style](https://docs.microsoft.com/en-us/windows/desktop/winmsg/extended-window-styles)
    /// for this new window.
    pub dwExStyle: DWORD,
}

pub type LPCREATESTRUCTW = *mut CREATESTRUCTW;

unsafe_impl_default_zeroed! { CREATESTRUCTW }

/// Contains information an application can use to paint the client area of a window it owns.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct PAINTSTRUCT {
    /// A handle to the display device context to be used for painting.
    pub hdc: HDC,
    /// Indicates whether the background should be erased. The value is nonzero if the application
    /// should erase the background - for example, if the window class is created without a
    /// background brush. See the
    /// [description of the `hbrBackground` member of the `WNDCLASS` structure on MSDN][msdn-wndclass]
    /// for details.
    ///
    /// [msdn-wndclass]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/ns-winuser-wndclassa
    pub fErase: BOOL,
    /// A rectangle that specifies the upper left and lower right corners of the rectangle in which
    /// the painting is requested, in device units relative to the upper-left corner of the client
    /// area.
    pub rcPaint: RECT,
    /// Reserved by Windows (used internally by the system).
    pub fRestore: BOOL,
    /// Reserved by Windows (used internally by the system).
    pub fIncUpdate: BOOL,
    /// Reserved by Windows (used internally by the system).
    pub rgbReserved: [BYTE; 32],
}

pub type PPAINTSTRUCT = *mut PAINTSTRUCT;
pub type NPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

unsafe_impl_default_zeroed! { PAINTSTRUCT }

/// Defines the xy-coordinates of a point.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point).
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

pub type PPOINT = *mut POINT;
pub type NPPOINT = *mut POINT;
pub type LPPOINT = *mut POINT;

unsafe_impl_default_zeroed! { POINT }

/// Defines a rectangle by the coordinates of its upper-left and lower-right corners.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect).
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

pub type PRECT = *mut RECT;
pub type NPRECT = *mut RECT;
pub type LPRECT = *mut RECT;

unsafe_impl_default_zeroed! { RECT }

/// Contains message information from a thread's message queue.
///
/// See [MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct MSG {
    /// A handle to the window whose window procedure receives the message. NULL if this message is
    /// a thread message.
    pub hwnd: HWND,
    /// The message identifier. Applications can only use the low work; the high word is reserved
    /// by the system.
    pub message: UINT,
    /// Additional information about the message.
    pub wParam: WPARAM,
    /// Additional information about the message.
    pub lParam: LPARAM,
    /// The time at which the message was posted.
    pub time: DWORD,
    /// The cursor position, in screen coordinates, when the message was posted.
    pub pt: POINT,
    /// Private memory, not to be touched.
    pub lPrivate: DWORD,
}

pub type PMSG = *mut MSG;
pub type NPMSG = *mut MSG;
pub type LPMSG = *mut MSG;

unsafe_impl_default_zeroed! { MSG }

/// Describes the pixel format of a drawing surface.
///
/// **See**: [MSDN](https://docs.microsoft.com/en-us/windows/win32/api/Wingdi/ns-wingdi-pixelformatdescriptor)
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PIXELFORMATDESCRIPTOR {
    /// The size of this struct. Set this to `core::mem::size_of::<PIXELFORMATDESCRIPTOR>()`. Or
    /// use this struct's [`Default::default()`] implementation, which sets this field properly for
    /// you.
    pub nSize: WORD,
    /// The version of this struct. Set this to `1`. Or use this struct's [`Default::default()`] implementation,
    /// which sets this field properly for you.
    pub nVersion: WORD,
    /// Bit flags that set properties of the pixel buffer. See MSDN for more information.
    pub dwFlags: DWORD,
    /// Specifies the type of pixel data. See MSDN for more information.
    pub iPixelType: BYTE,
    /// Specifies the number of color bitplanes in each color buffer. For RGBA pixel types, this
    /// is the size of the color buffer excluding the alpha bitplanes. For color-index pixels, this
    /// is the size of the color-index buffer.
    pub cColorBits: BYTE,
    /// Specifies the number of red bitplanes in the RGBA color buffer.
    pub cRedBits: BYTE,
    /// Specifies the shift count for red bitplanes in each RGBA color buffer.
    pub cRedShift: BYTE,
    /// Specifies the number of green bitplanes in the RGBA color buffer.
    pub cGreenBits: BYTE,
    /// Specifies the shift count for green bitplanes in each RGBA color buffer.
    pub cGreenShift: BYTE,
    /// Specifies the number of blue bitplanes in the RGBA color buffer.
    pub cBlueBits: BYTE,
    /// Specifies the shift count for blue bitplanes in each RGBA color buffer.
    pub cBlueShift: BYTE,
    /// Specifies the number of alpha bitplanes in the RGBA color buffer. **Alpha bitplanes are not supported.**
    pub cAlphaBits: BYTE,
    /// Specifies the shift count for alpha bitplanes in each RGBA color buffer. **Alpha bitplanes are not supported.**
    pub cAlphaShift: BYTE,
    /// Specifies the total number of bitplanes in the accumulation buffer.
    pub cAccumBits: BYTE,
    /// Specifies the number of red bitplanes in the accumulation buffer.
    pub cAccumRedBits: BYTE,
    /// Specifies the number of green bitplanes in the accumulation buffer.
    pub cAccumGreenBits: BYTE,
    /// Specifies the number of blue bitplanes in the accumulation buffer.
    pub cAccumBlueBits: BYTE,
    /// Specifies the number of alpha bitplanes in the accumulation buffer.
    pub cAccumAlphaBits: BYTE,
    /// Speciies the depth of the depth (z-axis) buffer.
    pub cDepthBits: BYTE,
    /// Specifies the depth of the stencil buffer.
    pub cStencilBits: BYTE,
    /// Specifies the number of auxilliary buffers. **Auxilliary buffers are not supported.**
    pub cAuxBuffers: BYTE,
    /// **Ignored**. Earlier implementations of OpenGL used this member, but it is no longer used.
    pub iLayerType: BYTE,
    /// Specifies the number of overlay and underlay planes.
    ///
    /// - Bits 0 through 3 specify up to 15 _overlay_ planes
    /// - Bits 4 through 7 specify up to 15 _underlay_ planes
    pub bReserved: BYTE,
    /// **Ignored**. Earlier implementations of OpenGL used this member, but it is no longer used.
    pub dwLayerMask: DWORD,
    /// Specifies the transperent colour or index of an underlay plane. When the pixel type is RGBA,
    /// `dwVisibleMask` is a transparent RGB color value. When the pixel type is a color index, it
    /// is a transperent index value.
    pub dwVisibleMask: DWORD,
    /// **Ignored**. Earlier implementations of OpenGL used this member, but it is no longer used.
    pub dwDamageMask: DWORD,
}

pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;

impl Default for PIXELFORMATDESCRIPTOR {
    fn default() -> Self {
        let mut out: Self = unsafe { core::mem::MaybeUninit::<Self>::zeroed().assume_init() };
        out.nSize = core::mem::size_of::<Self>() as WORD;
        out.nVersion = 1;
        out
    }
}

/// Frees a local block of memory upon being dropped.
#[derive(Debug)]
pub struct OnDropLocalFree(HLOCAL);

impl OnDropLocalFree {
    /// Wraps a HLOCAL handle.
    ///
    /// ## Safety
    ///
    /// `hlocal` **must ACTUALLY be** a handle to a valid block of local memory. If this is not the
    /// case, undefined behaviour will occur when this structure is dropped.
    ///
    /// Do not attempt to access the memory pointed to by `hlocal` after this structure is dropped,
    /// as this is a use-after-free error and will likely result in program crashes and incorrect
    /// behaviour.
    #[must_use]
    pub unsafe fn from_raw_handle(hlocal: HLOCAL) -> Self {
        Self(hlocal)
    }
}

impl Drop for OnDropLocalFree {
    fn drop(&mut self) {
        // Safety: as long as self.0 is actually a handle to a valid block of local memory,
        // freeing it should be fine.
        unsafe { LocalFree(self.0) };
    }
}

/// Represents an error from some Win32 API call.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Win32Error(pub DWORD);

impl fmt::Debug for Win32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Win32Error")
            .field(&format!("{} => {}", &self.0, &self))
            .finish()
    }
}

impl fmt::Display for Win32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If the 29th bit is set, then it's an application error. The system doesn't know how to
        // format those, so we won't even ask it. Instead, we'll just show display that and return
        // early.
        if self.0 & (1 << 29) > 0 {
            return write!(f, "Win32ApplicationError({})", self.0);
        }

        let dwFlags =
              FORMAT_MESSAGE_ALLOCATE_BUFFER              // allocate a large enough buffer for the message
            | FORMAT_MESSAGE_FROM_SYSTEM                  // error messages are system messages
            | FORMAT_MESSAGE_IGNORE_INSERTS               // don't process inserted variadic arguments
            ;

        let lpSource = ptr::null_mut();
        let dwMessageId = self.0;
        let dwLanguageId = 0;

        let mut buffer: *mut u16 = ptr::null_mut();
        let lpBuffer = &mut buffer as *mut *mut u16 as *mut u16;

        let nSize = 0;
        let Arguments = ptr::null_mut();

        // Safety: Assuming all the parameters are correct, this shouldn't fail in any unrecoverable
        // manner.
        let tchar_count_excluding_null = unsafe {
            FormatMessageW(
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                Arguments,
            )
        };

        if tchar_count_excluding_null == 0 || buffer.is_null() {
            // some sort of problem happened. we can't usefully get_last_error since
            // Display formatting doesn't let you give an error value.
            return Err(core::fmt::Error);
        }

        // Safety: This slice is valid as long as the buffer filled by FormatMessageW is not
        // deallocated. The drop gaurd below ensures deallocation only happens at the end of the
        // function.
        let buffer_slice: &[u16] =
            unsafe { core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize) };

        // Safety: Per FormatMessageW's MSDN, the buffer must be freed with LocalFree. Therefore,
        // it is a valid HLOCAL.
        let _buffer_on_drop = unsafe { OnDropLocalFree::from_raw_handle(buffer as HLOCAL) };

        for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) {
            match decode_result {
                Ok('\r') | Ok('\n') => write!(f, " ")?, // eat newlines
                Ok(chr) => write!(f, "{chr}")?,
                Err(_) => write!(f, "�")?,
            }
        }

        Ok(())
    }
}

impl std::error::Error for Win32Error {}
