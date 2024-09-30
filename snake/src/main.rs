use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

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
		.insert(SnakeHead);
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.add_systems(Startup, spawn_snake)
		.run();
}
