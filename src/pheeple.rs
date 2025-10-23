use crate::utils;
use crate::utils::RandomPoint;
use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;
const INITIAL_POP: i32 = 2000;
const ARRIVAL_RADIUS: f32 = 10.;
const PHEEPLE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const PHEEPLE_SIZE: f32 = 3.;
const MEEPLE_SPEED: f32 = 500.;

pub fn pheeple_plugin(app: &mut App) {
    app.add_systems(Startup, init_from_basemap.after(crate::gis::spawn_basemap));
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

const MAX_PER_AREA: i32 = 300;

fn init_from_basemap(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    basemap: Res<crate::gis::Basemap>,
) {
    let mut rng = rand::rng();
    let reprojection = basemap.geo_to_game_proj.as_ref().unwrap();
    for area in &basemap.areas {
        let n_pheeples = rng.random_range(0..MAX_PER_AREA);
        let name = &area.name;
        info!("Generating {n_pheeples} for {name}");
        for _ in 0..n_pheeples {
            let home_map = area.geometry.random_point();
            let work_map = area.geometry.random_point();
            let home = reprojection.do_transform(home_map);
            let work = reprojection.do_transform(work_map);
            spawn_pheeple(&mut commands, &mut meshes, &mut materials, home, work)
        }
    }
}

fn spawn_pheeple(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    home: Vec2,
    work: Vec2,
) {
    info!("Spawning pheeple at {home}");
    commands.spawn((
        Pheeple {
            behavior: Behavior::AtHome,
        },
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(PHEEPLE_COLOR)),
        Transform::from_translation(home.extend(1.))
            .with_scale(Vec2::splat(PHEEPLE_SIZE).extend(1.)),
        MoveBehavior {
            velocity: MEEPLE_SPEED,
            target: work.extend(1.),
        },
        Home(home.extend(1.)),
        Work(work.extend(1.)),
    ));
}

fn init_pheeples(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let whole_neighbourhood = Aabb2d::new(vec2(0., 0.), vec2(340., 250.));
    let home_neighbourhood = Aabb2d::new(vec2(-250., -30.), vec2(80., 100.));
    let work_neighbourhood = Aabb2d::new(vec2(250., 30.), vec2(80., 100.));

    for _ in 0..INITIAL_POP {
        let home = home_neighbourhood.random_point();
        let work = work_neighbourhood.random_point();
        spawn_pheeple(&mut commands, &mut meshes, &mut materials, home, work)
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
        }
    }
}
