//! This module provides deserialisation to WKT primitives using [`serde`]

use crate::{Geometry, Wkt, WktFloat};
use serde::de::{Deserializer, Error, Visitor};
use std::{
    default::Default,
    fmt::{self, Debug},
    marker::PhantomData,
    str::FromStr,
};

#[cfg(feature = "geo-types")]
pub mod geo_types;


struct WktVisitor<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for WktVisitor<T> {
    fn default() -> Self {
        WktVisitor {
            _marker: PhantomData::default(),
        }
    }
}

impl<'de, T> Visitor<'de> for WktVisitor<T>
where
    T: FromStr + Default + Debug + WktFloat,
{
    type Value = Wkt<T>;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a valid WKT format")
    }
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Wkt::from_str(s).map_err(|e| serde::de::Error::custom(e))
    }
}

impl<'de, T> serde::Deserialize<'de> for Wkt<T>
where
    T: FromStr + Default + Debug + WktFloat,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WktVisitor::default())
    }
}

struct GeometryVisitor<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for GeometryVisitor<T> {
    fn default() -> Self {
        GeometryVisitor {
            _marker: PhantomData::default(),
        }
    }
}

impl<'de, T> Visitor<'de> for GeometryVisitor<T>
where
    T: FromStr + Default + WktFloat,
{
    type Value = Geometry<T>;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a valid WKT format")
    }
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let wkt = Wkt::from_str(s).map_err(|e| serde::de::Error::custom(e))?;
        Ok(wkt.item)
    }
}

impl<'de, T> serde::Deserialize<'de> for Geometry<T>
where
    T: FromStr + Default + WktFloat,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(GeometryVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        types::{Coord, Point},
        Geometry,
    };
    use serde::de::{
        value::{Error, StrDeserializer},
        Deserializer, Error as _, IntoDeserializer,
    };

    mod wkt {
        use super::*;

        #[test]
        fn deserialize() {
            let deserializer: StrDeserializer<'_, Error> = "POINT (10 20.1)".into_deserializer();
            let wkt = deserializer
                .deserialize_any(WktVisitor::<f64>::default())
                .unwrap();
            assert!(matches!(
                wkt.item,
                Geometry::Point(Point(Some(Coord {
                    x: _, // floating-point types cannot be used in patterns
                    y: _, // floating-point types cannot be used in patterns
                    z: None,
                    m: None,
                })))
            ));
        }

        #[test]
        fn deserialize_error() {
            let deserializer: StrDeserializer<'_, Error> = "POINT (10 20.1A)".into_deserializer();
            let wkt = deserializer.deserialize_any(WktVisitor::<f64>::default());
            assert_eq!(
                wkt.unwrap_err(),
                Error::custom("Expected a number for the Y coordinate")
            );
        }
    }

    mod geometry {
        use super::*;

        #[test]
        fn deserialize() {
            let deserializer: StrDeserializer<'_, Error> = "POINT (42 3.14)".into_deserializer();
            let geometry = deserializer
                .deserialize_any(GeometryVisitor::<f64>::default())
                .unwrap();
            assert!(matches!(
                geometry,
                Geometry::Point(Point(Some(Coord {
                    x: _, // floating-point types cannot be used in patterns
                    y: _, // floating-point types cannot be used in patterns
                    z: None,
                    m: None,
                })))
            ));
        }

        #[test]
        fn deserialize_error() {
            let deserializer: StrDeserializer<'_, Error> = "POINT (42 PI3.14)".into_deserializer();
            let geometry = deserializer.deserialize_any(GeometryVisitor::<f64>::default());
            assert_eq!(
                geometry.unwrap_err(),
                Error::custom("Expected a number for the Y coordinate")
            );
        }
    }
}
