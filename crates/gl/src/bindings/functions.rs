//! Bindings to OpenGL functions.
//!
//! This module primarily consists of type declarations for pointers to OpenGL functions using the
//! `"system"` calling convention. However, it may also contain explicit links to functions in
//! shared libraries.

use super::typedefs::*;

/// Specify clear values for the colour buffers.
///
/// **See**: [`glClearColor` on docs.gl](https://docs.gl/gl4/glClearColor)
pub type glClearColor_t =
    Option<unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat)>;

/// Clear buffers to preset values.
///
/// **See**: [`glClear` on docs.gl](https://docs.gl/gl4/glClear)
pub type glClear_t = Option<unsafe extern "system" fn(mask: GLbitfield)>;
