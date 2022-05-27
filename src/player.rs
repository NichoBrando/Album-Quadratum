use crate::helpers;
use crate::components;
use crate::GameTextures;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(player_keyboard_event_system)
			.add_system(player_spawn_system);
	}
}

fn player_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	did_spawn_player: Res<bool>,
) {
	if *did_spawn_player { 
		return;
	}
		commands
			.spawn_bundle(SpriteBundle {
				texture: game_textures.square_sprite.clone(),
				transform: Transform {
					translation: Vec3::new(
						0.,
						0.,
						10.,
					),
					scale: Vec3::new(0.25, 0.25, 0.25),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Player)
			.insert(components::Velocity { x: 0., y: 0. });
	commands.insert_resource(true);
}

pub fn player_keyboard_event_system(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<&mut components::Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        println!("player x: {} | player y: {} ", velocity.x, velocity.y);
		velocity.x = helpers::get_axis_value(
			&keyboard_input, 
			KeyCode::Left, 
			KeyCode::Right
		);
        velocity.y = helpers::get_axis_value(
			&keyboard_input, 
			KeyCode::Down, 
			KeyCode::Up
		);
    };
}
