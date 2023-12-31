use glfw::{ffi::glfwSwapInterval, Context, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    pub(crate) window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.make_current();

        return Window {
            glfw,
            window_handle: window,
            events,
        };
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn should_close(&self) -> bool {
        return self.window_handle.should_close();
    }

    pub fn set_should_close(&mut self, val: bool) {
        self.window_handle.set_should_close(val);
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                _ => {}
            }
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let (width, height) = self.window_handle.get_framebuffer_size();
        return (width as u32, height as u32);
    }

    pub fn get_content_scale(&self) -> (f32, f32) {
        return self.window_handle.get_content_scale();
    }

    pub fn get_time(&self) -> f32 {
        return self.glfw.get_time() as f32;
    }

    pub fn set_vsync(&mut self, val: bool) {
        unsafe {
            glfwSwapInterval(if val { 1 } else { 0 });
        }
    }
}
