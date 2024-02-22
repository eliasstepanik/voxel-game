use bevy::prelude::*;
use bevy::asset::{AssetServer};
use bevy::core_pipeline::Skybox;
use bevy::render::texture::CompressedImageFormats;
use bevy_third_person_camera::*;
pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>){

    let camera = (
        Camera3dBundle{
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            zoom: Zoom::new(1.5, 10.0),
            cursor_lock_key: KeyCode::F1,
            ..default()
        }
    );


    commands.spawn(camera);

}
