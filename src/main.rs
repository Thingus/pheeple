use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;

const PHEEPLE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const MEEPLE_SPEED: f32 = 50.;
const INITIAL_POP: i32 = 20;
const ARRIVAL_RADIUS: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, init_pheeple))
        .add_systems(Update, (move_towards, check_arrived))
        .run();
}

#[derive(Component)]
struct Pheeple;

// This could be Sparse
#[derive(Component)]
struct MoveBehavior {
    velocity: f32,
    target: Vec3,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn random_point(bbox: Aabb2d) -> Vec2 {
    let mut rng = rand::rng();
    Vec2 {
        x: rng.random_range(bbox.min.x..bbox.max.x),
        y: rng.random_range(bbox.min.y..bbox.max.y),
    }
}

fn init_pheeple(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let home_neighbourhood = Aabb2d::new(vec2(-400., -400.), vec2(20., 20.));
    let work_neighbourhood = Aabb2d::new(vec2(400., 400.), vec2(20., 20.));

    for _ in 0..INITIAL_POP {
        let home = random_point(home_neighbourhood).extend(1.);
        let work = random_point(work_neighbourhood).extend(1.);
        commands.spawn((
            Pheeple,
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(PHEEPLE_COLOR)),
            Transform::from_translation(home).with_scale(Vec2::splat(10.).extend(1.)),
            MoveBehavior {
                velocity: MEEPLE_SPEED,
                target: work,
            },
        ));
    }
}

fn move_towards(mut query: Query<(&mut Transform, &MoveBehavior)>, time: Res<Time>) {
    for (mut transform, move_behavior) in &mut query {
        transform.translation = transform.translation.move_towards(
            move_behavior.target,
            move_behavior.velocity * time.delta_secs(),
        )
    }
}

fn check_arrived(mut query: Query<(&mut Transform, &mut MoveBehavior)>) {
    for (mut transform, move_behavior) in &mut query {
        if transform.translation.distance(move_behavior.target) <= ARRIVAL_RADIUS {
            print!("At work")
        }
    }
}
