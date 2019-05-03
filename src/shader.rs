use gl::types::*;
use crate::RGLResult;
use crate::Error;
use crate::get_rgl_result;
use either::Either;

#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    fn to_gl_code(&self) -> GLenum {
        match *self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Shader {
    shader_id: GLuint,
    shader_type: ShaderType
}

pub enum ShaderObjectParameter {
    ShaderType,
    DeleteStatus,
    CompileStatus,
    InfoLogLength,
    ShaderSourceLength,
}

impl ShaderObjectParameter {
    fn to_gl_code(&self) -> GLenum {
        match *self {
            ShaderObjectParameter::ShaderType => gl::SHADER_TYPE,
            ShaderObjectParameter::DeleteStatus => gl::DELETE_STATUS,
            ShaderObjectParameter::CompileStatus => gl::COMPILE_STATUS,
            ShaderObjectParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ShaderObjectParameter::ShaderSourceLength => gl::SHADER_SOURCE_LENGTH,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ShaderProgram {
    pub(crate) program_id: GLuint,
}

pub enum ShaderProgramObjectParameter {
    DeleteStatus,
    LinkStatus,
    ValidateStatus,
    InfoLogLength,
    AttachedShaders,
    ActiveAttributes,
    ActiveAttributeMaxLength,
    ActiveUniforms,
    ActiveUniformMaxLength,
}

impl ShaderProgramObjectParameter {
    fn to_gl_code(&self) -> GLenum {
        match *self {
            ShaderProgramObjectParameter::DeleteStatus => gl::DELETE_STATUS,
            ShaderProgramObjectParameter::LinkStatus => gl::LINK_STATUS,
            ShaderProgramObjectParameter::ValidateStatus => gl::VALIDATE_STATUS,
            ShaderProgramObjectParameter::InfoLogLength => gl::INFO_LOG_LENGTH,
            ShaderProgramObjectParameter::AttachedShaders => gl::ATTACHED_SHADERS,
            ShaderProgramObjectParameter::ActiveAttributes => gl::ACTIVE_ATTRIBUTES,
            ShaderProgramObjectParameter::ActiveAttributeMaxLength => gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
            ShaderProgramObjectParameter::ActiveUniforms => gl::ACTIVE_UNIFORMS,
            ShaderProgramObjectParameter::ActiveUniformMaxLength => gl::ACTIVE_UNIFORM_MAX_LENGTH,
        }
    }
}

////////////////////////////////////////////////////////////////

// We ensure the GLenum value is valid already, so
// we don't return RGLResult in this case.
pub fn create_shader(shader_type: ShaderType) -> Shader {
    let shader_id = unsafe {
        gl::CreateShader(shader_type.to_gl_code())
    };
    Shader {
        shader_id,
        shader_type
    }
}

pub fn shader_source(shader: Shader, source: &str) -> RGLResult<()> {
    use std::ffi::CString;
    use std::ptr::null;

    let c_str_src = CString::new(source.as_bytes()).map_err(|_| ()).expect("Non utf-8 shader source.");

    unsafe {
        gl::ShaderSource(shader.shader_id, 1, &c_str_src.as_ptr(), null());
    }

    get_rgl_result(())
}

pub enum CompileShaderError {
    Standard(Vec<Error>),
    Compile(String),
}

pub fn compile_shader(shader: Shader) -> Result<(), CompileShaderError> {

    unsafe {
        gl::CompileShader(shader.shader_id);
    }

    let se_map = |e| { CompileShaderError::Standard(e) };

    let failure = get_shader_iv(shader, ShaderObjectParameter::CompileStatus).map_err(se_map)? == 0;

    if failure {
        let info_log_len = get_shader_iv(shader, ShaderObjectParameter::InfoLogLength).map_err(se_map)?;

        let log = if let Ok(success) = get_shader_info_log(shader, info_log_len) {
            success
        }
        else {
            String::from("Failed to retrieve info log.")
        };

        Err(CompileShaderError::Compile(log))
    }
    else {
        get_rgl_result(()).map_err(|e| CompileShaderError::Standard(e))
    }
}

pub fn is_shader(shader: Shader) -> bool {
    let result = unsafe {
        gl::IsShader(shader.shader_id)
    };

    result != 0
}

pub fn delete_shader(shader: Shader) -> RGLResult<()> {
    unsafe {
        gl::DeleteShader(shader.shader_id);
    }

    get_rgl_result(())
}

pub fn get_shader_iv(shader: Shader, pname: ShaderObjectParameter) -> RGLResult<GLint> {
    let mut result : GLint = 0;

    unsafe {
        gl::GetShaderiv(shader.shader_id, pname.to_gl_code(), &mut result);
    }

    get_rgl_result(result)
}

pub enum InfoLogError {
    InvalidLog,
    Standard(Vec<Error>),
}

pub fn get_shader_info_log(shader: Shader, len: GLsizei) -> Result<String, InfoLogError> {
    use std::ptr::null_mut;
    use std::ffi::CString;

    if len > 0 {
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);

        unsafe {
            gl::GetShaderInfoLog(shader.shader_id, len, null_mut(), buffer.as_mut_ptr() as *mut GLchar);
        }

        let c_str : CString = CString::new(buffer).map_err(|_| InfoLogError::InvalidLog)?;
        let data = c_str.into_string().map_err(|_| InfoLogError::InvalidLog)?;
        get_rgl_result(data).map_err(|e| InfoLogError::Standard(e))
    }
    else {
        Err(InfoLogError::Standard(vec![Error::InvalidValue]))
    }
}

pub fn create_program() -> ShaderProgram {
    let program_id = unsafe {
        gl::CreateProgram()
    };

    ShaderProgram {
        program_id
    }
}

pub fn attach_shader(program: ShaderProgram, shader: Shader) -> RGLResult<()> {
    unsafe {
        gl::AttachShader(program.program_id, shader.shader_id);
    }

    get_rgl_result(())
}

pub enum LinkProgramError {
    LinkFailure(String),
    Standard(Vec<Error>),
}

pub fn link_program(program: ShaderProgram) -> Result<(), LinkProgramError> {
    unsafe {
        gl::LinkProgram(program.program_id);
    }

    let se_map = |e| { LinkProgramError::Standard(e) };

    let failure = get_program_iv(program, ShaderProgramObjectParameter::LinkStatus).map_err(se_map)? == 0;

    if failure {
        let log_len = get_program_iv(program, ShaderProgramObjectParameter::InfoLogLength).map_err(se_map)?;

        let log = if let Ok(success) = get_program_info_log(program, log_len) {
            success
        }
        else {
            String::from("rgl: Failed to get program info log.")
        };

        Err(LinkProgramError::LinkFailure(log))
    }
    else {
        get_rgl_result(()).map_err(|e| LinkProgramError::Standard(e))
    }
}

pub fn get_program_iv(program: ShaderProgram, pname: ShaderProgramObjectParameter) -> RGLResult<GLint> {
    let mut result: GLint = 0;

    unsafe {
        gl::GetProgramiv(program.program_id, pname.to_gl_code(), &mut result);
    }

    get_rgl_result(result)
}

pub fn get_program_info_log(program: ShaderProgram, len: GLsizei) -> Result<String, InfoLogError> {
    use std::ptr::null_mut;
    use std::ffi::CString;

    if len > 0 {
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);

        unsafe {
            gl::GetProgramInfoLog(program.program_id, len, null_mut(), buffer.as_mut_ptr() as *mut GLchar);
        }

        let c_str = CString::new(buffer).map_err(|_| InfoLogError::InvalidLog)?;
        let data = c_str.into_string().map_err(|_| InfoLogError::InvalidLog)?;
        get_rgl_result(data).map_err(|e| InfoLogError::Standard(e))
    }
    else {
        Err(InfoLogError::Standard(vec![Error::InvalidValue]))
    }
}

pub fn use_program(program: ShaderProgram) -> RGLResult<()> {
    unsafe {
        gl::UseProgram(program.program_id);
    }

    get_rgl_result(())
}
