use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Player {
	Player1,
	Player2,
}

impl Player {
	pub(crate) fn start_speed(&self) -> Velocity {
		match self {
			Player::Player1 => Velocity::linear(Vec2::new(100.0, 0.0)),
			Player::Player2 => Velocity::linear(Vec2::new(-100.0, 0.0)),
		}
	}
}
