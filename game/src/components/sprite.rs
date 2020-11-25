use crate::{assets::Map, events::MapMaterials, prefab_component};

use bevy::{ecs::Bundle, type_registry::IntoComponent, math::vec2};
use bevy_rapier2d::{physics::RapierConfiguration};
use bevy::prelude::*;
use crate::assets::{TiledObject,Prefab};

prefab_component!(SpriteC, _nil: bool);

impl IntoComponent<SpriteComponents> for SpriteC {
    fn into_component(&self,resources: &Resources) -> SpriteComponents {
        let maps = resources.get::<Assets<Map>>().unwrap();
        let map_materials = resources.get::<MapMaterials>().unwrap();
        let rapier_conf = resources.get::<RapierConfiguration>().unwrap();
        let map = maps.iter().next().unwrap().1;
        let sobj = vec2(self.width,self.height);
        let tsize = map.tile_size();
        let vobj = map.obj_project(self,rapier_conf.scale);

        let material = map_materials.materials.get(&self.gid).expect(&format!("Unknown object material {}", &self.gid));

        let mut pos = Transform::from_translation(vobj.extend(0.0));
        pos.apply_non_uniform_scale((sobj/tsize).extend(0.0));


        SpriteComponents {
            material: material.clone(),
            transform: pos,
            ..Default::default()
        }
    }
}