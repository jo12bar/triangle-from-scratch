//! OpenGL constants.
//!
//! Unless otherwise specified, all constants are from
//! [`gl.xml`](https://github.com/KhronosGroup/OpenGL-Registry/blob/main/xml/gl.xml)
//! in the OpenGL registry.

use super::{prelude::GLboolean, typedefs::GLenum};

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;

pub const GL_CURRENT_BIT: GLenum = 0x00000001;
pub const GL_POINT_BIT: GLenum = 0x00000002;
pub const GL_LINE_BIT: GLenum = 0x00000004;
pub const GL_POLYGON_BIT: GLenum = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT: GLenum = 0x00000010;
pub const GL_PIXEL_MODE_BIT: GLenum = 0x00000020;
pub const GL_LIGHTING_BIT: GLenum = 0x00000040;
pub const GL_FOG_BIT: GLenum = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT: GLenum = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const GL_VIEWPORT_BIT: GLenum = 0x00000800;
pub const GL_TRANSFORM_BIT: GLenum = 0x00001000;
pub const GL_ENABLE_BIT: GLenum = 0x00002000;
pub const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const GL_HINT_BIT: GLenum = 0x00008000;
pub const GL_EVAL_BIT: GLenum = 0x00010000;
pub const GL_LIST_BIT: GLenum = 0x00020000;
pub const GL_TEXTURE_BIT: GLenum = 0x00040000;
pub const GL_SCISSOR_BIT: GLenum = 0x00080000;
pub const GL_MULTISAMPLE_BIT: GLenum = 0x20000000;
pub const GL_MULTISAMPLE_BIT_ARB: GLenum = 0x20000000;
pub const GL_MULTISAMPLE_BIT_EXT: GLenum = 0x20000000;
pub const GL_MULTISAMPLE_BIT_3DFX: GLenum = 0x20000000;
pub const GL_ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;

pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
pub const GL_QUADS: GLenum = 0x0007;
pub const GL_QUADS_EXT: GLenum = 0x0007;
pub const GL_QUADS_OES: GLenum = 0x0007;
pub const GL_QUAD_STRIP: GLenum = 0x0008;
pub const GL_POLYGON: GLenum = 0x0009;
pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
pub const GL_LINES_ADJACENCY_ARB: GLenum = 0x000A;
pub const GL_LINES_ADJACENCY_EXT: GLenum = 0x000A;
pub const GL_LINES_ADJACENCY_OES: GLenum = 0x000A;
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const GL_LINE_STRIP_ADJACENCY_ARB: GLenum = 0x000B;
pub const GL_LINE_STRIP_ADJACENCY_EXT: GLenum = 0x000B;
pub const GL_LINE_STRIP_ADJACENCY_OES: GLenum = 0x000B;
pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const GL_TRIANGLES_ADJACENCY_ARB: GLenum = 0x000C;
pub const GL_TRIANGLES_ADJACENCY_EXT: GLenum = 0x000C;
pub const GL_TRIANGLES_ADJACENCY_OES: GLenum = 0x000C;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const GL_TRIANGLE_STRIP_ADJACENCY_ARB: GLenum = 0x000D;
pub const GL_TRIANGLE_STRIP_ADJACENCY_EXT: GLenum = 0x000D;
pub const GL_TRIANGLE_STRIP_ADJACENCY_OES: GLenum = 0x000D;
pub const GL_PATCHES: GLenum = 0x000E;
pub const GL_PATCHES_EXT: GLenum = 0x000E;
pub const GL_PATCHES_OES: GLenum = 0x000E;

pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_ARRAY_BUFFER_ARB: GLenum = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const GL_ELEMENT_ARRAY_BUFFER_ARB: GLenum = 0x8893;

pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_STATIC_DRAW_ARB: GLenum = 0x88E4;

pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_FRAGMENT_SHADER_ARB: GLenum = 0x8B30;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_VERTEX_SHADER_ARB: GLenum = 0x8B31;

pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_OBJECT_COMPILE_STATUS_ARB: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_OBJECT_LINK_STATUS_ARB: GLenum = 0x8B82;

pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_FLOAT: GLenum = 0x1406;
