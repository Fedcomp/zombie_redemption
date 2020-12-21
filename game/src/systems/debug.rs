use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub fn setup_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font_handle,
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
