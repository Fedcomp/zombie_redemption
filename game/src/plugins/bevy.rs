use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct BevyPlugins;

impl PluginGroup for BevyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy::type_registry::TypeRegistryPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());

        // #[cfg(feature = "bevy_render")]
        group.add(bevy::render::RenderPlugin::default());

        // #[cfg(feature = "bevy_sprite")]
        group.add(bevy::sprite::SpritePlugin::default());

        // #[cfg(feature = "bevy_pbr")]
        group.add(bevy::pbr::PbrPlugin::default());

        // #[cfg(feature = "bevy_ui")]
        group.add(bevy::ui::UiPlugin::default());

        // #[cfg(feature = "bevy_text")]
        group.add(bevy::text::TextPlugin::default());

        // #[cfg(feature = "bevy_audio")]
        // Not completely working on bevy 3.0 on my system
        // we don't have any sounds yet so i turned it off
        // group.add(bevy::audio::AudioPlugin::default());

        // #[cfg(feature = "bevy_gltf")]
        group.add(bevy::gltf::GltfPlugin::default());

        // #[cfg(feature = "bevy_winit")]
        group.add(bevy::winit::WinitPlugin::default());

        // #[cfg(feature = "bevy_wgpu")]
        group.add(bevy::wgpu::WgpuPlugin::default());
    }
}
