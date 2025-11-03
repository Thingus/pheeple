mod config;
mod data_writer;
mod gis;
mod pheeple;
mod time_and_date;
mod tower;
mod utils;
use args::{Args, ArgsError};
use bevy::prelude::*;
use config::Config;
use getopts::Occur;
use std::env;
use std::path::PathBuf;
use std::process::exit;

#[derive(Component)]
struct GameCamera;

const PROGRAM_NAME: &str = "Pheeple";
const PROGRAM_DESC: &str =
    "A multi-agent simulation of a mobile population interacting with cellular towers.";

fn parse_cli() -> Result<Config, ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("h", "help", "Shows the help message");
    args.option(
        "b",
        "basemap",
        "Path to a geojson containing the basemap for the population you want to model.",
        "BASEMAP",
        Occur::Req,
        None,
    );

    match args.parse_from_cli() {
        Ok(_) => (),
        Err(error) => return Err(error),
    }

    if args.value_of::<bool>("help").unwrap_or(false) {
        print!("{}", args.full_usage());
        exit(0);
    };

    let map_path = args.value_of::<PathBuf>("basemap")?;
    if !map_path.is_file() {
        return Err(ArgsError::new("", "Basemap path does not exist"));
    }

    println!("Config loaded successfully");

    Ok(Config {
        map_path,
        ..Default::default()
    })
}
fn main() {
    println!("Loading config from cli...");
    let config = match parse_cli() {
        Ok(config) => config,
        Err(err) => {
            print!("{err}");
            exit(1);
        }
    };
    println!("App config:\n{config:?}");
    App::new()
        .insert_resource(config)
        .add_plugins((
            DefaultPlugins,
            pheeple::pheeple_plugin,
            tower::tower_plugin,
            gis::gis_plugin,
            data_writer::data_writer_plugin,
            time_and_date::time_and_date_plugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}
