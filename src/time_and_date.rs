use crate::pheeple;
use bevy::prelude::*;
use std::time::Duration;

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

pub fn time_and_date_plugin(app: &mut App) {
    app.add_systems(Startup, setup_time_and_date);
    app.add_systems(Update, day_night_cycle);
    app.insert_state(DayNight::Day);
    app.add_systems(OnEnter::<DayNight>(DayNight::Day), pheeple::worktime);
    app.add_systems(OnEnter::<DayNight>(DayNight::Night), pheeple::hometime);
}

fn setup_time_and_date(mut commands: Commands, config: Res<crate::config::Config>) {
    commands.insert_resource(HalfDayTimer {
        timer: Timer::new(
            Duration::from_secs(config.half_day_duration_secs),
            TimerMode::Repeating,
        ),
    })
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
