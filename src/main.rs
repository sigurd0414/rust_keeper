// src/main.rs

use bevy::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(pub String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world_system)
        .add_startup_system(add_people)
        .add_system(greet_people)
        .run();
}

fn hello_world_system() {
    println!("Hello, world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}