use bevy::{prelude::World, property::DynamicProperties, prelude::Entity};

#[derive(Default)]
pub struct Prefab {
    pub class: String,
    pub shape: String,
    pub components: Vec<DynamicProperties>,
}

pub trait AddComponent where Self: Send + Sync + 'static {
    fn add(self,world: &mut World,entity: Entity);
}

#[macro_export]
macro_rules! prefab_component {
    ($Name:ident, $($element: ident: $ty: ty),*) => {
        #[derive(Bundle, Properties, Default)]
        pub struct $Name {
        pub prefab: Handle<Prefab>,
        pub width: f32,
        pub height: f32,
        pub x: f32,
        pub y: f32,
        pub rotation: f32,
        pub visible: bool,
        $($element: $ty),*
        }
    }
}