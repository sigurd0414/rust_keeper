// src/main.rs

use bevy::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world_system.system())
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .run();
}

fn hello_world_system() {
    println!("Hello, world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Alice".to_string()));
    commands.spawn().insert(Person).insert(Name("Bob".to_string()));
    commands.spawn().insert(Person).insert(Name("Charlie".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}