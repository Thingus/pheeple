use crate::config::Config;
use bevy::prelude::*;

#[derive(Component)]
pub struct MoveBehavior {
    velocity: f32,
    target: Vec3,
    arrived: fn(Entity, &mut Commands),
}

pub fn movement_plugin(app: &mut App) {
    app.add_systems(Update, (move_towards, check_arrived));
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
    mut query: Query<(Entity, &Transform, &mut MoveBehavior)>,
    mut commands: Commands,
    config: Res<Config>,
) {
    for (entity, transform, move_behavior) in &mut query {
        if transform.translation.distance(move_behavior.target) <= config.arrival_radius {
            (move_behavior.arrived)(entity, &mut commands);
            commands.entity(entity).remove::<MoveBehavior>();
        }
    }
}
