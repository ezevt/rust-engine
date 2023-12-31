use crate::renderer::VertexArray;

pub struct RenderCommand;

impl RenderCommand {
    pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_indexed(vertex_array: &VertexArray, count: Option<u32>) {
        vertex_array.bind();
        let count_ = count.unwrap_or(vertex_array.get_index_buffer().get_count());
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                count_ as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
