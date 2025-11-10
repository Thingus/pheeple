use crate::gis::init_basemap;
use crate::utils::coord_to_vec;
use bevy::prelude::*;
use geo::BoundingRect;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup.after(init_basemap));
    app.add_systems(
        Update,
        (camera_movement, camera_reset.after(camera_movement)),
    );
}

#[derive(Component)]
pub struct GameCamera {
    start_origin: Vec2,
    start_scale: f32,
}

fn setup(mut commands: Commands, basemap: Res<crate::gis::Basemap>) {
    let extent = basemap.bounding_rect().unwrap();
    let start_origin = coord_to_vec(extent.center());
    let start_scale = 0.003;
    let mut proj = OrthographicProjection::default_2d();
    proj.scale = start_scale;

    commands.spawn((
        Camera2d,
        GameCamera {
            start_origin,
            start_scale,
        },
        Transform::from_translation(start_origin.extend(100.)),
        Projection::Orthographic(proj),
    ));
}

const SCROLL_RATE: f32 = 0.01;
const ZOOM_RATE: f32 = 0.0001;
const ZOOM_RANGE: std::ops::Range<f32> = 0.0001..0.003;

fn camera_movement(
    camera: Single<(&mut Transform, &mut Projection), With<GameCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut trans, proj) = camera.into_inner();
    let mut velocity = Vec2 { x: 0., y: 0. };
    let mut zoom = 0.;
    if keys.pressed(KeyCode::KeyW) {
        velocity.y += SCROLL_RATE;
    }
    if keys.pressed(KeyCode::KeyS) {
        velocity.y -= SCROLL_RATE;
    }
    if keys.pressed(KeyCode::KeyA) {
        velocity.x -= SCROLL_RATE;
    }
    if keys.pressed(KeyCode::KeyD) {
        velocity.x += SCROLL_RATE;
    }
    if keys.pressed(KeyCode::KeyQ) {
        zoom += ZOOM_RATE;
    }
    if keys.pressed(KeyCode::KeyE) {
        zoom -= ZOOM_RATE;
    }
    trans.translation += velocity.extend(0.);
    match proj.into_inner() {
        Projection::Orthographic(orth) => {
            orth.scale = (orth.scale * 1. + zoom).clamp(ZOOM_RANGE.start, ZOOM_RANGE.end)
        }
        _ => panic!("Invalid projection"),
    };
}

fn camera_reset(
    camera: Single<(&mut Transform, &mut Projection, &GameCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (mut trans, proj, game_camera) = camera.into_inner();
    if keys.just_pressed(KeyCode::Space) {
        trans.translation = game_camera.start_origin.extend(0.);
        match proj.into_inner() {
            Projection::Orthographic(orth) => orth.scale = game_camera.start_scale,
            _ => panic!("Invalid projection"),
        };
    };
}
