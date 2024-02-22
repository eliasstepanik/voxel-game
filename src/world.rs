use bevy::prelude::*;
use crate::voxel_world::*;
use crate::voxel_world::voxel_terrain_plugin::VoxelTerrainPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, (spawn_light));
        app.add_plugins(VoxelTerrainPlugin);
    }
}

fn spawn_light(mut commands: Commands){
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0,5.0,0.0),
            ..default()
        },
        Name::new("Main Light"),
    );
    commands.spawn(light);
}