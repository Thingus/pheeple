use bevy::{
    math::bounding::{Aabb2d, Bounded2d},
    prelude::Isometry2d,
    prelude::Polyline2d,
    prelude::Vec2,
};
use geo::{BoundingRect, Contains, Coord, Polygon};
use rand::Rng;

pub trait RandomPoint {
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

// Thought for the future.
// I really don't like this, and I do think there's a better way;
// step through the bounding box at instantiation and chuck away
// any points that aren't within the geom.
const TIMEOUT: u16 = 65535;
impl RandomPoint for Polygon<f32> {
    fn random_point(&self) -> Vec2 {
        let mut rng = rand::rng();
        for _ in 0..TIMEOUT {
            let bbox = self.bounding_rect().unwrap();
            let candidate = Coord {
                x: rng.random_range(bbox.min().x..bbox.max().x),
                y: rng.random_range(bbox.min().y..bbox.max().y),
            };
            if self.contains(&candidate) {
                return Vec2 {
                    x: candidate.x as f32,
                    y: candidate.y as f32,
                };
            }
        }
        panic!("John's kludgy random assigner failed")
    }
}

pub fn coord_to_vec(coord: geo::Coord<f32>) -> Vec2 {
    Vec2 {
        x: coord.x,
        y: coord.y,
    }
}

pub fn rect_to_aabb2d(rect: geo::Rect<f32>) -> Aabb2d {
    Aabb2d::new(
        coord_to_vec(rect.center()),
        Vec2 {
            x: rect.width() / 2.,
            y: rect.height() / 2.,
        },
    )
}
