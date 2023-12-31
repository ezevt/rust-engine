use crate::core::{GameEngine, MouseCode};
use crate::math::*;
use crate::renderer::Renderer;

#[derive(Debug)]
pub enum UiState {
    Inactive,
    Active,
    Hot,
    Fired,
}

pub enum UiLayoutKind {
    Horizontal,
    Vertical,
}

pub struct UiLayout {
    kind: UiLayoutKind,
    position: Vector2<f32>,
    size: Vector2<f32>,
    padding: f32,
}

impl UiLayout {
    pub fn new(
        kind: UiLayoutKind,
        position: Vector2<f32>,
        size: Vector2<f32>,
        padding: f32,
    ) -> UiLayout {
        UiLayout {
            kind,
            position,
            size,
            padding,
        }
    }

    pub fn available_position(&self) -> Vector2<f32> {
        match self.kind {
            UiLayoutKind::Horizontal => self.position + Vector2::new(self.size.x, 0.0),
            UiLayoutKind::Vertical => self.position + Vector2::new(0.0, self.size.y),
        }
    }

    pub fn push_widget(&mut self, widget_size: Vector2<f32>) {
        match self.kind {
            UiLayoutKind::Horizontal => {
                self.size.x += widget_size.x + self.padding;
                self.size.y = f32::max(widget_size.y, self.size.y);
            }
            UiLayoutKind::Vertical => {
                self.size.y += widget_size.y + self.padding;
                self.size.x = f32::max(widget_size.x, self.size.x);
            }
        }
    }
}

pub struct Ui {
    active_id: Option<u32>,
    hot_id: Option<u32>,

    layouts: Vec<UiLayout>,

    renderer: Renderer,

    cursor_position: Vector2<f32>,
    right_click: bool,
    left_click: bool,

    window_size: Vector2<f32>,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            active_id: None,
            hot_id: None,

            layouts: Vec::new(),

            renderer: Renderer::new(),

            cursor_position: Vector2::new(0.0, 0.0),
            right_click: false,
            left_click: false,

            window_size: Vector2::new(0.0, 0.0),
        }
    }

    pub fn update_state(&mut self, id: u32, rect: AABB) -> UiState {
        if self.active_id == Some(id) {
            if !self.left_click {
                self.active_id = None;
                if rect.contains(self.cursor_position) {
                    return UiState::Fired;
                }

                return UiState::Inactive;
            }
            return UiState::Active;
        } else if self.hot_id == Some(id) {
            if !rect.contains(self.cursor_position) {
                self.hot_id = None;
                return UiState::Inactive;
            } else if self.left_click && !self.active_id.is_some() {
                self.active_id = Some(id);
                return UiState::Hot;
            }

            return UiState::Hot;
        }

        if !self.active_id.is_some() && rect.contains(self.cursor_position) {
            self.hot_id = Some(id);
            return UiState::Hot;
        }

        UiState::Inactive
    }

    pub fn push_layout(&mut self, layout: UiLayout) {
        self.layouts.push(layout);
    }

    pub fn pop_layout(&mut self) -> UiLayout {
        self.layouts.pop().unwrap()
    }

    pub fn top_layout(&mut self) -> Option<&mut UiLayout> {
        if self.layouts.is_empty() {
            return None;
        }

        self.layouts.last_mut()
    }

    fn render_button(&mut self, rect: AABB, color: Vector4<f32>) {
        self.renderer
            .draw_quad(rect.center(), rect.size(), color, None);
    }

    pub fn new_frame(&mut self, engine: &mut GameEngine) {
        let (width, height) = engine.window.get_size();
        self.window_size = Vector2::new(width as f32, height as f32);

        let (x, y) = engine.get_cursor_pos();
        self.cursor_position = Vector2::new(x as f32, y as f32);

        self.right_click = engine.get_mouse_button(MouseCode::ButtonRight);
        self.left_click = engine.get_mouse_button(MouseCode::ButtonLeft);
    }

    pub fn begin(&mut self, position: Vector2<f32>, padding: f32) {
        self.renderer.begin_scene_with_matrix(
            Matrix4::from_nonuniform_scale(
                2.0 / self.window_size.x,
                2.0 / -self.window_size.y,
                1.0,
            ) * Matrix4::from_translation(Vector3::new(
                -self.window_size.x / 2.0,
                -self.window_size.y / 2.0,
                0.0,
            )),
        );

        let layout = UiLayout::new(
            UiLayoutKind::Horizontal,
            position,
            Vector2::new(0.0, 0.0),
            padding,
        );

        self.layouts.push(layout);
    }

    pub fn begin_layout(&mut self, kind: UiLayoutKind, padding: f32) {
        let previous_layout = self.top_layout().unwrap();

        let next = UiLayout::new(
            kind,
            previous_layout.available_position(),
            Vector2::new(0.0, 0.0),
            padding,
        );

        self.layouts.push(next);
    }

    pub fn end(&mut self) {
        self.pop_layout();
        self.renderer.end_scene();
    }

    pub fn end_layout(&mut self) {
        let child = self.pop_layout();
        let parent = self.top_layout().unwrap();
        parent.push_widget(child.size);
    }

    pub fn button(&mut self, size: Vector2<f32>, color: Vector4<f32>, id: u32) -> bool {
        let rect;
        let position;

        {
            let layout = self.top_layout().unwrap();
            position = layout.available_position();
        }

        rect = AABB::new(position, position + size);
        let state = self.update_state(id, rect);

        match state {
            UiState::Inactive => {
                self.render_button(rect, color);
                let layout = self.top_layout().unwrap();
                layout.push_widget(size);
                return false;
            }
            UiState::Hot => {
                self.render_button(rect, color + Vector4::from_value(0.3));
                let layout = self.top_layout().unwrap();
                layout.push_widget(size);
                return false;
            }
            UiState::Active => {
                self.render_button(rect, color + Vector4::from_value(0.6));
                let layout = self.top_layout().unwrap();
                layout.push_widget(size);
                return false;
            }
            UiState::Fired => {
                self.render_button(rect, color + Vector4::from_value(0.9));
                let layout = self.top_layout().unwrap();
                layout.push_widget(size);
                return true;
            }
        }
    }
}
