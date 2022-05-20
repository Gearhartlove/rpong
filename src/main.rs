use std::borrow::BorrowMut;
use std::ops::Neg;
use std::process::exit;
use std::time::SystemTime;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::{Query, Color, Timer};
use rand::{Rng, thread_rng};
use std::time::Duration;
use bevy::core::CoreSystem::Time;
use bevy::sprite::collide_aabb::collide;

const WIDTH_WINDOW: i32 = 600;
const HEIGHT_WINDOW: i32 = 800;
const PADDLE_X_OFFSET: i32 = 50;
const PADDLE_Y_OFFSET: i32 = 5;
const PADDLE_SPEED: i32 = 5;
const PADDLE_HEIGHT: i32 = 200;
const FONT_PATH: &str = "JetBrainsMono-2.242/fonts/ttf/JetBrainsMono-Medium.ttf";
const TEXT_OFFSET: f32 = 40.;
const PONG_BALL_MAX_SPEED: f32 = 10.;
const SPEED_SCALING: f32 = 0.26;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        // .add_startup_system(load_ui_font)
        .insert_resource(WindowDescriptor {
            title: "rpong".to_string(),
            width: WIDTH_WINDOW as f32,
            height: HEIGHT_WINDOW as f32,
            ..default()
        })
        .insert_resource(
            ColorTimer(Timer::new(Duration::from_secs_f32(1.0), true)))
        .insert_resource(
            Scoreboard::default()
        )
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
    commands.spawn_bundle(UiCameraBundle::default());
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
    // rpong text
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(2.),
                right: Val::Px(15.0 / 2.),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "rpong",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 100.,
                color: Color::ANTIQUE_WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        ),
        ..default()
    });
    // left text
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(HEIGHT_WINDOW as f32 / 2.),
                right: Val::Px(WIDTH_WINDOW as f32 / 2. + TEXT_OFFSET),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "0",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 25.,
                color: Color::ANTIQUE_WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        ),
        ..default()
    })
        .insert(PlayerTwo);
    // right text
    // left text
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(HEIGHT_WINDOW as f32 / 2.),
                right: Val::Px(WIDTH_WINDOW as f32 / 2. - TEXT_OFFSET),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "0",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 25.,
                color: Color::ANTIQUE_WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        ),
        ..default()
    })
        .insert(PlayerOne);
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

fn move_pong_ball(
    windows: ResMut<Windows>, mut scoreboard: ResMut<Scoreboard>,
    mut pong_query: Query<(&mut Transform, &mut PongBall)>,
    mut p1_text_query: Query<&mut Text, (With<PlayerOne>, Without<PlayerTwo>)>,
    mut p2_text_query: Query<&mut Text, (With<PlayerTwo>, Without<PlayerOne>)>
    ){
    let window = windows.get_primary().unwrap();
    let x_window_bounds = window.width() / 2.;
    let y_window_bounds = window.height() / 2.;

    for (mut transform, mut pong_ball) in pong_query.iter_mut() {
        if transform.translation.x.abs() > x_window_bounds {
            pong_ball.flip_horizontal_direction();
            pong_ball.increase_horizontal_speed();

            let update_score = |new_score: &mut i32, text_to_update: &mut Text| {
                const POINT_VALUE: i32 = 1;
                *new_score += POINT_VALUE;
                let new_text_val: String = new_score.to_string();
                text_to_update.sections[0].value = new_text_val;
            };

            // Updating OnScreenScoreboard
            // check if left side or right side of the screen
            if transform.translation.x < 0. {
                for mut text in p1_text_query.iter_mut() {
                    update_score(scoreboard.p1_score.borrow_mut(), &mut text);
                }
            }
            else {
                for mut text in p2_text_query.iter_mut() {
                    update_score(scoreboard.p2_score.borrow_mut(), &mut text);
                }
            }

            transform.translation.x = 0.;
            transform.translation.y = 0.;
            pong_ball.randomize_direction();
            pong_ball.reset_speed();
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
    let qwaoffset = 1.;
    if keys.any_pressed([KeyCode::W]) {
        for (mut paddle, sprite) in query.iter_mut() {
            if paddle.is_bounded() {
                paddle.translation.y += PADDLE_SPEED as f32;
            }
        }
    } else if keys.any_pressed([KeyCode::S]) {
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
    } else if keys.any_pressed([KeyCode::Down]) {
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
        let mut rng = rand::thread_rng();
        let n: f32 = rng.gen();
        self.vertical_speed += self.horizontal_speed * n;

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

    // todo: clamp speed
    fn increase_horizontal_speed(&mut self) {
        if self.horizontal_speed < PONG_BALL_MAX_SPEED {
            self.horizontal_speed += self.horizontal_speed * SPEED_SCALING ;
        }
    }

    fn increase_vertical_speed(&mut self) {
        if self.vertical_speed < PONG_BALL_MAX_SPEED {
            self.vertical_speed += self.vertical_speed * SPEED_SCALING ;
        }
    }

    fn reset_speed(&mut self) {
        self.horizontal_speed = PongBall::default().horizontal_speed;
        self.vertical_speed = PongBall::default().vertical_speed;
    }

    fn randomize_direction(&mut self) {
        let random_horizontal_direction = || -> Direction {
            let mut rng =  thread_rng();
            match rng.gen_range(0..=1) {
                0 => Direction::East,
                _ => Direction::West,
            }
        };
        let random_vertical_direction = || -> Direction {
            let mut rng =  thread_rng();
            match rng.gen_range(0..=1) {
                0 => Direction::North,
                _ => Direction::South,
            }
        };
        self.horizontal_direction = random_horizontal_direction();
        self.vertical_direction = random_vertical_direction();
    }
}

impl Default for PongBall {
    fn default() -> Self {
        Self {
            horizontal_direction: Direction::West,
            vertical_direction: Direction::South,
            horizontal_speed: 5.,
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

// todo: Refactor Player logic
// #[derive(Component)]
// enum Player {
//     PlayerOne,
//     PlayerTwo,
// }

#[derive(Component)]
struct PlayerOne;

#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
struct Scoreboard {
    p1_score: i32,
    p2_score: i32,
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard {
            p1_score: 0,
            p2_score: 0,
        }
    }
}