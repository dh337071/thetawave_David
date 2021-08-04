use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::*};
use ron::de::from_bytes;

mod game;
mod misc;
mod player;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Theta Wave".to_string(),
            width: 960.0,  // TODO: move to config file
            height: 720.0, // TODO: move to config file
            ..Default::default()
        })
        .insert_resource(
            from_bytes::<player::CharactersResource>(include_bytes!("../data/characters.ron"))
                .unwrap(),
        )
        .insert_resource(
            from_bytes::<game::GameParametersResource>(include_bytes!(
                "../data/game_parameters.ron"
            ))
            .unwrap(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_game.system().label("init"))
        .add_startup_system(misc::spawn_barrier_system.system().after("init"))
        .add_startup_system(player::spawn_player_system.system().after("init"))
        .add_system(player::player_movement_system.system())
        //.add_system(print_player_position.system())
        .run();
}

fn setup_game(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_parameters: Res<game::GameParametersResource>,
) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // setup rapier
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = game_parameters.physics_scale;
}

// print position of the player
fn print_player_position(query: Query<&RigidBodyPosition, With<player::PlayerComponent>>) {
    for player_pos in query.iter() {
        info!(
            "Player position: {:?}",
            player_pos.position.translation.vector.data
        );
    }
}
