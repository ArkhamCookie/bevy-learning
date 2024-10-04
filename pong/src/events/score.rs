use crate::events::game_events::GameEvents;
use crate::internal::player::Player;

use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default, Resource)]
pub(crate) struct Score(HashMap<Player, i32>);

/// Detect if player scored and give
pub(crate) fn score_run(
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
