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
