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
        self.ui = Some(Ui::new());
        engine.get_window().set_vsync(true);
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
        let ui = self.ui.as_mut().unwrap();
        ui.new_frame(engine);

        ui.begin(Vector2::new(0.0, 0.0), 10.0);
        ui.begin_layout(UiLayoutKind::Vertical, 10.0);

        if ui.button(
            Vector2::new(50.0, 50.0),
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            0,
        ) {
            self.player_color = Vector4::new(1.0, 0.0, 0.0, 1.0);
        }
        if ui.button(
            Vector2::new(50.0, 50.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            1,
        ) {
            self.player_color = Vector4::new(0.0, 1.0, 0.0, 1.0);
        }
        if ui.button(
            Vector2::new(50.0, 50.0),
            Vector4::new(0.0, 0.0, 1.0, 1.0),
            2,
        ) {
            self.player_color = Vector4::new(0.0, 0.0, 1.0, 1.0);
        }

        ui.end_layout();
        ui.end();

        engine.renderer.begin_scene(&self.camera);

        engine.renderer.draw_quad(
            self.player_position,
            Vector2::new(1.0, 1.0),
            self.player_color,
            None,
        );

        engine.renderer.end_scene();
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
