use crate::{assets::AddComponent, prefab_component};

use bevy::{ecs::Bundle, prelude::Component, type_registry::ComponentRegistration};
use bevy_rapier2d::rapier::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder};
use bevy::prelude::*;
use crate::assets::Prefab;

prefab_component!(Cuboid, _nil: bool);

impl AddComponent for Cuboid {
    fn add(self,world: &mut World,entity: Entity) {
        let body = RigidBodyBuilder::new_dynamic()
        .translation(self.x,self.y)
        .rotation(self.rotation.to_radians());

        world.insert_one(entity, body).unwrap();
    }
}