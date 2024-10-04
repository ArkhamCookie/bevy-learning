use events::game_events::GameEvents;
use events::paddle::move_paddle;
use events::reset::{detect_reset, reset_ball};
use events::score::{score_run, Score};

use internal::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use internal::spawn::*;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::WindowResolution;

use bevy_rapier2d::prelude::*;

mod events;
mod internal;

/// Create and start game
fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
			resizable: false,
			..Default::default()
		}),
		..Default::default()
	}));
	app.insert_resource(RapierConfiguration {
		gravity: Vec2::ZERO,
		..RapierConfiguration::new(1.0)
	});
	app.insert_resource(Score(HashMap::new()));
	app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
	#[cfg(debug_assertions)]
	app.add_plugins(RapierDebugRenderPlugin::default());
	app.add_event::<GameEvents>();
	app.add_systems(
		Startup,
		(
			spawn_camera,
			spawn_border,
			spawn_players,
			spawn_ball,
			spawn_score,
		),
	);
	app.add_systems(Update, (move_paddle, detect_reset));
	app.add_systems(PostUpdate, (reset_ball, score_run));
	app.run();
}
