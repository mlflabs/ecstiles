#![allow(unused)]
use std::{collections::{HashMap}, u16};
use bevy::{asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset}, prelude::*};
use ldtk_rust::{Project, TilesetDefinition};

use bevy_ecs_tilemap::prelude::*;
mod helpers;

use log::{debug, error, log_enabled, info, Level};


fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(5, 5));


    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
/*
    let handle: Handle<LdtkMap> = asset_server.load("map.ldtk");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity)
        .insert_bundle(LdtkMapBundle {
            ldtk_map: handle,
            map: Map::new(0u16, map_entity),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
    });
*/  
    
} 

const LDTK_FILE_PATH: &str = "assets/map.ldtk";
const TILE_SCALE: f32 = 2.5;
struct MapData {
    map: Project,
    redraw: bool,
    current_level: usize,
    //tilesets: HashMap<i64, Handle<Texture>>,
}

fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
        




    //println!("map_setup");
    debug!("map_setup");
    debug!("this is a debug {}", "message");
    info!("------------------------------------Map SETUP");




    
    let ldtk = MapData {
        map: Project::new(LDTK_FILE_PATH.to_string()),
        redraw: true,
        current_level: 0,
    };

    // Tilesets/Tilesets
    //let mut tileset: &TilesetDefinition;// = ldtk.map.defs.tilesets[0].as_ref();
    let tileset = ldtk.map.defs.tilesets.iter().next().unwrap();
    



    let texture_path: &str = tileset.rel_path.as_ref();
    let texture_handle:Handle<Texture> = asset_server.load(texture_path);
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));





    // Entity Setp
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let default_grid_size = ldtk.map.default_grid_size;
    let map_tile_count_x = (ldtk.map.levels[0].px_wid / default_grid_size) as u32;
    let map_tile_count_y = (ldtk.map.levels[0].px_hei / default_grid_size) as u32;

    //Here we will need to change the chunk size
    let map_size = UVec2::new(
        (map_tile_count_x as f32 / 8.0).ceil() as u32,
        (map_tile_count_y as f32 / 8.0).ceil() as u32,
    );

    
    
    
    for (layer_id, layer) in ldtk.map.levels[0].layer_instances.as_ref().unwrap().iter().rev().enumerate() {
        //for now all layers/levels use the same texture
        
        let mut settings = LayerSettings::new(
            map_size,
            UVec2::new(8, 8),
            Vec2::new(tileset.tile_grid_size as f32, tileset.tile_grid_size as f32),
            Vec2::new(tileset.px_wid as f32, tileset.px_hei as f32)
        );
        debug!("Settings: {:?}", settings);
        settings.set_layer_id(layer_id as u16);
        
        let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
            &mut commands,
            settings,
            ldtk.map.levels[0].uid as u16, 
            layer_id as u16,
            None,
        );

        //add layer to map
        //map.add_layer(&mut commands,layer_id as u16, layer_entity);

        let tileset_width_in_tiles = (tileset.px_wid / default_grid_size) as u32;


        //Setup our tiles
        for tile in layer.grid_tiles.iter() {
            let tileset_x = (tile.src[0] / default_grid_size) as u32;
            let tileset_y = (tile.src[1] / default_grid_size) as u32;

            let mut pos = UVec2::new(
                (tile.px[0] / default_grid_size) as u32,
                (tile.px[1] / default_grid_size) as u32
            );

            pos.y = map_tile_count_y - pos.y;
            debug!("Tile post {:?}", pos);
            layer_builder.set_tile(
                pos,
                TileBundle {
                    tile: Tile {
                        texture_index: (tileset_y * tileset_width_in_tiles + tileset_x) as u16,
                        ..Default::default()
                    },
                    ..Default::default()  
                },               
            );
        }


        /* 
        let layer_bundle = layer_builder.build(
            &mut commands, 
            &mut meshes, 
            material_handle.clone());
        */

        map_query.build_layer(
            &mut commands, 
            layer_builder, 
            material_handle.clone());

        map.add_layer(&mut commands, layer_id as u16, layer_entity);
        /*
        let mut layer = layer_bundle.layer;
        let mut transform = Transform::from_xyz(
            0.0, 
            -ldtk.map.levels[0].px_hei as f32, 
            layer_bundle.transform.translation.z);

            
        transform.translation.z = layer.settings.layer_id as f32;
         */
        //map.add_layer(&mut commands, layer.settings.layer_id, layer_entity);
        
        

    }

    debug!("Transform:: {:?}",Transform::from_xyz(
        0.0, 
        -ldtk.map.levels[0].px_hei as f32, 
        0.0
    ));
    //setup layer_builder
    commands
            .entity(map_entity)
            .insert(map)
            .insert(Transform::from_xyz(
                0.0, 
                -ldtk.map.levels[0].px_hei as f32, 
                0.0
            ));
    debug!("Test");

}




fn main() {
    
    //env_logger::init();
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    App::build()
        .insert_resource(WindowDescriptor {
            width: 600.0,
            height: 720.0,
            title: String::from("LDTK Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup.system())
        .add_startup_system(map_setup.system())
        .add_system(helpers::camera::movement.system())
        .add_system(helpers::texture::set_texture_filters_to_nearest.system())
        .run();
}
