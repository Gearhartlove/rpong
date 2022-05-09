use std::ops::Neg;
use std::process::exit;
use std::time::SystemTime;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::{Query, Color, Timer};
use rand::Rng;
use std::time::Duration;
use bevy::core::CoreSystem::Time;
use crate::KeyCode::P;

const WIDTH_WINDOW: i32 = 600;
const HEIGHT_WINDOW: i32 = 800;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(keyboard_input)
        .add_startup_system(setup_camera)
        .insert_resource(WindowDescriptor {
            title: "Square Magic!".to_string(),
            width: WIDTH_WINDOW as f32,
            height: HEIGHT_WINDOW as f32,
            ..default()
        })
        .insert_resource(
            ColorTimer(Timer::new(Duration::from_secs_f32(1.0),true)))
        .add_plugins(DefaultPlugins)
        .add_system(timer_change_color)
        .add_system(move_sprite_horizontal)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    })
        .insert(PongBall { direction: Direction::East, speed: 5. });
}

fn move_sprite_horizontal(windows: ResMut<Windows>, mut query: Query<(&mut Transform, &mut PongBall)>) {
    let window = windows.get_primary().unwrap();
    let mut speed = 5.;
    let x_window_bounds = window.width()/2.;

    for (mut transform, mut pong_ball) in query.iter_mut() {
        if transform.translation.x.abs() > x_window_bounds {
            pong_ball.flip_direction();
        }
        match pong_ball.direction {
            Direction::East => {
                transform.translation.x += pong_ball.speed;
            }
            Direction::West => {
                transform.translation.x -= pong_ball.speed;
            }
        }

        //     match transform.translation.x {
    //         x > *window  => {}
    //         _ => {}
    //     }
    //     if transform.translation.x >= window.width() {
    //         transform.translation.x -= move_x;
    //         println!("left")
    //     } else if transform.translation.x >= window.width().neg() {
    //         transform.translation.x += move_x;
    //         println!("right")
    //     }

    }
}

struct ColorTimer(Timer);

fn timer_change_color(mut timer: ResMut<ColorTimer>, mut query: Query<&mut Sprite>) {
    timer.0.tick(Duration::from_secs_f32(0.01)); //manually tick timer
    // change sprites color
    if timer.0.finished() {
        timer.0.reset();
        for mut sprite in query.iter_mut() {
            *sprite.color.set_r(rand::random::<f32>());
            *sprite.color.set_g(rand::random::<f32>());
            *sprite.color.set_b(rand::random::<f32>());
        }
    }
}

fn change_color(color: &mut Color) {
    // change sprites color
    *color.set_r(rand::random::<f32>());
    *color.set_g(rand::random::<f32>());
    *color.set_b(rand::random::<f32>());
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Sprite>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("Space pressed {:?}", SystemTime::now());
        // for mut sprite in query.iter_mut() {
            // change_color(&mut sprite.color)
        // }
    }
}

#[derive(Component)]
struct PongBall {
    direction: Direction,
    speed: f32,
}

impl PongBall {
    fn flip_direction(&mut self) {
        match self.direction {
            Direction::West => {
                self.direction = Direction::East
            }
            Direction::East => {
                self.direction = Direction::West
            }
        }
    }
}

enum Direction {
    East,
    West
}