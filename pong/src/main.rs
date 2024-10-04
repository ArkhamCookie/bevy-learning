use events::game_events::*;
use events::reset::*;
use events::score::score_run;
use events::score::Score;

use internal::components::*;
use internal::consts::*;
use internal::spawn::*;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::WindowResolution;

use bevy_rapier2d::prelude::*;

mod events;
mod internal;

/// Move paddles based on input
fn move_paddle(
	mut paddles: Query<(&mut Transform, &Paddle)>,
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	for (mut pos, settings) in &mut paddles {
		if input.pressed(settings.move_up) {
			pos.translation.y += 100.0 * time.delta_seconds();
			pos.translation.y = pos
				.translation
				.y
				.clamp((-WINDOW_HEIGHT / 2.0) + 75.0, (WINDOW_HEIGHT / 2.) - 75.0);
		}

		if input.pressed(settings.move_down) {
			pos.translation.y -= 100.0 * time.delta_seconds();
			pos.translation.y = pos
				.translation
				.y
				.clamp((-WINDOW_HEIGHT / 2.0) + 75.0, (WINDOW_HEIGHT / 2.0) - 75.0);
		}
	}
}

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
