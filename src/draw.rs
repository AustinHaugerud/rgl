use gl::types::*;
use crate::{Primitive,  RGLResult, get_rgl_result};

pub fn draw_arrays<P>(first: GLint, count: GLsizei) -> RGLResult<()> where P: Primitive {
    unsafe {
        gl::DrawArrays(P::to_gl_code(), first, count);
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

pub fn draw_elements<P, T>(count: GLsizei, indices: Option<&[T]>) -> RGLResult<()> where T: ElementIndexType, P: Primitive {
    use std::ptr;
    use std::ffi::c_void;

    if let Some(index_data) = indices {
        unsafe {
            gl::DrawElements(P::to_gl_code(), count, T::to_gl_code(), &index_data[0] as *const T as *const c_void);
        }
    }
    else {
        unsafe {
            gl::DrawElements(P::to_gl_code(), count, T::to_gl_code(), ptr::null());
        }
    }

    get_rgl_result(())
}
