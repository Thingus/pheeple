use bevy::math::bounding::BoundingVolume;
use bevy::{math::bounding::Aabb2d, prelude::*};
use geojson::{
    Feature, FeatureCollection, GeoJson, Geometry, LineStringType, PointType, PolygonType,
};
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

const MAP_PATH: &str = "/home/john/personal/pheeples/data/haiti_admin2.geojson";
pub fn gis_plugin(app: &mut App) {
    app.init_resource::<Basemap>();
    app.add_systems(Startup, ());
}

struct GeoToGameProj {
    ratio: Vec2,
    offset: Vec2,
}

impl GeoToGameProj {
    fn do_transform(&self, geo_point: Vec2) -> Vec2 {
        (geo_point * self.ratio) + self.offset
    }
}

#[derive(Clone)]
struct Area {
    geometry: geojson::Value,
    name: String,
}

#[derive(Resource)]
pub struct Basemap {
    bbox: Aabb2d,
    areas: Vec<Area>,
    geo_to_game_proj: Option<GeoToGameProj>,
}

impl Default for Basemap {
    fn default() -> Basemap {
        info!("Loading admin areas from {MAP_PATH}");
        let geojson_str = fs::read_to_string(MAP_PATH).unwrap();
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let admin_areas = FeatureCollection::try_from(geojson).unwrap();
        let n_areas = admin_areas.features.len();
        info!("{n_areas} admin areas loaded");

        let mut growing_bbox = smallest_bbox();
        let mut areas: Vec<Area> = [].to_vec();

        for feature in admin_areas.features {
            let geometry = feature.geometry.clone().unwrap().value;
            let name = feature.property("id_com").unwrap().to_string().clone();
            growing_bbox = growing_bbox.merge(&geometry.feature_bbox());
            let new_area = Area { geometry, name };
            areas.push(new_area);
        }
        Basemap {
            bbox: growing_bbox,
            areas,
            geo_to_game_proj: None,
        }
    }
}

fn smallest_bbox() -> Aabb2d {
    Aabb2d {
        min: Vec2::new(f32::MIN, f32::MIN),
        max: Vec2::new(f32::MIN, f32::MIN),
    }
}

fn point_list_to_bbox(point_list: &Vec<Vec<f64>>) -> Aabb2d {
    let mut xmin = -f64::INFINITY;
    let mut xmax = f64::INFINITY;
    let mut ymin = -f64::INFINITY;
    let mut ymax = f64::INFINITY;
    // GeoJson should always be [x,y]
    // We have no way to prove this, so we'll just have to trust.
    for coord in point_list {
        xmin = if coord[0] < xmin { coord[0] } else { xmin };
        xmax = if coord[0] > xmax { coord[0] } else { xmax };
        ymin = if coord[1] < ymin { coord[1] } else { ymin };
        ymax = if coord[1] > ymax { coord[1] } else { ymax };
    }
    Aabb2d {
        min: Vec2::new(xmin as f32, ymin as f32),
        max: Vec2::new(xmax as f32, ymax as f32),
    }
}

trait GeoBbox {
    fn feature_bbox(&self) -> Aabb2d;
}

impl GeoBbox for geojson::Value {
    fn feature_bbox(&self) -> Aabb2d {
        match self {
            geojson::Value::MultiPoint(geom) => geom.feature_bbox(),
            geojson::Value::Polygon(geom) => geom.feature_bbox(),
            geojson::Value::MultiPolygon(geom) => geom.feature_bbox(),
            geojson::Value::MultiLineString(geom) => geom.feature_bbox(),
            _ => panic!("Unsupported geometry in basemap"),
        }
    }
}

// A MultiLine is just an alias for this, too.
impl GeoBbox for Vec<PointType> {
    fn feature_bbox(&self) -> Aabb2d {
        point_list_to_bbox(self)
    }
}

impl GeoBbox for PolygonType {
    fn feature_bbox(&self) -> Aabb2d {
        let mut outer_bbox = smallest_bbox();
        for linear_ring in self {
            outer_bbox = outer_bbox.merge(&linear_ring.feature_bbox());
        }
        outer_bbox
    }
}

impl GeoBbox for Vec<PolygonType> {
    fn feature_bbox(&self) -> Aabb2d {
        let mut outer_bbox = smallest_bbox();
        for polygon in self {
            outer_bbox = outer_bbox.merge(&polygon.feature_bbox())
        }
        outer_bbox
    }
}

fn build_map_to_game_projection(mut basemap: ResMut<Basemap>, world_bbox: Aabb2d) {
    let basemap_bbox = basemap.bbox;
    let basemap_size = basemap_bbox.max - basemap_bbox.min;
    let world_size = world_bbox.max - world_bbox.min;
    // Note; world_bbox.center _should_ always be 0,0 but just in case it isnt...
    let geo_offset = basemap_bbox.center() - world_bbox.center();
    let offset = Vec2::new(geo_offset[0], geo_offset[1]);
    // Moves the world to the game
    let ratio = world_size / basemap_size;
    basemap.geo_to_game_proj = Some(GeoToGameProj { offset, ratio })
}

fn spawn_area(
    area: Area,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    basemap: Res<Basemap>,
) {
    let area_vertices = match area.geometry {
        geojson::Value::Polygon(geom) => geom[0],
        geojson::Value::LineString(geom) => geom,
        geojson::Value::MultiPolygon(geom) => geom[0][0],
        geojson::Value::MultiLineString(geom) => geom[0],
        _ => panic!("Geometry must be polygon, multipolygon or linestring"),
    };
    let vertices = area_vertices
        .iter()
        .map(|vert| Vec2 {
            x: vert[0] as f32,
            y: vert[1] as f32,
        })
        .map(|vert| basemap.geo_to_game_proj.unwrap().do_transform(vert));
    let area_polygon = Polygon::from_iter(vertices);
    commands.spawn((Mesh2d(meshes.add(area_polygon))))
}
