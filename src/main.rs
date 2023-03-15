use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::prelude::*;

pub const START_NUMBER_OF_ENEMIES: usize = 3;
pub const MAX_NUMBER_OF_ENEMIES: usize = 20;
pub const ENEMY_SPAWN_TIME: f32 = 5.;
pub const ENEMY_SPEED: f32 = 250.;
pub const ENEMY_SPEED_MIN: f32 = 250.;
pub const ENEMY_SPEED_MAX: f32 = 600.;
pub const ENEMY_SIZE: f32 = 64.;

pub const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: f32 = 64.;

pub const MAX_NUMBER_OF_STARS: usize = 10;

pub const STAR_SIZE: f32 = 30.;
pub const STAR_SPAWN_TIME: f32 = 1.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(update_enemy_direction)
        .add_system(confine_enemy_movement)
        .add_system(enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        // .add_system(enemy_hit_enemy)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub struct GameOver {
    pub score: u32,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..START_NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                speed: ENEMY_SPEED,
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..MAX_NUMBER_OF_STARS {
        let x_pos = random::<f32>() * window.width();
        let y_pos = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.)
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * enemy.speed * time.delta_seconds();
    }
}

pub fn enemy_hit_enemy(mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>) {
    let mut iter = enemy_query.iter_combinations_mut();
    while let Some([(enemy_a_transform, mut enemy_a), (enemy_b_transform, mut enemy_b)]) =
        iter.fetch_next()
    {
        let distance = enemy_a_transform
            .translation
            .distance(enemy_b_transform.translation);
        let enemy_radius = ENEMY_SIZE / 2.;
        // let enemy_radius = ENEMY_SIZE / 2.;
        if distance < 2. * enemy_radius {
            // let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
            // audio.play(sound_effect);
            // commands.entity(player_entity).despawn();
            if random::<f32>() < 0.5 {
                if enemy_a.speed * 1.1 < ENEMY_SPEED_MAX {
                    enemy_a.speed *= 1.1;
                }
                if enemy_b.speed * 0.9 > ENEMY_SPEED_MIN {
                    enemy_b.speed *= 0.9;
                }
            } else {
                if enemy_b.speed * 1.1 < ENEMY_SPEED_MAX {
                    enemy_b.speed *= 1.1;
                }
                if enemy_a.speed * 0.9 > ENEMY_SPEED_MIN {
                    enemy_a.speed *= 0.9;
                }
            }
            enemy_a.direction.x *= -1.;
            enemy_a.direction.y *= -1.;
            enemy_b.direction.x *= -1.;
            enemy_b.direction.y *= -1.;
            // enemy_a.direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
            // enemy_b.direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max
        }

        player_transform.translation = translation;
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.;

    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
            direction_changed = true;
        }
        // if direction_changed {
        //     let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
        //     let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
        //     let sound_effect = if random::<f32>() > 0.5 {
        //         sound_effect_1
        //     } else {
        //         sound_effect_2
        //     };
        //     audio.play(sound_effect);
        // }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let half_enemy_size = ENEMY_SIZE / 2.;

    let window = window_query.get_single().unwrap();
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max
        }

        enemy_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let enemy_radius = ENEMY_SIZE / 2.;
            if distance < player_radius + enemy_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let star_radius = STAR_SIZE / 2.;
            if distance < player_radius + star_radius {
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}!", score.value);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    stars_query: Query<Entity, With<Star>>,
) {
    if star_spawn_timer.timer.finished() {
        let current_number_of_stars = stars_query.iter().len();
        if current_number_of_stars < MAX_NUMBER_OF_STARS {
            let window = window_query.get_single().unwrap();
            let x_pos = random::<f32>() * window.width();
            let y_pos = random::<f32>() * window.height();
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ));
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if enemy_spawn_timer.timer.finished() {
        let current_number_of_enemies = enemy_query.iter().len();
        if current_number_of_enemies < MAX_NUMBER_OF_ENEMIES {
            let window = window_query.get_single().unwrap();
            let x_pos = random::<f32>() * window.width();
            let y_pos = random::<f32>() * window.height();
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2 {
                        x: random::<f32>(),
                        y: random::<f32>(),
                    }
                    .normalize(),
                    speed: ENEMY_SPEED,
                },
            ));
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("FINAL SCORE: {}", event.score.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: ResMut<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
