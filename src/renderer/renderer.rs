use crate::renderer::{
    BufferElement, BufferLayout, Camera, IndexBuffer, RenderCommand, ShaderDataType, ShaderProgram,
    Texture2D, VertexArray, VertexBuffer,
};

use cgmath::*;
use std::ffi::c_void;
use std::mem;
use std::sync::Arc;

#[repr(C)]
struct QuadVertex {
    position: Vector3<f32>,
    color: Vector4<f32>,
    texture_coord: Vector2<f32>,
    texture_index: f32,
}

const MAX_QUADS: u32 = 10000;
const MAX_VERTICES: u32 = MAX_QUADS * 4;
const MAX_INDICES: u32 = MAX_QUADS * 6;

const QUAD_VERTEX_POSITIONS: [Vector4<f32>; 4] = [
    Vector4::new(-0.5, -0.5, 0.0, 1.0),
    Vector4::new(0.5, -0.5, 0.0, 1.0),
    Vector4::new(0.5, 0.5, 0.0, 1.0),
    Vector4::new(-0.5, 0.5, 0.0, 1.0),
];

pub struct Renderer {
    quad_vertex_array: VertexArray,
    quad_vertex_buffer: VertexBuffer,
    quad_shader: ShaderProgram,

    white_texture: Texture2D,

    quad_index_count: u32,
    quad_vertices: Vec<QuadVertex>,

    texture_slots: [u32; 32],
    texture_slot_index: u32,

    view_projection: Matrix4<f32>,
}

impl Renderer {
    pub fn new() -> Self {
        let mut renderer = Renderer {
            quad_vertex_array: VertexArray::new(),
            quad_vertex_buffer: VertexBuffer::new(
                MAX_VERTICES as usize * mem::size_of::<QuadVertex>(),
            ),
            quad_shader: ShaderProgram::new("resources/quad.vert", "resources/quad.frag"),

            white_texture: Texture2D::new(1, 1),

            quad_index_count: 0,
            quad_vertices: Vec::with_capacity(MAX_VERTICES.try_into().unwrap()),

            texture_slots: [0; 32],
            texture_slot_index: 1, // 0 is for the white texture

            view_projection: Matrix4::identity(),
        };

        renderer.quad_vertex_array.bind();
        renderer
            .quad_vertex_buffer
            .set_layout(BufferLayout::new(vec![
                BufferElement::new(String::from("a_position"), ShaderDataType::Float3, None),
                BufferElement::new(String::from("a_color"), ShaderDataType::Float4, None),
                BufferElement::new(
                    String::from("a_texture_coord"),
                    ShaderDataType::Float2,
                    None,
                ),
                BufferElement::new(String::from("a_texture_index"), ShaderDataType::Float, None),
            ]));
        let quad_vertex_buffer_arc = Arc::new(renderer.quad_vertex_buffer.clone());

        renderer
            .quad_vertex_array
            .add_vertex_buffer(quad_vertex_buffer_arc);

        let mut quad_indices: Vec<u32> = Vec::with_capacity(MAX_INDICES as usize);

        let mut offset = 0;
        for _ in (0..MAX_INDICES).step_by(6) {
            quad_indices.push(offset + 0);
            quad_indices.push(offset + 1);
            quad_indices.push(offset + 2);

            quad_indices.push(offset + 2);
            quad_indices.push(offset + 3);
            quad_indices.push(offset + 0);

            offset += 4;
        }

        let quad_index_buffer = IndexBuffer::new(quad_indices);
        renderer
            .quad_vertex_array
            .set_index_buffer(Arc::new(quad_index_buffer.clone()));

        renderer.quad_shader.create_uniform("u_view_projection");

        let white_texture_data: u32 = 0xffffffff;
        renderer.white_texture.set_data(
            &white_texture_data as *const u32 as *const c_void,
            mem::size_of::<u32>(),
        );

        renderer.texture_slots[0] = renderer.white_texture.id;

        renderer
    }

    pub fn begin_scene(&mut self, camera: &Camera) {
        self.view_projection = camera.get_projection() * camera.get_view();

        self.start_batch();
    }

    pub fn begin_scene_with_matrix(&mut self, view_projection: Matrix4<f32>) {
        self.view_projection = view_projection;

        self.start_batch();
    }

    pub fn end_scene(&self) {
        self.flush();
    }

    fn start_batch(&mut self) {
        self.quad_index_count = 0;
        self.quad_vertices.clear();
        self.texture_slot_index = 1;
    }

    fn flush(&self) {
        if self.quad_vertices.len() == 0 {
            return;
        }

        let data_size = self.quad_vertices.len() * mem::size_of::<QuadVertex>();
        self.quad_vertex_buffer
            .set_data(self.quad_vertices.as_ptr() as *const c_void, data_size);

        for i in 0..self.texture_slot_index {
            unsafe {
                gl::BindTextureUnit(i, self.texture_slots[i as usize]);
            }
        }

        self.quad_shader.bind();
        self.quad_shader
            .set_matrix4fv_uniform("u_view_projection", &self.view_projection);
        RenderCommand::draw_indexed(&self.quad_vertex_array, Some(self.quad_index_count));
    }

    pub fn next_batch(&mut self) {
        self.flush();
        self.start_batch();
    }

    pub fn draw_quad(
        &mut self,
        position: Vector2<f32>,
        size: Vector2<f32>,
        color: Vector4<f32>,
        texture: Option<&Texture2D>,
    ) {
        const QUAD_VERTEX_COUNT: u32 = 4;
        const TEXTURE_COORDS: [Vector2<f32>; 4] = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ];

        if self.quad_index_count >= MAX_INDICES as u32 {
            self.next_batch();
        }

        let mut texture_index = 0.0;
        if texture.is_some() {
            let texture = texture.unwrap_or(&self.white_texture).id;

            for i in 0..self.texture_slot_index {
                if self.texture_slots[i as usize] == texture {
                    texture_index = i as f32;
                }
            }

            if texture_index == 0.0 {
                if self.texture_slot_index >= 32 as u32 {
                    self.next_batch();
                }

                texture_index = self.texture_slot_index as f32;
                self.texture_slots[self.texture_slot_index as usize] = texture;
                self.texture_slot_index += 1;
            }
        }

        for i in 0..QUAD_VERTEX_COUNT {
            let vertex_position = QUAD_VERTEX_POSITIONS[i as usize]
                .truncate()
                .truncate()
                .mul_element_wise(size)
                + position;

            let vertex = QuadVertex {
                position: Vector3::new(vertex_position.x, vertex_position.y, 0.0),
                color,
                texture_coord: TEXTURE_COORDS[i as usize],
                texture_index,
            };

            self.quad_vertices.push(vertex);
        }

        self.quad_index_count += 6;
    }
}
