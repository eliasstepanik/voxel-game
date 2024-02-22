use std::ops::Add;
use bevy::prelude::*;
use bevy::render::primitives::Sphere;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);


fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>)
{

    for (mut player_transform, player_speed) in player_q.iter_mut(){

        let cam: &Transform = cam_q.get_single().unwrap_or_else(|e| Err("Error retriving camera: ").unwrap());

        let mut direction = Vec3::ZERO;

        //forward
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z += cam.forward().z;
            direction.x += cam.forward().x;
        }

        //back
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += cam.back().z;
            direction.x += cam.back().x;
        }
        //right
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.z += cam.right().z;
            direction.x += cam.right().x;
        }
        //left
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.z += cam.left().z;
            direction.x += cam.left().x;
        }


        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to movement direction
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}

fn spawn_player(mut commands: Commands,
                server: Res<AssetServer>) {

    let flashlight = (SpotLightBundle::default(),Name::new("Flashlight"));

    let player_mesh = server.load("rocket.gltf#Scene0");
    let player = (
        SceneBundle{
            scene: player_mesh,
            transform: Transform::from_xyz(0.0,0.5,0.0).with_scale(Vec3::new(0.001,0.001,0.001)),
            ..default()
        },
        Speed(2.0),
        Player,
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );
    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}