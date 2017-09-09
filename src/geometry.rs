extern crate gl;
use gl::types::*;
use std::mem;
use std::ptr;
use std::os::raw::c_void;
use std::error::Error;

use shader::ShaderProgram;

pub struct Geometry<'a> {
    VAO: GLuint,
    program: &'a ShaderProgram,
}

impl<'a> Geometry<'a> {
    pub fn new(
        shader_program: &'a ShaderProgram,
        vertices: &[f32],
    ) -> Result<Geometry<'a>, Box<Error>> {
        let mut VAO = unsafe {

            let (mut VBO, mut VAO) = (0, 0);

            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            VAO
        };

        Ok(Geometry {
            VAO: VAO,
            program: shader_program,
        })
    }

    pub fn render(&self) {
        self.program.activate();
        unsafe {
            gl::BindVertexArray(self.VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        self.program.deactivate();
    }
}
