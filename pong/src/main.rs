use bevy::{prelude::*, window::WindowResolution};
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
struct Ball(Vec2);

/// Spawn in camera
fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

/// Spawn in players and play area
fn spawn_players(mut commands: Commands) {
	// Spawn Player 1
	commands.spawn((SpriteBundle {
		transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 + 20.0 , 0.0, 0.0)),
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
		transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0 + -20.0, 0.0, 0.0)),
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
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		sprite: Sprite {
			color: Color::srgb(0.0, 0.0, 10.0),
			custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
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
			pos.translation.y = pos.translation.y.clamp((-WINDOW_HEIGHT / 2.0) + 75.0, (WINDOW_HEIGHT / 2.) - 75.0);
		}

		if input.pressed(settings.move_down) {
			pos.translation.y -= 100.0 * time.delta_seconds();
			pos.translation.y = pos.translation.y.clamp((-WINDOW_HEIGHT / 2.0) + 75.0, (WINDOW_HEIGHT / 2.0) - 75.0);
		}
	}
}

/// Move ball
fn move_ball(
	mut balls: Query<(&mut Transform, &Ball)>,
	time: Res<Time>,
) {
	for (mut pos, ball) in &mut balls {
		pos.translation += ball.0.extend(0.0) * time.delta_seconds();
	}
}

/// Add ball collision
fn ball_collide(
	mut balls: Query<(&Transform, &mut Ball)>,
	paddles: Query<&Transform, With<Paddle>>,
) {
	for (ball, mut velocity) in &mut balls {
		if ball.translation.y.abs() + BALL_SIZE / 2.0 > WINDOW_HEIGHT / 2.0 {
			velocity.0.y *= -1.0;
		}

		for paddle in &paddles {
			if
				ball.translation.x - BALL_SIZE / 2.0 < paddle.translation.x + PADDLE_WIDTH / 2.0 &&
				ball.translation.y - BALL_SIZE / 2.0 < paddle.translation.y + PADDLE_HEIGHT / 2.0 &&
				ball.translation.x + BALL_SIZE / 2.0 > paddle.translation.x - PADDLE_WIDTH / 2.0 &&
				ball.translation.y + BALL_SIZE / 2.0 > paddle.translation.y - PADDLE_HEIGHT / 2.0 {
					velocity.0 *= -1.0;
					// TODO: Make ball direction based on paddle direction
					velocity.0.y = rand::thread_rng().gen_range(-1.0..1.0) * 100.0;
				}
		}
	}
}

/// Create and start game
fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins
	.set(WindowPlugin {
		primary_window: Some(Window {
			resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
			resizable: false,
			..Default::default()
		}),
		..Default::default()
	}));
	app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
	app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
	app.add_systems(Update, (move_paddle, move_ball, ball_collide));
	app.run();
}
