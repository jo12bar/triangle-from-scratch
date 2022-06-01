//! Predefined Win32 constants. These are either C `static` variables, `const` variables, or (more
//! often) `#define` preprocessor macros. This module does not contain any Rusty macro replacements
//! for C preprocessor macros that act like functions. For that, see [`super::c_macros`].

use super::typedefs::*;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Aligns the window's client area on a byte boundary (in the x-direction). This style affects the
/// > width of the window and its horizontal placement on the display.
pub const CS_BYTEALIGNCLIENT: UINT = 0x0000_1000;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Aligns the window on a byte boundary (in the x-direction). This style affects the width of the
/// > window and its horizontal placement on the display.
pub const CS_BYTEALIGNWINDOW: UINT = 0x0000_2000;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Allocates one device context to be shared by all windows in the class. Because window classes
/// > are process specific, it is possible for multiple threads of an application to create a window
/// > of the same class. It is also possible for the threads to attempt to use the device context
/// > simultaneously. When this happens, the system allows only one thread to successfully finish
/// > its drawing operation.
pub const CS_CLASSDC: UINT = 0x0000_0040;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Sends a double-click message to the window procedure when the user double-clicks the mouse
/// > while the cursor is within a window belonging to the class.
pub const CS_DBLCLKS: UINT = 0x0000_0008;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Enables the drop shadow effect on a window. The effect is turned on and off through
/// > `SPI_SETDROPSHADOW`. Typically, this is enabled for small, short-lived windows such as menus
/// > to emphasize their Z-order relationship to other windows. Windows created from a class with
/// > this style must be top-level windows; they may not be child windows.
pub const CS_DROPSHADOW: UINT = 0x0002_0000;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Indicates that the window class is an application global class. For more information, see the
/// > "Application Global Classes" section of
/// > [About Window Classes](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes).
pub const CS_GLOBALCLASS: UINT = 0x0000_4000;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Redraws the entire window if a movement or size adjustment changes the width of the client area.
pub const CS_HREDRAW: UINT = 0x0000_0002;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Disables **Close** on the window menu.
pub const CS_NOCLOSE: UINT = 0x0000_0200;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Allocates a unique device context for each window in the class.
pub const CS_OWNDC: UINT = 0x0000_0020;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Sets the clipping rectangle of the child window to that of the parent window so that the child
/// > can draw on the parent. A window with the `CS_PARENTDC` style bit receives a regular device
/// > context from the system's cache of device contexts. It does not give the child the parent's
/// > device context or device context settings. Specifying `CS_PARENTDC` enhances an application's
/// > performance.
pub const CS_PARENTDC: UINT = 0x0000_0080;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Saves, as a bitmap, the portion of the screen image obscured by a window of this class. When
/// > the window is removed, the system uses the saved bitmap to restore the screen image, including
/// > other windows that were obscured. Therefore, the system does not send
/// > [WM_PAINT](https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint) messages to windows
/// > that were obscured if the memory used by the bitmap has not been discarded and if other screen
/// > actions have not invalidated the stored image.
/// >
/// > This style is useful for small windows (for example, menus or dialog boxes) that are displayed
/// > briefly and then removed before other screen activity takes place. This style increases the
/// > time required to display the window, because the system must first allocate memory to store
/// > the bitmap.
pub const CS_SAVEBITS: UINT = 0x0000_0800;

/// A [window class style](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles).
///
/// > Redraws the entire window if a movement or size adjustment changes the height of the client area.
pub const CS_VREDRAW: UINT = 0x0000_0001;

pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// Allocates a buffer large enough to hold the formatted message.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x0000_0100;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// Indicates the *Arguments* parameter is not a [`va_list`] structure, but is a pointer to an array
/// of values that represent the arguments.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x0000_2000;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// The _lpSource_ parameter is a module handle containing the message-table resource(s) to search.
/// If this _lpSource_ handle is null, the current process's application image file will be searched.
/// This flag cannot be used with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x0000_0800;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// The _lpSource_ parameter is a pointer to a null-terminated string that contains a message
/// definition. The message definition may contain insert sequences, just as the message text in a
/// message table resource may. This flag cannot be used with [`FORMAT_MESSAGE_FROM_HMODULE`] or
/// [`FORMAT_MESSAGE_FROM_SYSTEM`].
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x0000_0400;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// The function should search the system message-table resource(s) for the requested message. If
/// this flag is specified with [`FORMAT_MESSAGE_FROM_HMODULE`], the function searches the system
/// message table if the message is not found in the module specified by lpSource. This flag cannot
/// be used with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// If this flag is specified, an application can pass the result of the [`GetLastError`][super::GetLastError]
/// function to retrieve the message text for a system-defined error.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x0000_1000;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// Insert sequences in the message definition such as `%1` are to be ignored and passed through to
/// the output buffer unchanged. This flag is useful for fetching a message for later formatting. If
/// this flag is set, the _Arguments_ parameter is ignored.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x0000_0200;

/// For use with [`super::SetWindowLongPtrW()`].
///
/// [From MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw#parameters):
///
/// > Sets the user data associated with the window. This data is intended for use by the
/// > application that created the window. Its value is initially zero.
pub const GWLP_USERDATA: c_int = -21;

/// The id of the "Ok" button on a message box.
pub const IDOK: c_int = 1;

/// Display "Ok" and "Cancel" buttons on a message box.
pub const MB_OKCANCEL: u32 = 1;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Allows the buffer to draw to a window or device surface.
pub const PFD_DRAW_TO_WINDOW: DWORD = 0x0000_0004;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Allows the buffer to draw to a memory map.
pub const PFD_DRAW_TO_BITMAP: DWORD = 0x0000_0008;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// The buffer supports GDI drawing.
/// This flag and [`PFD_DOUBLEBUFFER`] are mutually exclusive in the current generic implementation.
pub const PFD_SUPPORT_GDI: DWORD = 0x0000_0010;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// This buffer supports OpenGL drawing.
pub const PFD_SUPPORT_OPENGL: DWORD = 0x0000_0020;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// The pixel format is supported by a device driver that accelerates the generic implementation.
/// If this flag is clear and the [`PFD_GENERIC_FORMAT`] flag is set, the pixel format is supported
/// by the generic implementation only.
pub const PFD_GENERIC_ACCELERATED: DWORD = 0x0000_1000;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// The pixel format is supported by the GDI software implementation, which is also known as the
/// generic implementation. If this bit is clear, the pixel format is supported by a device driver
/// or hardware.
pub const PFD_GENERIC_FORMAT: DWORD = 0x0000_0040;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// The buffer uses RGBA pixels on a palette-managed device. A logical palette is required to
/// achieve the best results for this pixel type. Colors in the palette should be specified
/// according to the values of the [`cRedBits`][`super::PIXELFORMATDESCRIPTOR::cRedBits`],
/// [`cRedShift`][`super::PIXELFORMATDESCRIPTOR::cRedShift`], [`cGreenBits`][`super::PIXELFORMATDESCRIPTOR::cGreenBits`],
/// [`cGreenShift`][`super::PIXELFORMATDESCRIPTOR::cGreenShift`], [`cBlueBits`][`super::PIXELFORMATDESCRIPTOR::cBlueBits`],
/// and [`cBlueShift`][`super::PIXELFORMATDESCRIPTOR::cBlueShift`] members. The palette should be
/// created and realized in the device context before calling
/// [`wglMakeCurrent`](https://docs.microsoft.com/en-us/windows/desktop/api/wingdi/nf-wingdi-wglmakecurrent).
pub const PFD_NEED_PALETTE: DWORD = 0x0000_0080;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Defined in the pixel format descriptors of hardware that supports one hardware palette in
/// 256-color mode only. For such systems to use hardware acceleration, the hardware palette must be
/// in a fixed order (for example, 3-3-2) when in RGBA mode or must match the logical palette when
/// in color-index mode. When this flag is set, you must call `SetSystemPaletteUse` in your program
/// to force a one-to-one mapping of the logical palette and the system palette. If your OpenGL
/// hardware supports multiple hardware palettes and the device driver can allocate spare hardware
/// palettes for OpenGL, this flag is typically clear.
///
/// This flag is not set in the generic pixel formats.
pub const PFD_NEED_SYSTEM_PALETTE: DWORD = 0x0000_0100;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// The buffer is double-buffered. This flag and [`PFD_SUPPORT_GDI`] are mutually exclusive in the
/// current generic implementation.
pub const PFD_DOUBLEBUFFER: DWORD = 0x0000_0001;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// The buffer is stereoscopic. This flag is not supported in the current generic implementation.
pub const PFD_STEREO: DWORD = 0x0000_0002;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Indicates whether a device can swap individual layer planes with pixel formats that include
/// double-buffered overlay or underlay planes. Otherwise all layer planes are swapped together as
/// a group. When this flag is set, wglSwapLayerBuffers is supported.
pub const PFD_SWAP_LAYER_BUFFERS: DWORD = 0x0000_0800;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Use when calling [`super::ChoosePixelFormat`].
///
/// > The requested pixel format can either have or not have a depth buffer. To select a pixel
/// > format without a depth buffer, you must specify this flag. The requested pixel format can be
/// > with or without a depth buffer. Otherwise, only pixel formats with a depth buffer are considered.
pub const PFD_DEPTH_DONTCARE: DWORD = 0x2000_0000;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Use when calling [`super::ChoosePixelFormat`].
///
/// > The requested pixel format can be either single- or double-buffered.
pub const PFD_DOUBLEBUFFER_DONTCARE: DWORD = 0x4000_0000;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Use when calling [`super::ChoosePixelFormat`].
///
/// > The requested pixel format can be either monoscopic or stereoscopic.
pub const PFD_STEREO_DONTCARE: DWORD = 0x8000_0000;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Only valid if the `glAddSwapHintRectWIN` extension function is available.
///
/// > Specifies the content of the back buffer in the double-buffered main color plane following a
/// > buffer swap. Swapping the color buffers causes the content of the back buffer to be copied to
/// > the front buffer. The content of the back buffer is not affected by the swap. PFD_SWAP_COPY is
/// > a hint only and might not be provided by a driver.
pub const PFD_SWAP_COPY: DWORD = 0x0000_0400;

/// A [`PIXELFORMATDESCRIPTOR` flag][`super::PIXELFORMATDESCRIPTOR::dwFlags`].
/// Only valid if the `glAddSwapHintRectWIN` extension function is available.
///
/// > Specifies the content of the back buffer in the double-buffered main color plane following a
/// > buffer swap. Swapping the color buffers causes the exchange of the back buffer's content with
/// > the front buffer's content. Following the swap, the back buffer's content contains the front
/// > buffer's content before the swap. PFD_SWAP_EXCHANGE is a hint only and might not be provided
/// > by a driver.
pub const PFD_SWAP_EXCHANGE: DWORD = 0x0000_0200;

/// A [`PIXELFORMATDESCRIPTOR` pixel data type][`super::PIXELFORMATDESCRIPTOR::iPixelType`].
/// Chooses RGBA pixels. Each pixel in this set has four components: red, green, blue, and alpha.
pub const PFD_TYPE_RGBA: BYTE = 0;

/// A [`PIXELFORMATDESCRIPTOR` pixel data type][`super::PIXELFORMATDESCRIPTOR::iPixelType`].
/// Chooses color-index pixels. Each pixel uses a color-index value.
pub const PFD_TYPE_COLORINDEX: BYTE = 1;

/// A [`PIXELFORMATDESCRIPTOR` layer type][`super::PIXELFORMATDESCRIPTOR::iLayerType`].
pub const PFD_MAIN_PLANE: BYTE = 0;
/// A [`PIXELFORMATDESCRIPTOR` layer type][`super::PIXELFORMATDESCRIPTOR::iLayerType`].
pub const PFD_OVERLAY_PLANE: BYTE = 1;
/// A [`PIXELFORMATDESCRIPTOR` layer type][`super::PIXELFORMATDESCRIPTOR::iLayerType`].
pub const PFD_UNDERLAY_PLANE: BYTE = u8::MAX; // was (-1) in the windows headers

pub const SW_SHOW: c_int = 5;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;

/// Excludes the area occupied by child windows when drawing occurs within the
/// parent window.
///
/// This style is used when creating the parent window.
pub const WS_CLIPCHILDREN: u32 = 0x02000000;

/// Clips child windows relative to each other.
///
/// That is, when a particular child window receives a WM_PAINT message,
/// the WS_CLIPSIBLINGS style clips all other overlapping child windows out of
/// the region of the child window to be updated. If WS_CLIPSIBLINGS is not
/// specified and child windows overlap, it is possible, when drawing within the
/// client area of a child window, to draw within the client area of a
/// neighboring child window.
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;

pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

/// Sent when the window is closed.
pub const WM_CLOSE: u32 = 0x0010;
/// Sent when an application requests a window be created. The window procedure will receive this
/// message after the window is created, but before the window becomes visible.
pub const WM_CREATE: u32 = 0x0001;
/// Sent when the window is being destroyed.
pub const WM_DESTROY: u32 = 0x0002;
/// Sent prior to the [`WM_CREATE`] message when a window is first created.
pub const WM_NCCREATE: u32 = 0x0081;
/// Sent when the window should be painted.
pub const WM_PAINT: u32 = 0x000F;
/// Indicates a request to termiante the application.
pub const WM_QUIT: u32 = 0x0012;

pub use wgl_pixel_format::*;
/// Base constants for use with the [`WGL_ARB_pixel_format`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
/// extension.
pub mod wgl_pixel_format {
    pub const WGL_NUMBER_PIXEL_FORMATS_ARB: super::c_int = 0x2000;
    pub const WGL_DRAW_TO_WINDOW_ARB: super::c_int = 0x2001;
    pub const WGL_DRAW_TO_BITMAP_ARB: super::c_int = 0x2002;
    pub const WGL_ACCELERATION_ARB: super::c_int = 0x2003;
    pub const WGL_NEED_PALETTE_ARB: super::c_int = 0x2004;
    pub const WGL_NEED_SYSTEM_PALETTE_ARB: super::c_int = 0x2005;
    pub const WGL_SWAP_LAYER_BUFFERS_ARB: super::c_int = 0x2006;
    pub const WGL_SWAP_METHOD_ARB: super::c_int = 0x2007;
    pub const WGL_NUMBER_OVERLAYS_ARB: super::c_int = 0x2008;
    pub const WGL_NUMBER_UNDERLAYS_ARB: super::c_int = 0x2009;
    pub const WGL_TRANSPARENT_ARB: super::c_int = 0x200A;
    pub const WGL_TRANSPARENT_RED_VALUE_ARB: super::c_int = 0x2037;
    pub const WGL_TRANSPARENT_GREEN_VALUE_ARB: super::c_int = 0x2038;
    pub const WGL_TRANSPARENT_BLUE_VALUE_ARB: super::c_int = 0x2039;
    pub const WGL_TRANSPARENT_ALPHA_VALUE_ARB: super::c_int = 0x203A;
    pub const WGL_TRANSPARENT_INDEX_VALUE_ARB: super::c_int = 0x203B;
    pub const WGL_SHARE_DEPTH_ARB: super::c_int = 0x200C;
    pub const WGL_SHARE_STENCIL_ARB: super::c_int = 0x200D;
    pub const WGL_SHARE_ACCUM_ARB: super::c_int = 0x200E;
    pub const WGL_SUPPORT_GDI_ARB: super::c_int = 0x200F;
    pub const WGL_SUPPORT_OPENGL_ARB: super::c_int = 0x2010;
    pub const WGL_DOUBLE_BUFFER_ARB: super::c_int = 0x2011;
    pub const WGL_STEREO_ARB: super::c_int = 0x2012;
    pub const WGL_PIXEL_TYPE_ARB: super::c_int = 0x2013;
    pub const WGL_COLOR_BITS_ARB: super::c_int = 0x2014;
    pub const WGL_RED_BITS_ARB: super::c_int = 0x2015;
    pub const WGL_RED_SHIFT_ARB: super::c_int = 0x2016;
    pub const WGL_GREEN_BITS_ARB: super::c_int = 0x2017;
    pub const WGL_GREEN_SHIFT_ARB: super::c_int = 0x2018;
    pub const WGL_BLUE_BITS_ARB: super::c_int = 0x2019;
    pub const WGL_BLUE_SHIFT_ARB: super::c_int = 0x201A;
    pub const WGL_ALPHA_BITS_ARB: super::c_int = 0x201B;
    pub const WGL_ALPHA_SHIFT_ARB: super::c_int = 0x201C;
    pub const WGL_ACCUM_BITS_ARB: super::c_int = 0x201D;
    pub const WGL_ACCUM_RED_BITS_ARB: super::c_int = 0x201E;
    pub const WGL_ACCUM_GREEN_BITS_ARB: super::c_int = 0x201F;
    pub const WGL_ACCUM_BLUE_BITS_ARB: super::c_int = 0x2020;
    pub const WGL_ACCUM_ALPHA_BITS_ARB: super::c_int = 0x2021;
    pub const WGL_DEPTH_BITS_ARB: super::c_int = 0x2022;
    pub const WGL_STENCIL_BITS_ARB: super::c_int = 0x2023;
    pub const WGL_AUX_BUFFERS_ARB: super::c_int = 0x2024;
    pub const WGL_NO_ACCELERATION_ARB: super::c_int = 0x2025;
    pub const WGL_GENERIC_ACCELERATION_ARB: super::c_int = 0x2026;
    pub const WGL_FULL_ACCELERATION_ARB: super::c_int = 0x2027;

    pub const WGL_SWAP_EXCHANGE_ARB: super::c_int = 0x2028;
    pub const WGL_SWAP_COPY_ARB: super::c_int = 0x2029;
    pub const WGL_SWAP_UNDEFINED_ARB: super::c_int = 0x202A;

    pub const WGL_TYPE_RGBA_ARB: super::c_int = 0x202B;
    pub const WGL_TYPE_COLORINDEX_ARB: super::c_int = 0x202C;
}

/// Use with [`WGL_ARB_pixel_format`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt)
/// to enable sRGB framebuffer update and rendering.
///
/// Defined in [`WGL_EXT_framebuffer_sRGB`](https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_framebuffer_sRGB.txt).
///
/// > Accepted by the `<piAttributes>` parameter of
/// > wglGetPixelFormatAttribivEXT, wglGetPixelFormatAttribfvEXT, and
/// > the `<piAttribIList>` and `<pfAttribIList>` of wglChoosePixelFormatEXT.
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_EXT: c_int = 0x20A9;

pub use arb_multisample::*;
/// Constants for controlling multisampling behaviour, defined in
/// [`ARB_multisample`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt).
pub mod arb_multisample {
    pub const GLX_SAMPLE_BUFFERS_ARB: super::c_int = 100000;
    pub const GLX_SAMPLES_ARB: super::c_int = 100001;

    pub const WGL_SAMPLE_BUFFERS_ARB: super::c_int = 0x2041;
    pub const WGL_SAMPLES_ARB: super::c_int = 0x2042;

    pub const MULTISAMPLE_ARB: super::c_int = 0x809D;
    pub const SAMPLE_ALPHA_TO_COVERAGE_ARB: super::c_int = 0x809E;
    pub const SAMPLE_ALPHA_TO_ONE_ARB: super::c_int = 0x809F;
    pub const SAMPLE_COVERAGE_ARB: super::c_int = 0x80A0;

    pub const MULTISAMPLE_BIT_ARB: super::c_int = 0x20000000;

    pub const SAMPLE_BUFFERS_ARB: super::c_int = 0x80A8;
    pub const SAMPLES_ARB: super::c_int = 0x80A9;
    pub const SAMPLE_COVERAGE_VALUE_ARB: super::c_int = 0x80AA;
    pub const SAMPLE_COVERAGE_INVERT_ARB: super::c_int = 0x80AB;
}

pub use wgl_arb_create_context::*;
/// Constants for creating OpenGL contexts using [`WGL_ARB_create_context`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt).
pub mod wgl_arb_create_context {
    pub const WGL_CONTEXT_MAJOR_VERSION_ARB: super::c_int = 0x2091;
    pub const WGL_CONTEXT_MINOR_VERSION_ARB: super::c_int = 0x2092;
    pub const WGL_CONTEXT_LAYER_PLANE_ARB: super::c_int = 0x2093;
    pub const WGL_CONTEXT_FLAGS_ARB: super::c_int = 0x2094;
    pub const WGL_CONTEXT_PROFILE_MASK_ARB: super::c_int = 0x9126;

    pub const WGL_CONTEXT_DEBUG_BIT_ARB: super::c_int = 0x0001;
    pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: super::c_int = 0x0002;

    pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: super::c_int = 0x00000001;
    pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: super::c_int = 0x00000002;

    pub const ERROR_INVALID_VERSION_ARB: super::c_int = 0x2095;
    pub const ERROR_INVALID_PROFILE_ARB: super::c_int = 0x2096;
}

/// The predefined cursor styles.
pub enum IDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    /// Standard arrow
    Arrow = 32512,
    /// Crosshair
    Cross = 32515,
    /// Hand
    Hand = 32649,
    /// Arrow and question mark
    Help = 32651,
    /// I-beam
    IBeam = 32513,
    /// Slashed circle
    No = 32648,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    /// Vertical arrow
    UpArrow = 32516,
    /// Hourglass
    Wait = 32514,
}

/// Predefined system colours.
///
/// **See:** [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
    ActiveBorder = 10,
    ActiveCaption = 2,
    AppWorkspace = 12,
    /// Button face, also "3D face" color.
    ButtonFace = 15,
    /// Button highlight, also "3D highlight" color.
    ButtonHighlight = 20,
    /// Button shadow, also "3D shadow" color.
    ButtonShadow = 16,
    ButtonText = 18,
    CaptionText = 9,
    /// AKA "3D dark shadow"
    D3DarkShadow = 21,
    /// AKA "3D light"
    D3Light = 22,
    /// Desktop background color
    Desktop = 1,
    GradientActiveCaption = 27,
    GradientInactiveCaption = 28,
    GrayText = 17,
    Highlight = 13,
    HighlightText = 14,
    HotLight = 26,
    InactiveBorder = 11,
    InactiveCaption = 3,
    InactiveCaptionText = 19,
    InfoBackground = 24,
    InfoText = 23,
    Menu = 4,
    MenuHighlight = 29,
    MenuBar = 30,
    MenuText = 7,
    ScrollBar = 0,
    Window = 5,
    WindowFrame = 6,
    WindowText = 8,
}
