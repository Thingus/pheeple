use crate::gis::init_basemap;
use crate::utils::{coord_to_vec, rect_to_aabb2d};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use geo::BoundingRect;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup.after(init_basemap));
}

#[derive(Component)]
struct GameCamera {
    start_origin: Vec2,
    start_extent: Aabb2d,
}

fn setup(mut commands: Commands, basemap: Res<crate::gis::Basemap>) {
    let extent = basemap.bounding_rect().unwrap();
    let start_origin = coord_to_vec(extent.center());
    let start_extent = rect_to_aabb2d(extent);
    let mut proj = OrthographicProjection::default_2d();
    proj.scale = 0.003;

    commands.spawn((
        Camera2d,
        GameCamera {
            start_origin,
            start_extent,
        },
        Transform::from_translation(start_origin.extend(100.)),
        Projection::Orthographic(proj),
    ));
}
