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
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;

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
        .add_default_plugins()
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
        //.add_startup_system(setup_physics.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut map_events: ResMut<Events<MapEvents>>,
) {
    commands.spawn(Camera2dComponents {
        transform: Transform::from_scale(1.0),
        ..Default::default()
    });
    map_events.send(LoadMap("zr_test".into()));
}


pub fn setup_physics(mut commands: Commands,asset_server: Res<AssetServer>,mut materials: ResMut<Assets<ColorMaterial>>) {

    let texture_handle = asset_server.load("assets/maps/missing_texture.png").unwrap();

    /*
     * Create the cubes
     */
    let num = 10;
    let rad = 16.0;

    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;

    for i in 0..num {
        for j in 0usize..num {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery + 0.0;

            // Build the rigid body.
            let body = RigidBodyBuilder::new_dynamic().translation(x*2.0, y*2.0);
            let collider = ColliderBuilder::cuboid(rad, rad).density(1.0);

            commands.spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            }).with(body).with(collider);
        }
    }
}
