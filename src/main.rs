use std::time::SystemTime;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::{Query, Color, Timer};
use rand::Rng;
use std::time::Duration;
use bevy::core::CoreSystem::Time;
use crate::KeyCode::P;


// goal : square changing to random color ever X seconds

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(keyboard_input)
        .insert_resource(
            ColorTimer(Timer::new(Duration::from_secs_f32(1.0),true)))
        .add_plugins(DefaultPlugins)
        .add_system(timer_change_color)
        .add_system(move_sprite_horizontal)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    })
        .insert(PongBall);
}

fn move_sprite_horizontal(windows: ResMut<Windows>, mut query: Query<&mut Transform, With<PongBall>>) {
    let window = windows.get_primary().unwrap();
    for mut transform in query.iter_mut() {
    //     match transform.translation.x {
    //         x > *window  => {}
    //         _ => {}
    //     }
        transform.translation.x += 3.;
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
struct PongBall;