use crate::renderer::{
    BufferElement, BufferLayout, IndexBuffer, RenderCommand, ShaderDataType, ShaderProgram,
    Texture2D, VertexArray, VertexBuffer,
};

use cgmath::*;
use std::ffi::c_void;
use std::mem;
use std::sync::Arc;

#[repr(C)]
struct RectVertex {
    position: Vector2<f32>,
    color: Vector4<f32>,
    texture_coord: Vector2<f32>,
    texture_index: f32,
    size: Vector2<f32>,
    corner_radius: f32,
    outline_thickness: f32,
    outline_color: Vector4<f32>,
}

const MAX_RECTS: u32 = 10000;
const MAX_VERTICES: u32 = MAX_RECTS * 4;
const MAX_INDICES: u32 = MAX_RECTS * 6;

const RECT_VERTEX_POSITIONS: [Vector4<f32>; 4] = [
    Vector4::new(-0.5, -0.5, 0.0, 1.0),
    Vector4::new(0.5, -0.5, 0.0, 1.0),
    Vector4::new(0.5, 0.5, 0.0, 1.0),
    Vector4::new(-0.5, 0.5, 0.0, 1.0),
];

pub struct UiRenderer {
    rect_vertex_array: VertexArray,
    rect_vertex_buffer: VertexBuffer,
    rect_shader: ShaderProgram,

    white_texture: Texture2D,

    rect_index_count: u32,
    rect_vertices: Vec<RectVertex>,

    texture_slots: [u32; 32],
    texture_slot_index: u32,

    screen_matrix: Matrix4<f32>,
}

impl UiRenderer {
    pub fn new() -> Self {
        let mut renderer = UiRenderer {
            rect_vertex_array: VertexArray::new(),
            rect_vertex_buffer: VertexBuffer::new(
                MAX_VERTICES as usize * mem::size_of::<RectVertex>(),
            ),
            rect_shader: ShaderProgram::new("resources/rect.vert", "resources/rect.frag"),

            white_texture: Texture2D::new(1, 1),

            rect_index_count: 0,
            rect_vertices: Vec::with_capacity(MAX_VERTICES.try_into().unwrap()),

            texture_slots: [0; 32],
            texture_slot_index: 1, // 0 is for the white texture

            screen_matrix: Matrix4::identity(),
        };

        renderer.rect_vertex_array.bind();
        renderer
            .rect_vertex_buffer
            .set_layout(BufferLayout::new(vec![
                BufferElement::new(String::from("a_position"), ShaderDataType::Float2, None),
                BufferElement::new(String::from("a_color"), ShaderDataType::Float4, None),
                BufferElement::new(
                    String::from("a_texture_coord"),
                    ShaderDataType::Float2,
                    None,
                ),
                BufferElement::new(String::from("a_texture_index"), ShaderDataType::Float, None),
                BufferElement::new(String::from("a_size"), ShaderDataType::Float2, None),
                BufferElement::new(String::from("a_corner_radius"), ShaderDataType::Float, None),
                BufferElement::new(
                    String::from("a_outline_thickness"),
                    ShaderDataType::Float,
                    None,
                ),
                BufferElement::new(
                    String::from("a_outline_color"),
                    ShaderDataType::Float4,
                    None,
                ),
            ]));
        let rect_vertex_buffer_arc = Arc::new(renderer.rect_vertex_buffer.clone());

        renderer
            .rect_vertex_array
            .add_vertex_buffer(rect_vertex_buffer_arc);

        let mut rect_indices: Vec<u32> = Vec::with_capacity(MAX_INDICES as usize);

        let mut offset = 0;
        for _ in (0..MAX_INDICES).step_by(6) {
            rect_indices.push(offset + 0);
            rect_indices.push(offset + 1);
            rect_indices.push(offset + 2);

            rect_indices.push(offset + 2);
            rect_indices.push(offset + 3);
            rect_indices.push(offset + 0);

            offset += 4;
        }

        let rect_index_buffer = IndexBuffer::new(rect_indices);
        renderer
            .rect_vertex_array
            .set_index_buffer(Arc::new(rect_index_buffer.clone()));

        renderer.rect_shader.create_uniform("u_screen_matrix");

        let white_texture_data: u32 = 0xffffffff;
        renderer.white_texture.set_data(
            &white_texture_data as *const u32 as *const c_void,
            mem::size_of::<u32>(),
        );

        renderer.texture_slots[0] = renderer.white_texture.id;

        renderer
    }

    pub fn begin_frame(&mut self, window_size: Vector2<f32>) {
        self.screen_matrix =
            Matrix4::from_nonuniform_scale(2.0 / window_size.x, 2.0 / -window_size.y, 1.0)
                * Matrix4::from_translation(Vector3::new(
                    -window_size.x / 2.0,
                    -window_size.y / 2.0,
                    0.0,
                ));
        self.start_batch();
    }

    pub fn end_frame(&self) {
        self.flush();
    }

    fn start_batch(&mut self) {
        self.rect_index_count = 0;
        self.rect_vertices.clear();
        self.texture_slot_index = 1;
    }

    fn flush(&self) {
        if self.rect_vertices.len() == 0 {
            return;
        }

        let data_size = self.rect_vertices.len() * mem::size_of::<RectVertex>();
        self.rect_vertex_buffer
            .set_data(self.rect_vertices.as_ptr() as *const c_void, data_size);

        for i in 0..self.texture_slot_index {
            unsafe {
                gl::BindTextureUnit(i, self.texture_slots[i as usize]);
            }
        }

        self.rect_shader.bind();
        self.rect_shader
            .set_matrix4fv_uniform("u_screen_matrix", &self.screen_matrix);
        RenderCommand::draw_indexed(&self.rect_vertex_array, Some(self.rect_index_count));
    }

    pub fn next_batch(&mut self) {
        self.flush();
        self.start_batch();
    }

    pub fn draw_rect(
        &mut self,
        position: Vector2<f32>,
        size: Vector2<f32>,
        color: Vector4<f32>,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Vector4<f32>,
        texture: Option<&Texture2D>,
    ) {
        const RECT_VERTEX_COUNT: u32 = 4;
        const TEXTURE_COORDS: [Vector2<f32>; 4] = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ];

        if self.rect_index_count >= MAX_INDICES as u32 {
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

        for i in 0..RECT_VERTEX_COUNT {
            let vertex_position = RECT_VERTEX_POSITIONS[i as usize]
                .truncate()
                .truncate()
                .mul_element_wise(size)
                + position;

            let vertex = RectVertex {
                position: Vector2::new(vertex_position.x, vertex_position.y),
                color,
                texture_coord: TEXTURE_COORDS[i as usize],
                texture_index,
                size,
                corner_radius,
                outline_thickness: outline_thickness,
                outline_color: outline_color,
            };

            self.rect_vertices.push(vertex);
        }

        self.rect_index_count += 6;
    }
}
