use bitflags::bitflags;
use rstar::{RTreeObject, AABB};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, path::PathBuf};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DamageType: u8 {
        const Pothole = 1 << 0;
        const Crack = 1 << 1;
        const Patch = 1 << 2;
        const Other = 1 << 3;
    }
}

impl From<u32> for DamageType {
    fn from(value: u32) -> Self {
        Self::from_bits(value as u8).unwrap_or(Self::Other)
    }
}

impl From<DamageType> for bool {
    fn from(value: DamageType) -> Self {
        value.bits() != 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoadDamage {
    pub id: u64,
    #[serde(serialize_with = "serialize_damage_type")]
    #[serde(deserialize_with = "deserialize_damage_type")]
    pub damage_type: DamageType,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoaddamageAdditionalInfo {
    pub file_path: PathBuf,
}

fn serialize_damage_type<S>(value: &DamageType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u8(value.bits())
}

fn deserialize_damage_type<'de, D>(deserializer: D) -> Result<DamageType, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = DamageType;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number that is a bitmask of damage types")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // TODO: return an error indicating that a value is not correct
            // (how does serde::de::Error work???)
            Ok(DamageType::from_bits_truncate(v as u8))
        }
    }
    deserializer.deserialize_u8(Visitor)
}

impl RTreeObject for RoadDamage {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.longitude, self.latitude])
    }
}
