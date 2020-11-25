use crate::{assets::Map, prefab_component};

use bevy::{ecs::Bundle, type_registry::{IntoComponent}};
use bevy_rapier2d::{physics::RapierConfiguration, rapier::{geometry::ColliderBuilder, dynamics::RigidBodyBuilder}};
use bevy::prelude::*;
use crate::assets::{TiledObject,Prefab};

prefab_component!(RigidBody, _nil: bool);

impl IntoComponent<RigidBodyBuilder> for RigidBody {
    fn into_component(&self,resources: &Resources) -> RigidBodyBuilder {
        let maps = resources.get::<Assets<Map>>().unwrap();
        let rapier_conf = resources.get::<RapierConfiguration>().unwrap();
        let map = maps.iter().next().unwrap().1;
        let vobj = map.obj_project(self,rapier_conf.scale);

        RigidBodyBuilder::new_dynamic()
            .translation(vobj.x(),vobj.y())
            .rotation(self.rotation.to_radians())
    }
}



prefab_component!(Cuboid, _nil: bool);

impl IntoComponent<ColliderBuilder> for Cuboid {
    fn into_component(&self,_resources: &Resources) -> ColliderBuilder {
        ColliderBuilder::cuboid(self.width, self.height)
    }
}

