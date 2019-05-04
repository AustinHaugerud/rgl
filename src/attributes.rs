use crate::{get_rgl_result, RGLResult, VertexArrayObject};
use gl::types::*;

pub struct Vec1;
pub struct Vec2;
pub struct Vec3;
pub struct Vec4;

mod private {

    use super::{Vec1, Vec2, Vec3, Vec4};

    pub trait PrivAttributeType : Sized {}

    impl PrivAttributeType for i8 {}
    impl PrivAttributeType for u8 {}
    impl PrivAttributeType for i16 {}
    impl PrivAttributeType for u16 {}
    impl PrivAttributeType for i32 {}
    impl PrivAttributeType for u32 {}
    impl PrivAttributeType for f32 {}
    impl PrivAttributeType for f64 {}

    pub trait PrivAttributeVector {}

    impl PrivAttributeVector for Vec1 {}
    impl PrivAttributeVector for Vec2 {}
    impl PrivAttributeVector for Vec3 {}
    impl PrivAttributeVector for Vec4 {}
}

pub trait AttributeType : private::PrivAttributeType {
    fn to_gl_code() -> GLenum;
}

impl AttributeType for i8 {
    fn to_gl_code() -> GLenum {
        gl::BYTE
    }
}

impl AttributeType for u8 {
    fn to_gl_code() -> GLenum {
        gl::UNSIGNED_BYTE
    }
}

impl AttributeType for i16 {
    fn to_gl_code() -> GLenum {
        gl::SHORT
    }
}

impl AttributeType for u16 {
    fn to_gl_code() -> GLenum {
        gl::UNSIGNED_SHORT
    }
}

impl AttributeType for i32 {
    fn to_gl_code() -> GLenum {
        gl::INT
    }
}

impl AttributeType for u32 {
    fn to_gl_code() -> GLenum {
        gl::UNSIGNED_SHORT
    }
}

impl AttributeType for f32 {
    fn to_gl_code() -> GLenum {
        gl::FLOAT
    }
}

impl AttributeType for f64 {
    fn to_gl_code() -> GLenum {
        gl::DOUBLE
    }
}

pub trait AttributeVector : private::PrivAttributeVector {
    fn len() -> GLsizei;
}

impl AttributeVector for Vec1 {
    fn len() ->  GLsizei {
        1
    }
}

impl AttributeVector for Vec2 {
    fn len() -> GLsizei {
        2
    }
}

impl AttributeVector for Vec3 {
    fn len() -> GLsizei {
        3
    }
}

impl  AttributeVector for Vec4 {
    fn len() -> GLsizei {
        4
    }
}

pub fn vertex_attrib_pointer<S, T>(
    index: GLuint,
    normalized: bool,
) -> RGLResult<()> where S: AttributeVector, T: AttributeType {
    use std::ptr;
    use std::mem;

    unsafe {
        gl::VertexAttribPointer(
            index,
            S::len(),
            T::to_gl_code(),
            normalized as GLboolean,
            mem::size_of::<T>() as GLsizei * S::len(),
            ptr::null(),
        );
    }

    get_rgl_result(())
}

pub fn enable_vertex_attrib_array(index: GLuint) -> RGLResult<()> {
    unsafe {
        gl::EnableVertexAttribArray(index);
    }

    get_rgl_result(())
}

pub fn disable_vertex_attrib_array(index: GLuint) -> RGLResult<()> {
    unsafe {
        gl::DisableVertexAttribArray(index);
    }

    get_rgl_result(())
}

pub fn enable_vertex_array_attrib(vao: VertexArrayObject, index: GLuint) -> RGLResult<()> {
    unsafe {
        gl::EnableVertexArrayAttrib(vao.vao_id, index);
    }

    get_rgl_result(())
}

pub fn disable_vertex_array_attrib(vao: VertexArrayObject, index: GLuint) -> RGLResult<()> {
    unsafe {
        gl::DisableVertexArrayAttrib(vao.vao_id, index);
    }

    get_rgl_result(())
}
