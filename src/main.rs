mod pheeple;
use bevy::prelude::*;
// use rand::Rng;
use std::time::Duration;
const HALF_DAY_DURATION_SECS: u64 = 5;

#[derive(Component)]
struct GameCamera;

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
        .add_plugins((DefaultPlugins, pheeple::pheeple_plugin))
        .add_systems(Startup, setup)
        .add_systems(Update, day_night_cycle)
        .insert_state(DayNight::Day)
        .add_systems(OnEnter::<DayNight>(DayNight::Day), pheeple::worktime)
        .add_systems(OnEnter::<DayNight>(DayNight::Night), pheeple::hometime)
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
        timer: Timer::new(
            Duration::from_secs(HALF_DAY_DURATION_SECS),
            TimerMode::Repeating,
        ),
    })
}
