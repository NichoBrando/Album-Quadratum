use bevy::prelude::*;

mod helpers;
mod player;
mod components;

struct GameTextures
{
    square_sprite: Handle<Image>
}

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;


const SQUARE_SPRITE: &str = "white_square.png"; 

fn main() 
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
			title: "Album Quadratum".to_string(),
			width: 400.0,
			height: 400.0,
			..Default::default()
		})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_plugin(player::PlayerPlugin)
        .add_system(movable_system)
        .run();
}

fn movable_system(
	mut query: Query<(&components::Velocity, &mut Transform)>,
) {
	for (velocity, mut transform) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.y * TIME_STEP * BASE_SPEED;
	}
}


fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	let game_textures: GameTextures = GameTextures {
		square_sprite: asset_server.load(SQUARE_SPRITE)
	};
    commands.insert_resource(false);
	commands.insert_resource(game_textures);
}
