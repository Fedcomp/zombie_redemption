use bevy::property::DynamicProperties;
use bevy::type_registry::TypeUuid;

/// Asset container for tiled map
#[derive(Debug, TypeUuid, Default)]
#[uuid = "981dc9a3-3f02-4e0d-a4ea-c97b90cd285e"]
pub struct Prefab {
    pub class: String,
    pub shape: String,
    pub components: Vec<DynamicProperties>,
}

pub trait TiledObject {
    fn gid(&self) -> u32;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn rotation(&self) -> f32;
}

#[macro_export]
macro_rules! prefab_component {
    ($Name:ident, $($element: ident: $ty: ty),*) => {
        #[derive(Bundle, Properties, Default)]
        pub struct $Name {
        pub prefab: Handle<Prefab>,
        pub gid: u32,
        pub width: f32,
        pub height: f32,
        pub x: f32,
        pub y: f32,
        pub rotation: f32,
        pub visible: bool,
        $($element: $ty),*
        }
        
        impl TiledObject for $Name {
            fn gid(&self) -> u32 {self.gid}
            fn width(&self) -> f32 {self.width}
            fn height(&self) -> f32 {self.height}
            fn x(&self) -> f32 {self.x}
            fn y(&self) -> f32 {self.y}
            fn rotation(&self) -> f32 {self.rotation}
        }
    }
}