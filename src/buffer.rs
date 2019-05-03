use crate::get_rgl_result;
use crate::RGLResult;
use gl::types::*;
use std::ffi::c_void;

////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct VertexArrayObject {
    pub(crate) vao_id: GLuint,
}

impl VertexArrayObject {
    pub fn none() -> Self {
        VertexArrayObject { vao_id: 0 }
    }
}

pub fn gen_vertex_arrays(num: GLint) -> RGLResult<Vec<VertexArrayObject>> {
    if num < 1 {
        panic!("rgl: Cannot give this value argument to glGenVertexArrays");
    }

    let mut ids = vec![0; num as usize];

    unsafe {
        gl::GenVertexArrays(num, ids.as_mut_ptr());
    }

    let result = ids
        .drain(..)
        .map(|vao_id| VertexArrayObject { vao_id })
        .collect();
    get_rgl_result(result)
}

pub fn bind_vertex_array(vao: VertexArrayObject) -> RGLResult<()> {
    unsafe {
        gl::BindVertexArray(vao.vao_id);
    }

    get_rgl_result(())
}

////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum BufferTarget {
    ArrayBuffer,
    AtomicCounterBuffer,
    CopyReadBuffer,
    CopyWriteBuffer,
    DispatchIndirectBuffer,
    DrawIndirectBuffer,
    ElementArrayBuffer,
    PixelPackBuffer,
    PixelUnpackBuffer,
    QueryBuffer,
    ShaderStorageBuffer,
    TextureBuffer,
    TransformFeedbackBuffer,
    UniformBuffer,
}

impl BufferTarget {
    fn to_gl_code(self) -> GLenum {
        match self {
            BufferTarget::ArrayBuffer => gl::ARRAY_BUFFER,
            BufferTarget::AtomicCounterBuffer => gl::ATOMIC_COUNTER_BUFFER,
            BufferTarget::CopyReadBuffer => gl::COPY_READ_BUFFER,
            BufferTarget::CopyWriteBuffer => gl::COPY_WRITE_BUFFER,
            BufferTarget::DispatchIndirectBuffer => gl::DISPATCH_INDIRECT_BUFFER,
            BufferTarget::DrawIndirectBuffer => gl::DRAW_INDIRECT_BUFFER,
            BufferTarget::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
            BufferTarget::PixelPackBuffer => gl::PIXEL_PACK_BUFFER,
            BufferTarget::PixelUnpackBuffer => gl::PIXEL_UNPACK_BUFFER,
            BufferTarget::QueryBuffer => gl::QUERY_BUFFER,
            BufferTarget::ShaderStorageBuffer => gl::SHADER_STORAGE_BUFFER,
            BufferTarget::TextureBuffer => gl::TEXTURE_BUFFER,
            BufferTarget::TransformFeedbackBuffer => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferTarget::UniformBuffer => gl::UNIFORM_BUFFER,
        }
    }
}

////////////////////////////////////////////////////////////////////

pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

impl BufferUsage {
    fn to_gl_code(&self) -> GLenum {
        match *self {
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StreamRead => gl::STREAM_READ,
            BufferUsage::StreamCopy => gl::STREAM_COPY,
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::StaticCopy => gl::STATIC_COPY,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}

pub unsafe trait GLBufferPrimitive {}

unsafe impl GLBufferPrimitive for f32 {}
unsafe impl GLBufferPrimitive for i32 {}

pub trait GLBufferData {
    fn void_ptr(&self) -> *const c_void;
    fn size(&self) -> GLsizeiptr;
}

impl<T> GLBufferData for Vec<T>
where
    T: GLBufferPrimitive,
{
    fn void_ptr(&self) -> *const c_void {
        &self[0] as *const T as *const c_void
    }

    fn size(&self) -> GLsizeiptr {
        use std::mem;
        (self.len() * mem::size_of::<T>()) as GLsizeiptr
    }
}

////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct BufferObject {
    pub(crate) buffer_id: GLuint,
}

impl BufferObject {
    pub fn none() -> Self {
        BufferObject { buffer_id: 0 }
    }
}

pub fn gen_buffers(num: GLint) -> RGLResult<Vec<BufferObject>> {
    if num < 1 {
        panic!(
            "rgl: Invalid parameter {} to glGenBuffers, must be 1 or greater.",
            num
        );
    }

    let mut ids = vec![0; num as usize];

    unsafe {
        gl::GenBuffers(num, ids.as_mut_ptr());
    }

    let result = ids
        .drain(..)
        .map(|buffer_id| BufferObject { buffer_id })
        .collect();
    get_rgl_result(result)
}

pub fn bind_buffer(target: BufferTarget, buffer: BufferObject) -> RGLResult<()> {
    unsafe {
        gl::BindBuffer(buffer.buffer_id, target.to_gl_code());
    }

    get_rgl_result(())
}

pub fn buffer_data<T>(target: BufferTarget, data: &T, usage: BufferUsage) -> RGLResult<()>
where
    T: GLBufferData,
{
    unsafe {
        gl::BufferData(
            target.to_gl_code(),
            data.size(),
            data.void_ptr(),
            usage.to_gl_code(),
        );
    }

    get_rgl_result(())
}

pub fn named_buffer_data<T>(buffer: BufferObject, data: &T, usage: BufferUsage) -> RGLResult<()>
where
    T: GLBufferData,
{
    unsafe {
        gl::NamedBufferData(
            buffer.buffer_id,
            data.size(),
            data.void_ptr(),
            usage.to_gl_code(),
        );
    }

    get_rgl_result(())
}

////////////////////////////////////////////////////////////////////
