use bevy::{math::bounding::Aabb2d, prelude::Vec2};
use rand::Rng;

pub fn random_point(bbox: Aabb2d) -> Vec2 {
    let mut rng = rand::rng();
    Vec2 {
        x: rng.random_range(bbox.min.x..bbox.max.x),
        y: rng.random_range(bbox.min.y..bbox.max.y),
    }
}
