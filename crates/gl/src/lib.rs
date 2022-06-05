//! Bindings and utility functions for working with OpenGL.

pub mod bindings;

use bindings::prelude::*;

use core::cell::RefCell;

/// Convert a UTF-8 rust string literal into a null-terminated `&[u8]`.
///
/// **Note**: This macro can only be passed string _literals_ (not variables or constants!) due to
/// its internal use of [`concat`] to add a null byte to the end of the string.
#[macro_export]
macro_rules! c_str {
    ($text:expr) => {{
        concat!($text, '\0').as_bytes()
    }};
}

/// Used by [`GlContext`] to load pointers to OpenGL procedures, probably from shared libaries.
///
/// This allows [`GlContext`] to be somewhat platform-agnostic. It is up to the trait implementor to
/// provide a way to load OpenGL procedure addresses from shader libraries, as methods to do so vary
/// on every platform.
pub trait GlProcLoader {
    /// Load a pointer to an OpenGL function.
    ///
    /// - `name` is a null-terminated ASCII c-style string specifying the name of the function to
    ///   load.
    ///
    /// ## Safety
    ///
    /// - The returned mutable pointer **must** be a pointer to the function specified by `name`.
    ///   If a pointer to some other function is returned, it will be called with wild abandon,
    ///   leading to undefined behaviour.
    unsafe fn load_proc(&self, name: &[u8]) -> *mut core::ffi::c_void;
}

/// Loads and holds pointers to various OpenGL functions, in a platform-agnostic manner.
///
/// For now, this struct is **not thread-safe**.
#[derive(Default)]
pub struct GlContext {
    loader: Option<Box<dyn GlProcLoader>>,

    gl_procs: GlProcs,
}

#[derive(Default)]
struct GlProcs {
    gl_attach_shader: RefCell<glAttachShader_t>,
    gl_bind_buffer: RefCell<glBindBuffer_t>,
    gl_bind_vertex_array: RefCell<glBindVertexArray_t>,
    gl_buffer_data: RefCell<glBufferData_t>,
    gl_clear: RefCell<glClear_t>,
    gl_clear_color: RefCell<glClearColor_t>,
    gl_compile_shader: RefCell<glCompileShader_t>,
    gl_create_program: RefCell<glCreateProgram_t>,
    gl_create_shader: RefCell<glCreateShader_t>,
    gl_delete_shader: RefCell<glDeleteShader_t>,
    gl_draw_arrays: RefCell<glDrawArrays_t>,
    gl_draw_elements: RefCell<glDrawElements_t>,
    gl_enable_vertex_attrib_array: RefCell<glEnableVertexAttribArray_t>,
    gl_gen_buffers: RefCell<glGenBuffers_t>,
    gl_gen_vertex_arrays: RefCell<glGenVertexArrays_t>,
    gl_get_program_info_log: RefCell<glGetProgramInfoLog_t>,
    gl_get_program_iv: RefCell<glGetProgramiv_t>,
    gl_get_shader_info_log: RefCell<glGetShaderInfoLog_t>,
    gl_get_shader_iv: RefCell<glGetShaderiv_t>,
    gl_link_program: RefCell<glLinkProgram_t>,
    gl_shader_source: RefCell<glShaderSource_t>,
    gl_use_program: RefCell<glUseProgram_t>,
    gl_vertex_attrib_pointer: RefCell<glVertexAttribPointer_t>,
}

/// This macro is used in the implementation of [`GlContext`] to cut down on
/// OpenGL procedure wrapper boilerplate.
macro_rules! impl_glcontext_proc_call {
    () => {};

    (
        $(#[$meta:meta])*
        $glDllName:ident => $ignored_visibility:vis unsafe fn $name:ident( $( $arg_name:ident : $arg_ty:ty ),* $(,)? )
        $( -> $ret_ty:ty )?
        ;
        $($tail:tt)*
    ) => {
        $( #[$meta] )*
        pub unsafe fn $name ( &self, $( $arg_name : $arg_ty ),* ) $( -> $ret_ty )? {
            let mut cell = self.gl_procs.$name.borrow_mut();

            if let Some(proc) = *cell {
                // Proecedure is already loaded, so go ahead and call it
                proc($( $arg_name ),*)
            } else {
                println!(concat!("OpenGL function `", stringify!($glDllName), "` has not yet been loaded! Loading."));

                // Load the procedure's address into the refcell
                *cell = core::mem::transmute(
                    self.loader
                        .as_ref()
                        .expect(
                            concat!(
                                "Attempted to load `",
                                stringify!($glDllName),
                                "` without an active procedure loader."
                            )
                        )
                        .load_proc(c_str!(stringify!($glDllName)))
                );

                // Try calling the function. If loading failed somehow, then just panic (for now!).
                (cell.expect(
                    concat!(
                        "Loading OpenGL function `",
                        stringify!($glDllName),
                        "` failed."
                    )
                ))($( $arg_name ),*)
            }
        }

        impl_glcontext_proc_call! { $($tail)* }
    };
}

impl GlContext {
    pub fn new_with_loader(loader: Box<dyn GlProcLoader>) -> Self {
        Self {
            loader: Some(loader),
            ..Default::default()
        }
    }

    pub fn set_loader(&mut self, loader: Box<dyn GlProcLoader>) {
        self.loader = Some(loader);
    }

    impl_glcontext_proc_call! {
        /// Attach a shader to a program object.
        ///
        /// **See**: [`glAttachShader` on docs.gl](https://docs.gl/gl4/glAttachShader)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glAttachShader => unsafe fn gl_attach_shader(program: GLuint, shader: GLuint);

        /// Bind a named buffer object.
        ///
        /// **See**: [`glBindBuffer` on docs.gl](https://docs.gl/gl4/glBindBuffer)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glBindBuffer => unsafe fn gl_bind_buffer(target: GLenum, buffer: GLuint);

        /// Bind a vertex array object.
        ///
        /// **See**: [`glBindVertexArray` on docs.gl](https://docs.gl/gl4/glBindVertexArray)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glBindVertexArray => unsafe fn gl_bind_vertex_array(array: GLuint);

        /// Creates and initializes a buffer object's data store.
        ///
        /// **See**: [`glBufferData` on docs.gl](https://docs.gl/gl4/glBufferData)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glBufferData => unsafe fn gl_buffer_data(
            target: GLenum,
            size: GLsizeiptr,
            data: *const GLvoid,
            usage: GLenum,
        );

        /// Clear buffers to preset values.
        ///
        /// **See**: [`glClear` on docs.gl](https://docs.gl/gl4/glClear)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glClear => unsafe fn gl_clear(mask: GLbitfield);

        /// Compiles a shader object.
        ///
        /// **See**: [`glCompileShader` on docs.gl](https://docs.gl/gl4/glCompileShader)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glCompileShader => unsafe fn gl_compile_shader(shader: GLuint);

        /// Specify clear values for the colour buffers.
        ///
        /// **See**: [`glClearColor` on docs.gl](https://docs.gl/gl4/glClearColor)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glClearColor => unsafe fn gl_clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

        /// Creates a program object.
        ///
        /// **See**: [`glCreateProgram` on docs.gl](https://docs.gl/gl4/glCreateProgram)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glCreateProgram => unsafe fn gl_create_program() -> GLuint;

        /// Creates a shader object.
        ///
        /// **See**: [`glCreateShader` on docs.gl](https://docs.gl/gl4/glCreateShader)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glCreateShader => unsafe fn gl_create_shader(shader_type: GLenum) -> GLuint;

        /// Deletes a shader object
        ///
        /// **See**: [`glDeleteShader` on docs.gl](https://docs.gl/gl4/glDeleteShader)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glDeleteShader => unsafe fn gl_delete_shader(shader: GLuint);

        /// Render primitives from array data
        ///
        /// **See**: [`glDrawArrays` on docs.gl](https://docs.gl/gl4/glDrawArrays)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glDrawArrays => unsafe fn gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei);

        /// Render primitives from array data
        ///
        /// **See**: [`glDrawElements` on docs.gl](https://docs.gl/gl4/glDrawElements)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glDrawElements => unsafe fn gl_draw_elements(
            mode: GLenum,
            count: GLsizei,
            gltype: GLenum,
            indices: *const GLvoid
        );

        /// Enable or disable a generic vertex attribute array
        ///
        /// **See**: [`glEnableVertexAttribArray` on docs.gl](https://docs.gl/gl4/glEnableVertexAttribArray)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glEnableVertexAttribArray => unsafe fn gl_enable_vertex_attrib_array(index: GLuint);

        /// Generate buffer object names.
        ///
        /// **See**: [`glGenBuffers` on docs.gl](https://docs.gl/gl4/glGenBuffers)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glGenBuffers => unsafe fn gl_gen_buffers(n: GLsizei, buffers: *mut GLuint);

        /// Generate vertex array object names
        ///
        /// **See**: [`glGenVertexArrays` on docs.gl](https://docs.gl/gl4/glGenVertexArrays)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glGenVertexArrays => unsafe fn gl_gen_vertex_arrays(n: GLsizei, arrays: *mut GLuint);

        /// Returns the information log for a program object.
        ///
        /// **See**: [`glGetProgramInfoLog` on docs.gl](https://docs.gl/gl4/glGetProgramInfoLog)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glGetProgramInfoLog => unsafe fn gl_get_program_info_log(
            program: GLuint,
            max_length: GLsizei,
            length: *mut GLsizei,
            info_log: *mut GLchar,
        );

        /// Returns a parameter from a program object.
        ///
        /// **See**: [`glGetProgram` on docs.gl](https://docs.gl/gl4/glGetProgram)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glGetProgramiv => unsafe fn gl_get_program_iv(
            program: GLuint,
            pname: GLenum,
            params: *mut GLint
        );

        /// Returns the information log for a shader object.
        ///
        /// **See**: [`glGetShaderInfoLog` on docs.gl](https://docs.gl/gl4/glGetShaderInfoLog);
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glGetShaderInfoLog => unsafe fn gl_get_shader_info_log(
            shader: GLuint,
            max_length: GLsizei,
            length: *mut GLsizei,
            info_log: *mut GLchar,
        );

        /// Returns a paraneter from a shader object.
        ///
        /// **See**: [`glGetShader` on docs.gl](https://docs.gl/gl4/glGetShader)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glGetShaderiv => unsafe fn gl_get_shader_iv(
            shader: GLuint,
            pname: GLenum,
            params: *mut GLint
        );

        /// Links a program object.
        ///
        /// **See**: [`glLinkProgram` on docs.gl](https://docs.gl/gl4/glLinkProgram)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glLinkProgram => unsafe fn gl_link_program(program: GLuint);

        /// Replaces the source code in a shader object.
        ///
        /// **See**: [`glShaderSource` on docs.gl](https://docs.gl/gl4/glShaderSource)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glShaderSource => unsafe fn gl_shader_source(
            shader: GLuint,
            count: GLsizei,
            string: *const *const GLchar,
            length: *const GLint,
        );

        /// Installs a program object as part of current rendering state
        ///
        /// **See**: [`glUseProgram` on docs.gl](https://docs.gl/gl4/glUseProgram)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glUseProgram => unsafe fn gl_use_program(program: GLuint);

        /// Define an array of generic vertex attribute data
        ///
        /// **See**: [`glVertexAttribPointer` on docs.gl](https://docs.gl/gl4/glVertexAttribPointer)
        ///
        /// ## Safety
        ///
        /// - If this struct's GL proc loader gives incorrect addresses to OpenGL procedures, undefined behaviour
        ///   will occur.
        glVertexAttribPointer => unsafe fn gl_vertex_attrib_pointer(
            index: GLuint,
            size: GLint,
            gltype: GLenum,
            normalized: GLboolean,
            stride: GLsizei,
            pointer: *const GLvoid
        );
    }
}
