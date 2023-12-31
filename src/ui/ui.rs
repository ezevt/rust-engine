use std::{cell::RefCell, rc::Rc};

use crate::math::*;

use crate::core::{GameEngine, MouseCode};
use crate::ui::UiRenderer;

pub enum CordinateType {
    Relative, // the element position is relative to its parent's size and the range of values inside the parent is [0, 1]
    Pixels,   // the element position is relative to the parent's center
}

#[derive(PartialEq)]
pub enum CordinateCenter {
    Min,
    Center,
    Max,
}

pub struct Cordinate {
    pub r#type: CordinateType,
    pub center: CordinateCenter,
    pub position: f32,
}

impl Cordinate {
    pub fn new(r#type: CordinateType, center: CordinateCenter, position: f32) -> Cordinate {
        Cordinate {
            r#type: r#type,
            center: center,
            position,
        }
    }

    pub fn default() -> Cordinate {
        Cordinate::new(CordinateType::Relative, CordinateCenter::Center, 0.5)
    }

    pub fn calculate_position(&self, parent_position: f32, parent_size: f32, size: f32) -> f32 {
        let mut position;

        match self.r#type {
            CordinateType::Relative => {
                position = parent_position + parent_size * (self.position - 0.5);
            }
            CordinateType::Pixels => match self.center {
                CordinateCenter::Min => {
                    position = parent_position - parent_size * 0.5 + self.position;
                }
                CordinateCenter::Center => {
                    position = parent_position + self.position;
                }
                CordinateCenter::Max => {
                    position = parent_position + parent_size * 0.5 - self.position;
                }
            },
        }

        if self.center == CordinateCenter::Min {
            position += size * 0.5;
        } else if self.center == CordinateCenter::Max {
            position -= size * 0.5;
        }

        return position;
    }
}

#[derive(PartialEq)]
pub enum DimensionType {
    Relative,
    Pixels,
    Aspect,
    Space,
}

pub struct Dimension {
    pub r#type: DimensionType,
    pub parameter: f32,
}

impl Dimension {
    pub fn new(r#type: DimensionType, parameter: f32) -> Dimension {
        Dimension { r#type, parameter }
    }

    pub fn default() -> Dimension {
        Dimension::new(DimensionType::Relative, 1.0)
    }

    pub fn calculate_render_size(&self, parent_size: f32) -> f32 {
        match self.r#type {
            DimensionType::Relative => parent_size * self.parameter,
            DimensionType::Pixels => self.parameter,
            DimensionType::Aspect => parent_size,
            DimensionType::Space => parent_size - self.parameter,
        }
    }
}

pub struct UiElementProps {
    pub children: Vec<Rc<RefCell<Box<dyn UiElement>>>>,
    pub x: Cordinate,
    pub y: Cordinate,
    pub width: Dimension,
    pub height: Dimension,

    pub active: bool,

    render_size: Vector2<f32>,
    render_position: Vector2<f32>,
}

pub trait UiElement {
    fn update(
        &mut self,
        parent_position: Vector2<f32>,
        parent_size: Vector2<f32>,
        ui_state: &UiState,
    );

    fn render(&self, renderer: &mut UiRenderer);

    fn push_child(&mut self, child: Box<dyn UiElement>) -> Rc<RefCell<Box<dyn UiElement>>>;

    fn calculate_render_size(
        &self,
        props: &UiElementProps,
        parent_size: Vector2<f32>,
    ) -> Vector2<f32> {
        if props.width.r#type == DimensionType::Aspect {
            let h = props.height.calculate_render_size(parent_size.y);
            return Vector2::new(h * props.width.parameter, h);
        } else if props.height.r#type == DimensionType::Aspect {
            let w = props.width.calculate_render_size(parent_size.x);
            return Vector2::new(w, w * props.height.parameter);
        } else {
            return Vector2::new(
                props.width.calculate_render_size(parent_size.x),
                props.height.calculate_render_size(parent_size.y),
            );
        }
    }

    fn calculate_render_position(
        &self,
        props: &UiElementProps,
        parent_parent_position: Vector2<f32>,
        parent_size: Vector2<f32>,
    ) -> Vector2<f32> {
        return Vector2::new(
            props.x.calculate_position(
                parent_parent_position.x,
                parent_size.x,
                props.render_size.x,
            ),
            props.y.calculate_position(
                parent_parent_position.y,
                parent_size.y,
                props.render_size.y,
            ),
        );
    }
}

pub struct UiBase {
    pub props: UiElementProps,
}

impl UiBase {
    pub fn new(x: Cordinate, y: Cordinate, width: Dimension, height: Dimension) -> UiBase {
        UiBase {
            props: UiElementProps {
                children: Vec::new(),
                x,
                y,
                width,
                height,

                active: true,

                render_size: Vector2::new(0.0, 0.0),
                render_position: Vector2::new(0.0, 0.0),
            },
        }
    }

    pub fn default() -> UiBase {
        UiBase::new(
            Cordinate::default(),
            Cordinate::default(),
            Dimension::default(),
            Dimension::default(),
        )
    }
}

impl UiElement for UiBase {
    fn update(
        &mut self,
        parent_position: Vector2<f32>,
        parent_size: Vector2<f32>,
        ui_state: &UiState,
    ) {
        self.props.render_size = self.calculate_render_size(&self.props, parent_size);
        self.props.render_position =
            self.calculate_render_position(&self.props, parent_position, parent_size);

        for child in self.props.children.iter_mut() {
            child
                .borrow_mut()
                .update(self.props.render_position, self.props.render_size, ui_state);
        }
    }

    fn render(&self, renderer: &mut UiRenderer) {
        for child in self.props.children.iter() {
            child.borrow_mut().render(renderer);
        }
    }

    fn push_child(&mut self, child: Box<dyn UiElement>) -> Rc<RefCell<Box<dyn UiElement>>> {
        let child_reference = Rc::new(RefCell::new(child));
        self.props.children.push(child_reference.clone());
        return child_reference;
    }
}

pub struct UiBox {
    pub props: UiElementProps,
    pub color: Vector4<f32>,
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub outline_color: Vector4<f32>,
}

impl UiBox {
    pub fn new(
        x: Cordinate,
        y: Cordinate,
        width: Dimension,
        height: Dimension,
        color: Vector4<f32>,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Vector4<f32>,
    ) -> UiBox {
        UiBox {
            props: UiElementProps {
                children: Vec::new(),
                x,
                y,
                width,
                height,

                active: true,

                render_size: Vector2::new(0.0, 0.0),
                render_position: Vector2::new(0.0, 0.0),
            },
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        }
    }
}

impl UiElement for UiBox {
    fn update(
        &mut self,
        parent_position: Vector2<f32>,
        parent_size: Vector2<f32>,
        ui_state: &UiState,
    ) {
        self.props.render_size = self.calculate_render_size(&self.props, parent_size);
        self.props.render_position =
            self.calculate_render_position(&self.props, parent_position, parent_size);

        for child in self.props.children.iter_mut() {
            child
                .borrow_mut()
                .update(self.props.render_position, self.props.render_size, ui_state);
        }
    }

    fn render(&self, renderer: &mut UiRenderer) {
        renderer.draw_rect(
            self.props.render_position,
            self.props.render_size,
            self.color,
            self.corner_radius,
            self.outline_thickness,
            self.outline_color,
            None,
        );

        for child in self.props.children.iter() {
            child.borrow_mut().render(renderer);
        }
    }

    fn push_child(&mut self, child: Box<dyn UiElement>) -> Rc<RefCell<Box<dyn UiElement>>> {
        let child_reference = Rc::new(RefCell::new(child));
        self.props.children.push(child_reference.clone());
        return child_reference;
    }
}

pub struct UiState {
    pub(crate) cursor_position: Vector2<f32>,
    pub(crate) right_click: bool,
    pub(crate) left_click: bool,
}

pub struct Ui {
    pub(crate) renderer: UiRenderer,
    pub base: UiBase,

    pub(crate) state: UiState,

    pub(crate) window_size: Vector2<f32>,
}

impl Ui {
    pub fn new(engine: &mut GameEngine) -> Ui {
        let window_size = engine.window.get_size();
        let window_size = Vector2::new(window_size.0 as f32, window_size.1 as f32);

        let cursor_position = engine.get_cursor_position();
        let cursor_position = Vector2::new(cursor_position.0, cursor_position.1);

        Ui {
            renderer: UiRenderer::new(),
            base: UiBase::default(),
            state: UiState {
                cursor_position,
                right_click: engine.get_mouse_button(MouseCode::ButtonRight),
                left_click: engine.get_mouse_button(MouseCode::ButtonLeft),
            },
            window_size,
        }
    }

    pub fn update(&mut self, engine: &mut GameEngine) {
        let window_size = engine.window.get_size();
        self.window_size = Vector2::new(window_size.0 as f32, window_size.1 as f32);

        self.base.update(
            Vector2::new(window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0),
            Vector2::new(window_size.0 as f32, window_size.1 as f32),
            &self.state,
        );
    }

    pub fn render(&mut self) {
        self.renderer.begin_frame(self.window_size.clone());

        self.base.render(&mut self.renderer);

        self.renderer.end_frame();
    }
}
