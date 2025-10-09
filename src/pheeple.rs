use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;
const INITIAL_POP: i32 = 2000;
const ARRIVAL_RADIUS: f32 = 10.;
const PHEEPLE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const PHEEPLE_SIZE: f32 = 3.;
const MEEPLE_SPEED: f32 = 500.;

pub fn pheeple_plugin(app: &mut App) {
    app.add_systems(Startup, init_pheeple);
    app.add_systems(Update, (move_towards, check_arrived));
}

pub enum Behavior {
    Working,
    AtHome,
    TravellingToWork,
    TravellingToHome,
}

#[derive(Component)]
pub struct Pheeple {
    behavior: Behavior,
}

#[derive(Component)]
pub struct MoveBehavior {
    velocity: f32,
    target: Vec3,
}

#[derive(Component)]
pub struct Home(Vec3);

#[derive(Component)]
pub struct Work(Vec3);

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
    let whole_neighbourhood = Aabb2d::new(vec2(0., 0.), vec2(340., 250.));

    for _ in 0..INITIAL_POP {
        let home = random_point(whole_neighbourhood).extend(1.);
        let work = random_point(whole_neighbourhood).extend(1.);
        commands.spawn((
            Pheeple {
                behavior: Behavior::AtHome,
            },
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(PHEEPLE_COLOR)),
            Transform::from_translation(home).with_scale(Vec2::splat(PHEEPLE_SIZE).extend(1.)),
            MoveBehavior {
                velocity: MEEPLE_SPEED,
                target: work,
            },
            Home(home),
            Work(work),
        ));
    }
}

pub fn hometime(query: Query<(Entity, &mut Pheeple, &Home)>, mut commands: Commands) {
    for (entity, mut pheeple, home) in query {
        pheeple.behavior = Behavior::TravellingToHome;
        commands.entity(entity).insert(MoveBehavior {
            velocity: MEEPLE_SPEED,
            target: home.0,
        });
    }
}

pub fn worktime(query: Query<(Entity, &mut Pheeple, &Work)>, mut commands: Commands) {
    for (entity, mut pheeple, work) in query {
        pheeple.behavior = Behavior::TravellingToWork;
        commands.entity(entity).insert(MoveBehavior {
            velocity: MEEPLE_SPEED,
            target: work.0,
        });
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

fn check_arrived(
    mut query: Query<(Entity, &Transform, &mut MoveBehavior, &mut Pheeple)>,
    mut commands: Commands,
) {
    for (entity, transform, move_behavior, mut pheeple) in &mut query {
        if transform.translation.distance(move_behavior.target) <= ARRIVAL_RADIUS {
            pheeple.behavior = match pheeple.behavior {
                Behavior::TravellingToWork => Behavior::Working,
                Behavior::TravellingToHome => Behavior::AtHome,
                Behavior::AtHome => Behavior::AtHome,
                Behavior::Working => Behavior::Working,
            };
            commands.entity(entity).remove::<MoveBehavior>();
            print!("{entity} has reached destination")
        }
    }
}
