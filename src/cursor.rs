use bevy::prelude::*;

/// Plugin to keep track of mouse cursor motion.
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_event::<CursorMotion>()
		.insert_resource(CursorPosition(Vec2::ZERO))
		.add_system(cursor_motion);
	}
}

/// 2D coordinate of the cursor.
#[derive(Debug)]
struct CursorPosition(pub Vec2);

/// 2D delta of the cursor's motion.
#[derive(Debug)]
pub struct CursorMotion(pub Vec2);

/// Event handler for cursor motion.
fn cursor_motion(
	mut previous_position: ResMut<CursorPosition>,
	mut cursor_position: EventReader<CursorMoved>,
	mut event_writer: EventWriter<CursorMotion>,
) {
	for new_position in cursor_position.iter() {
		let cursor_delta = new_position.position - previous_position.0;
		event_writer.send(CursorMotion(cursor_delta));
		previous_position.0 = new_position.position;
	}
}
