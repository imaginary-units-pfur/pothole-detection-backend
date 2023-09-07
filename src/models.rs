use std::path::PathBuf;

use bitflags::bitflags;
bitflags! {
    pub struct DamageType: u8 {
        const None = 1 << 0;
        const Pothole = 1 << 1;
        const Crack = 1 << 2;
        const Patch = 1 << 3;
        const Other = 1 << 4;
    }
}

impl From<i64> for DamageType {
    fn from(value: i64) -> Self {
        Self::from_bits(value as u8).unwrap_or(Self::Other)
    }
}

pub struct RoadDamage {
    pub damage_type: DamageType,
    pub file_path: PathBuf,
    pub latitude: f64,
    pub longitude: f64,
}
