use gl::types::*;
use crate::{RGLResult, get_rgl_result};

#[derive(Copy, Clone, Debug)]
pub enum Primitive {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl Primitive {
    pub(crate) fn to_gl_code(self) -> GLenum {
        match self {
            Primitive::Points => gl::POINTS,
            Primitive::LineStrip => gl::LINE_STRIP,
            Primitive::LineLoop => gl::LINE_LOOP,
            Primitive::Lines => gl::LINES,
            Primitive::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            Primitive::LinesAdjacency => gl::LINES_ADJACENCY,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
            Primitive::TriangleFan => gl::TRIANGLE_FAN,
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            Primitive::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            Primitive::Patches => gl::PATCHES
        }
    }
}

pub fn draw_arrays(mode: Primitive, first: GLint, count: GLsizei) -> RGLResult<()> {
    unsafe {
        gl::DrawArrays(mode.to_gl_code(), first, count);
    }

    get_rgl_result(())
}

//////////////////////////////////////////////////////////////////////

mod private_element_index {
    pub trait SealedElementIndexType {}

    impl SealedElementIndexType for u8 {}
    impl SealedElementIndexType for u16 {}
    impl SealedElementIndexType for u32 {}
}

pub trait ElementIndexType : private_element_index::SealedElementIndexType {
    fn to_gl_code() -> GLenum;
}

impl ElementIndexType for u8 {
    fn to_gl_code() -> GLenum {
        gl::UNSIGNED_BYTE
    }
}

impl ElementIndexType for u16 {
    fn to_gl_code() -> GLenum {
        gl::UNSIGNED_SHORT
    }
}

impl ElementIndexType for u32 {
    fn to_gl_code() -> GLenum {
        gl::UNSIGNED_INT
    }
}

pub fn draw_elements<T>(mode: Primitive, count: GLsizei, indices: Option<&[T]>) -> RGLResult<()> where T: ElementIndexType {
    use std::ptr;
    use std::ffi::c_void;

    if let Some(index_data) = indices {
        unsafe {
            gl::DrawElements(mode.to_gl_code(), count, T::to_gl_code(), &index_data[0] as *const T as *const c_void);
        }
    }
    else {
        unsafe {
            gl::DrawElements(mode.to_gl_code(), count, T::to_gl_code(), ptr::null());
        }
    }

    get_rgl_result(())
}
