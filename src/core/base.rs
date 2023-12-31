use crate::core::{KeyCode, MouseCode, Window};
use crate::renderer::{RenderCommand, Renderer};

pub struct GameEngine {
    ts: f32,
    pub(crate) window: Window,
    pub renderer: Renderer,
}

impl GameEngine {
    pub fn get_key(&self, key: KeyCode) -> bool {
        let code = unsafe { std::mem::transmute::<i32, glfw::Key>(key as i32) };
        let state = self.window.window_handle.get_key(code);

        return state == glfw::Action::Press || state == glfw::Action::Repeat;
    }

    pub fn hide_cursor(&mut self, hide: bool) {
        if hide {
            self.window
                .window_handle
                .set_cursor_mode(glfw::CursorMode::Hidden);
        } else {
            self.window
                .window_handle
                .set_cursor_mode(glfw::CursorMode::Normal);
        }
    }

    pub fn set_cursor_pos(&mut self, x: f32, y: f32) {
        self.window.window_handle.set_cursor_pos(x as f64, y as f64);
    }

    pub fn get_cursor_pos(&self) -> (f32, f32) {
        let (posx, posy) = self.window.window_handle.get_cursor_pos();
        return (posx as f32, posy as f32);
    }

    pub fn get_mouse_button(&self, button: MouseCode) -> bool {
        let code = unsafe { std::mem::transmute::<i32, glfw::MouseButton>(button as i32) };
        let state = self.window.window_handle.get_mouse_button(code);

        return state == glfw::Action::Press;
    }

    pub fn timestep(&self) -> f32 {
        return self.ts;
    }

    pub fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn exit(&mut self) {
        self.window.set_should_close(true);
    }
}

pub trait Game {
    fn init(&mut self, _engine: &mut GameEngine) {}
    fn update(&mut self, _engine: &mut GameEngine) {}
    fn draw(&mut self, _engine: &mut GameEngine) {}
    fn close(&mut self) {}

    fn run(&mut self, window_width: u32, window_height: u32, title: &str) {
        let mut last_frame_time = 0.0;

        let mut window = Window::new(window_width, window_height, title);
        window.init_gl();

        let renderer = Renderer::new();

        let mut game_engine = GameEngine {
            ts: 0.0,
            window: window,
            renderer: renderer,
        };

        self.init(&mut game_engine);

        while !game_engine.window.should_close() {
            let time = game_engine.window.get_time();
            game_engine.ts = time - last_frame_time;
            last_frame_time = time;

            self.update(&mut game_engine);

            RenderCommand::clear();
            self.draw(&mut game_engine);

            game_engine.window.update();
        }

        self.close();
    }
}
