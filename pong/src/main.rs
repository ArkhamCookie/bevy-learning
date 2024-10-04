use components::*;
use player::*;
use spawn::*;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::WindowResolution;

use bevy_rapier2d::prelude::*;

use rand::Rng;

mod components;
mod player;
mod spawn;

const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_WIDTH: f32 = 720.0;

const PADDLE_HEIGHT: f32 = 150.0;
const PADDLE_WIDTH: f32 = 10.0;
const BALL_SIZE: f32 = 25.0;

#[derive(Event)]
enum GameEvents {
	ResetBall(Player),
	GainPoint(Player),
}

#[derive(Default, Resource)]
struct Score(HashMap<Player, i32>);

/// Detect when a reset should be triggered
fn detect_reset(
	input: Res<ButtonInput<KeyCode>>,
	balls: Query<&CollidingEntities, With<Ball>>,
	goal: Query<&Player, With<Sensor>>,
	mut game_events: EventWriter<GameEvents>,
) {
	if input.just_pressed(KeyCode::KeyR) {
		let player = if rand::thread_rng().gen::<bool>() {
			Player::Player1
		} else {
			Player::Player2
		};

		game_events.send(GameEvents::ResetBall(player));
		return;
	}
	for ball in &balls {
		for hit in ball.iter() {
			if let Ok(player) = goal.get(hit) {
				game_events.send(GameEvents::ResetBall(*player));
				game_events.send(GameEvents::GainPoint(*player));
			}
		}
	}
}

/// Reset the ball when triggered
fn reset_ball(
	mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
	mut game_events: EventReader<GameEvents>,
) {
	for events in game_events.read() {
		if let GameEvents::ResetBall(player) = events {
			for (mut ball, mut speed) in &mut balls {
				ball.translation = Vec3::ZERO;
				*speed = player.start_speed();
			}
		}
	}
}

/// Detect if player scored and give
fn score_run(
	mut events: EventReader<GameEvents>,
	mut score_text: Query<(&mut Text, &Player)>,
	mut scores: ResMut<Score>,
) {
	for event in events.read() {
		if let GameEvents::GainPoint(player) = event {
			*scores.0.entry(*player).or_default() += 1;
			let scores = scores.0.get(player).cloned().unwrap_or(0);
			for (mut text, owner) in &mut score_text {
				if owner != player {
					continue;
				}
				text.sections[0].value = scores.to_string();
			}
		}
	}
}

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
