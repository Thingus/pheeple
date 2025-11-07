use crate::pheeple::Pheeple;
use crate::utils::RandomPoint;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;
use uuid::Uuid;

pub fn tower_plugin(app: &mut App) {
    app.add_systems(Startup, init_towers.after(crate::gis::spawn_basemap));
    app.add_systems(Update, (make_call, draw_calls, end_calls));
}

#[derive(Component)]
pub struct Tower {
    id: Uuid,
}

fn init_towers(
    mut commands: Commands,
    config: Res<crate::config::Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    basemap: Res<crate::gis::Basemap>,
) {
    let tower_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 0.002,
        Vec2::new(-0.0025, -0.0025),
        Vec2::new(0.0025, -0.0025),
    ));
    let tower_color = materials.add(config.tower_color);
    for area in &basemap.areas {
        for _ in 0..config.towers_per_area {
            let tower_pos = area.geometry.random_point();

            commands.spawn((
                Tower { id: Uuid::new_v4() },
                Transform::from_translation(tower_pos.extend(1.)),
                Mesh2d(tower_mesh.clone()),
                MeshMaterial2d(tower_color.clone()),
            ));
        }
    }
}

#[derive(Component)]
pub struct Call {
    caller: Entity,
    time_remaining: f32,
}

#[derive(Event)]
pub struct CallStarted {
    pub caller: Uuid,
    pub tower: Uuid,
}

fn make_call(
    pheeples: Query<(Entity, &Transform, &Pheeple)>,
    towers: Query<(&Transform, &Tower)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<crate::config::Config>,
) {
    let mut rng = rand::rng();
    for (pheeple, pheeple_trans, pheeple_data) in pheeples {
        if rng.random_ratio(config.call_chance, 10000) {
            let tower_list: Vec<(&Transform, &Tower)> = towers.iter().collect();
            let (tower_trans, tower_data) = tower_list
                .choose_weighted(&mut rng, |t| {
                    1. / t.0.translation.distance(pheeple_trans.translation)
                })
                .unwrap();
            commands.spawn((
                Call {
                    caller: pheeple,
                    time_remaining: rng.random_range(0.1..1.5),
                },
                Mesh2d(create_line(
                    &mut meshes,
                    pheeple_trans.translation.xy(),
                    tower_trans.translation.xy(),
                )),
                MeshMaterial2d(materials.add(config.call_color)),
            ));
            commands.trigger(CallStarted {
                caller: pheeple_data.phone_id,
                tower: tower_data.id,
            });
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
