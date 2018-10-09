use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for super::HyperLTL {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", *self))
    }
}

use std::fmt;

use serde::de::{self, Visitor};

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = super::HyperLTL;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a HyperLTL formula")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        super::parse::parse(value).map_err(|e| E::custom(format!("Error parsing LTL {}", e)))
    }
}

impl<'de> Deserialize<'de> for super::HyperLTL {
    fn deserialize<D>(deserializer: D) -> Result<super::HyperLTL, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(StrVisitor)
    }
}
