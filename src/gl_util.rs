use gl;
use gl::types::{GLchar, GLint, GLuint};
use std;
use std::ffi::{CStr, CString};
use texture::Texture;

pub struct Shader {
    id: GLuint,
}
impl Shader {
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
    pub fn from_str(src: &str, shader_type: GLuint) -> Result<Shader, String> {
        let c_src = CString::new(src).unwrap();
        let id = compile_shader(&c_src, shader_type)?;
        Ok(id)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub fn create_c_string_buffer(len: usize) -> CString {
    let mut c_string_buffer = Vec::with_capacity(len + 1);
    c_string_buffer.extend(std::iter::repeat(b' ').take(len));
    unsafe { CString::from_vec_unchecked(c_string_buffer) }
}

pub fn compile_shader(shader_source: &CStr, shader_type: GLuint) -> Result<Shader, String> {
    let id = unsafe { gl::CreateShader(shader_type) };

    unsafe {
        gl::ShaderSource(id, 1, &shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    let mut success: GLint = 0; // Dummy value replaced by the GetShaderiv function.
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success != 0 {
        // 0 is false in C.
        Ok(Shader { id })
    } else {
        let mut len: GLint = 0; // Dummy value
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }        
        let mut error_buffer: CString = create_c_string_buffer(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error_buffer.as_ptr() as *mut GLchar,
            );
        }
        Err(error_buffer.to_string_lossy().into_owned())
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn id(&self) -> GLuint {
        self.id
    }
}

pub fn link_program(shaders: &[Shader]) -> Result<ShaderProgram, String> {
    unsafe {
        let program = ShaderProgram {
            id: gl::CreateProgram(),
        };
        for shader in shaders {
            gl::AttachShader(program.id(), shader.id());
        }
        gl::LinkProgram(program.id());
        for shader in shaders {
            gl::DetachShader(program.id(), shader.id());
        }
        let mut success: GLint = 0;
        gl::GetProgramiv(program.id(), gl::LINK_STATUS, &mut success);
        if success != 0 {
            // 0 is failure
            Ok(program)
        } else {
            let mut len: GLint = 0;
            gl::GetProgramiv(program.id(), gl::INFO_LOG_LENGTH, &mut len);
            let mut error_buffer = create_c_string_buffer(len as usize);
            gl::GetProgramInfoLog(
                program.id,
                len,
                std::ptr::null_mut(),
                error_buffer.as_ptr() as *mut GLchar,
            );
            Err(error_buffer.to_string_lossy().into_owned())
        }
    }
}

pub fn use_program(program: &ShaderProgram) {
    unsafe {
        gl::UseProgram(program.id);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform(&self, name: &CStr) -> GLint {
        unsafe { gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar) }
    }
}

use gl::types::*;
use gl_util;
use graphics::V2T2C4;
pub fn draw_textured_colored_quads(
    vertices: &[V2T2C4],
    shader_program: &gl_util::ShaderProgram,
    texture: &Texture,
    vertex_buffer: GLuint,
    vertex_attributes_array: GLuint,
) {
    gl_util::use_program(&shader_program);
    unsafe {
        texture.bind();
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<V2T2C4>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindVertexArray(vertex_attributes_array);
        gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as GLint);
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        texture.release();
        gl::UseProgram(0);
    }
}
