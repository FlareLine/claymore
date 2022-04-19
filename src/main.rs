use std::time::Duration;

use animation::{AnimationPlugin, CharacterAnimationTimer, TileAnimation};
use assets::{CharacterAssets, TileAssets};
use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use camera::CameraPlugin;
use cursor::CursorPlugin;
use state::GameState;

mod animation;
mod assets;
mod camera;
mod cursor;
mod state;

/// Application entrypoint.
fn main() {
	let mut app = App::new();

	AssetLoader::new(GameState::Loading)
		.continue_to_state(GameState::Game)
		.with_collection::<TileAssets>()
		.with_collection::<CharacterAssets>()
		.build(&mut app);

	app.insert_resource(window_descriptor())
		.add_state(GameState::Loading)
		.add_plugin(CameraPlugin)
		.add_plugin(AnimationPlugin)
		.add_plugin(CursorPlugin)
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup_camera)
		.add_system_set(SystemSet::on_enter(GameState::Game).with_system(draw_texture_test))
		.run();
}

/// Window descriptor to control initial window settings.
/// TODO: Load these from saved settings.
fn window_descriptor() -> WindowDescriptor {
	WindowDescriptor {
		title: String::from("Claymore"),
		width: 800.0,
		height: 600.0,
		resizable: false,
		..Default::default()
	}
}

fn setup_camera(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn draw_texture_test(mut commands: Commands, assets: Res<CharacterAssets>) {
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: assets.adventurer.clone(),
			sprite: TextureAtlasSprite::new(1),
			transform: Transform::from_xyz(0., -150., 0.).with_scale(Vec3::splat(3.)),
			..Default::default()
		})
		.insert(TileAnimation {
			start_index: 0,
			end_index: 5,
		})
		.insert(CharacterAnimationTimer::new(Duration::from_millis(100)));
}
