use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

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

#[derive(Component)]
enum Player {
	Player1,
	Player2,
}

/// Spawn in camera
fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

fn spawn_border(mut commands: Commands) {
	// Add collision to top of screen
	commands.spawn((
		SpatialBundle {
			transform: Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT / 2.0, 0.0)),
			..Default::default()
		},
		RigidBody::Fixed,
		Collider::cuboid(WINDOW_WIDTH / 2.0, 3.0)
	));
	// Add collision to bottom of screen
	commands.spawn((
		SpatialBundle {
			transform: Transform::from_translation(Vec3::new(0.0, - WINDOW_HEIGHT / 2.0, 0.0)),
			..Default::default()
		},
		RigidBody::Fixed,
		Collider::cuboid(WINDOW_WIDTH / 2.0, 3.0)
	));
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
	},
		RigidBody::KinematicPositionBased,
		Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
	));

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
	},
		RigidBody::KinematicPositionBased,
		Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
	));
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
	},
		Ball,
		RigidBody::Dynamic,
		Collider::ball(BALL_SIZE),
		Velocity::linear(Vec2::new(100.0, 0.0)),
		Restitution {
			coefficient: 1.1,
			combine_rule: CoefficientCombineRule::Max,
		}
	));
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
	app.insert_resource(RapierConfiguration {
		gravity: Vec2::ZERO,
		..RapierConfiguration::new(1.0)
	});
	app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
	#[cfg(debug_assertions)]
	app.add_plugins(RapierDebugRenderPlugin::default());
	app.add_systems(Startup, (spawn_camera, spawn_border, spawn_players, spawn_ball));
	app.add_systems(Update, move_paddle);
	app.run();
}
