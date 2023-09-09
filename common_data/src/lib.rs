use bitflags::bitflags;
use rstar::{RTreeObject, AABB};
use serde::{Serialize, Serializer};
use std::path::PathBuf;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct DamageType: u8 {
        const Pothole = 1 << 0;
        const Crack = 1 << 1;
        const Patch = 1 << 2;
        const Other = 1 << 3;
    }
}

impl From<i64> for DamageType {
    fn from(value: i64) -> Self {
        Self::from_bits(value as u8).unwrap_or(Self::Other)
    }
}

impl From<DamageType> for bool {
    fn from(value: DamageType) -> Self {
        value.bits() != 0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadDamage {
    #[serde(serialize_with = "serialize_damage_type")]
    pub damage_type: DamageType,
    pub file_path: PathBuf,
    pub latitude: f64,
    pub longitude: f64,
}

fn serialize_damage_type<S>(value: &DamageType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u8(value.bits())
}

impl RTreeObject for RoadDamage {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.longitude, self.latitude])
    }
}
