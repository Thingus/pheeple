use bevy::prelude::*;
use dbase::FieldValue;
use geo::{Convert, Geometry, MultiPolygon, Polygon, TryConvert};
use geojson::{FeatureCollection, GeoJson};
use shapefile::Reader;
use std::convert::TryFrom;
use std::ffi::OsStr;
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
    fn load(map_path: PathBuf, id_feature: &str) -> Basemap {
        info!("Loading admin areas from {map_path:?}");
        let extension = map_path.extension().expect("No extension");
        match extension.to_str() {
            Some("shx") | Some("dbf") => panic!("Shapefiles not yet implemented"),
            Some("geojson") | Some("json") => Self::load_geojson(map_path, id_feature),
            _ => panic!("Invalid format"),
        }
    }

    fn load_geojson(geo_path: PathBuf, id_feature: &str) -> Basemap {
        let geojson_str = fs::read_to_string(geo_path).unwrap();
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let admin_areas = FeatureCollection::try_from(geojson).unwrap();
        let n_areas = admin_areas.features.len();
        info!("{n_areas} admin areas loaded");
        let mut areas: Vec<Area> = vec![];

        for feature in admin_areas.features {
            let name = match feature.property(id_feature) {
                Some(f) => f.to_string().clone(),
                None => {
                    warn!("Feature missing {id_feature}, continuing");
                    continue;
                }
            };
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

    // fn load_shapefile(geo_path: PathBuf, id_feature: &str) -> Basemap {
    //     info!("Attempting to read shapefile from {geo_path:#?}");
    //     let mut reader =
    //         Reader::from_path(geo_path).expect("Something has gone wrong with the shapefile");
    //     let mut areas: Vec<Area> = vec![];
    //     for feature in reader.iter_shapes_and_records() {
    //         let (shape, record) = match feature {
    //             Ok(feature) => feature,
    //             Err(_) => {
    //                 warn!("Bad feature in basemap shapfile");
    //                 continue;
    //             }
    //         };
    //
    //         let name = match record.get(id_feature) {
    //             Some(record) => match record {
    //                 FieldValue::Character(record) => match record {
    //                     Some(str) => str.clone(),
    //                     None => continue,
    //                 },
    //                 _ => continue,
    //             },
    //             _ => continue,
    //         };
    //
    //         let geometry = match geo::Geometry::<f64>::try_from(shape) {
    //             Err(err) => panic!("{err}"),
    //             Ok(geom) => match geom {
    //                 Geometry::Polygon(geom) => geom,
    //                 Geometry::MultiPolygon(geom) => geom.iter().next().unwrap().clone(),
    //                 _ => panic!("Invalid geometry for {name}"),
    //             },
    //         };
    //         let new_area = Area {
    //             geometry: geometry.convert(),
    //             name,
    //         };
    //         areas.push(new_area);
    //     }
    //
    //     let multipolygon = MultiPolygon::new(areas.iter().map(|a| a.geometry.clone()).collect());
    //
    //     Basemap {
    //         areas,
    //         multipolygon,
    //     }
    // }
}

// Shapefile kludge in progress
// fn poly_to_f32(poly: Polygon<f64>) -> Polygon<f32>{
//     let out = Polygon::<f32>::new(
//         poly.exterior().iter().map(|coord| coord as f32).collect(),
//         poly.interiors().iter().map(| r | r.clone().into())
//     )
// }
//
// fn linestr_to_f32(linestr: LineString<f64>) -> LineString<

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
    commands.insert_resource(Basemap::load(
        config.map_path.clone(),
        &config.id_feature_name,
    ))
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
        x: vert.x,
        y: vert.y,
    });
    let area_polygon = Polyline2d::from_iter(vertices);
    let poly_coord = &area_polygon.vertices[0];
    info!("Gameword coord: {poly_coord:#?}");
    commands.spawn((
        Mesh2d(meshes.add(area_polygon)),
        MeshMaterial2d(materials.add(basemap_color)),
    ));
}
