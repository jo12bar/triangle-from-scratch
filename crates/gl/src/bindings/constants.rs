//! OpenGL constants.
//!
//! Unless otherwise specified, all constants are from
//! [`gl.xml`](https://github.com/KhronosGroup/OpenGL-Registry/blob/main/xml/gl.xml)
//! in the OpenGL registry.

use c_types::*;

pub const GL_CURRENT_BIT: CUInt = 0x00000001;
pub const GL_POINT_BIT: CUInt = 0x00000002;
pub const GL_LINE_BIT: CUInt = 0x00000004;
pub const GL_POLYGON_BIT: CUInt = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT: CUInt = 0x00000010;
pub const GL_PIXEL_MODE_BIT: CUInt = 0x00000020;
pub const GL_LIGHTING_BIT: CUInt = 0x00000040;
pub const GL_FOG_BIT: CUInt = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT: CUInt = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT: CUInt = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT: CUInt = 0x00000400;
pub const GL_VIEWPORT_BIT: CUInt = 0x00000800;
pub const GL_TRANSFORM_BIT: CUInt = 0x00001000;
pub const GL_ENABLE_BIT: CUInt = 0x00002000;
pub const GL_COLOR_BUFFER_BIT: CUInt = 0x00004000;
pub const GL_HINT_BIT: CUInt = 0x00008000;
pub const GL_EVAL_BIT: CUInt = 0x00010000;
pub const GL_LIST_BIT: CUInt = 0x00020000;
pub const GL_TEXTURE_BIT: CUInt = 0x00040000;
pub const GL_SCISSOR_BIT: CUInt = 0x00080000;
pub const GL_MULTISAMPLE_BIT: CUInt = 0x20000000;
pub const GL_MULTISAMPLE_BIT_ARB: CUInt = 0x20000000;
pub const GL_MULTISAMPLE_BIT_EXT: CUInt = 0x20000000;
pub const GL_MULTISAMPLE_BIT_3DFX: CUInt = 0x20000000;
pub const GL_ALL_ATTRIB_BITS: CUInt = 0xFFFFFFFF;
