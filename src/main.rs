use bevy::{prelude::App, window::WindowDescriptor, DefaultPlugins};

/// Application entrypoint.
fn main() {
	App::new()
    .insert_resource(window_descriptor())
    .add_plugins(DefaultPlugins)
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
