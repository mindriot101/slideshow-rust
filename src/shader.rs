extern crate gl;
use std::error::Error;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::cell::Cell;

use errors::Result;

#[derive(Debug)]
pub struct ShaderProgram {
    id: Cell<GLuint>,
    vertex_filename: String,
    fragment_filename: String,
}

#[derive(Debug)]
pub struct ActivatedShader {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new(
        vertex_filename: &str,
        fragment_filename: &str,
    ) -> Result<ShaderProgram> {
        let vertex_src: &str = &read_from_file(vertex_filename);
        let fragment_src: &str = &read_from_file(fragment_filename);

        let id = unsafe { create_shader_program(vertex_src, fragment_src)? };
        Ok(ShaderProgram {
            id: Cell::new(id),
            vertex_filename: vertex_filename.to_string(),
            fragment_filename: fragment_filename.to_string(),
        })
    }

    pub fn activate<F>(&self, f: F)
        where F: Fn(&ActivatedShader) {
            let activated_shader = ActivatedShader::new(self.id.get());
            f(&activated_shader);
    }

    pub fn reload(&self) {
        println!("Reloading shader ({} + {})", self.vertex_filename, self.fragment_filename);
        let vertex_src: &str = &read_from_file(&self.vertex_filename);
        let fragment_src: &str = &read_from_file(&self.fragment_filename);
        let id = unsafe { create_shader_program(vertex_src, fragment_src).expect("Could not create shader program") };
        self.id.set(id);
    }

}

impl Drop for ActivatedShader {
    fn drop(&mut self) {
        self.deactivate();
    }
}

impl ActivatedShader {

    pub fn new(id: GLuint) -> ActivatedShader {
        unsafe {
            gl::UseProgram(id);
        }
        ActivatedShader { id: id }
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_float4(&self, name: &str, v1: f32, v2: f32, v3: f32, v4: f32) -> Result<()> {
        let loc = self.location(name)?;
        unsafe {
            gl::Uniform4f(loc, v1, v2, v3, v4);
        }
        Ok(())
    }

    fn location(&self, name: &str) -> Result<GLint> {
        let c_name = CString::new(name)?;
        let loc = unsafe {
            gl::GetUniformLocation(self.id, c_name.as_ptr())
        };
        if loc == -1 {
            return Err(format!("Cannot find location {} in current shader", name).into());
        }

        Ok(loc)
    }
}

fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Could not read file");
    s
}

unsafe fn create_shader(src: &str, shader_type: GLuint) -> Result<GLuint> {
    let vertex_shader = gl::CreateShader(shader_type);
    let c_str_vert = CString::new(src.as_bytes()).expect("Could not create vertex shader c string");
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

        let s = info_log_to_str(info_log);
        return Err(format!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", s).into());
    }
    Ok(vertex_shader)
}

unsafe fn create_shader_program(
    vertex_src: &str,
    fragment_src: &str,
) -> Result<GLuint> {
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
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(
            format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            ).into(),
        );
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    Ok(shader_program)
}

unsafe fn update_shader_program(
    shader_program: GLuint,
    vertex_src: &str,
    fragment_src: &str,
) -> Result<()> {
    let vertex_shader = create_shader(vertex_src, gl::VERTEX_SHADER)?;
    let fragment_shader = create_shader(fragment_src, gl::FRAGMENT_SHADER)?;

    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(
            format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                info_log_to_str(info_log)
            ).into(),
        );
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);
    Ok(())
}

fn info_log_to_str(info_log: Vec<u8>) -> String {
    let s = str::from_utf8(&info_log).unwrap();
    let s: String = s.chars().filter(|c| *c != '\0').collect();
    let s = s.trim_right();
    s.to_string()
}
