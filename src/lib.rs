extern crate gl;
use gl::types::*;

mod attributes;
mod buffer;
mod shader;
mod uniform;

pub use attributes::*;
pub use buffer::*;
pub use shader::*;
pub use uniform::*;

pub type RGLResult<T> = Result<T, Vec<Error>>;

///////////////////////////////////////////////////////////////////////////////

pub enum BufferType {
    ColorBuffer,
    DepthBuffer,
    StencilBuffer,
}

impl BufferType {
    fn to_gl_bit(&self) -> GLbitfield {
        match *self {
            BufferType::ColorBuffer => gl::COLOR_BUFFER_BIT,
            BufferType::DepthBuffer => gl::DEPTH_BUFFER_BIT,
            BufferType::StencilBuffer => gl::STENCIL_BUFFER_BIT,
        }
    }
}

////////////////////////////////////////////////////

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}

////////////////////////////////////////////////////

pub fn clear(buffer: BufferType) {
    unsafe {
        gl::Clear(buffer.to_gl_bit());
    }
}

////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Error {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow,
    ContextLost,
}

impl Error {
    fn from_gl_code(code: GLenum) -> Result<Error, ()> {
        match code {
            gl::NO_ERROR => Ok(Error::NoError),
            gl::INVALID_ENUM => Ok(Error::InvalidEnum),
            gl::INVALID_VALUE => Ok(Error::InvalidValue),
            gl::INVALID_OPERATION => Ok(Error::InvalidOperation),
            gl::INVALID_FRAMEBUFFER_OPERATION => Ok(Error::InvalidFramebufferOperation),
            gl::OUT_OF_MEMORY => Ok(Error::OutOfMemory),
            gl::STACK_UNDERFLOW => Ok(Error::StackUnderflow),
            gl::STACK_OVERFLOW => Ok(Error::StackOverflow),
            gl::CONTEXT_LOST => Ok(Error::ContextLost),
            _ => Err(()),
        }
    }
}

fn make_rgl_result<T>(success: T, errors: Vec<Error>) -> RGLResult<T> {
    if errors.is_empty() {
        Ok(success)
    } else {
        Err(errors)
    }
}

pub fn get_error() -> Result<Error, ()> {
    let code = unsafe { gl::GetError() };

    Error::from_gl_code(code)
}

pub fn get_error_unchecked() -> Error {
    get_error().expect("rgl: Unexpected error code.")
}

pub fn get_errors_unchecked() -> Vec<Error> {
    let mut errors = vec![];

    loop {
        let error = get_error_unchecked();
        if error != Error::NoError {
            errors.push(error);
        } else {
            break;
        }
    }

    errors
}

pub fn get_rgl_result<T>(success: T) -> RGLResult<T> {
    let errors = get_errors_unchecked();
    make_rgl_result(success, errors)
}

////////////////////////////////////////////////////
