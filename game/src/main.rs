mod assets;
mod components;
mod events;
mod loaders;
mod plugins;
mod systems;

use env_logger::Env;
use bevy::prelude::*;
use crate::plugins::MapPlugin;
use crate::events::MapEvents::{self, LoadMap};
use crate::plugins::DebugUiPlugin;

use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};

const GAME_NAME: &str = "Zombie Redemption";

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    App::build()
        .add_resource(WindowDescriptor {
            title: GAME_NAME.into(),
            width: 1024,
            height: 768,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(RapierConfiguration {
            gravity: Vector2::new(0.0,-100.0),
            scale: 0.5,
            ..Default::default()
        })
        //.add_plugin(RapierRenderPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(DebugUiPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut map_events: ResMut<Events<MapEvents>>,
) {
    commands.spawn(Camera2dComponents::default());
    map_events.send(LoadMap("zr_test".into()));
}
