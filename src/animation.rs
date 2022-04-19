use std::time::Duration;

use bevy::prelude::*;

/// Plugin responsible for tile animations.
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
		.add_system(run_tile_animations);
	}
}

#[derive(Component)]
pub struct CharacterAnimationTimer(pub Timer);

impl CharacterAnimationTimer {
	pub fn new(duration: Duration) -> CharacterAnimationTimer {
		CharacterAnimationTimer(Timer::new(duration, true))
	}
}

/// Run all animations for entities with TileAnimation components.
fn run_tile_animations(
	time: Res<Time>,
	mut query: Query<(&mut CharacterAnimationTimer, &TileAnimation, &mut TextureAtlasSprite)>,
) {
	for (mut timer, animation, mut sprite) in query.iter_mut() {
		timer.0.tick(time.delta());
		if timer.0.finished() {
			if sprite.index == animation.end_index {
				sprite.index = animation.start_index;
			} else {
				sprite.index = sprite.index + 1;
			}
		}
	}
}

/// Component to indicate an entity has an animated tile sprite.
#[derive(Component)]
pub struct TileAnimation {
	/// The index of the starting animation sprite.
	pub start_index: usize,
	/// The index of the ending animation sprite.
	pub end_index: usize,
}
