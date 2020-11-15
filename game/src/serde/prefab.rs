
use bevy::property::{DynamicProperties,PropertyTypeRegistry};
use bevy::property::property_serde::{DynamicPropertiesDeserializer,DynamicPropertiesSerializer};
use bevy::property::serde::de::{Visitor,Error,DeserializeSeed,SeqAccess,MapAccess};
use bevy::property::serde;
use bevy::ecs::bevy_utils::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize, ser::{SerializeSeq, SerializeStruct}};
use crate::assets::Prefab;

pub struct PrefabSerializer<'a> {
    pub prefab: &'a Prefab,
    pub registry: &'a PropertyTypeRegistry,
}

impl<'a> Serialize for PrefabSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct(PREFAB_STRUCT, 2)?;
        state.serialize_field(PREFAB_FIELD_CLASS, &self.prefab.class)?;
        state.serialize_field(
            PREFAB_FIELD_COMPONENTS,
            &ComponentsSerializer {
                components: &self.prefab.components,
                registry: self.registry,
            },
        )?;
        state.end()
    }
}

pub struct ComponentsSerializer<'a> {
    pub components: &'a [DynamicProperties],
    pub registry: &'a PropertyTypeRegistry,
}

impl<'a> Serialize for ComponentsSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.components.len()))?;
        for dynamic_properties in self.components.iter() {
            state.serialize_element(&DynamicPropertiesSerializer::new(
                dynamic_properties,
                self.registry,
            ))?;
        }
        state.end()
    }
}

struct PrefabSeqVisiter<'a> {
    pub property_type_registry: &'a PropertyTypeRegistry,
}

impl<'a, 'de> Visitor<'de> for PrefabSeqVisiter<'a> {
    type Value = HashMap<String,Prefab>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("list of prefabs")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut prefabs = HashMap::default();
        while let Some(prefab) = seq.next_element_seed(PrefabDeserializer {
            property_type_registry: self.property_type_registry,
        })? {
            prefabs.insert(prefab.class.clone(),prefab);
        }

        Ok(prefabs)
    }
}

pub struct PrefabDeserializer<'a> {
    pub property_type_registry: &'a PropertyTypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for PrefabDeserializer<'a> {
    type Value = Prefab;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            PREFAB_STRUCT,
            &[PREFAB_FIELD_CLASS, PREFAB_FIELD_COMPONENTS],
            PrefabVisiter {
                registry: self.property_type_registry,
            },
        )
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum PrefabField {
    Class,
    Components,
}

pub const PREFAB_STRUCT: &str = "Prefab";
pub const PREFAB_FIELD_CLASS: &str = "class";
pub const PREFAB_FIELD_SHAPE: &str = "shape";
pub const PREFAB_FIELD_COMPONENTS: &str = "components";

struct PrefabVisiter<'a> {
    pub registry: &'a PropertyTypeRegistry,
}

impl<'a, 'de> Visitor<'de> for PrefabVisiter<'a> {
    type Value = Prefab;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("prefabs")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut id = None;
        let mut components = None;
        while let Some(key) = map.next_key()? {
            match key {
                PrefabField::Class => {
                    if id.is_some() {
                        return Err(Error::duplicate_field(PREFAB_FIELD_CLASS));
                    }
                    id = Some(map.next_value::<String>()?);
                }
                PrefabField::Components => {
                    if components.is_some() {
                        return Err(Error::duplicate_field(PREFAB_FIELD_COMPONENTS));
                    }

                    components = Some(map.next_value_seed(ComponentVecDeserializer {
                        registry: self.registry,
                    })?);
                }
            }
        }

        let class = id
            .as_ref()
            .ok_or_else(|| Error::missing_field(PREFAB_FIELD_CLASS))?;

        let components = components
            .take()
            .ok_or_else(|| Error::missing_field(PREFAB_FIELD_COMPONENTS))?;
        Ok(Prefab {
            class: class.into(),
            components,
            ..Default::default()
        })
    }
}

pub struct ComponentVecDeserializer<'a> {
    pub registry: &'a PropertyTypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for ComponentVecDeserializer<'a> {
    type Value = Vec<DynamicProperties>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ComponentSeqVisiter {
            registry: self.registry,
        })
    }
}

struct ComponentSeqVisiter<'a> {
    pub registry: &'a PropertyTypeRegistry,
}

impl<'a, 'de> Visitor<'de> for ComponentSeqVisiter<'a> {
    type Value = Vec<DynamicProperties>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("list of components")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut dynamic_properties = Vec::new();
        while let Some(prefab) =
            seq.next_element_seed(DynamicPropertiesDeserializer::new(self.registry))?
        {
            dynamic_properties.push(prefab);
        }

        Ok(dynamic_properties)
    }
}