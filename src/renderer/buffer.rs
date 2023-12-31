use gl::types::*;
use std::ffi::c_void;
use std::mem;
use std::ptr::null;

#[derive(Clone, Copy, Debug)]
pub enum ShaderDataType {
    Float,
    Float2,
    Float3,
    Float4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool,
}

fn shader_data_type_size(data_type: &ShaderDataType) -> u32 {
    match data_type {
        ShaderDataType::Float => 4,
        ShaderDataType::Float2 => 4 * 2,
        ShaderDataType::Float3 => 4 * 3,
        ShaderDataType::Float4 => 4 * 4,
        ShaderDataType::Mat3 => 4 * 3 * 3,
        ShaderDataType::Mat4 => 4 * 4 * 4,
        ShaderDataType::Int => 4,
        ShaderDataType::Int2 => 4 * 2,
        ShaderDataType::Int3 => 4 * 3,
        ShaderDataType::Int4 => 4 * 4,
        ShaderDataType::Bool => 1,
    }
}

pub struct BufferElement {
    pub name: String,
    pub data_type: ShaderDataType,
    pub size: u32,
    pub offset: u32,
    pub normalized: bool,
}

impl BufferElement {
    pub fn new(name: String, data_type: ShaderDataType, normalized: Option<bool>) -> Self {
        let size = shader_data_type_size(&data_type);
        BufferElement {
            name,
            data_type,
            size,
            offset: 0,
            normalized: normalized.unwrap_or(false),
        }
    }

    pub fn get_component_count(&self) -> u32 {
        match self.data_type {
            ShaderDataType::Float => 1,
            ShaderDataType::Float2 => 2,
            ShaderDataType::Float3 => 3,
            ShaderDataType::Float4 => 4,
            ShaderDataType::Mat3 => 3, // 3* float3
            ShaderDataType::Mat4 => 4, // 4* float4
            ShaderDataType::Int => 1,
            ShaderDataType::Int2 => 2,
            ShaderDataType::Int3 => 3,
            ShaderDataType::Int4 => 4,
            ShaderDataType::Bool => 1,
        }
    }
}

pub struct BufferLayout {
    elements: Vec<BufferElement>,
    stride: u32,
}

impl BufferLayout {
    pub fn new(elements: Vec<BufferElement>) -> Self {
        let mut buffer_layout = BufferLayout {
            elements,
            stride: 0,
        };

        buffer_layout.calculate_offset_and_stride();

        buffer_layout
    }

    pub fn get_elements(&self) -> &Vec<BufferElement> {
        &self.elements
    }

    fn calculate_offset_and_stride(&mut self) {
        let mut offset = 0;
        self.stride = 0;
        for element in &mut self.elements {
            element.offset = offset;
            offset += element.size;
            self.stride += element.size;
        }
    }

    pub fn get_stride(&self) -> u32 {
        self.stride
    }
}

pub struct VertexBuffer {
    id: u32,
    layout: BufferLayout,
}

impl VertexBuffer {
    pub fn new(size: usize) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size.try_into().unwrap(),
                null(),
                gl::DYNAMIC_DRAW,
            )
        }

        VertexBuffer {
            id,
            layout: BufferLayout::new(Vec::new()),
        }
    }

    pub fn set_layout(&mut self, layout: BufferLayout) {
        self.layout = layout;
    }

    pub fn get_layout(&self) -> &BufferLayout {
        &self.layout
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_data(&self, data: *const std::ffi::c_void, size: usize) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0 as GLintptr, size as GLsizeiptr, data);
        }
    }
}

pub struct IndexBuffer {
    id: u32,
    count: u32,
}

impl IndexBuffer {
    pub fn new(indices: Vec<u32>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);

            // GL_ELEMENT_ARRAY_BUFFER is not valid without an actively bound VAO
            // Binding with GL_ARRAY_BUFFER allows the data to be loaded regardless of VAO state.
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()).try_into().unwrap(),
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        IndexBuffer {
            id,
            count: indices.len() as u32,
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Clone for BufferElement {
    fn clone(&self) -> Self {
        BufferElement {
            name: self.name.clone(),
            data_type: self.data_type,
            size: self.size,
            offset: self.offset,
            normalized: self.normalized,
        }
    }
}

impl Clone for BufferLayout {
    fn clone(&self) -> Self {
        BufferLayout {
            elements: self.elements.clone(),
            stride: self.stride,
        }
    }
}

impl Clone for VertexBuffer {
    fn clone(&self) -> Self {
        VertexBuffer {
            id: self.id,
            layout: self.layout.clone(),
        }
    }
}

impl Clone for IndexBuffer {
    fn clone(&self) -> Self {
        IndexBuffer {
            id: self.id,
            count: self.count,
        }
    }
}
