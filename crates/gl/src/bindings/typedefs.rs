//! OpenGL type definitions.
//!
//! Unless otherwise specified, all type definitions are from
//! [`gl.xml`](https://github.com/KhronosGroup/OpenGL-Registry/blob/main/xml/gl.xml)
//! in the OpenGL registry.

use core::ffi::c_void;

use c_types::*;

pub type GLenum = CUInt;

pub type GLboolean = CUChar;

pub type GLbitfield = CUInt;

/// > Not an actual GL type, though used in headers in the past
pub type GLvoid = c_void;

pub type GLbyte = khronos_int8_t;
pub type GLubyte = khronos_uint8_t;

pub type GLshort = khronos_int16_t;
pub type GLushort = khronos_uint16_t;

pub type GLint = CInt;
pub type GLuint = CUInt;

pub type GLclampx = khronos_int32_t;

pub type GLsizei = CInt;

pub type GLfloat = khronos_float_t;
pub type GLclampf = khronos_float_t;

pub type GLdouble = CDouble;
pub type GLclampd = CDouble;

pub type GLeglClientBufferEXT = *mut c_void;
pub type GLeglImageOES = *mut c_void;

pub type GLchar = CChar;
pub type GLcharARB = CChar;

#[cfg(target_vendor = "apple")]
pub type GLhandleARB = *mut c_void;
#[cfg(not(target_vendor = "apple"))]
pub type GLhandleARB = CUInt;

pub type GLhalf = khronos_uint16_t;
pub type GLhalfARB = khronos_uint16_t;

pub type GLfixed = khronos_int32_t;

pub type GLintptr = khronos_intptr_t;
pub type GLintptrARB = khronos_intptr_t;

pub type GLsizeiptr = khronos_ssize_t;
pub type GLsizeiptrARB = khronos_ssize_t;

pub type GLint64 = khronos_int64_t;
pub type GLint64EXT = khronos_int64_t;

pub type GLuint64 = khronos_uint64_t;
pub type GLuint64EXT = khronos_uint64_t;

pub type GLDEBUGPROC = Option<
    unsafe extern "system" fn(
        source: GLenum,
        gltype: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *const c_void,
    ),
>;
pub type GLDEBUGPROCARB = GLDEBUGPROC;
pub type GLDEBUGPROCKHR = GLDEBUGPROC;

/// This is a vendor extension type for AMD platforms.
pub type GLDEBUGPROCAMD = Option<
    unsafe extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut c_void,
    ),
>;

/// This is a vendor extension type for Nvidia platforms.
pub type GLhalfNV = CUShort;
/// This is a vendor extension type for Nvidia platforms.
pub type GLvdpauSurfaceNV = GLintptr;

/// This is a vendor extension type for Nvidia platforms.
pub type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;

use khrplatform_h::*;
/// Types defined in [`khrplatform.h`](https://www.khronos.org/registry/EGL/api/KHR/khrplatform.h),
/// which is maintained in the [EGL Registry](http://www.khronos.org/registry/EGL/).
pub mod khrplatform_h {
    use c_types::*;

    pub type khronos_int8_t = CChar;
    pub type khronos_int16_t = CShort;
    pub type khronos_int32_t = i32;
    pub type khronos_int64_t = i64;

    pub type khronos_uint8_t = CUChar;
    pub type khronos_uint16_t = CUShort;
    pub type khronos_uint32_t = u32;
    pub type khronos_uint64_t = u64;

    pub type khronos_intptr_t = isize;
    pub type khronos_uintptr_t = usize;

    pub type khronos_ssize_t = isize;
    pub type khronos_usize_t = usize;

    pub type khronos_float_t = CFloat;

    pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
    pub type khronos_stime_nanoseconds_t = khronos_int64_t;
}
