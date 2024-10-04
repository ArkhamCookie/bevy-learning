use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::WindowResolution;

use bevy_rapier2d::prelude::*;

use rand::Rng;

const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_WIDTH: f32 = 720.0;

const PADDLE_HEIGHT: f32 = 150.0;
const PADDLE_WIDTH: f32 = 10.0;
const BALL_SIZE: f32 = 25.0;

#[derive(Component)]
struct Paddle {
	move_up: KeyCode,
	move_down: KeyCode,
}

#[derive(Component)]
struct Ball;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
	Player1,
	Player2,
}

impl Player {
	fn start_speed(&self) -> Velocity {
		match self {
			Player::Player1 => Velocity::linear(Vec2::new(100.0, 0.0)),
			Player::Player2 => Velocity::linear(Vec2::new(-100.0, 0.0)),
		}
	}
}

#[derive(Event)]
enum GameEvents {
	ResetBall(Player),
	GainPoint(Player),
}

#[derive(Default, Resource)]
struct Score(HashMap<Player, i32>);

/// Spawn in camera
fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

/// Spawn in border of game
fn spawn_border(mut commands: Commands) {
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
fn spawn_players(mut commands: Commands) {
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
fn spawn_ball(mut commands: Commands) {
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
fn spawn_score(mut commands: Commands) {
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
