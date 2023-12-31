use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vector2<f32>,
    max: Vector2<f32>,
}

impl AABB {
    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        AABB { min, max }
    }

    pub fn from_min_max(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        AABB {
            min: Vector2::new(min_x, min_y),
            max: Vector2::new(max_x, max_y),
        }
    }

    pub fn from_position_and_size(center: Vector2<f32>, size: Vector2<f32>) -> Self {
        let half_size = size / 2.0;
        let min = center - half_size;
        let max = center + half_size;

        AABB::new(min, max)
    }

    pub fn contains(&self, point: Vector2<f32>) -> bool {
        point.x >= self.min.x
            && point.y >= self.min.y
            && point.x <= self.max.x
            && point.y <= self.max.y
    }

    pub fn merge(&self, other: &AABB) -> AABB {
        let min = Vector2::new(
            f32::min(self.min.x, other.min.x),
            f32::min(self.min.y, other.min.y),
        );
        let max = Vector2::new(
            f32::max(self.max.x, other.max.x),
            f32::max(self.max.y, other.max.y),
        );

        AABB::new(min, max)
    }

    pub fn center(&self) -> Vector2<f32> {
        Vector2::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }

    pub fn size(&self) -> Vector2<f32> {
        Vector2::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }
}
