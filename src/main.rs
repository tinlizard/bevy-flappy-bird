use bevy::{
    prelude::*, 
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle}
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
        .add_systems(Update, (key_input, animate_sprite))
        .run();
}

fn setup(mut command: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>){
    command.spawn(Camera2dBundle::default());

    command.spawn(MaterialMesh2dBundle{
        mesh:  Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        material: materials.add(Color::rgb(0.0,255.0,0.0)),
        transform: Transform::from_xyz(100.0, 100.0, 0.0),
        ..default()
    });

    let texture = asset_server.load("bird_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(34.0, 72.0), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2};
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

fn set_window_size(mut windows: Query<&mut Window>){
    let mut window = windows.single_mut();
    window.resolution.set(800.0,600.0);
}

fn key_input(keys: Res<ButtonInput<KeyCode>>, mut rect_pos: Query<&mut Transform, With<Mesh2dHandle>>, mut sprite_pos: Query<(&mut Transform, &TextureAtlas), Without<Mesh2dHandle>>){
    if keys.pressed(KeyCode::KeyD){
        for mut rect in &mut rect_pos {
            rect.translation.x += 5.0;
            println!("rect x is {}",rect.translation.x);
        }
        for (mut rect, _sprite) in &mut sprite_pos {
            rect.translation.x += 5.0;
        }
        println!("Key D pressed!");
    }
    else if keys.pressed(KeyCode::KeyA){
        for mut rect in &mut rect_pos {
            rect.translation.x -= 5.0;
            println!("rect x is {}",rect.translation.x);
        }
        for (mut rect, _sprite) in &mut sprite_pos {
            rect.translation.x -= 5.0;
        }
        println!("Key A pressed!");
    }
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>){
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