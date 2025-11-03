use bevy::math::bounding::BoundingVolume;
use bevy::{math::bounding::Aabb2d, prelude::*};
use geojson::{FeatureCollection, GeoJson, PointType, PolygonType};
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

pub fn gis_plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (
            init_basemap,
            build_map_to_game_projection.after(init_basemap),
            spawn_basemap.after(build_map_to_game_projection),
        ),
    );
}

#[derive(Clone)]
pub struct GeoToGameProj {
    ratio: Vec2,
    offset: Vec2,
}

impl GeoToGameProj {
    pub fn do_transform(&self, geo_point: Vec2) -> Vec2 {
        (geo_point - self.offset) * self.ratio
    }
}

#[derive(Clone)]
pub struct Area {
    pub geometry: geojson::Value,
    pub name: String,
}

#[derive(Resource)]
pub struct Basemap {
    pub bbox: Aabb2d,
    pub areas: Vec<Area>,
    pub geo_to_game_proj: Option<GeoToGameProj>,
}

impl Basemap {
    fn load(map_path: PathBuf) -> Basemap {
        info!("Loading admin areas from {map_path:?}");
        let geojson_str = fs::read_to_string(map_path).unwrap();
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let admin_areas = FeatureCollection::try_from(geojson).unwrap();
        let n_areas = admin_areas.features.len();
        info!("{n_areas} admin areas loaded");

        let mut growing_bbox = admin_areas.features[0]
            .geometry
            .clone()
            .unwrap()
            .value
            .feature_bbox();
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

fn init_basemap(config: Res<crate::config::Config>, mut commands: Commands) {
    commands.insert_resource(Basemap::load(config.map_path.clone()))
}

fn point_list_to_bbox(point_list: &Vec<Vec<f64>>) -> Aabb2d {
    let mut xmin = -f64::INFINITY;
    let mut xmax = f64::INFINITY;
    let mut ymin = -f64::INFINITY;
    let mut ymax = f64::INFINITY;
    // GeoJson should always be [x,y]
    // We have no way to prove this, so we'll just have to trust.
    for coord in point_list {
        xmin = if coord[0] > xmin { coord[0] } else { xmin };
        xmax = if coord[0] < xmax { coord[0] } else { xmax };
        ymin = if coord[1] > ymin { coord[1] } else { ymin };
        ymax = if coord[1] < ymax { coord[1] } else { ymax };
    }
    // If we get bbox that crosses boundaries, we end up with mixed coord. This should fix that.
    (ymin, ymax) = if ymax < ymin {
        (ymax, ymin)
    } else {
        (ymin, ymax)
    };

    (xmin, xmax) = if xmax < xmin {
        (xmax, xmin)
    } else {
        (xmin, xmax)
    };
    Aabb2d {
        min: Vec2::new(xmin as f32, ymin as f32),
        max: Vec2::new(xmax as f32, ymax as f32),
    }
    // info!("point list translation: {out:#?}");
}

pub trait GeoBbox {
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
        self.iter().fold(self[0].feature_bbox(), |running, this| {
            running.merge(&this.feature_bbox())
        })
    }
}

impl GeoBbox for Vec<PolygonType> {
    fn feature_bbox(&self) -> Aabb2d {
        self.iter().fold(self[0].feature_bbox(), |running, this| {
            running.merge(&this.feature_bbox())
        })
    }
}

fn build_map_to_game_projection(mut basemap: ResMut<Basemap>, window: Single<&Window>) {
    let world_bbox = Aabb2d::new(Vec2 { x: 0., y: 0. }, window.size() / 2.);

    let basemap_bbox = basemap.bbox;
    // This is too dang small
    info!("Basemap_bbox: {basemap_bbox:#?}");
    let basemap_size = basemap_bbox.max - basemap_bbox.min;
    info!("Basemap_size: {basemap_size:#?}");
    let world_size = world_bbox.max - world_bbox.min;
    info!("world_size: {world_size:#?}");
    // Note; world_bbox.center _should_ always be 0,0 but just in case it isnt...
    let geo_offset = basemap_bbox.center() - world_bbox.center();
    info!("geo_offset: {geo_offset:#?}");

    // Moves the centre of the basemap to the center of the gameworld
    let offset = Vec2::new(geo_offset[0], geo_offset[1]);
    info!("Offset: {offset:#?}");

    // Stretches the basemap from the origin to cover the world coord system neatly
    let ratio = world_size / basemap_size;
    info!("Ratio: {ratio:#?}");
    basemap.geo_to_game_proj = Some(GeoToGameProj { offset, ratio })
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
            &basemap,
            config.basemap_color,
        )
    }
}

fn spawn_area(
    area: &Area,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    basemap: &Res<Basemap>,
    basemap_color: Color,
) {
    let area_vertices = match &area.geometry {
        geojson::Value::Polygon(geom) => geom[0].clone(),
        geojson::Value::LineString(geom) => geom.to_vec(),
        geojson::Value::MultiPolygon(geom) => geom[0][0].clone(),
        geojson::Value::MultiLineString(geom) => geom[0].clone(),
        _ => panic!("Geometry must be polygon, multipolygon or linestring"),
    };
    let area_coord = &area_vertices[0];
    info!("Geo coord: {area_coord:#?}");
    let vertices = area_vertices
        .iter()
        .map(|vert| Vec2 {
            x: vert[0] as f32,
            y: vert[1] as f32,
        })
        .map(|vert| {
            basemap
                .geo_to_game_proj
                .as_ref()
                .unwrap()
                .do_transform(vert)
        });
    // let area_polygon = Polyline2d::from_iter(vec![
    //     Vec2 { x: 100., y: 100. },
    //     Vec2 { x: 100., y: -100. },
    //     Vec2 { x: -100., y: -100. },
    // ]);
    let area_polygon = Polyline2d::from_iter(vertices);
    let poly_coord = &area_polygon.vertices[0];
    info!("Gameword coord: {poly_coord:#?}");
    commands.spawn((
        Mesh2d(meshes.add(area_polygon)),
        MeshMaterial2d(materials.add(basemap_color)),
    ));
}
