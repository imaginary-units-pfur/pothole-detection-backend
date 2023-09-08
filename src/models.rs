use std::path::PathBuf;

use bitflags::bitflags;
use rstar::{RTreeObject, AABB};
bitflags! {
    #[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct RoadDamage {
    pub damage_type: DamageType,
    pub file_path: PathBuf,
    pub latitude: f64,
    pub longitude: f64,
}

impl RTreeObject for RoadDamage {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.longitude, self.latitude])
    }
}
