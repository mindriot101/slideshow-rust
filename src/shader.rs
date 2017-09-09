extern crate gl;
use std::error::Error;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<ShaderProgram, Box<Error>> {
        let id = unsafe {
            create_shader_program(vertex_src, fragment_src)?
        };
        Ok(ShaderProgram { id: id })
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}


unsafe fn create_shader(src: &str, shader_type: GLuint) -> Result<GLuint, Box<Error>> {
    let vertex_shader = gl::CreateShader(shader_type);
    let c_str_vert = CString::new(src.as_bytes()).expect(
        "Could not create vertex shader c string",
    );
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(vertex_shader);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            vertex_shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).expect("Cannot read info_log")
                ).into());
    }
    Ok(vertex_shader)
}

unsafe fn create_shader_program(vertex_src: &str, fragment_src: &str) -> Result<GLuint, Box<Error>> {
    let vertex_shader = create_shader(vertex_src, gl::VERTEX_SHADER)?;
    let fragment_shader = create_shader(fragment_src, gl::FRAGMENT_SHADER)?;

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        return Err(format!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()).into());
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    Ok(shader_program)
}
