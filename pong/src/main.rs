use bevy::prelude::*;

const PADDLE_HEIGHT: f32 = 150.0;
const PADDLE_WIDTH: f32 = 10.0;

#[derive(Component)]
struct Paddle {
	move_up: KeyCode,
	move_down: KeyCode,
}

#[derive(Component)]
struct Ball(Vec2);

/// Spawn in camera
fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

/// Spawn in players and play area
fn spawn_players(mut commands: Commands) {
	// Spawn Play Area
	commands.spawn(SpriteBundle {
		sprite: Sprite {
			color: Color::BLACK,
			custom_size: Some(Vec2::new(700.0, 500.0)),
			..Default::default()
		},
		..Default::default()
	});

	// Spawn Player 1
	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
		sprite: Sprite {
			color: Color::WHITE,
			custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
			..Default::default()
		},
		..Default::default()
	}, Paddle {
		move_up: KeyCode::KeyW,
		move_down: KeyCode::KeyS,
	}));

	// Spawn Player 2
	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
		sprite: Sprite {
			color: Color::WHITE,
			custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
			..Default::default()
		},
		..Default::default()
	}, Paddle {
		move_up: KeyCode::ArrowUp,
		move_down: KeyCode::ArrowDown,
	}));
}

/// Spawn in ball
fn spawn_ball(mut commands: Commands) {
	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
		sprite: Sprite {
			color: Color::srgb(0.0, 0.0, 10.0),
			custom_size: Some(Vec2::new(25.0, 25.0)),
			..Default::default()
		},
		..Default::default()
	}, Ball(Vec2::new(-100.0, 0.0))));
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
			pos.translation.y = pos.translation.y.clamp(-250.0 + (PADDLE_HEIGHT / 2.0), 250.0 - (PADDLE_HEIGHT / 2.0));
		}

		if input.pressed(settings.move_down) {
			pos.translation.y -= 100.0 * time.delta_seconds();
			pos.translation.y = pos.translation.y.clamp(-250.0 + (PADDLE_HEIGHT / 2.0), 250.0 - (PADDLE_HEIGHT / 2.0));
		}
	}
}

/// Create and start game
fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins);
	app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
	app.add_systems(Update, move_paddle);
	app.run();
}
