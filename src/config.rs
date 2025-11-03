use bevy::color::palettes::css::{DARK_GREEN, LIGHT_CYAN, LIME};
use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Resource, Debug)]
pub struct Config {
    pub half_day_duration_secs: u64,
    pub out_dir: PathBuf,
    pub tower_color: Color,
    pub towers_per_area: i32,
    pub call_color: Color,
    pub call_chance: u32,
    pub arrival_radius: f32,
    pub pheeple_color: Color,
    pub pheeple_size: f32,
    pub pheeple_speed: f32,
    pub max_pheeple_per_area: i32,
    pub map_path: PathBuf,
    pub basemap_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        let mut map_path = PathBuf::new();
        map_path.push("data/haiti_admin2.geojson");
        let mut out_dir = PathBuf::new();
        out_dir.push(std::env::var("PHEEPLE_OUT_DIR").unwrap());
        Self {
            half_day_duration_secs: 5,
            out_dir,
            tower_color: Color::Srgba(LIME),
            towers_per_area: 3,
            call_color: Color::Srgba(DARK_GREEN),
            call_chance: 1,
            arrival_radius: 10.,
            pheeple_color: Color::srgb(1.0, 0.5, 0.5),
            pheeple_size: 3.,
            pheeple_speed: 500.,
            max_pheeple_per_area: 300,
            map_path,
            basemap_color: Color::Srgba(LIGHT_CYAN),
        }
    }
}
