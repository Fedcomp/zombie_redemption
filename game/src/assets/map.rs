use bevy::prelude::*;
use bevy::math::{Quat, Vec2, vec3, vec2};
use tiled::Object;
use bevy::type_registry::TypeUuid;

/// Asset container for tiled map
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "a94ea61a-ba7d-4a1c-abe4-6edb6ce9d878"]

pub struct Map {
    pub source: tiled::Map,
}

impl Map {
    pub fn new(source: tiled::Map) -> Self {
        Map { source }
    }

    pub fn tile_size(&self) -> Vec2 {
        vec2(self.source.tile_width as f32,self.source.tile_height as f32)
    }

    pub fn map_size(&self) -> Vec2 {
        let tsize = self.tile_size();
        vec2((self.source.width as f32)*tsize.x(), (self.source.height as f32)*tsize.y())
    }

    // TODO: If needed add support for layers offset

    // Using only ortho projections
    pub fn tile_project(&self,ix:f32,iy:f32) -> Vec2 {
        vec2(ix, iy) * self.tile_size()
    }

    pub fn obj_project(&self,obj: &Object, phys_scale: f32) -> Vec2 {
        let sobj = vec3(obj.width,obj.height,0.0);
        let pobj = vec2(obj.x,self.map_size().y() - obj.y);
        (Transform::from_translation(sobj)
            .with_rotate(Quat::from_rotation_z(obj.rotation.to_radians()))
            .translation() - sobj)
        .truncate() + (sobj.truncate() - self.tile_size() ) * 0.5 / phys_scale + pobj / phys_scale
    }
}
