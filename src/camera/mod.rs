use crate::game;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cameras_system);
    }
}

pub fn setup_cameras_system(
    mut commands: Commands,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup cameras
    // 2d camera for sprites

    let camera_2d = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z),
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    };

    commands.spawn(camera_2d);

    // 3d cemate for background objects
    let camera_3d = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            far: 10000.0,
            ..Default::default()
        }),
        ..Default::default()
    };
    commands.spawn(camera_3d);
}
