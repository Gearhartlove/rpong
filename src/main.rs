use std::ops::Neg;
use std::process::exit;
use std::time::SystemTime;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::{Query, Color, Timer};
use rand::Rng;
use std::time::Duration;
use bevy::core::CoreSystem::Time;
use bevy::sprite::collide_aabb::collide;
use crate::KeyCode::P;

const WIDTH_WINDOW: i32 = 600;
const HEIGHT_WINDOW: i32 = 800;
const PADDLE_X_OFFSET: i32 = 50;
const PADDLE_Y_OFFSET: i32 = 5;
const PADDLE_SPEED: i32 = 5;
const PADDLE_HEIGHT: i32 = 200;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_startup_system(load_ui_font)
        .insert_resource(WindowDescriptor {
            title: "rpong".to_string(),
            width: WIDTH_WINDOW as f32,
            height: HEIGHT_WINDOW as f32,
            ..default()
        })
        .insert_resource(
            ColorTimer(Timer::new(Duration::from_secs_f32(1.0), true)))
        .add_plugins(DefaultPlugins)
        .add_system(player_two_keyboard_input)
        .add_system(player_one_keyboard_input)
        .add_system(timer_change_color)
        .add_system(move_pong_ball)
        .add_system(pong_collision)
        .add_system(bound_paddle)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // left paddle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.75, 0.28, 0.65),
            custom_size: Some(Vec2::new(50., PADDLE_HEIGHT as f32)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new((WIDTH_WINDOW as f32) / 2. - PADDLE_X_OFFSET as f32, 0., 0.),
            ..default()
        },
        ..default()
    })
        .insert(Paddle)
        .insert(PlayerTwo);
    // left text
    commands.spawn_bundle(TextBundle {
        transform: Transform {
            translation: Vec3::new(-50., 0., 0.),
            ..default()
        },
        text: Text::with_section(
            "0",
            TextStyle {
                font: asset_server.get_handle("JetBrainsMono-2.242/fonts/ttf/JetBrainsMono-Medium.ttf"),
                font_size: 12.,
                color: Color::ANTIQUE_WHITE
            },
            TextAlignment::default(),
        ),
        ..default()
    })
        .insert(PlayerTwo);
    // right paddle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.35, 0.68, 0.65),
            custom_size: Some(Vec2::new(50., PADDLE_HEIGHT as f32)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-(WIDTH_WINDOW as f32) / 2. + PADDLE_X_OFFSET as f32, 0., 0.),
            ..default()
        },
        ..default()
    })
        .insert(Paddle)
        .insert(PlayerOne);
    // right text

    // pong ball
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    })
        .insert(PongBall::default());

}

fn move_pong_ball(windows: ResMut<Windows>, mut query: Query<(&mut Transform, &mut PongBall)>) {
    let window = windows.get_primary().unwrap();
    let x_window_bounds = window.width() / 2.;
    let y_window_bounds = window.height() / 2.;

    for (mut transform, mut pong_ball) in query.iter_mut() {
        if transform.translation.x.abs() > x_window_bounds {
            pong_ball.flip_horizontal_direction();
            pong_ball.increase_horizontal_speed();
        }
        if transform.translation.y.abs() > y_window_bounds {
            pong_ball.flip_vertical_direction();
            pong_ball.increase_vertical_speed();
        }
        match pong_ball.horizontal_direction {
            Direction::East => {
                transform.translation.x += pong_ball.horizontal_speed;
            }
            Direction::West => {
                transform.translation.x -= pong_ball.horizontal_speed;
            }
            _ => {}
        }
        match pong_ball.vertical_direction {
            Direction::North => {
                transform.translation.y += pong_ball.vertical_speed;
            }
            Direction::South => {
                transform.translation.y -= pong_ball.vertical_speed;
            }

            _ => {}
        }
    }
}

struct ColorTimer(Timer);

fn timer_change_color(mut timer: ResMut<ColorTimer>, mut query: Query<&mut Sprite, Without<Paddle>>) {
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

fn player_one_keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Sprite), (With<Paddle>, With<PlayerOne>)>) {
    let offset = 1.;
    if keys.any_pressed([KeyCode::W]) {
        for (mut paddle, sprite) in query.iter_mut() {
            if paddle.is_bounded() {
                paddle.translation.y += PADDLE_SPEED as f32;
            }
        }
    }
    else if keys.any_pressed([KeyCode::S]) {
        for (mut paddle, sprite) in query.iter_mut() {
            if paddle.is_bounded() {
                paddle.translation.y -= PADDLE_SPEED as f32;
            }
        }
    }
}

fn player_two_keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Sprite), (With<Paddle>, With<PlayerTwo>)>) {
    let offset = 1.;
    if keys.any_pressed([KeyCode::Up]) {
        for (mut paddle, sprite) in query.iter_mut() {
            if paddle.is_bounded() {
                paddle.translation.y += PADDLE_SPEED as f32;
            }
        }
    }
    else if keys.any_pressed([KeyCode::Down]) {
        for (mut paddle, sprite) in query.iter_mut() {
            if paddle.is_bounded() {
                paddle.translation.y -= PADDLE_SPEED as f32;
            }
        }
    }
}

#[derive(Component)]
struct PongBall {
    horizontal_direction: Direction,
    vertical_direction: Direction,
    horizontal_speed: f32,
    vertical_speed: f32,
}

impl PongBall {
    fn flip_horizontal_direction(&mut self) {
        match self.horizontal_direction {
            Direction::West => {
                self.horizontal_direction = Direction::East
            }
            Direction::East => {
                self.horizontal_direction = Direction::West
            }
            _ => {}
        }
    }

    fn flip_vertical_direction(&mut self) {
        match self.vertical_direction {
            Direction::North => {
                self.vertical_direction = Direction::South
            }
            Direction::South => {
                self.vertical_direction = Direction::North
            }
            _ => {}
        }
    }

    fn increase_horizontal_speed(&mut self) {
        let mut rng = rand::thread_rng();
        let n: f32 = rng.gen();
        self.horizontal_speed += self.horizontal_speed * n;
    }

    fn increase_vertical_speed(&mut self) {
        let mut rng = rand::thread_rng();
        let n: f32 = rng.gen();
        self.vertical_speed += self.vertical_speed * n;
    }
}

impl Default for PongBall {
    fn default() -> Self {
        Self {
            horizontal_direction: Direction::West,
            vertical_direction: Direction::South,
            horizontal_speed: 1.2,
            vertical_speed: 1.,
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Component)]
struct Paddle;

fn pong_collision(mut ball_query: Query<(&mut Transform, &Sprite, &mut PongBall), Without<Paddle>>,
                  mut paddle_query: Query<(&Transform, &Sprite), With<Paddle>>) {
    for (mut pong_transform, sprite, mut pong_ball) in ball_query.iter_mut() {
        let pong_pos = pong_transform.translation.clone();
        let pong_size = sprite.custom_size.unwrap().clone();
        for (paddle, sprite) in paddle_query.iter() {
            let paddle_pos = paddle.translation.clone();
            let paddle_size = sprite.custom_size.unwrap().clone();
            // check for collision between pong ball and paddle
            if let Some(c) =
            collide(pong_pos, pong_size, paddle_pos, paddle_size) {
                pong_ball.flip_horizontal_direction();
            }
        }
    }
}

trait Bounded {
    fn is_bounded(&self) -> bool;
    fn correct_bound(&mut self);
}

impl Bounded for Transform {
    fn is_bounded(&self) -> bool {
        let paddle_offset = (WIDTH_WINDOW as f32 / 2.) - PADDLE_Y_OFFSET as f32;
        if self.translation.y.abs() < paddle_offset {
            true
        } else {
            false
        }
    }

    fn correct_bound(&mut self) {
        let arbitrary_shift = 0.01;
        let boundry_reset = (HEIGHT_WINDOW as f32 / 2.)
            - PADDLE_Y_OFFSET as f32 - arbitrary_shift - PADDLE_HEIGHT as f32 / 2.;
        if self.translation.y < 0. {
            self.translation.y = -boundry_reset;
        } else {
            self.translation.y = boundry_reset;
        }
    }
}


fn bound_paddle(mut query: Query<&mut Transform, With<Paddle>>) {
    for mut transform in query.iter_mut() {
        // if not bounded, correct the bound
        if !transform.is_bounded() {
            transform.correct_bound()
        }
    }
}

#[derive(Component)]
struct PlayerOne;
#[derive(Component)]
struct PlayerTwo;

// todo: load the jetbrains mono font into asset server and load it on line 73
// learn: https://bevy-cheatbook.github.io/assets/assetserver.html
struct UIFont(Handle<Font>);
fn load_ui_font(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Font> = server.load("JetBrainsMono-2.242/fonts/ttf/JetBrainsMono-Medium.ttf");
    // commands.insert_resource(UIFont(handle));
}