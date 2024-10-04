use crate::player::Player;
use crate::{Ball, Paddle};
use crate::{BALL_SIZE, PADDLE_HEIGHT, PADDLE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Spawn in camera
pub(crate) fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

/// Spawn in border of game
pub(crate) fn spawn_border(mut commands: Commands) {
	// Add collision to top of screen
	commands.spawn((
		SpatialBundle {
			transform: Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT / 2.0, 0.0)),
			..Default::default()
		},
		RigidBody::Fixed,
		Collider::cuboid(WINDOW_WIDTH / 2.0, 3.0),
	));
	// Add collision to bottom of screen
	commands.spawn((
		SpatialBundle {
			transform: Transform::from_translation(Vec3::new(0.0, -WINDOW_HEIGHT / 2.0, 0.0)),
			..Default::default()
		},
		RigidBody::Fixed,
		Collider::cuboid(WINDOW_WIDTH / 2.0, 3.0),
	));
	// Add trigger for right side border of screen
	commands.spawn((
		SpatialBundle {
			transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0, 0.0, 0.0)),
			..Default::default()
		},
		RigidBody::Fixed,
		Collider::cuboid(3.0, WINDOW_HEIGHT / 2.0),
		Player::Player1,
		Sensor,
	));
	// Add trigger for left side border of screen
	commands.spawn((
		SpatialBundle {
			transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0, 0.0, 0.0)),
			..Default::default()
		},
		RigidBody::Fixed,
		Collider::cuboid(3.0, WINDOW_HEIGHT / 2.0),
		Player::Player2,
		Sensor,
	));
}

/// Spawn in players and play area
pub(crate) fn spawn_players(mut commands: Commands) {
	// Spawn Player 1
	commands.spawn((
		SpriteBundle {
			transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 + 20.0, 0.0, 0.0)),
			sprite: Sprite {
				color: Color::WHITE,
				custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		Paddle {
			move_up: KeyCode::KeyW,
			move_down: KeyCode::KeyS,
		},
		RigidBody::KinematicPositionBased,
		Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
	));

	// Spawn Player 2
	commands.spawn((
		SpriteBundle {
			transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0 + -20.0, 0.0, 0.0)),
			sprite: Sprite {
				color: Color::WHITE,
				custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
				..Default::default()
			},
			..Default::default()
		},
		Paddle {
			move_up: KeyCode::ArrowUp,
			move_down: KeyCode::ArrowDown,
		},
		RigidBody::KinematicPositionBased,
		Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
	));
}

/// Spawn in ball
pub(crate) fn spawn_ball(mut commands: Commands) {
	commands.spawn((
		SpriteBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			sprite: Sprite {
				color: Color::srgb(0.0, 0.0, 10.0),
				custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
				..Default::default()
			},
			..Default::default()
		},
		Ball,
		RigidBody::Dynamic,
		CollidingEntities::default(),
		ActiveEvents::COLLISION_EVENTS,
		Collider::ball(BALL_SIZE),
		Velocity::linear(Vec2::new(100.0, 0.0)),
		Restitution {
			coefficient: 1.1,
			combine_rule: CoefficientCombineRule::Max,
		},
	));
}

/// Render/spawn in score
pub(crate) fn spawn_score(mut commands: Commands) {
	commands
		.spawn(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				margin: UiRect::horizontal(Val::Auto),
				top: Val::ZERO,
				width: Val::Percent(30.0),
				height: Val::Percent(20.0),
				..Default::default()
			},
			..Default::default()
		})
		.with_children(|p| {
			p.spawn((
				TextBundle {
					text: Text {
						sections: vec![TextSection {
							value: "0".to_string(),
							style: TextStyle {
								font_size: 100.,
								..Default::default()
							},
						}],
						..Default::default()
					},
					..Default::default()
				}
				.with_text_justify(JustifyText::Center),
				Player::Player1,
			));

			p.spawn(TextBundle {
				text: Text {
					sections: vec![TextSection {
						value: "|".to_string(),
						style: TextStyle {
							font_size: 100.,
							..Default::default()
						},
					}],
					..Default::default()
				},
				..Default::default()
			});

			p.spawn((
				TextBundle {
					text: Text {
						sections: vec![TextSection {
							value: "0".to_string(),
							style: TextStyle {
								font_size: 100.,
								..Default::default()
							},
						}],
						..Default::default()
					},
					..Default::default()
				}
				.with_text_justify(JustifyText::Center),
				Player::Player2,
			));
		});
}
