use crate::get_rgl_result;
use crate::RGLResult;
use gl::types::*;
use std::marker::PhantomData;

////////////////////////////////////////////////////////////////////

pub struct ArrayBuffer<T> where T: private::ArrayBufferPermitted { marker: PhantomData<T> }
pub struct AtomicCounterBuffer;
pub struct CopyReadBuffer;
pub struct CopyWriteBuffer;
pub struct DispatchIndirectBuffer;
pub struct DrawIndirectBuffer;
pub struct ElementArrayBuffer<T> where T: private::ElementArrayBufferPermitted { marker: PhantomData<T> }
pub struct PixelPackBuffer;
pub struct PixelUnpackBuffer;
pub struct QueryBuffer;
pub struct ShaderStorageBuffer;
pub struct TextureBuffer;
pub struct TransformFeedbackBuffer;
pub struct UniformBuffer;

pub struct StreamDraw;
pub struct StreamRead;
pub struct StreamCopy;
pub struct StaticDraw;
pub struct StaticRead;
pub struct StaticCopy;
pub struct DynamicDraw;
pub struct DynamicRead;
pub struct DynamicCopy;

mod private {

    use super::{
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

        StreamDraw,
        StreamRead,
        StreamCopy,
        StaticDraw,
        StaticRead,
        StaticCopy,
        DynamicDraw,
        DynamicRead,
        DynamicCopy
    };

    ////////////////////////////////////////////////////////////

    pub trait PrivBufferType {}

    /////////////////////

    pub trait ArrayBufferPermitted {}

    impl ArrayBufferPermitted for f32 {}
    impl ArrayBufferPermitted for f64 {}

    impl<T> PrivBufferType for ArrayBuffer<T> where T: ArrayBufferPermitted {}

    /////////////////////

    impl PrivBufferType for AtomicCounterBuffer {}

    /////////////////////

    impl PrivBufferType for CopyReadBuffer {}

    /////////////////////

    impl PrivBufferType for CopyWriteBuffer {}

    /////////////////////

    impl PrivBufferType for DispatchIndirectBuffer {}

    /////////////////////

    impl PrivBufferType for DrawIndirectBuffer {}

    /////////////////////

    pub trait ElementArrayBufferPermitted {}

    impl ElementArrayBufferPermitted for u8 {}
    impl ElementArrayBufferPermitted for u16 {}
    impl ElementArrayBufferPermitted for u32 {}

    impl<T> PrivBufferType for ElementArrayBuffer<T> where T: ElementArrayBufferPermitted {}

    /////////////////////

    impl PrivBufferType for PixelPackBuffer {}

    /////////////////////

    impl PrivBufferType for PixelUnpackBuffer {}

    /////////////////////

    impl PrivBufferType for QueryBuffer {}

    /////////////////////

    impl PrivBufferType for ShaderStorageBuffer {}

    /////////////////////

    impl PrivBufferType for TextureBuffer {}

    /////////////////////

    impl PrivBufferType for TransformFeedbackBuffer {}

    /////////////////////

    impl PrivBufferType for UniformBuffer {}

    ////////////////////////////////////////////////////////////

    pub trait PrivBufferUsage {}

    impl PrivBufferUsage for StreamDraw {}
    impl PrivBufferUsage for StreamRead {}
    impl PrivBufferUsage for StreamCopy {}
    impl PrivBufferUsage for StaticDraw {}
    impl PrivBufferUsage for StaticRead {}
    impl PrivBufferUsage for StaticCopy {}
    impl PrivBufferUsage for DynamicDraw {}
    impl PrivBufferUsage for DynamicRead {}
    impl PrivBufferUsage for DynamicCopy {}
}

////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct VertexArrayObject {
    pub(crate) vao_id: GLuint,
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        if self.vao_id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao_id);
            }
        }
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

pub fn bind_vertex_array(vao_option: Option<&VertexArrayObject>) -> RGLResult<()> {

    if let Some(vao) = vao_option.as_ref() {
        unsafe {
            gl::BindVertexArray(vao.vao_id);
        }
    }
    else {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    get_rgl_result(())
}

////////////////////////////////////////////////////////////////////

pub trait BufferType<K> : private::PrivBufferType {
    fn to_gl_code() -> GLenum;
}

impl<T> BufferType<T> for ArrayBuffer<T> where T: private::ArrayBufferPermitted {
    fn to_gl_code() -> GLenum {
        gl::ARRAY_BUFFER
    }
}

impl<T> BufferType<T> for AtomicCounterBuffer {
    fn to_gl_code() -> GLenum {
        gl::ATOMIC_COUNTER_BUFFER
    }
}

impl<T> BufferType<T> for CopyReadBuffer {
    fn to_gl_code() -> GLenum {
        gl::COPY_READ_BUFFER
    }
}

impl<T> BufferType<T> for CopyWriteBuffer {
    fn to_gl_code() -> GLenum {
        gl::COPY_WRITE_BUFFER
    }
}

impl<T> BufferType<T> for DispatchIndirectBuffer {
    fn to_gl_code() -> GLenum {
        gl::DISPATCH_INDIRECT_BUFFER
    }
}

impl<T> BufferType<T> for DrawIndirectBuffer {
    fn to_gl_code() -> GLenum {
        gl::DRAW_INDIRECT_BUFFER
    }
}

impl<T> BufferType<T> for ElementArrayBuffer<T> where T: private::ElementArrayBufferPermitted {
    fn to_gl_code() -> GLenum {
        gl::ELEMENT_ARRAY_BUFFER
    }
}

impl<T> BufferType<T> for PixelPackBuffer {
    fn to_gl_code() -> GLenum {
        gl::PIXEL_PACK_BUFFER
    }
}

impl<T> BufferType<T> for PixelUnpackBuffer {
    fn to_gl_code() -> GLenum {
        gl::PIXEL_UNPACK_BUFFER
    }
}

impl<T> BufferType<T> for QueryBuffer {
    fn to_gl_code() -> GLenum {
        gl::QUERY_BUFFER
    }
}

impl<T> BufferType<T> for ShaderStorageBuffer {
    fn to_gl_code() -> GLenum {
        gl::SHADER_STORAGE_BUFFER
    }
}

impl<T> BufferType<T> for TextureBuffer {
    fn to_gl_code() -> GLenum {
        gl::TEXTURE_BUFFER
    }
}

impl<T> BufferType<T> for TransformFeedbackBuffer {
    fn to_gl_code() -> GLenum {
        gl::TRANSFORM_FEEDBACK_BUFFER
    }
}

impl<T> BufferType<T> for UniformBuffer {
    fn to_gl_code() -> GLenum {
        gl::UNIFORM_BUFFER
    }
}

////////////////////////////////////////////////////////////////////

pub trait BufferUsage : private::PrivBufferUsage {
    fn to_gl_code() -> GLenum;
}

impl BufferUsage for StreamDraw {
    fn to_gl_code() -> GLenum {
        gl::STREAM_DRAW
    }
}

impl BufferUsage for StreamRead {
    fn to_gl_code() -> GLenum {
        gl::STREAM_READ
    }
}

impl BufferUsage for StreamCopy {
    fn to_gl_code() -> GLenum {
        gl::STREAM_COPY
    }
}

impl BufferUsage for StaticDraw {
    fn to_gl_code() -> GLenum {
        gl::STATIC_DRAW
    }
}

impl BufferUsage for StaticRead {
    fn to_gl_code() -> GLenum {
        gl::STATIC_READ
    }
}

impl BufferUsage for StaticCopy {
    fn to_gl_code() -> GLenum {
        gl::STATIC_COPY
    }
}

impl BufferUsage for DynamicDraw {
    fn to_gl_code() -> GLenum {
        gl::DYNAMIC_DRAW
    }
}

impl BufferUsage for DynamicRead {
    fn to_gl_code() -> GLenum {
        gl::DYNAMIC_READ
    }
}

impl BufferUsage for DynamicCopy {
    fn to_gl_code() -> GLenum {
        gl::DYNAMIC_COPY
    }
}

////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct BufferObject<T, K, U> where T: BufferType<K>, U: BufferUsage {
    pub(crate) buffer_id: GLuint,
    buffer_type_marker: PhantomData<T>,
    buffer_data_type_marker: PhantomData<K>,
    buffer_usage_marker: PhantomData<U>,
}

impl<T, K, U> Drop for BufferObject<T, K, U> where T: BufferType<K>, U: BufferUsage {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer_id);
        }
    }
}

pub fn gen_buffers<T, K, U>(num: GLint) -> RGLResult<Vec<BufferObject<T, K, U>>> where T: BufferType<K>, U: BufferUsage {
    if num < 1 {
        panic!(
            "rgl: Invalid parameter {} to glGenBuffers, must be 1 or greater.",
            num
        );
    }

    let mut ids = vec![0u32; num as usize];

    unsafe {
        gl::GenBuffers(num, ids.as_mut_ptr());
    }

    let result = ids
        .drain(..)
        .map(|buffer_id| BufferObject { buffer_id, buffer_type_marker: PhantomData, buffer_data_type_marker: PhantomData, buffer_usage_marker: PhantomData })
        .collect();
    get_rgl_result(result)
}

pub fn bind_buffer<T, K, U>(buffer_option: Option<&BufferObject<T, K, U>>) -> RGLResult<()> where T: BufferType<K>, U: BufferUsage {
    if let Some(buffer) = buffer_option.as_ref() {
        unsafe {
            gl::BindBuffer(T::to_gl_code(), buffer.buffer_id);
        }
    }
    else {
        unsafe {
            gl::BindBuffer(T::to_gl_code(), 0);
        }
    }

    get_rgl_result(())
}

pub fn named_buffer_data<T, K, U>(buffer: &BufferObject<T, K, U>, data: &[K]) -> RGLResult<()> where T: BufferType<K>, U: BufferUsage {
    use std::mem;
    use std::ffi::c_void;

    unsafe {
        gl::NamedBufferData(
            buffer.buffer_id,
            (data.len() * mem::size_of::<K>()) as isize,
            &data[0] as *const K as *const c_void,
            U::to_gl_code()
        );
    }

    get_rgl_result(())
}

////////////////////////////////////////////////////////////////////
