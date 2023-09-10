use bitflags::bitflags;
use rstar::{RTreeObject, AABB};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, path::PathBuf, str::FromStr};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DamageType: u16 {
        const Alligator_crack = 1 << 0;
        const Rutting_bump_pothole_separation = 1 << 1;
        const Linear_longitudinal_crack = 1 << 2;
        const White_line_blur = 1 << 3;
        const Linear_lateral_crack = 1 << 4;
        const Cross_walk_blur = 1 << 5;
        const Utility_hole_maintenance_hatch = 1 << 6;
        const Repair = 1 << 7;
    }
}

impl From<u32> for DamageType {
    fn from(value: u32) -> Self {
        Self::from_bits(value as u16).unwrap()
    }
}

impl FromStr for DamageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "Продольная трещина" => Self::Linear_longitudinal_crack,
            "Поперечная трещина" => Self::Linear_lateral_crack,
            "Аллегаторная трещина" => Self::Alligator_crack,
            "Колея, неровность, выбоина, расслоение" => {
                Self::Rutting_bump_pothole_separation
            }
            "Размытие пешеходного перехода" => Self::Cross_walk_blur,
            "Размытие дорожной разметки" => Self::White_line_blur,
            "Ремонт" => Self::Repair,
            "Служебное отверстие (люк для обслуживания)" => {
                Self::Utility_hole_maintenance_hatch
            }
            _ => unreachable!(),
        };
        Ok(res)
    }
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Self::Linear_longitudinal_crack => "Продольная трещина",
            Self::Linear_lateral_crack => "Поперечная трещина",
            Self::Alligator_crack => "Аллегаторная трещина",
            Self::Rutting_bump_pothole_separation => "Колея, неровность, выбоина, расслоение",
            Self::Cross_walk_blur => "Размытие пешеходного перехода",
            Self::White_line_blur => "Размытие дорожной разметки",
            Self::Repair => "Ремонт",
            Self::Utility_hole_maintenance_hatch => "Служебное отверстие (люк для обслуживания)",
            _ => unreachable!(),
        };
        write!(f, "{s}")
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
    pub top_certainty: f64,
    pub top_type: String,
}

fn serialize_damage_type<S>(value: &DamageType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(value.bits())
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
            Ok(DamageType::from_bits_truncate(v as u16))
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
