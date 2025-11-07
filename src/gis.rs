use bevy::prelude::*;
use geo::{Geometry, MultiPolygon, Polygon};
use geojson::{FeatureCollection, GeoJson};
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

pub fn gis_plugin(app: &mut App) {
    app.add_systems(Startup, (init_basemap, spawn_basemap.after(init_basemap)));
}

#[derive(Clone)]
pub struct Area {
    pub geometry: Polygon<f32>,
    pub name: String,
}

#[derive(Resource)]
pub struct Basemap {
    pub areas: Vec<Area>,
    multipolygon: MultiPolygon<f32>,
}

impl Basemap {
    fn load(map_path: PathBuf) -> Basemap {
        info!("Loading admin areas from {map_path:?}");
        let geojson_str = fs::read_to_string(map_path).unwrap();
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let admin_areas = FeatureCollection::try_from(geojson).unwrap();
        let n_areas = admin_areas.features.len();
        info!("{n_areas} admin areas loaded");
        let mut areas: Vec<Area> = vec![];

        for feature in admin_areas.features {
            let name = feature.property("id_com").unwrap().to_string().clone();
            let geometry = match geo::Geometry::<f32>::try_from(feature) {
                Err(err) => panic!("{err}"),
                Ok(geom) => match geom {
                    Geometry::Polygon(geom) => geom,
                    Geometry::MultiPolygon(geom) => geom.iter().next().unwrap().clone(),
                    _ => panic!("Invalid geometry for {name}"),
                },
            };
            let new_area = Area { geometry, name };
            areas.push(new_area);
        }

        let multipolygon = MultiPolygon::new(areas.iter().map(|a| a.geometry.clone()).collect());

        Basemap {
            areas,
            multipolygon,
        }
    }
}

impl geo::BoundingRect<f32> for Area {
    type Output = Option<geo::Rect<f32>>;
    fn bounding_rect(&self) -> Self::Output {
        self.geometry.bounding_rect()
    }
}

impl geo::BoundingRect<f32> for Basemap {
    type Output = Option<geo::Rect<f32>>;
    fn bounding_rect(&self) -> Self::Output {
        self.multipolygon.bounding_rect()
    }
}

pub fn init_basemap(config: Res<crate::config::Config>, mut commands: Commands) {
    commands.insert_resource(Basemap::load(config.map_path.clone()))
}

pub fn spawn_basemap(
    basemap: Res<Basemap>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<crate::config::Config>,
) {
    for area in &basemap.areas {
        spawn_area(
            area,
            &mut commands,
            &mut meshes,
            &mut materials,
            config.basemap_color,
        )
    }
}

fn spawn_area(
    area: &Area,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    basemap_color: Color,
) {
    let vertices = area.geometry.exterior().coords().map(|vert| Vec2 {
        x: vert.x as f32,
        y: vert.y as f32,
    });
    let area_polygon = Polyline2d::from_iter(vertices);
    let poly_coord = &area_polygon.vertices[0];
    info!("Gameword coord: {poly_coord:#?}");
    commands.spawn((
        Mesh2d(meshes.add(area_polygon)),
        MeshMaterial2d(materials.add(basemap_color)),
    ));
}
