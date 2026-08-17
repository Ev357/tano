use serde::{Deserialize, Deserializer, de};

pub fn deserialize_percentage<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    let cleaned = s.trim().trim_end_matches('%');

    cleaned.parse::<i32>().map_err(de::Error::custom)
}
