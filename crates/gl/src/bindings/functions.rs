//! Bindings to OpenGL functions.
//!
//! This module primarily consists of type declarations for pointers to OpenGL functions using the
//! `"system"` calling convention. However, it may also contain explicit links to functions in
//! shared libraries.

use super::typedefs::*;

/// Attaches a shader object to a program object.
///
/// **See**: [`glAttachShader` on docs.gl](https://docs.gl/gl4/glAttachShader)
pub type glAttachShader_t = Option<unsafe extern "system" fn(program: GLuint, shader: GLuint)>;

/// Bind a named buffer object.
///
/// **See**: [`glBindBuffer` on docs.gl](https://docs.gl/gl4/glBindBuffer)
pub type glBindBuffer_t = Option<unsafe extern "system" fn(target: GLenum, buffer: GLuint)>;

/// Bind a vertex array object.
///
/// **See**: [`glBindVertexArray` on docs.gl](https://docs.gl/gl4/glBindVertexArray)
pub type glBindVertexArray_t = Option<unsafe extern "system" fn(array: GLuint)>;

/// Creates and initializes a buffer object's data store.
///
/// **See**: [`glBufferData` on docs.gl](https://docs.gl/gl4/glBufferData)
pub type glBufferData_t = Option<
    unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum),
>;

/// Clear buffers to preset values.
///
/// **See**: [`glClear` on docs.gl](https://docs.gl/gl4/glClear)
pub type glClear_t = Option<unsafe extern "system" fn(mask: GLbitfield)>;

/// Compiles a shader object.
///
/// **See**: [`glCompileShader` on docs.gl](https://docs.gl/gl4/glCompileShader)
pub type glCompileShader_t = Option<unsafe extern "system" fn(shader: GLuint)>;

/// Specify clear values for the colour buffers.
///
/// **See**: [`glClearColor` on docs.gl](https://docs.gl/gl4/glClearColor)
pub type glClearColor_t =
    Option<unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>;

/// Creates a program object.
///
/// **See**: [`glCreateProgram` on docs.gl](https://docs.gl/gl4/glCreateProgram)
pub type glCreateProgram_t = Option<unsafe extern "system" fn() -> GLuint>;

/// Creates a shader object.
///
/// **See**: [`glCreateShader` on docs.gl](https://docs.gl/gl4/glCreateShader)
pub type glCreateShader_t = Option<unsafe extern "system" fn(shaderType: GLenum) -> GLuint>;

/// Deletes a shader object
///
/// **See**: [`glDeleteShader` on docs.gl](https://docs.gl/gl4/glDeleteShader)
pub type glDeleteShader_t = Option<unsafe extern "system" fn(shader: GLuint)>;

/// Render primitives from array data
///
/// **See**: [`glDrawArrays` on docs.gl](https://docs.gl/gl4/glDrawArrays)
pub type glDrawArrays_t =
    Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei)>;

/// Render primitives from array data
///
/// **See**: [`glDrawElements` on docs.gl](https://docs.gl/gl4/glDrawElements)
pub type glDrawElements_t = Option<
    unsafe extern "system" fn(mode: GLenum, count: GLsizei, gltype: GLenum, indices: *const GLvoid),
>;

/// Enable or disable a generic vertex attribute array
///
/// **See**: [`glEnableVertexAttribArray` on docs.gl](https://docs.gl/gl4/glEnableVertexAttribArray)
pub type glEnableVertexAttribArray_t = Option<unsafe extern "system" fn(index: GLuint)>;

/// Generate buffer object names.
///
/// **See**: [`glGenBuffers` on docs.gl](https://docs.gl/gl4/glGenBuffers)
pub type glGenBuffers_t = Option<unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint)>;

/// Generate vertex array object names
///
/// **See**: [`glGenVertexArrays` on docs.gl](https://docs.gl/gl4/glGenVertexArrays)
pub type glGenVertexArrays_t = Option<unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint)>;

/// Returns the information log for a program object.
///
/// **See**: [`glGetProgramInfoLog` on docs.gl](https://docs.gl/gl4/glGetProgramInfoLog)
pub type glGetProgramInfoLog_t = Option<
    unsafe extern "system" fn(
        program: GLuint,
        maxLength: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ),
>;

/// Returns a parameter from a program object.
///
/// **See**: [`glGetProgram` on docs.gl](https://docs.gl/gl4/glGetProgram)
pub type glGetProgramiv_t =
    Option<unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint)>;

/// Returns the information log for a shader object.
///
/// **See**: [`glGetShaderInfoLog` on docs.gl](https://docs.gl/gl4/glGetShaderInfoLog);
pub type glGetShaderInfoLog_t = Option<
    unsafe extern "system" fn(
        shader: GLuint,
        maxLength: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    ),
>;

/// Returns a paraneter from a shader object.
///
/// **See**: [`glGetShader` on docs.gl](https://docs.gl/gl4/glGetShader)
pub type glGetShaderiv_t =
    Option<unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint)>;

/// Links a program object.
///
/// **See**: [`glLinkProgram` on docs.gl](https://docs.gl/gl4/glLinkProgram)
pub type glLinkProgram_t = Option<unsafe extern "system" fn(program: GLuint)>;

/// Replaces the source code in a shader object.
///
/// **See**: [`glShaderSource` on docs.gl](https://docs.gl/gl4/glShaderSource)
pub type glShaderSource_t = Option<
    unsafe extern "system" fn(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    ),
>;

/// Installs a program object as part of current rendering state
///
/// **See**: [`glUseProgram` on docs.gl](https://docs.gl/gl4/glUseProgram)
pub type glUseProgram_t = Option<unsafe extern "system" fn(program: GLuint)>;

/// Define an array of generic vertex attribute data
///
/// **See**: [`glVertexAttribPointer` on docs.gl](https://docs.gl/gl4/glVertexAttribPointer)
pub type glVertexAttribPointer_t = Option<
    unsafe extern "system" fn(
        index: GLuint,
        size: GLint,
        gltype: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const GLvoid,
    ),
>;
