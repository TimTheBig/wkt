use std::fmt;
use std::fmt::Write;

use geo_traits::{
    CoordTrait, GeometryCollectionTrait, GeometryTrait, LineStringTrait, LineTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
    TriangleTrait,
};

use crate::error::Error;
use crate::types::Coord;
use crate::WktNum;

/// The physical size of the coordinate dimension
///
/// This is used so that we don't have to call `.dim()` on **every** coordinate. We infer it once
/// from the `geo_traits::Dimensions` and then pass it to each coordinate.
#[derive(Clone, Copy)]
enum PhysicalCoordinateDimension {
    Two,
    Three,
    Four,
}

impl From<geo_traits::Dimensions> for PhysicalCoordinateDimension {
    fn from(value: geo_traits::Dimensions) -> Self {
        match value.size() {
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            size => panic!("Unexpected dimension for coordinate: {}", size),
        }
    }
}

pub fn write_point<T: WktNum + fmt::Display, G: PointTrait<T = T>>(
    f: &mut impl Write,
    g: &G,
) -> Result<(), Error> {
    let dim = g.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => f.write_str("POINT"),
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => f.write_str("POINT Z"),
        geo_traits::Dimensions::Xym => f.write_str("POINT M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("POINT ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);
    if let Some(coord) = g.coord() {
        f.write_char('(')?;
        write_coord(f, &coord, size)?;
        f.write_char(')')?;
        Ok(())
    } else {
        Ok(f.write_str(" EMPTY")?)
    }
}

pub fn write_linestring<T: WktNum + fmt::Display, G: LineStringTrait<T = T>>(
    f: &mut impl Write,
    linestring: &G,
) -> Result<(), Error> {
    let dim = linestring.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => {
            f.write_str("LINESTRING")
        }
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("LINESTRING Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("LINESTRING M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("LINESTRING ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);
    if linestring.num_coords() == 0 {
        Ok(f.write_str(" EMPTY")?)
    } else {
        write_coord_sequence(f, linestring.coords(), size)
    }
}

pub fn write_polygon<T: WktNum + fmt::Display, G: PolygonTrait<T = T>>(
    f: &mut impl Write,
    polygon: &G,
) -> Result<(), Error> {
    let dim = polygon.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => f.write_str("POLYGON"),
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("POLYGON Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("POLYGON M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("POLYGON ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);
    if let Some(exterior) = polygon.exterior() {
        if exterior.num_coords() != 0 {
            f.write_str("(")?;
            write_coord_sequence(f, exterior.coords(), size)?;

            for interior in polygon.interiors() {
                f.write_char(',')?;
                write_coord_sequence(f, interior.coords(), size)?;
            }

            Ok(f.write_char(')')?)
        } else {
            Ok(f.write_str(" EMPTY")?)
        }
    } else {
        Ok(f.write_str(" EMPTY")?)
    }
}

pub fn write_multi_point<T: WktNum + fmt::Display, G: MultiPointTrait<T = T>>(
    f: &mut impl Write,
    multipoint: &G,
) -> Result<(), Error> {
    let dim = multipoint.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => {
            f.write_str("MULTIPOINT")
        }
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("MULTIPOINT Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("MULTIPOINT M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("MULTIPOINT ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);

    let mut points = multipoint.points();

    // Note: This is largely copied from `write_coord_sequence`, because `multipoint.points()`
    // yields a sequence of Point, not Coord.
    if let Some(first_point) = points.next() {
        f.write_str("((")?;

        // Assume no empty points within this MultiPoint
        write_coord(f, &first_point.coord().unwrap(), size)?;

        for point in points {
            f.write_str("),(")?;
            write_coord(f, &point.coord().unwrap(), size)?;
        }

        f.write_str("))")?;
    } else {
        f.write_str(" EMPTY")?;
    }

    Ok(())
}

pub fn write_multi_linestring<T: WktNum + fmt::Display, G: MultiLineStringTrait<T = T>>(
    f: &mut impl Write,
    multilinestring: &G,
) -> Result<(), Error> {
    let dim = multilinestring.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => {
            f.write_str("MULTILINESTRING")
        }
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("MULTILINESTRING Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("MULTILINESTRING M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("MULTILINESTRING ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);
    let mut line_strings = multilinestring.line_strings();
    if let Some(first_linestring) = line_strings.next() {
        f.write_str("(")?;
        write_coord_sequence(f, first_linestring.coords(), size)?;

        for linestring in line_strings {
            f.write_char(',')?;
            write_coord_sequence(f, linestring.coords(), size)?;
        }

        f.write_char(')')?;
    } else {
        f.write_str(" EMPTY")?;
    };

    Ok(())
}

pub fn write_multi_polygon<T: WktNum + fmt::Display, G: MultiPolygonTrait<T = T>>(
    f: &mut impl Write,
    multipolygon: &G,
) -> Result<(), Error> {
    let dim = multipolygon.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => {
            f.write_str("MULTIPOLYGON")
        }
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("MULTIPOLYGON Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("MULTIPOLYGON M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("MULTIPOLYGON ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);

    let mut polygons = multipolygon.polygons();

    if let Some(first_polygon) = polygons.next() {
        f.write_str("((")?;

        write_coord_sequence(f, first_polygon.exterior().unwrap().coords(), size)?;
        for interior in first_polygon.interiors() {
            f.write_char(',')?;
            write_coord_sequence(f, interior.coords(), size)?;
        }

        for polygon in polygons {
            f.write_str("),(")?;

            write_coord_sequence(f, polygon.exterior().unwrap().coords(), size)?;
            for interior in polygon.interiors() {
                f.write_char(',')?;
                write_coord_sequence(f, interior.coords(), size)?;
            }
        }

        f.write_str("))")?;
    } else {
        f.write_str(" EMPTY")?;
    };

    Ok(())
}

/// Create geometry to WKT representation.

pub fn write_geometry<T: WktNum + fmt::Display, G: GeometryTrait<T = T>>(
    f: &mut impl Write,
    geometry: &G,
) -> Result<(), Error> {
    match geometry.as_type() {
        geo_traits::GeometryType::Point(point) => write_point(f, point),
        geo_traits::GeometryType::LineString(linestring) => write_linestring(f, linestring),
        geo_traits::GeometryType::Polygon(polygon) => write_polygon(f, polygon),
        geo_traits::GeometryType::MultiPoint(multi_point) => write_multi_point(f, multi_point),
        geo_traits::GeometryType::MultiLineString(mls) => write_multi_linestring(f, mls),
        geo_traits::GeometryType::MultiPolygon(multi_polygon) => {
            write_multi_polygon(f, multi_polygon)
        }
        geo_traits::GeometryType::GeometryCollection(gc) => write_geometry_collection(f, gc),
        geo_traits::GeometryType::Rect(rect) => write_rect(f, rect),
        geo_traits::GeometryType::Triangle(triangle) => write_triangle(f, triangle),
        geo_traits::GeometryType::Line(line) => write_line(f, line),
    }
}

pub fn write_geometry_collection<T: WktNum + fmt::Display, G: GeometryCollectionTrait<T = T>>(
    f: &mut impl Write,
    gc: &G,
) -> Result<(), Error> {
    let dim = gc.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => {
            f.write_str("GEOMETRYCOLLECTION")
        }
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("GEOMETRYCOLLECTION Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("GEOMETRYCOLLECTION M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("GEOMETRYCOLLECTION ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let mut geometries = gc.geometries();

    if let Some(first_geometry) = geometries.next() {
        f.write_str("(")?;

        write_geometry(f, &first_geometry)?;
        for geom in geometries {
            f.write_char(',')?;
            write_geometry(f, &geom)?;
        }

        f.write_char(')')?;
    } else {
        f.write_str(" EMPTY")?;
    }
    Ok(())
}

pub fn write_rect<T: WktNum + fmt::Display, G: RectTrait<T = T>>(
    f: &mut impl Write,
    rect: &G,
) -> Result<(), Error> {
    // Write prefix and error if not 2D
    match rect.dim() {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => f.write_str("POLYGON"),
        _ => return Err(Error::RectUnsupportedDimension),
    }?;

    let min_coord = rect.min();
    let max_coord = rect.max();

    // We need to construct the five points of the rect that make up the exterior Polygon
    let coords = [
        Coord {
            x: min_coord.x(),
            y: min_coord.y(),
            z: None,
            m: None,
        },
        Coord {
            x: min_coord.x(),
            y: max_coord.y(),
            z: None,
            m: None,
        },
        Coord {
            x: max_coord.x(),
            y: max_coord.y(),
            z: None,
            m: None,
        },
        Coord {
            x: max_coord.x(),
            y: min_coord.y(),
            z: None,
            m: None,
        },
        Coord {
            x: min_coord.x(),
            y: min_coord.y(),
            z: None,
            m: None,
        },
    ];

    f.write_str("(")?;
    write_coord_sequence(f, coords.iter(), PhysicalCoordinateDimension::Two)?;
    Ok(f.write_char(')')?)
}

pub fn write_triangle<T: WktNum + fmt::Display, G: TriangleTrait<T = T>>(
    f: &mut impl Write,
    triangle: &G,
) -> Result<(), Error> {
    let dim = triangle.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => f.write_str("POLYGON"),
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("POLYGON Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("POLYGON M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("POLYGON ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);
    f.write_str("(")?;

    let coords_iter = triangle
        .coords()
        .into_iter()
        .chain(std::iter::once(triangle.first()));
    write_coord_sequence(f, coords_iter, size)?;

    Ok(f.write_char(')')?)
}

pub fn write_line<T: WktNum + fmt::Display, G: LineTrait<T = T>>(
    f: &mut impl Write,
    line: &G,
) -> Result<(), Error> {
    let dim = line.dim();
    // Write prefix
    match dim {
        geo_traits::Dimensions::Xy | geo_traits::Dimensions::Unknown(2) => {
            f.write_str("LINESTRING")
        }
        geo_traits::Dimensions::Xyz | geo_traits::Dimensions::Unknown(3) => {
            f.write_str("LINESTRING Z")
        }
        geo_traits::Dimensions::Xym => f.write_str("LINESTRING M"),
        geo_traits::Dimensions::Xyzm | geo_traits::Dimensions::Unknown(4) => {
            f.write_str("LINESTRING ZM")
        }
        geo_traits::Dimensions::Unknown(_) => return Err(Error::UnknownDimension),
    }?;
    let size = PhysicalCoordinateDimension::from(dim);
    write_coord_sequence(f, line.coords().into_iter(), size)
}

/// Write a single coordinate to the writer.
///
/// Will not include any start or end `()` characters.
fn write_coord<T: WktNum + fmt::Display, G: CoordTrait<T = T>>(
    f: &mut impl Write,
    coord: &G,
    size: PhysicalCoordinateDimension,
) -> Result<(), std::fmt::Error> {
    match size {
        PhysicalCoordinateDimension::Two => write!(f, "{} {}", coord.x(), coord.y()),
        PhysicalCoordinateDimension::Three => {
            // Safety:
            // We've validated that there are three dimensions
            write!(f, "{} {} {}", coord.x(), coord.y(), unsafe {
                coord.nth_unchecked(2)
            })
        }
        PhysicalCoordinateDimension::Four => {
            write!(
                f,
                "{} {} {} {}",
                coord.x(),
                coord.y(),
                // Safety:
                // We've validated that there are four dimensions
                unsafe { coord.nth_unchecked(2) },
                // Safety:
                // We've validated that there are four dimensions
                unsafe { coord.nth_unchecked(3) }
            )
        }
    }
}

/// Includes the `()` characters to start and end this sequence.
///
/// E.g. it will write:
/// ```notest
/// (1 2, 3 4, 5 6)
/// ```
/// for a coordinate sequence with three coordinates.
fn write_coord_sequence<T: WktNum + fmt::Display, C: CoordTrait<T = T>>(
    f: &mut impl Write,
    mut coords: impl Iterator<Item = C>,
    size: PhysicalCoordinateDimension,
) -> Result<(), Error> {
    f.write_char('(')?;

    if let Some(first_coord) = coords.next() {
        write_coord(f, &first_coord, size)?;

        for coord in coords {
            f.write_char(',')?;
            write_coord(f, &coord, size)?;
        }
    }

    f.write_char(')')?;
    Ok(())
}
