// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::de::{self, Deserializer, Visitor};
use std::fmt;

pub fn int<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct V;
    impl Visitor<'_> for V {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer or a numeric string")
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<i64, E> {
            Ok(value)
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<i64, E> {
            i64::try_from(value).map_err(E::custom)
        }

        fn visit_f64<E: de::Error>(self, value: f64) -> Result<i64, E> {
            if value.fract() == 0.0 {
                Ok(value as i64)
            } else {
                Err(E::custom("expected an integer"))
            }
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<i64, E> {
            value.parse().map_err(E::custom)
        }

        fn visit_unit<E: de::Error>(self) -> Result<i64, E> {
            Ok(0)
        }
    }

    deserializer.deserialize_any(V)
}

pub fn boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct V;
    impl Visitor<'_> for V {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean, 0, 1, or a boolean string")
        }

        fn visit_bool<E: de::Error>(self, value: bool) -> Result<bool, E> {
            Ok(value)
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<bool, E> {
            Ok(value != 0)
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<bool, E> {
            match value {
                "1" | "true" | "True" => Ok(true),
                "0" | "false" | "False" => Ok(false),
                _ => Err(E::custom("expected a boolean string")),
            }
        }

        fn visit_unit<E: de::Error>(self) -> Result<bool, E> {
            Ok(false)
        }
    }

    deserializer.deserialize_any(V)
}

pub fn id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct V;
    impl Visitor<'_> for V {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number id")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<String, E> {
            Ok(value.to_owned())
        }

        fn visit_string<E: de::Error>(
            self, value: String,
        ) -> Result<String, E> {
            Ok(value)
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<String, E> {
            Ok(value.to_string())
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<String, E> {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(V)
}
