use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics,FrameTimeDiagnosticsPlugin};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf");
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
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
                },
            },
            ..Default::default()
        });
}

pub fn text_update_system(diagnostics: Res<Diagnostics>, mut text: Mut<Text>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.value = format!("FPS: {:.2}", average);
        }
    }
}
