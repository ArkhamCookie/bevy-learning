use crate::internal::player::Player;

use bevy::prelude::*;

#[derive(Event)]
pub(crate) enum GameEvents {
	ResetBall(Player),
	GainPoint(Player),
}
