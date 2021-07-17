use bevy::{prelude::*, render::camera::Camera};



impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_stage(
            "map_setup",
            SystemStage::single(map_load.system()),
        )
            .add_system(player_movement.system())
            .add_system(player_fire.system())
            .add_system(lazer_movement.system());  
        }
}


fn map_load(
    mut commands: Commands, 
    materials: Res<Materials>){

    //spawn sprite

    let bottom = -win_size.h / 2.0;

    commands.spawn_bundle(SpriteBundle{
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, bottom + 60.0, 10.),
                scale: Vec3::new(0.5,0.5,0.5),
                ..Default::default()
            },            
            ..Default::default()  
    })
    .insert(Player)
    .insert(PlayerReadyFire(true))
    .insert(Speed::default());

}