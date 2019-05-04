use gl::types::GLenum;

pub struct Points;
pub struct LineStrip;
pub struct LineLoop;
pub struct Lines;
pub struct LineStripAdjacency;
pub struct LinesAdjacency;
pub struct TriangleStrip;
pub struct TriangleFan;
pub struct Triangles;
pub struct TriangleStripAdjacency;
pub struct TrianglesAdjacency;
pub struct Patches;

mod private {

    use super::{
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
        Patches
    };

    pub trait PrivPrimitive {}

    impl PrivPrimitive for Points {}
    impl PrivPrimitive for LineStrip {}
    impl PrivPrimitive for LineLoop {}
    impl PrivPrimitive for Lines {}
    impl PrivPrimitive for LineStripAdjacency {}
    impl PrivPrimitive for LinesAdjacency {}
    impl PrivPrimitive for TriangleStrip {}
    impl PrivPrimitive for TriangleFan {}
    impl PrivPrimitive for Triangles {}
    impl PrivPrimitive for TriangleStripAdjacency {}
    impl PrivPrimitive for TrianglesAdjacency {}
    impl PrivPrimitive for Patches {}
}

pub trait Primitive : private::PrivPrimitive {
    fn to_gl_code() -> GLenum;
}

impl Primitive for Points {
    fn to_gl_code() -> GLenum {
        gl::POINTS
    }
}

impl Primitive for LineStrip {
    fn to_gl_code() -> GLenum {
        gl::LINE_STRIP
    }
}

impl Primitive for LineLoop {
    fn to_gl_code() -> GLenum {
        gl::LINE_LOOP
    }
}

impl Primitive for Lines {
    fn to_gl_code() -> GLenum {
        gl::LINES
    }
}

impl Primitive for LineStripAdjacency {
    fn to_gl_code() -> GLenum {
        gl::LINE_STRIP_ADJACENCY
    }
}

impl Primitive for LinesAdjacency {
    fn to_gl_code() -> GLenum {
        gl::LINES_ADJACENCY
    }
}

impl Primitive for TriangleStrip {
    fn to_gl_code() -> GLenum {
        gl::TRIANGLE_STRIP
    }
}

impl Primitive for TriangleFan {
    fn to_gl_code() -> GLenum {
        gl::TRIANGLE_FAN
    }
}

impl Primitive for Triangles {
    fn to_gl_code() -> GLenum {
        gl::TRIANGLES
    }
}

impl Primitive for TriangleStripAdjacency {
    fn to_gl_code() -> GLenum {
        gl::TRIANGLE_STRIP_ADJACENCY
    }
}

impl Primitive for TrianglesAdjacency {
    fn to_gl_code() -> GLenum {
        gl::TRIANGLES_ADJACENCY
    }
}

impl Primitive for Patches {
    fn to_gl_code() ->  GLenum {
        gl::PATCHES
    }
}
