
use serde::{Serialize, Deserialize};

pub fn des<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<u8>, D::Error> {
    let temp = <&str as Deserialize>::deserialize(deserializer)?;
    Ok(Some(temp.parse().map_err(serde::de::Error::custom)?))
}

#[derive(Serialize, Deserialize)]
struct Test1 {
    #[serde(rename = "foo")]
    pub bar: String,
}

#[derive(Serialize, Deserialize)]
struct Test2 {
    #[serde(deserialize_with = "des")]
    pub i: Option<u8>,
}
