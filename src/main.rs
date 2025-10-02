use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;
use std::time::Duration;

const PHEEPLE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const MEEPLE_SPEED: f32 = 500.;
const INITIAL_POP: i32 = 20;
const ARRIVAL_RADIUS: f32 = 10.;

#[derive(Component)]
struct GameCamera;

enum Behavior {
    Working,
    AtHome,
    TravellingToWork,
    TravellingToHome,
}

#[derive(Component)]
struct Pheeple {
    behavior: Behavior,
}

#[derive(Component)]
struct MoveBehavior {
    velocity: f32,
    target: Vec3,
}

#[derive(Component)]
struct Home(Vec3);

#[derive(Component)]
struct Work(Vec3);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum DayNight {
    #[default]
    Day,
    Night,
}

#[derive(Resource)]
struct HalfDayTimer {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, init_pheeple))
        .add_systems(Update, (move_towards, check_arrived, day_night_cycle))
        .insert_state(DayNight::Day)
        .add_systems(OnEnter::<DayNight>(DayNight::Day), worktime)
        .add_systems(OnEnter::<DayNight>(DayNight::Night), hometime)
        .run();
}

fn day_night_cycle(
    time: Res<Time>,
    mut half_day_timer: ResMut<HalfDayTimer>,
    current_day_night: Res<State<DayNight>>,
    mut next_day_night: ResMut<NextState<DayNight>>,
) {
    half_day_timer.timer.tick(time.delta());
    if half_day_timer.timer.just_finished() {
        match current_day_night.get() {
            DayNight::Day => next_day_night.set(DayNight::Night),
            DayNight::Night => next_day_night.set(DayNight::Day),
        };
    };
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
    commands.insert_resource(HalfDayTimer {
        timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    })
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
    let home_neighbourhood = Aabb2d::new(vec2(-400., -200.), vec2(200., 200.));
    let work_neighbourhood = Aabb2d::new(vec2(400., 200.), vec2(200., 200.));

    for _ in 0..INITIAL_POP {
        let home = random_point(home_neighbourhood).extend(1.);
        let work = random_point(work_neighbourhood).extend(1.);
        commands.spawn((
            Pheeple {
                behavior: Behavior::AtHome,
            },
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(PHEEPLE_COLOR)),
            Transform::from_translation(home).with_scale(Vec2::splat(10.).extend(1.)),
            MoveBehavior {
                velocity: MEEPLE_SPEED,
                target: work,
            },
            Home(home),
            Work(work),
        ));
    }
}

fn hometime(query: Query<(Entity, &mut Pheeple, &Home)>, mut commands: Commands) {
    for (entity, mut pheeple, home) in query {
        pheeple.behavior = Behavior::TravellingToHome;
        commands.entity(entity).insert(MoveBehavior {
            velocity: MEEPLE_SPEED,
            target: home.0,
        });
    }
}

fn worktime(query: Query<(Entity, &mut Pheeple, &Work)>, mut commands: Commands) {
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
