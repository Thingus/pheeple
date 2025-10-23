use bevy::{
    math::bounding::{Aabb2d, Bounded2d},
    prelude::Isometry2d,
    prelude::Polyline2d,
    prelude::Vec2,
};
use rand::Rng;

trait RandomPoint {
    fn random_point(&self) -> Vec2;
}

impl RandomPoint for Aabb2d {
    fn random_point(&self) -> Vec2 {
        let mut rng = rand::rng();
        Vec2 {
            x: rng.random_range(self.min.x..self.max.x),
            y: rng.random_range(self.min.y..self.max.y),
        }
    }
}

impl RandomPoint for Polyline2d {
    fn random_point(&self) -> Vec2 {
        self.aabb_2d(Isometry2d::IDENTITY).random_point()
    }
}
