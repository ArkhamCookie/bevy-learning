use bevy::prelude::*;

#[derive(Component)]
struct Paddle {
	move_up: KeyCode,
	move_down: KeyCode,
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

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
			custom_size: Some(Vec2::new(10.0, 150.0)),
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
			custom_size: Some(Vec2::new(10.0, 150.0)),
			..Default::default()
		},
		..Default::default()
	}, Paddle {
		move_up: KeyCode::ArrowUp,
		move_down: KeyCode::ArrowDown,
	}));
}

fn move_paddle(
	mut paddles: Query<(&mut Transform, &Paddle)>,
	input: Res<ButtonInput<KeyCode>>,
) {
	for (mut pos, settings) in &mut paddles {
		if input.pressed(settings.move_up) {
			pos.translation.y += 1.0;
		}

		if input.pressed(settings.move_down) {
			pos.translation.y -= 1.0;
		}
	}
}

fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins);
	app.add_systems(Startup, (spawn_camera, spawn_players));
	app.add_systems(Update, move_paddle);
	app.run();
}
