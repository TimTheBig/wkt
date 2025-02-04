// Copyright 2014-2015 The GeoRust Developers
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

use geo_traits::CoordTrait;

use crate::tokenizer::{PeekableTokens, Token};
use crate::types::Dimension;
use crate::{FromTokens, WktNum};
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Coord<T>
where
    T: WktNum,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> FromTokens<T> for Coord<T>
where
    T: WktNum + FromStr + Default,
{
    fn from_tokens(tokens: &mut PeekableTokens<T>, dim: Dimension) -> Result<Self, &'static str> {
        let x = match tokens.next().transpose()? {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the X coordinate"),
        };
        let y = match tokens.next().transpose()? {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the Y coordinate"),
        };
        let z = match tokens.next().transpose()? {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the Z coordinate"),
        };

        match dim {
            Dimension::XY => { return Err("x, y, and z fields are expected") },
            Dimension::XYZ => (),
            _ => { return Err("x, y, and z fields are expected") }
        }

        Ok(Coord { x, y, z })
    }
}

impl<T: WktNum> CoordTrait for Coord<T> {
    type T = T;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyz
    }

    fn x(&self) -> Self::T {
        self.x
    }

    fn y(&self) -> Self::T {
        self.y
    }

    fn z(&self) -> Self::T {
        self.z
    }

    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("n out of range"),
        }
    }
}

impl<T: WktNum> CoordTrait for &Coord<T> {
    type T = T;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyz
    }

    fn x(&self) -> Self::T {
        self.x
    }

    fn y(&self) -> Self::T {
        self.y
    }

    fn z(&self) -> Self::T {
        self.z
    }

    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("n out of range"),
        }
    }
}
