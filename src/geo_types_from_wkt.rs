//! This module provides conversions between WKT primitives and [`geo_types`] primitives.
//!
//! See the [`std::convert::From`] and [`std::convert::TryFrom`] impls on individual [`crate::types`] and [`Wkt`] for details.
// Copyright 2014-2018 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::types::*;
use crate::{TryFromWkt, Wkt};

use std::any::type_name;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::str::FromStr;

use geo_types::{coord, CoordNum};
use thiserror::Error;

#[derive(Error, Debug)]
/// WKT to [`geo_types`] conversions errors
pub enum Error {
    #[error("The WKT Point was empty, but geo_type::Points cannot be empty")]
    PointConversionError,
    #[error("Mismatched geometry (expected {expected:?}, found {found:?})")]
    MismatchedGeometry {
        expected: &'static str,
        found: &'static str,
    },
    #[error("Wrong number of Geometries: {0}")]
    WrongNumberOfGeometries(usize),
    #[error("Invalid WKT: {0}")]
    InvalidWKT(&'static str),
    #[error("External error: {0}")]
    External(Box<dyn std::error::Error>),
}

macro_rules! try_from_wkt_impl {
    ($($type: ident),+) => {
        $(
            /// Fallibly convert this WKT primitive into this [`geo_types`] primitive
            impl<T: CoordNum + Default> TryFrom<Wkt<T>> for geo_types::$type<T> {
                type Error = Error;

                fn try_from(wkt: Wkt<T>) -> Result<Self, Self::Error> {
                    let geometry = geo_types::Geometry::try_from(wkt)?;
                    Self::try_from(geometry).map_err(|e| {
                        match e {
                            geo_types::Error::MismatchedGeometry { expected, found } => {
                                Error::MismatchedGeometry { expected, found }
                            }
                            // currently only one error type in geo-types error enum, but that seems likely to change
                            #[allow(unreachable_patterns)]
                            other => Error::External(Box::new(other)),
                        }
                    })
                }
            }
        )+
    }
}

try_from_wkt_impl!(
    Point,
    Line,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    // See impl below.
    // GeometryCollection,
    Rect,
    Triangle
);

/// Fallibly convert this WKT primitive into this [`geo_types`] primitive
impl<T: CoordNum + Default> TryFrom<Wkt<T>> for geo_types::GeometryCollection<T> {
    type Error = Error;

    fn try_from(wkt: Wkt<T>) -> Result<Self, Self::Error> {
        match wkt {
            Wkt::GeometryCollection(collection) => {
                let geometries: Result<Vec<geo_types::Geometry<T>>, _> =
                    collection.0.into_iter().map(TryFrom::try_from).collect();
                Ok(geo_types::GeometryCollection(geometries?))
            }
            // geo_types doesn't implement `Geometry::try_from(geom_collec)` yet
            // (see https://github.com/georust/geo/pull/821).
            // So instead we synthesize the type of error it *would* return.
            Wkt::Point(_) => Err(Error::MismatchedGeometry {
                expected: type_name::<Self>(),
                found: type_name::<geo_types::Point<T>>(),
            }),
            Wkt::LineString(_) => Err(Error::MismatchedGeometry {
                expected: type_name::<Self>(),
                found: type_name::<geo_types::LineString<T>>(),
            }),
            Wkt::Polygon(_) => Err(Error::MismatchedGeometry {
                expected: type_name::<Self>(),
                found: type_name::<geo_types::Polygon<T>>(),
            }),
            Wkt::MultiPoint(_) => Err(Error::MismatchedGeometry {
                expected: type_name::<Self>(),
                found: type_name::<geo_types::MultiPoint<T>>(),
            }),
            Wkt::MultiLineString(_) => Err(Error::MismatchedGeometry {
                expected: type_name::<Self>(),
                found: type_name::<geo_types::MultiLineString<T>>(),
            }),
            Wkt::MultiPolygon(_) => Err(Error::MismatchedGeometry {
                expected: type_name::<Self>(),
                found: type_name::<geo_types::MultiPolygon<T>>(),
            }),
        }
    }
}

impl<T: CoordNum + Default> From<Coord<T>> for geo_types::Coord<T> {
    /// Convert from a WKT Coordinate to a [`geo_types::Coordinate`]
    fn from(coord: Coord<T>) -> geo_types::Coord<T> {
        coord! { x: coord.x, y: coord.y, z: coord.z }
    }
}

impl<T: CoordNum + Default> TryFrom<Point<T>> for geo_types::Point<T> {
    type Error = Error;

    /// Fallibly convert from a WKT `POINT` to a [`geo_types::Point`]
    fn try_from(point: Point<T>) -> Result<Self, Self::Error> {
        match point.0 {
            Some(coord) => Ok(Self::new(coord.x, coord.y, coord.z)),
            None => Err(Error::PointConversionError),
        }
    }
}

impl<'a, T: CoordNum + Default> From<&'a LineString<T>> for geo_types::Geometry<T> {
    fn from(line_string: &'a LineString<T>) -> Self {
        Self::LineString(line_string.clone().into())
    }
}

impl<T: CoordNum + Default> From<LineString<T>> for geo_types::LineString<T> {
    /// Convert from a WKT `LINESTRING` to a [`geo_types::LineString`]
    fn from(line_string: LineString<T>) -> Self {
        let coords = line_string
            .0
            .into_iter()
            .map(geo_types::Coord::from)
            .collect();

        geo_types::LineString(coords)
    }
}

impl<'a, T> From<&'a MultiLineString<T>> for geo_types::Geometry<T>
where
    T: CoordNum + Default,
{
    fn from(multi_line_string: &'a MultiLineString<T>) -> geo_types::Geometry<T> {
        Self::MultiLineString(multi_line_string.clone().into())
    }
}

impl<T> From<MultiLineString<T>> for geo_types::MultiLineString<T>
where
    T: CoordNum + Default,
{
    /// Convert from a WKT `MULTILINESTRING` to a [`geo_types::MultiLineString`]
    fn from(multi_line_string: MultiLineString<T>) -> geo_types::MultiLineString<T> {
        let geo_line_strings: Vec<geo_types::LineString<T>> = multi_line_string
            .0
            .into_iter()
            .map(geo_types::LineString::from)
            .collect();

        geo_types::MultiLineString(geo_line_strings)
    }
}

impl<'a, T> From<&'a Polygon<T>> for geo_types::Geometry<T>
where
    T: CoordNum + Default,
{
    fn from(polygon: &'a Polygon<T>) -> geo_types::Geometry<T> {
        Self::Polygon(polygon.clone().into())
    }
}

impl<T: CoordNum + Default> From<Polygon<T>> for geo_types::Polygon<T> {
    /// Convert from a WKT `POLYGON` to a [`geo_types::Polygon`]
    fn from(polygon: Polygon<T>) -> Self {
        let mut iter = polygon.0.into_iter().map(geo_types::LineString::from);
        match iter.next() {
            Some(interior) => geo_types::Polygon::new(interior, iter.collect()),
            None => geo_types::Polygon::new(geo_types::LineString(vec![]), vec![]),
        }
    }
}

impl<'a, T> TryFrom<&'a MultiPoint<T>> for geo_types::Geometry<T>
where
    T: CoordNum + Default,
{
    type Error = Error;

    fn try_from(multi_point: &'a MultiPoint<T>) -> Result<Self, Self::Error> {
        Ok(Self::MultiPoint(multi_point.clone().try_into()?))
    }
}

impl<T> TryFrom<MultiPoint<T>> for geo_types::MultiPoint<T>
where
    T: CoordNum + Default,
{
    type Error = Error;
    /// Fallibly convert from a WKT `MULTIPOINT` to a [`geo_types::MultiPoint`]
    fn try_from(multi_point: MultiPoint<T>) -> Result<Self, Self::Error> {
        let points: Vec<geo_types::Point<T>> = multi_point
            .0
            .into_iter()
            .map(geo_types::Point::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(geo_types::MultiPoint(points))
    }
}

impl<'a, T> From<&'a MultiPolygon<T>> for geo_types::Geometry<T>
where
    T: CoordNum + Default,
{
    fn from(multi_polygon: &'a MultiPolygon<T>) -> Self {
        Self::MultiPolygon(multi_polygon.clone().into())
    }
}

impl<T> From<MultiPolygon<T>> for geo_types::MultiPolygon<T>
where
    T: CoordNum + Default,
{
    /// Convert from a WKT `MULTIPOLYGON` to a [`geo_types::MultiPolygon`]
    fn from(multi_polygon: MultiPolygon<T>) -> Self {
        let geo_polygons: Vec<geo_types::Polygon<T>> = multi_polygon
            .0
            .into_iter()
            .map(geo_types::Polygon::from)
            .collect();

        geo_types::MultiPolygon(geo_polygons)
    }
}

impl<T> TryFrom<GeometryCollection<T>> for geo_types::GeometryCollection<T>
where
    T: CoordNum + Default,
{
    type Error = Error;

    fn try_from(geometry_collection: GeometryCollection<T>) -> Result<Self, Self::Error> {
        let geo_geometries = geometry_collection
            .0
            .into_iter()
            .map(Wkt::try_into)
            .collect::<Result<_, _>>()?;

        Ok(geo_types::GeometryCollection(geo_geometries))
    }
}

impl<T> TryFrom<Wkt<T>> for geo_types::Geometry<T>
where
    T: CoordNum + Default,
{
    type Error = Error;

    fn try_from(geometry: Wkt<T>) -> Result<Self, Self::Error> {
        Ok(match geometry {
            Wkt::Point(g) => {
                // Special case as `geo::Point` can't be empty
                if g.0.is_some() {
                    geo_types::Point::try_from(g)?.into()
                } else {
                    geo_types::MultiPoint(vec![]).into()
                }
            }
            Wkt::LineString(g) => geo_types::Geometry::LineString(g.into()),
            Wkt::Polygon(g) => geo_types::Geometry::Polygon(g.into()),
            Wkt::MultiLineString(g) => geo_types::Geometry::MultiLineString(g.into()),
            Wkt::MultiPoint(g) => geo_types::Geometry::MultiPoint(g.try_into()?),
            Wkt::MultiPolygon(g) => geo_types::Geometry::MultiPolygon(g.into()),
            Wkt::GeometryCollection(g) => geo_types::Geometry::GeometryCollection(g.try_into()?),
        })
    }
}

/// Macro for implementing `TryFromWkt` for all the geo-types.
/// Alternatively, we could try to have a kind of blanket implementation on `TryFrom<Wkt<T>>`,
/// but:
///   1. what would be the type of `TryFromWkt::Error`?
///   2. that would preclude ever having a specialized implementation for geo-types as they'd
///      be ambiguous/redundant.
macro_rules! try_from_wkt_impl {
   ($($type: ty),*$(,)?)  => {
       $(
            impl<T: CoordNum + FromStr + Default> TryFromWkt<T> for $type {
                type Error = Error;
                fn try_from_wkt_str(wkt_str: &str) -> Result<Self, Self::Error> {
                    let wkt = Wkt::from_str(wkt_str).map_err(|e| Error::InvalidWKT(e))?;
                    Self::try_from(wkt)
                }

                fn try_from_wkt_reader(mut wkt_reader: impl Read) -> Result<Self, Self::Error> {
                    let mut bytes = vec![];
                    wkt_reader.read_to_end(&mut bytes).map_err(|e| Error::External(Box::new(e)))?;
                    let wkt_str = String::from_utf8(bytes).map_err(|e| Error::External(Box::new(e)))?;
                    Self::try_from_wkt_str(&wkt_str)
                }
            }
       )*
   }
}

try_from_wkt_impl![
    geo_types::Geometry<T>,
    geo_types::Point<T>,
    geo_types::Line<T>,
    geo_types::LineString<T>,
    geo_types::Polygon<T>,
    geo_types::MultiPoint<T>,
    geo_types::MultiLineString<T>,
    geo_types::MultiPolygon<T>,
    geo_types::GeometryCollection<T>,
    geo_types::Triangle<T>,
    geo_types::Rect<T>,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_single_item_wkt() {
        let wkt = Wkt::from(Point(Some(Coord {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })));

        let converted = geo_types::Geometry::try_from(wkt).unwrap();
        let g_point: geo_types::Point<f64> = geo_types::Point::new(1.0, 2.0, 3.0);

        assert_eq!(converted, geo_types::Geometry::Point(g_point));
    }

    #[test]
    fn convert_empty_point() {
        let point = Point(None);
        let res: Result<geo_types::Point<f64>, Error> = point.try_into();
        assert!(res.is_err());
    }

    #[test]
    fn convert_point() {
        let point = Wkt::from(Point(Some(Coord {
            x: 10.,
            y: 20.,
            z: 30.,
        })));

        let g_point: geo_types::Point<f64> = (10., 20., 30.).into();
        assert_eq!(
            geo_types::Geometry::Point(g_point),
            point.try_into().unwrap()
        );
    }

    #[test]
    fn convert_empty_linestring() {
        let w_linestring = Wkt::from(LineString(vec![]));
        let g_linestring: geo_types::LineString<f64> = geo_types::LineString(vec![]);
        assert_eq!(
            geo_types::Geometry::LineString(g_linestring),
            w_linestring.try_into().unwrap()
        );
    }

    #[test]
    fn convert_linestring() {
        let w_linestring: Wkt<f64> = LineString(vec![
            Coord {
                x: 10.,
                y: 20.,
                z: 30.,
            },
            Coord {
                x: 40.,
                y: 50.,
                z: 60.,
            },
        ])
        .into();
        let g_linestring: geo_types::LineString<f64> = vec![(10., 20., 30.), (40., 50., 60.)].into();
        assert_eq!(
            geo_types::Geometry::LineString(g_linestring),
            w_linestring.try_into().unwrap()
        );
    }

    #[test]
    fn convert_empty_polygon() {
        let w_polygon: Wkt<f64> = Polygon(vec![]).into();
        let g_polygon: geo_types::Polygon<f64> =
            geo_types::Polygon::new(geo_types::LineString(vec![]), vec![]);
        assert_eq!(
            geo_types::Geometry::Polygon(g_polygon),
            w_polygon.try_into().unwrap()
        );
    }

    #[test]
    fn convert_polygon() {
        let w_polygon: Wkt<f64> = Polygon(vec![
            LineString(vec![
                Coord {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Coord {
                    x: 20.,
                    y: 40.,
                    z: 60.,
                },
                Coord {
                    x: 40.,
                    y: 0.,
                    z: -40.,
                },
                Coord {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
            ]),
            LineString(vec![
                Coord {
                    x: 5.,
                    y: 5.,
                    z: 5.,
                },
                Coord {
                    x: 20.,
                    y: 30.,
                    z: 40.,
                },
                Coord {
                    x: 30.,
                    y: 5.,
                    z: -30.,
                },
                Coord {
                    x: 5.,
                    y: 5.,
                    z: 5.,
                },
            ]),
        ])
        .into();
        let g_polygon: geo_types::Polygon<f64> = geo_types::Polygon::new(
            vec![(0., 0., 0.), (20., 40., 60.), (40., 0., -40.), (0., 0., 0.)].into(),
            vec![vec![(5., 5., 5.), (20., 30., 40.), (30., 5., -30.), (5., 5., 5.)].into()],
        );
        assert_eq!(
            geo_types::Geometry::Polygon(g_polygon),
            w_polygon.try_into().unwrap()
        );
    }

    #[test]
    fn convert_empty_multilinestring() {
        let w_multilinestring: Wkt<f64> = MultiLineString(vec![]).into();
        let g_multilinestring: geo_types::MultiLineString<f64> = geo_types::MultiLineString(vec![]);
        assert_eq!(
            geo_types::Geometry::MultiLineString(g_multilinestring),
            w_multilinestring.try_into().unwrap()
        );
    }

    #[test]
    fn convert_multilinestring() {
        let w_multilinestring: Wkt<f64> = MultiLineString(vec![
            LineString(vec![
                Coord {
                    x: 10.,
                    y: 20.,
                    z: 30.
                },
                Coord {
                    x: 40.,
                    y: 50.,
                    z: 60.,
                },
            ]),
            LineString(vec![
                Coord {
                    x: 70.,
                    y: 80.,
                    z: 90.,
                },
                Coord {
                    x: 100.,
                    y: 110.,
                    z: 120.,
                },
            ]),
        ])
        .into();
        let g_multilinestring: geo_types::MultiLineString<f64> = geo_types::MultiLineString(vec![
            vec![(10., 20., 30.), (40., 50., 60.)].into(),
            vec![(70., 80., 90.), (100., 110., 120.)].into(),
        ]);
        assert_eq!(
            geo_types::Geometry::MultiLineString(g_multilinestring),
            w_multilinestring.try_into().unwrap()
        );
    }

    #[test]
    fn convert_empty_multipoint() {
        let w_multipoint: Wkt<f64> = MultiPoint(vec![]).into();
        let g_multipoint: geo_types::MultiPoint<f64> = geo_types::MultiPoint(vec![]);
        assert_eq!(
            geo_types::Geometry::MultiPoint(g_multipoint),
            w_multipoint.try_into().unwrap()
        );
    }

    #[test]
    fn convert_multipoint() {
        let w_multipoint: Wkt<f64> = MultiPoint(vec![
            Point(Some(Coord {
                x: 10.,
                y: 20.,
                z: 25.,
            })),
            Point(Some(Coord {
                x: 30.,
                y: 40.,
                z: 45.,
            })),
        ])
        .into();
        let g_multipoint: geo_types::MultiPoint<f64> = vec![(10., 20., 25.), (30., 40., 45.)].into();
        assert_eq!(
            geo_types::Geometry::MultiPoint(g_multipoint),
            w_multipoint.try_into().unwrap()
        );
    }

    #[test]
    fn convert_empty_multipolygon() {
        let w_multipolygon: Wkt<f64> = MultiPolygon(vec![]).into();
        let g_multipolygon: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon(vec![]);
        assert_eq!(
            geo_types::Geometry::MultiPolygon(g_multipolygon),
            w_multipolygon.try_into().unwrap()
        );
    }

    #[test]
    fn convert_multipolygon() {
        let w_multipolygon: Wkt<f64> = MultiPolygon(vec![
            Polygon(vec![
                LineString(vec![
                    Coord {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    },
                    Coord {
                        x: 20.,
                        y: 40.,
                        z: -20.,
                    },
                    Coord {
                        x: 40.,
                        y: 0.,
                        z: -40.,
                    },
                    Coord {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    },
                ]),
                LineString(vec![
                    Coord {
                        x: 5.,
                        y: 5.,
                        z: 5.,
                    },
                    Coord {
                        x: 20.,
                        y: 30.,
                        z: -20.,
                    },
                    Coord {
                        x: 30.,
                        y: 5.,
                        z: -30.,
                    },
                    Coord {
                        x: 5.,
                        y: 5.,
                        z: 5.,
                    },
                ]),
            ]),
            Polygon(vec![LineString(vec![
                Coord {
                    x: 40.,
                    y: 40.,
                    z: 40.,
                },
                Coord {
                    x: 20.,
                    y: 45.,
                    z: -20.,
                },
                Coord {
                    x: 45.,
                    y: 30.,
                    z: -45.,
                },
                Coord {
                    x: 40.,
                    y: 40.,
                    z: 40.,
                },
            ])]),
        ])
        .into();

        let g_multipolygon: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon(vec![
            geo_types::Polygon::new(
                vec![(0., 0., 0.), (20., 40., -20.), (40., 0., -40.), (0., 0., 0.)].into(),
                vec![vec![(5., 5., 5.), (20., 30., -20.), (30., 5., -30.), (5., 5., 5.)].into()],
            ),
            geo_types::Polygon::new(
                vec![(40., 40., 40.), (20., 45., -20.), (45., 30., -45.), (40., 40., 40.)].into(),
                vec![],
            ),
        ]);
        assert_eq!(
            geo_types::Geometry::MultiPolygon(g_multipolygon),
            w_multipolygon.try_into().unwrap()
        );
    }

    #[test]
    fn convert_empty_geometrycollection() {
        let w_geometrycollection: Wkt<f64> = GeometryCollection(vec![]).into();
        let g_geometrycollection: geo_types::GeometryCollection<f64> =
            geo_types::GeometryCollection(vec![]);
        assert_eq!(
            geo_types::Geometry::GeometryCollection(g_geometrycollection),
            w_geometrycollection.try_into().unwrap()
        );
    }

    #[test]
    fn convert_geometrycollection() {
        let w_point = Point(Some(Coord {
            x: 10.,
            y: 20.,
            z: 30.,
        }))
        .into();

        let w_linestring = LineString(vec![
            Coord {
                x: 10.,
                y: 20.,
                z: 30.,
            },
            Coord {
                x: 40.,
                y: 50.,
                z: 60.,
            },
        ])
        .into();

        let w_polygon = Polygon(vec![LineString(vec![
            Coord {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Coord {
                x: 20.,
                y: 40.,
                z: 60.
            },
            Coord {
                x: 40.,
                y: 0.,
                z: -40.,
            },
            Coord {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        ])])
        .into();

        let w_multilinestring = MultiLineString(vec![
            LineString(vec![
                Coord {
                    x: 10.,
                    y: 20.,
                    z: 30.,
                },
                Coord {
                    x: 40.,
                    y: 50.,
                    z: 60.,
                },
            ]),
            LineString(vec![
                Coord {
                    x: 70.,
                    y: 80.,
                    z: 90.,
                },
                Coord {
                    x: 100.,
                    y: 110.,
                    z: 120.,
                },
            ]),
        ])
        .into();

        let w_multipoint = MultiPoint(vec![
            Point(Some(Coord {
                x: 10.,
                y: 20.,
                z: 30.,
            })),
            Point(Some(Coord {
                x: 40.,
                y: 50.,
                z: 60.,
            })),
        ])
        .into();

        let w_multipolygon = MultiPolygon(vec![
            Polygon(vec![LineString(vec![
                Coord {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Coord {
                    x: 20.,
                    y: 40.,
                    z: 60.,
                },
                Coord {
                    x: 40.,
                    y: 0.,
                    z: -40.,
                },
                Coord {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
            ])]),
            Polygon(vec![LineString(vec![
                Coord {
                    x: 40.,
                    y: 40.,
                    z: 40.,
                },
                Coord {
                    x: 20.,
                    y: 45.,
                    z: -20.,
                },
                Coord {
                    x: 45.,
                    y: 30.,
                    z: -45.,
                },
                Coord {
                    x: 40.,
                    y: 40.,
                    z: 40.,
                },
            ])]),
        ])
        .into();

        let w_geometrycollection: Wkt<f64> = GeometryCollection(vec![
            w_point,
            w_multipoint,
            w_linestring,
            w_multilinestring,
            w_polygon,
            w_multipolygon,
        ])
        .into();

        let g_point: geo_types::Point<f64> = (10., 20., 30.).into();
        let g_linestring: geo_types::LineString<f64> = vec![(10., 20., 30.), (40., 50., 60.)].into();
        let g_polygon: geo_types::Polygon<f64> = geo_types::Polygon::new(
            vec![(0., 0., 0.), (20., 40., 60.), (40., 0., -40.), (0., 0., 0.)].into(),
            vec![],
        );
        let g_multilinestring: geo_types::MultiLineString<f64> = geo_types::MultiLineString(vec![
            vec![(10., 20., 30.), (40., 50., 60.)].into(),
            vec![(70., 80., 90.), (100., 110., 120.)].into(),
        ]);
        let g_multipoint: geo_types::MultiPoint<f64> = vec![(10., 20., 30.), (40., 50., 60.)].into();
        let g_multipolygon: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon(vec![
            geo_types::Polygon::new(
                vec![(0., 0., 0.), (20., 40., 60.), (40., 0., -40.), (0., 0., 0.)].into(),
                vec![],
            ),
            geo_types::Polygon::new(
                vec![(40., 40., 40.), (20., 45., -20.), (45., 30., -45.), (40., 40., 40.)].into(),
                vec![],
            ),
        ]);

        let g_geometrycollection: geo_types::GeometryCollection<f64> =
            geo_types::GeometryCollection(vec![
                geo_types::Geometry::Point(g_point),
                geo_types::Geometry::MultiPoint(g_multipoint),
                geo_types::Geometry::LineString(g_linestring),
                geo_types::Geometry::MultiLineString(g_multilinestring),
                geo_types::Geometry::Polygon(g_polygon),
                geo_types::Geometry::MultiPolygon(g_multipolygon),
            ]);
        assert_eq!(
            geo_types::Geometry::GeometryCollection(g_geometrycollection),
            w_geometrycollection.try_into().unwrap()
        );
    }

    #[test]
    fn geom_collection_from_wkt_str() {
        // geometry collections have some special handling vs. other geometries, so we test them separately.
        let collection = geo_types::GeometryCollection::<f64>::try_from_wkt_str(
            "GeometryCollection Z(POINT Z(1 2 3))",
        )
        .unwrap();
        let point: geo_types::Point<_> = collection[0].clone().try_into().unwrap();
        assert_eq!(point.y(), 2.0);
    }

    #[test]
    fn geom_collection_from_invalid_wkt_str() {
        // geometry collections have some special handling vs. other geometries, so we test them separately.
        let err = geo_types::GeometryCollection::<f64>::try_from_wkt_str("GeomColl(POINT Z(1 2 3))")
            .unwrap_err();
        match err {
            Error::InvalidWKT(err_text) => assert_eq!(err_text, "Invalid type encountered"),
            e => panic!("Not the error we expected. Found: {}", e),
        }
    }

    #[test]
    fn geom_collection_from_other_wkt_str() {
        // geometry collections have some special handling vs. other geometries, so we test them separately.
        let not_a_collection = geo_types::GeometryCollection::<f64>::try_from_wkt_str("POINT Z(1 2 3)");
        let err = not_a_collection.unwrap_err();
        match err {
            Error::MismatchedGeometry {
                expected: "geo_types::geometry::geometry_collection::GeometryCollection",
                found: "geo_types::geometry::point::Point",
            } => {}
            e => panic!("Not the error we expected. Found: {}", e),
        }
    }

    #[test]
    fn from_invalid_wkt_str() {
        let a_point_too_many = geo_types::Point::<f64>::try_from_wkt_str("PINT Z(1 2 3)");
        let err = a_point_too_many.unwrap_err();
        match err {
            Error::InvalidWKT(err_text) => assert_eq!(err_text, "Invalid type encountered"),
            e => panic!("Not the error we expected. Found: {}", e),
        }
    }

    #[test]
    fn from_other_geom_wkt_str() {
        let not_actually_a_line_string =
            geo_types::LineString::<f64>::try_from_wkt_str("POINT Z(1 2 3)");
        let err = not_actually_a_line_string.unwrap_err();
        match err {
            Error::MismatchedGeometry {
                expected: "geo_types::geometry::line_string::LineString",
                found: "geo_types::geometry::point::Point",
            } => {}
            e => panic!("Not the error we expected. Found: {}", e),
        }
    }

    #[test]
    fn integer_geometry() {
        use crate::to_wkt::ToWkt;
        let point: geo_types::Point<f32> =
            geo_types::Point::try_from_wkt_str("POINT Z(1 2 3)").unwrap();
        assert_eq!(point, geo_types::Point::new(1.0, 2.0, 3.0));

        let wkt_string = point.wkt_string();
        assert_eq!("POINT Z(1 2 3)", &wkt_string);
    }
}
