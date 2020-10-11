mod assets;
mod loaders;
mod plugins;

use env_logger::Env;
use bevy::prelude::*;
use crate::assets::Map;
use crate::plugins::MapPlugin;

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
) {
    let texture_handle = asset_server.load("assets/player.png").unwrap();
    let map_handle = asset_server.load::<Map, &str>("assets/maps/zr_test.tmx").expect("Failed to load map");

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}
