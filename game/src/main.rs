mod assets;
mod components;
mod events;
mod loaders;
mod plugins;
mod resources;
mod systems;
mod serde;

use env_logger::Env;
use bevy::{prelude::*, math::vec3};
use crate::plugins::PrefabPlugin;
use crate::plugins::MapPlugin;
use crate::events::PrefabEvents::{self, LoadPrefab};
use crate::events::MapEvents::{self, LoadMap};
use crate::plugins::BevyPlugins;
use crate::plugins::DebugUiPlugin;
use bevy_rapier2d::{na::Vector2};
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};
// use bevy_rapier2d::render::RapierRenderPlugin;

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
        .add_plugins(BevyPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(RapierConfiguration {
            gravity: Vector2::new(0.0, -100.0),
            scale: 0.5,
            ..Default::default()
        })
        .add_plugin(PrefabPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(DebugUiPlugin)
        //.add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pschema_events: ResMut<Events<PrefabEvents>>,
    mut map_events: ResMut<Events<MapEvents>>
) {

    pschema_events.send(LoadPrefab("sprite_cuboid".into()));

    commands.spawn(Camera2dComponents {
        transform: Transform::from_scale(vec3(2.0,2.0,2.0)),
        ..Default::default()
    });
    map_events.send(LoadMap("zr_test".into()));
}
