use bevy::prelude::*;

#[derive(Component)]
pub (crate) struct Paddle {
	pub (crate) move_up: KeyCode,
	pub (crate) move_down: KeyCode,
}

#[derive(Component)]
pub (crate) struct Ball;
