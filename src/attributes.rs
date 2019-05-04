use crate::{get_rgl_result, RGLResult, VertexArrayObject};
use gl::types::*;
use std::mem;

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
    fn size() -> GLsizei {
        mem::size_of::<Self>() as GLsizei
    }
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
    fn len() -> GLint;
}

impl AttributeVector for Vec1 {
    fn len() ->  GLint {
        1
    }
}

impl AttributeVector for Vec2 {
    fn len() -> GLint {
        2
    }
}

impl AttributeVector for Vec3 {
    fn len() -> GLint {
        3
    }
}

impl  AttributeVector for Vec4 {
    fn len() -> GLint {
        4
    }
}

pub fn vertex_attrib_pointer<S, T>(
    index: GLuint,
    normalized: bool,
) -> RGLResult<()> where S: AttributeVector, T: AttributeType {
    use std::ptr;

    unsafe {
        gl::VertexAttribPointer(
            index,
            S::len(),
            T::to_gl_code(),
            normalized as GLboolean,
            T::size() * S::len(),
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
