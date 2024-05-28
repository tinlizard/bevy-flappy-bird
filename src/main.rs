use std::time::Duration;
use rand::prelude::*;
use bevy::{
    prelude::*, 
    window::PrimaryWindow
};

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PipesTop;

#[derive(Component)]
struct PipesBottom;

#[derive(Component)]
struct Pipes;

#[derive(Component)]
struct PipesTimer {
    time: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(setup,set_window_size).chain())
        .add_systems(Update, 
            (spawn_pipes, 
            key_input,
            animate_sprite, 
            check_offscreen,
            move_pipes))
        .run();
}

//setup basic stuff and spawn intial Entities
fn setup(
    mut command: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
){
    command.spawn(Camera2dBundle::default());

    let texture = asset_server.load("bird_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(34.0, 72.0), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let background_texture = asset_server.load("background-day.png");

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2};

    command.spawn(SpriteBundle{
        texture: background_texture,
        ..default()
    });
    command.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(1.2)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player
    ));
    command.spawn(PipesTimer {
        time: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });

}

//set window size at initialization
fn set_window_size(mut windows: Query<&mut Window, With<PrimaryWindow>>){
    let mut window = windows.single_mut();
    window.resolution.set(288.0,512.0);
    window.title = "Flappy Bird".to_string();
}

//change position of player when the space key is pressed
fn key_input(
    keys: Res<ButtonInput<KeyCode>>, 
    mut sprite_pos: Query<(&mut Transform, &TextureAtlas)>
){
     if keys.pressed(KeyCode::Space){
        for (mut rect, _sprite) in &mut sprite_pos {
            rect.translation.y += 4.0;
            //println!("rect y pos is {}",rect.translation.y);
        }
    }
    for (mut rect, _sprite) in &mut sprite_pos {
        rect.translation.y -= 2.1;
    }
}


//animate bird flapping when space key is pressed
fn animate_sprite(
    keys: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>, 
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<Player>>, 
){
    if keys.pressed(KeyCode::Space){
        for (indices, mut timer, mut atlas) in &mut query{
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } 
                else {
                    atlas.index + 1
                };
            }
        }
    } 
}

//check if the player is dead (offscreen)
fn check_offscreen(mut command: Commands,mut sprite_pos: Query<(&mut Transform, &mut TextureAtlas)>, pipes: Query<Entity, With<Pipes>>){
    //reset bird position if the player dies (goes offscreen), and despawn all pipes 
    for (mut rect, _sprite) in &mut sprite_pos {
        if rect.translation.y < -240.0 {
            rect.translation.y = 70.0;
            for pipe in &pipes {
                command.entity(pipe).despawn();
            }
        }
    }
}

//spawn pipes on a timer
fn spawn_pipes(mut commands: Commands, asset_server: Res<AssetServer>, mut pipe_query: Query<&mut PipesTimer>, time: Res<Time>){
    let pipe_top_texture = asset_server.load("top_pipe_green.png");
    let pipe_bot_texture = asset_server.load("bottom_pipe_green.png");

    for mut timer in &mut pipe_query {
        timer.time.tick(time.delta());
        //println!("time is {:? }", timer.time);
        if timer.time.finished(){
            /*generate a random number between zero and 1 to be used for the randomized y coordinates for the pipe 
            spawn locations */
            let mut rng = rand::thread_rng();
            let rand_num: f32 = rng.gen();  

            //spawn pipes 
            commands.spawn((
                SpriteBundle{
                 texture: pipe_top_texture.clone(),
                 transform:
                     Transform::from_xyz(160.0, (rand_num*100.0)+220.0, 0.0),
                 ..default()
                },
                 PipesTop,
                 Pipes,
             ),
            );
             commands.spawn((
                 SpriteBundle{
                  texture: pipe_bot_texture.clone(),
                  transform:
                     Transform::from_xyz(160.0, -200.0+(rand_num*100.0), 0.0),
                  ..default()
                 },
                  PipesBottom,
                  Pipes,
              ));
        }
    }
    
}

//move pipes
fn move_pipes(mut command: Commands, mut pipes: Query<(Entity, &mut Transform), With<Pipes>>){
        for (pipe, mut pos) in &mut pipes {
            pos.translation.x -= 1.0;

            if pos.translation.x < -170.0 {
                    command.entity(pipe).despawn();
            }
        }
}