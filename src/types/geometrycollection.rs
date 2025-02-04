// Copyright 2015 The GeoRust Developers
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

use geo_traits::{GeometryCollectionTrait, GeometryTrait};

use crate::to_wkt::write_geometry_collection;
use crate::tokenizer::{PeekableTokens, Token};
use crate::types::Dimension;
use crate::{FromTokens, Wkt, WktNum};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeometryCollection<T: WktNum>(pub Vec<Wkt<T>>);

impl<T> From<GeometryCollection<T>> for Wkt<T>
where
    T: WktNum,
{
    fn from(value: GeometryCollection<T>) -> Self {
        Wkt::GeometryCollection(value)
    }
}

impl<T> fmt::Display for GeometryCollection<T>
where
    T: WktNum + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(write_geometry_collection(f, self)?)
    }
}

impl<T> FromTokens<T> for GeometryCollection<T>
where
    T: WktNum + FromStr + Default,
{
    // Unsure if the dimension should be used in parsing GeometryCollection; is it
    // GEOMETRYCOLLECTION ( POINT Z (...) , POINT ZM (...))
    // or does a geometry collection have a known dimension?
    fn from_tokens(tokens: &mut PeekableTokens<T>, _dim: Dimension) -> Result<Self, &'static str> {
        let mut items = Vec::new();

        let word = match tokens.next().transpose()? {
            Some(Token::Word(w)) => w,
            _ => return Err("Expected a word in GEOMETRYCOLLECTION"),
        };

        let item = Wkt::from_word_and_tokens(&word, tokens)?;
        items.push(item);

        while let Some(&Ok(Token::Comma)) = tokens.peek() {
            tokens.next(); // throw away comma

            let word = match tokens.next().transpose()? {
                Some(Token::Word(w)) => w,
                _ => return Err("Expected a word in GEOMETRYCOLLECTION"),
            };

            let item = Wkt::from_word_and_tokens(&word, tokens)?;
            items.push(item);
        }

        Ok(GeometryCollection(items))
    }
}

impl<T: WktNum> GeometryCollectionTrait for GeometryCollection<T> {
    type T = T;
    type GeometryType<'a>
        = &'a Wkt<T>
    where
        Self: 'a;

    fn dim(&self) -> geo_traits::Dimensions {
        // TODO: infer dimension from empty WKT
        if self.0.is_empty() {
            geo_traits::Dimensions::Xy
        } else {
            self.0[0].dim()
        }
    }

    fn num_geometries(&self) -> usize {
        self.0.len()
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        self.0.get_unchecked(i)
    }
}

#[cfg(test)]
mod tests {
    use super::GeometryCollection;
    use crate::types::*;
    use crate::Wkt;
    use std::str::FromStr;

    #[test]
    fn basic_geometrycollection() {
        let wkt: Wkt<f64> = Wkt::from_str("GEOMETRYCOLLECTION Z(POINT Z(8 4 9)))")
            .ok()
            .unwrap();
        let items = match wkt {
            Wkt::GeometryCollection(GeometryCollection(items)) => items,
            _ => unreachable!(),
        };
        assert_eq!(1, items.len());
    }

    #[test]
    fn complex_geometrycollection() {
        let wkt: Wkt<f64> = Wkt::from_str("GEOMETRYCOLLECTION Z(POINT Z(8 4 -8),LINESTRING Z(4 6 9,7 10 2)))")
            .ok()
            .unwrap();
        let items = match wkt {
            Wkt::GeometryCollection(GeometryCollection(items)) => items,
            _ => unreachable!(),
        };
        assert_eq!(2, items.len());
    }

    #[test]
    fn write_empty_geometry_collection() {
        let geometry_collection: GeometryCollection<f64> = GeometryCollection(vec![]);

        assert_eq!(
            "GEOMETRYCOLLECTION EMPTY",
            format!("{}", geometry_collection)
        );
    }

    #[test]
    fn write_geometry_collection() {
        let point = Wkt::Point(Point(Some(Coord {
            x: 10.,
            y: 20.,
            z: 30.,
        })));

        let multipoint = Wkt::MultiPoint(MultiPoint(vec![
            Point(Some(Coord {
                x: 10.1,
                y: 20.2,
                z: 30.3,
            })),
            Point(Some(Coord {
                x: 30.3,
                y: 40.4,
                z: 50.5,
            })),
        ]));

        let linestring = Wkt::LineString(LineString(vec![
            Coord {
                x: 10.,
                y: 20.,
                z: 30.,
            },
            Coord {
                x: 30.,
                y: 40.,
                z: 50.,
            },
        ]));

        let polygon = Wkt::Polygon(Polygon(vec![LineString(vec![
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
        ])]));

        let multilinestring = Wkt::MultiLineString(MultiLineString(vec![
            LineString(vec![
                Coord {
                    x: 10.1,
                    y: 20.2,
                    z: 30.3,
                },
                Coord {
                    x: 30.3,
                    y: 40.4,
                    z: 50.5,
                },
            ]),
            LineString(vec![
                Coord {
                    x: 50.5,
                    y: 60.6,
                    z: 70.7,
                },
                Coord {
                    x: 70.7,
                    y: 80.8,
                    z: 90.9,
                },
            ]),
        ]));

        let multipolygon = Wkt::MultiPolygon(MultiPolygon(vec![
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
        ]));

        let geometrycollection = GeometryCollection(vec![
            point,
            multipoint,
            linestring,
            polygon,
            multilinestring,
            multipolygon,
        ]);

        assert_eq!(
            "GEOMETRYCOLLECTION Z(\
             POINT Z(10 20 30),\
             MULTIPOINT Z((10.1 20.2 30.3),(30.3 40.4 50.5)),\
             LINESTRING Z(10 20 30,30 40 50),\
             POLYGON Z((0 0 0,20 40 60,40 0 -40,0 0 0)),\
             MULTILINESTRING Z((10.1 20.2 30.3,30.3 40.4 50.5),(50.5 60.6 70.7,70.7 80.8 90.9)),\
             MULTIPOLYGON Z(((0 0 0,20 40 60,40 0 -40,0 0 0)),((40 40 40,20 45 -20,45 30 -45,40 40 40)))\
             )",
            format!("{}", geometrycollection)
        );
    }
}
