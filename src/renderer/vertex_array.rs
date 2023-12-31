use gl::types::*;
use std::sync::Arc;

use crate::renderer::{IndexBuffer, ShaderDataType, VertexBuffer};

fn shader_data_type_to_gl_base_type(shader_data_type: &ShaderDataType) -> GLenum {
    match shader_data_type {
        ShaderDataType::Float => gl::FLOAT,
        ShaderDataType::Float2 => gl::FLOAT,
        ShaderDataType::Float3 => gl::FLOAT,
        ShaderDataType::Float4 => gl::FLOAT,
        ShaderDataType::Mat3 => gl::FLOAT,
        ShaderDataType::Mat4 => gl::FLOAT,
        ShaderDataType::Int => gl::INT,
        ShaderDataType::Int2 => gl::INT,
        ShaderDataType::Int3 => gl::INT,
        ShaderDataType::Int4 => gl::INT,
        ShaderDataType::Bool => gl::BOOL,
    }
}

pub struct VertexArray {
    id: u32,
    vertex_buffer_index: u32,
    vertex_buffers: Vec<Arc<VertexBuffer>>,
    index_buffer: Option<Arc<IndexBuffer>>,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        return VertexArray {
            id,
            vertex_buffer_index: 0,
            vertex_buffers: Vec::new(),
            index_buffer: None,
        };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn add_vertex_buffer(&mut self, vertex_buffer: Arc<VertexBuffer>) {
        self.bind();
        vertex_buffer.bind();

        let layout = vertex_buffer.get_layout();

        for element in layout.get_elements() {
            match element.data_type {
                ShaderDataType::Float
                | ShaderDataType::Float2
                | ShaderDataType::Float3
                | ShaderDataType::Float4 => unsafe {
                    gl::VertexAttribPointer(
                        self.vertex_buffer_index,
                        element.get_component_count() as i32,
                        shader_data_type_to_gl_base_type(&element.data_type),
                        gl::FALSE,
                        layout.get_stride() as i32,
                        element.offset as *const _,
                    );
                    gl::EnableVertexAttribArray(self.vertex_buffer_index);
                    self.vertex_buffer_index += 1;
                },
                ShaderDataType::Int
                | ShaderDataType::Int2
                | ShaderDataType::Int3
                | ShaderDataType::Int4
                | ShaderDataType::Bool => unsafe {
                    gl::EnableVertexAttribArray(self.vertex_buffer_index);
                    gl::VertexAttribPointer(
                        self.vertex_buffer_index,
                        element.get_component_count() as i32,
                        shader_data_type_to_gl_base_type(&element.data_type),
                        element.normalized as u8,
                        layout.get_stride() as i32,
                        element.offset as *const _,
                    );
                    self.vertex_buffer_index += 1;
                },
                ShaderDataType::Mat3 | ShaderDataType::Mat4 => unsafe {
                    let count = element.get_component_count();
                    for i in 0..count {
                        gl::EnableVertexAttribArray(self.vertex_buffer_index);
                        gl::VertexAttribPointer(
                            self.vertex_buffer_index,
                            count as i32,
                            shader_data_type_to_gl_base_type(&element.data_type),
                            element.normalized as u8,
                            layout.get_stride() as i32,
                            (element.offset as *const u8)
                                .add((std::mem::size_of::<f32>() as u32 * count * i) as usize)
                                as *const _,
                        );
                        gl::VertexAttribDivisor(self.vertex_buffer_index, 1);
                        self.vertex_buffer_index += 1;
                    }
                },
            }
        }
        self.vertex_buffers.push(vertex_buffer);
    }

    pub fn set_index_buffer(&mut self, index_buffer: Arc<IndexBuffer>) {
        self.bind();
        index_buffer.bind();
        self.index_buffer = Some(index_buffer);
    }

    pub fn get_index_buffer(&self) -> &Arc<IndexBuffer> {
        &self.index_buffer.as_ref().unwrap()
    }
}
