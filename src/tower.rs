use crate::pheeple::Pheeple;
use bevy::color::palettes::css::DARK_GREEN;
use bevy::mesh::VertexAttributeValues;
use bevy::{color::palettes::css::LIME, prelude::*};
use rand::Rng;
use rand::seq::IndexedRandom;

const TOWER_COLOR: Color = Color::Srgba(LIME);
const CALL_COLOR: Color = Color::Srgba(DARK_GREEN);
const CALL_CHANCE: u32 = 1;

pub fn tower_plugin(app: &mut App) {
    app.add_systems(Startup, init_towers);
    app.add_systems(Update, (make_call, draw_calls, end_calls));
}

#[derive(Component)]
pub struct Tower {
    id: String,
}

fn init_towers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tower_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 20.0,
        Vec2::new(-10.0, -10.0),
        Vec2::new(10.0, -10.0),
    ));
    let tower_color = materials.add(TOWER_COLOR);
    for x_pos in (-400..400).step_by(100) {
        let tower_position = Vec2 {
            x: x_pos as f32,
            y: 0.,
        };
        commands.spawn((
            Tower {
                id: "tower_central".to_string(),
            },
            Transform::from_translation(tower_position.extend(1.)),
            Mesh2d(tower_mesh.clone()),
            MeshMaterial2d(tower_color.clone()),
        ));
    }
}

#[derive(Component)]
struct Call {
    caller: Entity,
    tower: Entity,
    time_remaining: f32,
}

fn make_call(
    pheeples: Query<(Entity, &Transform), With<Pheeple>>,
    towers: Query<(Entity, &Transform), With<Tower>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    for (pheeple, pheeple_trans) in pheeples {
        if rng.random_ratio(CALL_CHANCE, 10000) {
            let tower_list: Vec<(Entity, &Transform)> = towers.iter().collect();
            let recieving_tower = tower_list
                .choose_weighted(&mut rng, |t| {
                    1. / t.1.translation.distance(pheeple_trans.translation)
                })
                .unwrap();
            commands.spawn((
                Call {
                    caller: pheeple,
                    tower: recieving_tower.0,
                    time_remaining: rng.random_range(0.1..1.5),
                },
                Mesh2d(create_line(
                    &mut meshes,
                    pheeple_trans.translation.xy(),
                    recieving_tower.1.translation.xy(),
                )),
                MeshMaterial2d(materials.add(CALL_COLOR)),
            ));
        }
    }
}

fn create_line(meshes: &mut ResMut<Assets<Mesh>>, start: Vec2, end: Vec2) -> Handle<Mesh> {
    meshes.add(Polyline2d::new(vec![start, end]))
}

fn move_line_startpoint(line: &mut Mesh, new_startpoint: Vec2) {
    let position_attribute = line.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(position_attribute) = position_attribute else {
        panic!("Unexpected vertex format, expected Float32x3.");
    };
    position_attribute[0] = new_startpoint.extend(0.).to_array();
}

fn draw_calls(
    calls: Query<(&Call, &Mesh2d)>,
    pheeple: Query<&Transform, With<Pheeple>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (call, mesh_handle) in calls {
        let caller_transform = pheeple.get(call.caller.entity()).unwrap();
        let mesh = meshes.get_mut(mesh_handle).expect("Mesh not found");
        move_line_startpoint(mesh, caller_transform.translation.xy());
    }
}

fn end_calls(calls: Query<(Entity, &mut Call)>, time: Res<Time>, mut commands: Commands) {
    for (entity, mut call) in calls {
        call.time_remaining -= time.delta_secs();
        if call.time_remaining <= 0. {
            commands.entity(entity).despawn()
        }
    }
}
