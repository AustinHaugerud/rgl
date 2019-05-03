use crate::ShaderProgram;
use crate::get_rgl_result;
use crate::RGLResult;
use std::ffi::CStr;
use gl::types::*;

#[derive(Copy, Clone, Debug)]
pub struct UniformLocation {
    pub(crate) loc: GLint
}

pub fn get_uniform_location(program: ShaderProgram, name: &CStr) -> RGLResult<UniformLocation> {
    let loc = unsafe {
        gl::GetUniformLocation(program.program_id, name.as_ptr())
    };

    let location = UniformLocation {
        loc
    };

    get_rgl_result(location)
}

pub fn uniform1f(location: UniformLocation, v0: f32) -> RGLResult<()> {
    unsafe {
        gl::Uniform1f(location.loc, v0);
    }

    get_rgl_result(())
}

pub fn uniform2f(location: UniformLocation, v0: f32, v1: f32) -> RGLResult<()> {
    unsafe {
        gl::Uniform2f(location.loc, v0, v1);
    }

    get_rgl_result(())
}

pub fn uniform3f(location: UniformLocation, v0: f32, v1: f32, v2: f32) -> RGLResult<()> {
    unsafe {
        gl::Uniform3f(location.loc, v0, v1, v2);
    }

    get_rgl_result(())
}

pub fn uniform4f(location: UniformLocation, v0: f32, v1: f32, v2: f32, v3: f32) -> RGLResult<()> {
    unsafe {
        gl::Uniform4f(location.loc, v0, v1, v2, v3);
    }

    get_rgl_result(())
}

pub fn uniform1i(location: UniformLocation, v0: i32) -> RGLResult<()> {
    unsafe {
        gl::Uniform1i(location.loc, v0);
    }

    get_rgl_result(())
}

pub fn uniform2i(location: UniformLocation, v0: i32, v1: i32) -> RGLResult<()> {
    unsafe {
        gl::Uniform2i(location.loc, v0, v1);
    }

    get_rgl_result(())
}

pub fn uniform3i(location: UniformLocation, v0: i32, v1: i32, v2: i32) -> RGLResult<()> {
    unsafe {
        gl::Uniform3i(location.loc, v0, v1, v2);
    }

    get_rgl_result(())
}

pub fn uniform4i(location: UniformLocation, v0: i32, v1: i32, v2: i32, v3: i32) -> RGLResult<()> {
    unsafe {
        gl::Uniform4i(location.loc, v0, v1, v2, v3);
    }

    get_rgl_result(())
}

pub fn uniform1ui(location: UniformLocation, v0: u32) -> RGLResult<()> {
    unsafe {
        gl::Uniform1ui(location.loc, v0);
    }

    get_rgl_result(())
}

pub fn uniform2ui(location: UniformLocation, v0: u32, v1: u32) -> RGLResult<()> {
    unsafe {
        gl::Uniform2ui(location.loc, v0, v1);
    }

    get_rgl_result(())
}

pub fn uniform3ui(location: UniformLocation, v0: u32, v1: u32, v2: u32) -> RGLResult<()> {
    unsafe {
        gl::Uniform3ui(location.loc, v0, v1, v2);
    }

    get_rgl_result(())
}

pub fn uniform4ui(location: UniformLocation, v0: u32, v1: u32, v2: u32, v3: u32) -> RGLResult<()> {
    unsafe {
        gl::Uniform4ui(location.loc, v0, v1, v2, v3);
    }

    get_rgl_result(())
}

pub fn uniform1fv(location: UniformLocation, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform1fv(location.loc, data.len() as GLsizei, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform2fv(location: UniformLocation, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform2fv(location.loc, (data.len() / 2) as GLsizei, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform3fv(location: UniformLocation, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform3fv(location.loc, (data.len() / 3) as GLsizei, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform4fv(location: UniformLocation, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform4fv(location.loc, (data.len() / 4) as GLsizei, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform1iv(location: UniformLocation, data: &[i32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform1iv(location.loc, data.len() as GLsizei, data.as_ptr() as *const GLint);
    }

    get_rgl_result(())
}

pub fn uniform2iv(location: UniformLocation, data: &[i32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform2iv(location.loc, (data.len() / 2) as GLsizei, data.as_ptr() as *const GLint);
    }

    get_rgl_result(())
}

pub fn uniform3iv(location: UniformLocation, data: &[i32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform3iv(location.loc, (data.len() / 3) as GLsizei, data.as_ptr() as *const GLint);
    }

    get_rgl_result(())
}

pub fn uniform4iv(location: UniformLocation, data: &[i32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform4iv(location.loc, (data.len() / 4) as GLsizei, data.as_ptr() as *const GLint);
    }

    get_rgl_result(())
}

pub fn uniform1uiv(location: UniformLocation, data: &[u32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform1uiv(location.loc, data.len() as GLsizei, data.as_ptr() as *const GLuint);
    }

    get_rgl_result(())
}

pub fn uniform2uiv(location: UniformLocation, data: &[u32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform2uiv(location.loc, (data.len() / 2) as GLsizei, data.as_ptr() as *const GLuint);
    }

    get_rgl_result(())
}

pub fn uniform3uiv(location: UniformLocation, data: &[u32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform3uiv(location.loc, (data.len() / 3) as GLsizei, data.as_ptr() as *const GLuint);
    }

    get_rgl_result(())
}

pub fn uniform4uiv(location: UniformLocation, data: &[u32]) -> RGLResult<()> {
    unsafe {
        gl::Uniform4uiv(location.loc, (data.len() / 4) as GLsizei, data.as_ptr() as *const GLuint);
    }

    get_rgl_result(())
}

pub fn uniform_matrix2fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix2fv(location.loc, (data.len() / (2 * 2)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix3fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix3fv(location.loc, (data.len() / (3 * 3)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix4fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix4fv(location.loc, (data.len() / (4 * 4)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix2x3fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix2x3fv(location.loc,  (data.len() / (2 * 3)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix3x2fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix3x2fv(location.loc,  (data.len() / (3 * 2)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix2x4fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix2x4fv(location.loc,  (data.len() / (2 * 4)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix4x2fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix4x2fv(location.loc,  (data.len() / (4 * 2)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix3x4fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix3x4fv(location.loc,  (data.len() / (3 * 4)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}

pub fn uniform_matrix4x3fv(location: UniformLocation, transpose: bool, data: &[f32]) -> RGLResult<()> {
    unsafe {
        gl::UniformMatrix4x3fv(location.loc,  (data.len() / (4 * 3)) as GLsizei, transpose as GLboolean, data.as_ptr() as *const GLfloat);
    }

    get_rgl_result(())
}
