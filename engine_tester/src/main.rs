use engine::core::{logger, Game, GameEngine, KeyCode};
use engine::renderer::{Camera, RenderCommand};
use engine::ui::*;

use engine::math::*;

struct MyGame {
    camera: Camera,
    player_position: Vector2<f32>,
    player_color: Vector4<f32>,
    ui: Option<Ui>,
}

const PLAYER_SPEED: f32 = 2.0;

impl Game for MyGame {
    fn init(&mut self, engine: &mut GameEngine) {
        RenderCommand::set_clear_color(0.06, 0.06, 0.06, 1.0);
        engine.get_window().set_vsync(true);

        self.ui = Some(Ui::new(engine));

        let ui = self.ui.as_mut().unwrap();

        let container = ui.base.push_child(Box::new(UiBox::new(
            Cordinate::default(),
            Cordinate::default(),
            Dimension::new(DimensionType::Relative, 0.5),
            Dimension::new(DimensionType::Relative, 0.5),
            Vector4::new(0.0, 0.0, 1.0, 1.0),
            20.0,
            3.0,
            Vector4::new(1.0, 1.0, 1.0, 1.0),
        )));

        let inner = container.borrow_mut().push_child(Box::new(UiBox::new(
            Cordinate::new(CordinateType::Relative, CordinateCenter::Min, 0.0),
            Cordinate::new(CordinateType::Relative, CordinateCenter::Min, 0.0),
            Dimension::new(DimensionType::Aspect, 1.0),
            Dimension::new(DimensionType::Relative, 0.3),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            10.0,
            3.0,
            Vector4::new(1.0, 1.0, 1.0, 1.0),
        )));
    }

    fn update(&mut self, engine: &mut GameEngine) {
        if engine.get_key(KeyCode::Escape) {
            engine.exit();
        }

        if engine.get_key(KeyCode::W) || engine.get_key(KeyCode::Up) {
            self.player_position.y += PLAYER_SPEED * engine.timestep();
        }
        if engine.get_key(KeyCode::S) || engine.get_key(KeyCode::Down) {
            self.player_position.y -= PLAYER_SPEED * engine.timestep();
        }
        if engine.get_key(KeyCode::A) || engine.get_key(KeyCode::Left) {
            self.player_position.x -= PLAYER_SPEED * engine.timestep();
        }
        if engine.get_key(KeyCode::D) || engine.get_key(KeyCode::Right) {
            self.player_position.x += PLAYER_SPEED * engine.timestep();
        }

        let (width, height) = engine.get_window().get_size();
        self.camera.set_viewport_size(width, height);

        println!("FPS: {}", 1.0 / engine.timestep());
    }

    fn draw(&mut self, engine: &mut GameEngine) {
        engine.renderer.begin_scene(&self.camera);

        engine.renderer.draw_quad(
            self.player_position,
            Vector2::new(1.0, 1.0),
            self.player_color,
            None,
        );

        engine.renderer.end_scene();

        let ui = self.ui.as_mut().unwrap();

        ui.update(engine);
        ui.render();
    }
}

fn main() {
    logger::init();

    let mut game = MyGame {
        camera: Camera::new(Vector2::new(0.0, 0.0), 10.0),
        player_position: Vector2::new(0.0, 0.0),
        player_color: Vector4::new(1.0, 0.0, 0.0, 1.0),
        ui: None,
    };

    game.run(800, 600, "Game");
}
