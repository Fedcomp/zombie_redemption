mod systems;

use self::systems::{setup_ui, text_update_system};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_ui.system())
            .add_system(text_update_system.system());
    }
}
