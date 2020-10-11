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
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_default_plugins()
        .add_plugin(MapPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_events: ResMut<Events<MapEvents>>,
) {
    let texture_handle = asset_server.load("assets/player.png").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });

    map_events.send(LoadMap("zr_test".into()));
}
