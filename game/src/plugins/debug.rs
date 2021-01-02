use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_ui.system())
            .add_system(text_update_system.system());
    }
}

pub fn setup_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CameraUiBundle::default()).spawn(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            value: "FPS:".to_string(),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                font_size: 18.0,
                color: Color::BLACK,
                ..Default::default()
            },
        },
        ..Default::default()
    });
}

pub fn text_update_system(diagnostics: Res<Diagnostics>, mut texts: Query<&mut Text>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in texts.iter_mut() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
