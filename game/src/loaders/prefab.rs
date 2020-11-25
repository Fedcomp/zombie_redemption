use bevy::{asset::{AssetLoader, LoadContext, LoadedAsset}, property::PropertyTypeRegistry};
use serde::de::DeserializeSeed;
use std::{ sync::Arc};
use bevy::prelude::{FromResources,Resources};
use bevy::type_registry::TypeRegistry;
use ron::de::Deserializer;
use parking_lot::RwLock;
use crate::{serde::PrefabDeserializer};
use bevy::utils::BoxedFuture;

#[derive(Debug)]
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

impl AssetLoader for PrefabLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
        let registry = self.property_type_registry.read();
        let mut deserializer = Deserializer::from_bytes(&bytes)?;
        let pschema_deserializer = PrefabDeserializer {
            property_type_registry: &registry,
        };
        let pschema = pschema_deserializer.deserialize(&mut deserializer)?;
        load_context.set_default_asset(LoadedAsset::new(pschema));
        Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["pfb"];
        EXTENSIONS
    }
}