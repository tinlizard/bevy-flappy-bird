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


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(setup,set_window_size).chain())
        .add_systems(Update, (key_input, animate_sprite, check_offscreen))
        .run();
}

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
    ));

}

fn set_window_size(mut windows: Query<&mut Window, With<PrimaryWindow>>){
    let mut window = windows.single_mut();
    window.resolution.set(288.0,512.0);
    window.title = "Flappy Bird".to_string();
}

fn key_input(
    keys: Res<ButtonInput<KeyCode>>, 
    mut sprite_pos: Query<(&mut Transform, &TextureAtlas)>
){
     if keys.pressed(KeyCode::Space){
        for (mut rect, _sprite) in &mut sprite_pos {
            rect.translation.y += 2.0;
            println!("rect y pos is {}",rect.translation.y);
        }
    }
    for (mut rect, _sprite) in &mut sprite_pos {
        rect.translation.y -= 1.0;
    }
}

fn animate_sprite(
    keys: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>, 
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>, 
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

fn check_offscreen(mut sprite_pos: Query<(&mut Transform, &mut TextureAtlas)>){
    for (mut rect, _sprite) in &mut sprite_pos {
        if rect.translation.y < -240.0 {
            rect.translation.y = 70.0;
        }
    }
}