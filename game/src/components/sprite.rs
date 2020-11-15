use crate::prefab_component;

use bevy::{ecs::Bundle, prelude::Component};
use bevy_rapier2d::rapier::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder};
use bevy::prelude::*;
use crate::assets::Prefab;

prefab_component!(Sprite, _nil: bool);

impl Sprite {
    fn into(self) -> SpriteComponents {
        SpriteComponents {
            material: *material,
            transform: Transform::from_non_uniform_scale((sobj/tsize).extend(0.0)),
            ..Default::default()
        }
    }
}