use bevy::prelude::*;

const GAME_NAME: &str = "Zombie Redemption";

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: GAME_NAME.into(),
            width: 800,
            height: 600,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_default_plugins()
        .run();
}
