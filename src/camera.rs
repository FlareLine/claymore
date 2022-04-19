use bevy::{prelude::*, input::mouse::{MouseWheel}};

use crate::{cursor::CursorMotion, state::GameState};

/// Plugin for pan
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system(setup_camera)
		.add_system_set(SystemSet::on_update(GameState::Game).with_system(mouse_pan).with_system(mouse_zoom));
	}
}

fn setup_camera(
	mut commands: Commands,
) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn mouse_pan(
	mut cursor_motion: EventReader<CursorMotion>,
	mut camera_transforms: Query<&mut Transform, With<Camera>>,
	mouse_input: Res<Input<MouseButton>>,
) {
	if mouse_input.pressed(MouseButton::Middle) {
		for motion in cursor_motion.iter() {
			for mut transform in camera_transforms.iter_mut() {
				let scale = transform.scale;
				transform.translation += Vec3::new(-motion.0.x, -motion.0.y, 0.) * scale;
			}
		}
	}
}

fn mouse_zoom(
	mut camera_transforms: Query<&mut Transform, With<Camera>>,
	mut scroll_event: EventReader<MouseWheel>,
) {
	for event in scroll_event.iter() {
		match event.unit {
			_ => {
				for mut transform in camera_transforms.iter_mut() {
					transform.scale = (transform.scale - (event.y / 2.)).clamp(Vec3::splat(1.), Vec3::splat(4.));
				}
			}
		}
	}
}
