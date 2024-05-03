use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(setup,set_window_size).chain())
        .add_systems(Update, key_input)
        .run();
}

fn setup(mut command: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    command.spawn(Camera2dBundle::default());

    command.spawn(MaterialMesh2dBundle{
        mesh:  Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        material: materials.add(Color::rgb(0.0,255.0,0.0)),
        transform: Transform::from_xyz(100.0, 100.0, 0.0),
        ..default()
    });
}

fn set_window_size(mut windows: Query<&mut Window>){
    let mut window = windows.single_mut();
    window.resolution.set(800.0,600.0);
}

fn key_input(keys: Res<ButtonInput<KeyCode>>, mut rect_pos: Query<&mut Transform, With<Mesh2dHandle>>){
    if keys.pressed(KeyCode::KeyD){
        for mut rect in &mut rect_pos {
            rect.translation.x += 5.0;
            println!("rect x is {}",rect.translation.x);
        }
        println!("Key D pressed!");
    }
    else if keys.pressed(KeyCode::KeyA){
        for mut rect in &mut rect_pos {
            rect.translation.x -= 5.0;
            println!("rect x is {}",rect.translation.x);
        }
        println!("Key A pressed!");
    }
}