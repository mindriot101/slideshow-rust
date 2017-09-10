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
    vertices: Option<&'a [f32]>,
    indices: Option<&'a [u32]>,
}

impl<'a> Geometry<'a> {
    pub fn new(shader_program: &'a ShaderProgram) -> Geometry<'a> {
        Geometry { VAO: 0, program: shader_program, vertices: None, indices: None }
    }

    pub fn add_vertices(mut self, vertices: &'a [f32]) -> Geometry<'a> {
        self.vertices = Some(vertices);
        self
    }

    pub fn add_indices(mut self, indices: &'a [u32]) -> Geometry<'a> {
        self.indices = Some(indices);
        self
    }

    pub fn build(mut self) -> Result<Geometry<'a>, Box<::std::error::Error>> {
        /* Initial checks */
        if let None = self.vertices {
            return Err("No vertices supplied".into())
        }

        let mut VAO = unsafe {
            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);

            let vertices = self.vertices.unwrap();

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

            if let Some(indices) = self.indices {
                gl::GenBuffers(1, &mut EBO);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &indices[0] as *const u32 as *const c_void,
                    gl::STATIC_DRAW,
                    );
            }

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

        self.VAO = VAO;
        Ok(self)
    }

    pub fn render(&self) {
        self.program.activate();
        unsafe {
            gl::BindVertexArray(self.VAO);
            if let Some(indices) = self.indices {
                gl::DrawElements(gl::TRIANGLES, indices.len() as _, gl::UNSIGNED_INT, ptr::null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
        self.program.deactivate();
    }
}
