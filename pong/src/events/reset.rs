use crate::events::game_events::GameEvents;

use crate::components::Ball;
use crate::player::Player;

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use rand::Rng;

/// Detect when a reset should be triggered
pub(crate) fn detect_reset(
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
pub(crate) fn reset_ball(
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
