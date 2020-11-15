use bevy::{asset::AssetLoader, property::PropertyTypeRegistry};
use serde::de::DeserializeSeed;
use std::{path::Path, sync::Arc};
use bevy::prelude::{FromResources,Resources};
use bevy::type_registry::TypeRegistry;
use ron::de::Deserializer;
use parking_lot::RwLock;
use anyhow::Result;
use crate::{assets::Prefab, serde::PrefabDeserializer};
pub struct PrefabLoader {
    property_type_registry: Arc<RwLock<PropertyTypeRegistry>>,
}

impl FromResources for PrefabLoader {
    fn from_resources(resources: &Resources) -> Self {
        let type_registry = resources.get::<TypeRegistry>().unwrap();
        PrefabLoader {
            property_type_registry: type_registry.property.clone(),
        }
    }
}

impl AssetLoader<Prefab> for PrefabLoader {
    fn from_bytes(&self, _asset_path: &Path, bytes: Vec<u8>) -> Result<Prefab> {
        let registry = self.property_type_registry.read();
        let mut deserializer = Deserializer::from_bytes(&bytes)?;
        let pschema_deserializer = PrefabDeserializer {
            property_type_registry: &registry,
        };
        let pschema = pschema_deserializer.deserialize(&mut deserializer)?;
        Ok(pschema)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["pfb"];
        EXTENSIONS
    }
}