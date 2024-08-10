use bevy::prelude::*;

mod ui_container_animation;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MyPlugin, ui_container_animation::MyPlugin))
        .run();
}

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
