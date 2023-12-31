use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;

use std::io::Read;

use std::ptr;

use gl::types::*;

use cgmath::*;

fn compile_errors(shader: u32, shader_type: &str) {
    let mut has_compiled: GLint = 1;

    unsafe {
        if shader_type != "program" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut has_compiled);
            if has_compiled == 0 {
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                // allocate buffer of correct size
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                // fill it with len spaces
                buffer.extend([b' '].iter().cycle().take(len as usize));
                // convert buffer to CString
                let error: CString = CString::from_vec_unchecked(buffer);

                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
                panic!(
                    "Shader compilation error for: {} \n {}",
                    shader_type,
                    error.to_string_lossy()
                );
            }
        } else {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut has_compiled);
            if has_compiled == 0 {
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                // allocate buffer of correct size
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                // fill it with len spaces
                buffer.extend([b' '].iter().cycle().take(len as usize));
                // convert buffer to CString
                let error: CString = CString::from_vec_unchecked(buffer);

                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
                panic!(
                    "Shader linking error for: {} \n {}",
                    shader_type,
                    error.to_string_lossy()
                );
            }
        }
    }
}

pub struct ShaderProgram {
    program_handle: u32,
    uniform_ids: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_shader_path));

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .expect("Failed to read vertex shader");

        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .expect("Failed to read fragment shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            compile_errors(vertex_shader, "vertex");

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            compile_errors(fragment_shader, "fragment");

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            compile_errors(program_handle, "program");

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            return ShaderProgram {
                program_handle,
                uniform_ids: HashMap::new(),
            };
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.program_handle,
                CString::new(uniform_name).unwrap().as_ptr(),
            )
        };
        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids
                .insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            )
        }
    }

    pub fn set_vector3f_uniform(&self, uniform_name: &str, vector: &Vector3<f32>) {
        unsafe { gl::Uniform3f(self.uniform_ids[uniform_name], vector.x, vector.y, vector.z) }
    }

    pub fn set_vector2f_uniform(&self, uniform_name: &str, vector: &Vector2<f32>) {
        unsafe { gl::Uniform2f(self.uniform_ids[uniform_name], vector.x, vector.y) }
    }

    pub fn set_float_uniform(&self, uniform_name: &str, float: f32) {
        unsafe { gl::Uniform1f(self.uniform_ids[uniform_name], float) }
    }
}
