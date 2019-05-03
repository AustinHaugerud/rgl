use crate::{get_rgl_result, RGLResult, VertexArrayObject};
use gl::types::*;

pub enum AttributeType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
    Double,
}

impl AttributeType {
    fn to_gl_code(&self) -> GLenum {
        match *self {
            AttributeType::Byte => gl::BYTE,
            AttributeType::UnsignedByte => gl::UNSIGNED_BYTE,
            AttributeType::Short => gl::SHORT,
            AttributeType::UnsignedShort => gl::UNSIGNED_SHORT,
            AttributeType::Int => gl::INT,
            AttributeType::UnsignedInt => gl::UNSIGNED_INT,
            AttributeType::Float => gl::FLOAT,
            AttributeType::Double => gl::DOUBLE,
        }
    }

    fn get_stride(&self) -> GLsizei {
        use std::mem::size_of;
        match *self {
            AttributeType::Byte => size_of::<i8>() as GLsizei,
            AttributeType::UnsignedByte => size_of::<u8>() as GLsizei,
            AttributeType::Short => size_of::<i16>() as GLsizei,
            AttributeType::UnsignedShort => size_of::<u16>() as GLsizei,
            AttributeType::Int => size_of::<i32>() as GLsizei,
            AttributeType::UnsignedInt => size_of::<u32>() as GLsizei,
            AttributeType::Float => size_of::<f32>() as GLsizei,
            AttributeType::Double => size_of::<f64>() as GLsizei,
        }
    }
}

pub fn vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    attribute_type: AttributeType,
    normalized: bool,
) -> RGLResult<()> {
    use std::ptr;

    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            attribute_type.to_gl_code(),
            normalized as GLboolean,
            attribute_type.get_stride(),
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
