use cgmath::*;

pub struct Camera {
    pub size: f32,
    pub near: f32,
    pub far: f32,

    pub position: Vector2<f32>,
    pub rotation: f32,

    aspect_ratio: f32,

    projection: Matrix4<f32>,
    view: Matrix4<f32>,
}

impl Camera {
    pub fn new(position: Vector2<f32>, size: f32) -> Self {
        Self {
            size: size,
            near: -1000.0,
            far: 1000.0,

            position,
            rotation: 0.0,

            aspect_ratio: 1.0,

            projection: Matrix4::identity(),
            view: Matrix4::identity(),
        }
    }

    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.recalculate_projection();
    }

    pub fn recalculate_projection(&mut self) {
        let ortho_left = -self.size * self.aspect_ratio * 0.5;
        let ortho_right = self.size * self.aspect_ratio * 0.5;
        let ortho_bottom = -self.size * 0.5;
        let ortho_top = self.size * 0.5;

        self.projection = ortho(
            ortho_left,
            ortho_right,
            ortho_bottom,
            ortho_top,
            self.near,
            self.far,
        );
    }

    pub fn update(&mut self) {
        self.recalculate_view();
        self.recalculate_projection();
    }

    pub fn recalculate_view(&mut self) {
        let translation = Vector3::new(-self.position.x, -self.position.y, 0.0);
        let rotation = Quaternion::from_axis_angle(Vector3::unit_z(), Rad(self.rotation));

        let translation_matrix = Matrix4::from_translation(translation);
        let rotation_matrix = Matrix4::from(rotation);

        self.view = rotation_matrix * translation_matrix;
    }

    pub fn get_projection(&self) -> Matrix4<f32> {
        self.projection
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        self.view
    }
}
