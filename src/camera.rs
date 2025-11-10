use crate::gis::init_basemap;
use crate::utils::{coord_to_vec, rect_to_aabb2d};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use geo::BoundingRect;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup.after(init_basemap));
    app.add_systems(Update, camera_movement);
}

#[derive(Component)]
pub struct GameCamera {
    start_origin: Vec2,
    start_extent: Aabb2d,
    start_scale: f32,
}

fn setup(mut commands: Commands, basemap: Res<crate::gis::Basemap>) {
    let extent = basemap.bounding_rect().unwrap();
    let start_origin = coord_to_vec(extent.center());
    let start_extent = rect_to_aabb2d(extent);
    let start_scale = 0.003;
    let mut proj = OrthographicProjection::default_2d();
    proj.scale = start_scale;

    commands.spawn((
        Camera2d,
        GameCamera {
            start_origin,
            start_extent,
            start_scale,
        },
        Transform::from_translation(start_origin.extend(100.)),
        Projection::Orthographic(proj),
    ));
}

const SCROLL_RATE: f32 = 0.1;
const ZOOM_RATE: f32 = 0.001;

fn camera_movement(
    camera: Single<(&mut Transform, &mut Projection, &GameCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut trans, proj, game_camera) = camera.into_inner();
    let mut velocity = Vec2 { x: 0., y: 0. };
    let mut zoom = 0.;
    if keys.just_pressed(KeyCode::KeyW) {
        velocity.y += SCROLL_RATE;
    }
    if keys.just_pressed(KeyCode::KeyS) {
        velocity.y -= SCROLL_RATE;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        velocity.x -= SCROLL_RATE;
    }
    if keys.just_pressed(KeyCode::KeyD) {
        velocity.x += SCROLL_RATE;
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        zoom -= ZOOM_RATE;
    }
    if keys.just_pressed(KeyCode::KeyE) {
        zoom += ZOOM_RATE;
    }
    trans.translation += velocity.extend(0.);
    match proj.into_inner() {
        Projection::Orthographic(orth) => orth.scale += zoom,
        _ => panic!("Invalid projection"),
    };
}
