// If we're running with debug assertions, we probably want a console auto-allocated for us too.
// In that case, only compile for the windows subsystem if debug assertions are disabled (e.g. in
// the release profile).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ffi::CStr, mem, ptr};

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

    has_setup_ran: bool,

    gl_attach_shader: glAttachShader_t,
    gl_bind_buffer: glBindBuffer_t,
    gl_bind_vertex_array: glBindVertexArray_t,
    gl_buffer_data: glBufferData_t,
    gl_clear: glClear_t,
    gl_clear_color: glClearColor_t,
    gl_compile_shader: glCompileShader_t,
    gl_create_program: glCreateProgram_t,
    gl_create_shader: glCreateShader_t,
    gl_delete_shader: glDeleteShader_t,
    gl_draw_arrays: glDrawArrays_t,
    gl_draw_elements: glDrawElements_t,
    gl_enable_vertex_attrib_array: glEnableVertexAttribArray_t,
    gl_gen_buffers: glGenBuffers_t,
    gl_gen_vertex_arrays: glGenVertexArrays_t,
    gl_get_program_info_log: glGetProgramInfoLog_t,
    gl_get_program_iv: glGetProgramiv_t,
    gl_get_shader_info_log: glGetShaderInfoLog_t,
    gl_get_shader_iv: glGetShaderiv_t,
    gl_link_program: glLinkProgram_t,
    gl_shader_source: glShaderSource_t,
    gl_use_program: glUseProgram_t,
    gl_vertex_attrib_pointer: glVertexAttribPointer_t,

    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    shader_program: GLuint,
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

        self.gl_attach_shader = transmute(self.gl_get_proc_address(c_str!("glAttachShader")));
        self.gl_bind_buffer = transmute(self.gl_get_proc_address(c_str!("glBindBuffer")));
        self.gl_bind_vertex_array =
            transmute(self.gl_get_proc_address(c_str!("glBindVertexArray")));
        self.gl_buffer_data = transmute(self.gl_get_proc_address(c_str!("glBufferData")));
        self.gl_clear = transmute(self.gl_get_proc_address(c_str!("glClear")));
        self.gl_clear_color = transmute(self.gl_get_proc_address(c_str!("glClearColor")));
        self.gl_compile_shader = transmute(self.gl_get_proc_address(c_str!("glCompileShader")));
        self.gl_create_program = transmute(self.gl_get_proc_address(c_str!("glCreateProgram")));
        self.gl_create_shader = transmute(self.gl_get_proc_address(c_str!("glCreateShader")));
        self.gl_delete_shader = transmute(self.gl_get_proc_address(c_str!("glDeleteShader")));
        self.gl_draw_arrays = transmute(self.gl_get_proc_address(c_str!("glDrawArrays")));
        self.gl_draw_elements = transmute(self.gl_get_proc_address(c_str!("glDrawElements")));
        self.gl_enable_vertex_attrib_array =
            transmute(self.gl_get_proc_address(c_str!("glEnableVertexAttribArray")));
        self.gl_gen_buffers = transmute(self.gl_get_proc_address(c_str!("glGenBuffers")));
        self.gl_gen_vertex_arrays =
            transmute(self.gl_get_proc_address(c_str!("glGenVertexArrays")));
        self.gl_get_program_info_log =
            transmute(self.gl_get_proc_address(c_str!("glGetProgramInfoLog")));
        self.gl_get_program_iv = transmute(self.gl_get_proc_address(c_str!("glGetProgramiv")));
        self.gl_get_shader_info_log =
            transmute(self.gl_get_proc_address(c_str!("glGetShaderInfoLog")));
        self.gl_get_shader_iv = transmute(self.gl_get_proc_address(c_str!("glGetShaderiv")));
        self.gl_link_program = transmute(self.gl_get_proc_address(c_str!("glLinkProgram")));
        self.gl_shader_source = transmute(self.gl_get_proc_address(c_str!("glShaderSource")));
        self.gl_use_program = transmute(self.gl_get_proc_address(c_str!("glUseProgram")));
        self.gl_vertex_attrib_pointer =
            transmute(self.gl_get_proc_address(c_str!("glVertexAttribPointer")));
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

                if !window_data.has_setup_ran {
                    gl_setup(window_data).unwrap();
                    window_data.has_setup_ran = true;
                }

                gl_paint(window_data).unwrap();

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

#[rustfmt::skip]
const TRIANGLE_VERTICES: [f32; 18] = [
    // positions      // colors
    -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
     0.5, -0.5, 0.0,  0.0, 1.0, 0.0,
     0.0,  0.5, 0.0,  0.0, 0.0, 1.0,
];

#[rustfmt::skip]
const TRIANGLE_INDICES: [GLuint; 3] = [
    0, 1, 2
];

const VERTEX_SHADER_SOURCE: &[u8] = c_str!(include_str!("./vertex.vs"));
const FRAGMENT_SHADER_SOURCE: &[u8] = c_str!(include_str!("./fragment.fs"));

fn gl_setup(window_data: &mut WindowData) -> Result<(), Box<dyn std::error::Error>> {
    let gl_attach_shader = window_data.gl_attach_shader.unwrap();
    let gl_bind_buffer = window_data.gl_bind_buffer.unwrap();
    let gl_bind_vertex_array = window_data.gl_bind_vertex_array.unwrap();
    let gl_buffer_data = window_data.gl_buffer_data.unwrap();
    let gl_compile_shader = window_data.gl_compile_shader.unwrap();
    let gl_create_program = window_data.gl_create_program.unwrap();
    let gl_create_shader = window_data.gl_create_shader.unwrap();
    let gl_delete_shader = window_data.gl_delete_shader.unwrap();
    let gl_enable_vertex_attrib_array = window_data.gl_enable_vertex_attrib_array.unwrap();
    let gl_gen_buffers = window_data.gl_gen_buffers.unwrap();
    let gl_gen_vertex_arrays = window_data.gl_gen_vertex_arrays.unwrap();
    let gl_link_program = window_data.gl_link_program.unwrap();
    let gl_shader_source = window_data.gl_shader_source.unwrap();
    let gl_vertex_attrib_pointer = window_data.gl_vertex_attrib_pointer.unwrap();

    unsafe {
        // Gen VAO, VBO, and EBO
        gl_gen_vertex_arrays(1, &mut window_data.vao);
        gl_gen_buffers(1, &mut window_data.vbo);
        gl_gen_buffers(1, &mut window_data.ebo);

        // Bind VAO
        gl_bind_vertex_array(window_data.vao);

        // Bind triangle VBO
        gl_bind_buffer(GL_ARRAY_BUFFER, window_data.vbo);
        gl_buffer_data(
            GL_ARRAY_BUFFER,
            mem::size_of_val(&TRIANGLE_VERTICES) as _,
            TRIANGLE_VERTICES.as_ptr() as _,
            GL_STATIC_DRAW,
        );

        // Bind triangle EBO
        gl_bind_buffer(GL_ELEMENT_ARRAY_BUFFER, window_data.ebo);
        gl_buffer_data(
            GL_ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(&TRIANGLE_INDICES) as _,
            TRIANGLE_INDICES.as_ptr() as _,
            GL_STATIC_DRAW,
        );

        // Set vertex attrbute pointers tied to the VBO and the VAO
        // position attribute
        gl_vertex_attrib_pointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            (6 * mem::size_of::<f32>()) as _,
            0 as _,
        );
        gl_enable_vertex_attrib_array(0);

        // color attribute
        gl_vertex_attrib_pointer(
            1,
            3,
            GL_FLOAT,
            GL_FALSE,
            (6 * mem::size_of::<f32>()) as _,
            (3 * mem::size_of::<f32>()) as _,
        );
        gl_enable_vertex_attrib_array(1);

        gl_bind_vertex_array(0);

        // Load and compile vertex shader
        let vertex_shader = gl_create_shader(GL_VERTEX_SHADER);
        gl_shader_source(
            vertex_shader,
            1,
            [VERTEX_SHADER_SOURCE.as_ptr()].as_ptr() as _,
            ptr::null(),
        );
        gl_compile_shader(vertex_shader);
        gl_print_shader_compile_status(window_data, vertex_shader, "vertex.vs");

        // Load and compile fragment shader
        let fragment_shader = gl_create_shader(GL_FRAGMENT_SHADER);
        gl_shader_source(
            fragment_shader,
            1,
            [FRAGMENT_SHADER_SOURCE.as_ptr()].as_ptr() as _,
            ptr::null(),
        );
        gl_compile_shader(fragment_shader);
        gl_print_shader_compile_status(window_data, fragment_shader, "fragment.fs");

        // Link shader objects into a program
        window_data.shader_program = gl_create_program();
        gl_attach_shader(window_data.shader_program, vertex_shader);
        gl_attach_shader(window_data.shader_program, fragment_shader);
        gl_link_program(window_data.shader_program);
        gl_print_program_link_status(window_data, window_data.shader_program, "main");

        // Delete now unneeded shader objects
        gl_delete_shader(fragment_shader);
        gl_delete_shader(vertex_shader);
    }

    Ok(())
}

fn gl_paint(window_data: &mut WindowData) -> Result<(), Box<dyn std::error::Error>> {
    let gl_bind_vertex_array = window_data.gl_bind_vertex_array.unwrap();
    let gl_clear_color = window_data.gl_clear_color.unwrap();
    let gl_clear = window_data.gl_clear.unwrap();
    let gl_draw_elements = window_data.gl_draw_elements.unwrap();
    let gl_use_program = window_data.gl_use_program.unwrap();

    unsafe {
        gl_clear_color(0.6, 0.7, 0.8, 1.0);
        gl_clear(GL_COLOR_BUFFER_BIT);

        gl_use_program(window_data.shader_program);
        gl_bind_vertex_array(window_data.vao);
        gl_draw_elements(GL_TRIANGLES, 3, GL_UNSIGNED_INT, 0 as _);
    }

    Ok(())
}

fn gl_print_shader_compile_status(window_data: &mut WindowData, shader: GLuint, shader_name: &str) {
    let gl_get_shader_iv = window_data.gl_get_shader_iv.unwrap();

    let mut success: GLint = 0;
    unsafe { gl_get_shader_iv(shader, GL_COMPILE_STATUS, &mut success) };

    if success != 1 {
        let gl_get_shader_info_log = window_data.gl_get_shader_info_log.unwrap();

        let mut info_log: [u8; 512] = [0; 512];
        let mut info_log_length: GLsizei = 0;
        unsafe {
            gl_get_shader_info_log(
                shader,
                512,
                &mut info_log_length,
                info_log.as_mut_ptr() as *mut GLchar,
            )
        };

        let info_log_str = CStr::from_bytes_with_nul(&info_log[..=info_log_length as usize])
            .expect("glGetShaderInfoLog returned in invalid C-style string")
            .to_str()
            .expect("glGetShaderInfoLog returned a valid C-style string with invalid UTF-8 data");

        eprintln!("Error compiling shader {shader} ({shader_name}):\n{info_log_str}");
    } else {
        println!("Successfully compiled shader {shader} ({shader_name})");
    }
}

fn gl_print_program_link_status(window_data: &mut WindowData, program: GLuint, program_name: &str) {
    let gl_get_program_iv = window_data.gl_get_program_iv.unwrap();

    let mut success: GLint = 0;
    unsafe { gl_get_program_iv(program, GL_LINK_STATUS, &mut success) };

    if success != 1 {
        let gl_get_program_info_log = window_data.gl_get_program_info_log.unwrap();

        let mut info_log: [u8; 512] = [0; 512];
        let mut info_log_length: GLsizei = 0;
        unsafe {
            gl_get_program_info_log(
                program,
                512,
                &mut info_log_length,
                info_log.as_mut_ptr() as *mut GLchar,
            )
        };

        let info_log_str = CStr::from_bytes_with_nul(&info_log[..=info_log_length as usize])
            .expect("glGetProgramInfoLog returned in invalid C-style string")
            .to_str()
            .expect("glGetProgramInfoLog returned a valid C-style string with invalid UTF-8 data");

        eprintln!("Error linking program {program} ({program_name}):\n{info_log_str}");
    } else {
        println!("Successfully linked program {program} ({program_name})");
    }
}
