use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
	x: i32,
	y: i32,
}

#[derive(Component)]
struct Size {
	width: f32,
	height: f32,
}
impl Size {
	fn square(x: f32) -> Self {
		Self {
			width: x,
			height: x,
		}
	}
}

#[derive(Component)]
struct SnakeHead;

fn spawn_snake(mut commands: Commands) {
	commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: SNAKE_HEAD_COLOR,
				..default()
			},
			transform: Transform {
				scale: Vec3::new(10.0, 10.0, 10.0),
				..default()
			},
			..default()
		})
		.insert(SnakeHead)
		.insert(Position { x: 3, y: 3})
		.insert(Size::square(0.8));
}

fn size_scaling(mut windows: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>) {
	let window = windows.single_mut();

	for (sprite_size, mut transfrom) in q.iter_mut() {
		transfrom.scale = Vec3::new(
			sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
			sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
			1.0,
		);
	}
}

fn position_translation(mut windows: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
	fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
		let tile_size = bound_window / bound_game;
		pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
	}

	let window = windows.single_mut();

	for (pos, mut transform) in q.iter_mut() {
		transform.translation = Vec3::new(
			convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
			convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
			0.0,
		);
	}
}

fn snake_movement(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
	for mut position in head_positions.iter_mut() {
		if keyboard_input.pressed(KeyCode::ArrowLeft) {
			position.x -= 1;
		}
		if keyboard_input.pressed(KeyCode::ArrowRight) {
			position.x += 1;
		}
		if keyboard_input.pressed(KeyCode::ArrowDown) {
			position.y -= 1;
		}
		if keyboard_input.pressed(KeyCode::ArrowUp) {
			position.y += 1;
		}
	};
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.add_systems(Startup, spawn_snake)
		.add_systems(Update, snake_movement)
		.add_systems(Update, (position_translation, size_scaling))
		.run();
}
