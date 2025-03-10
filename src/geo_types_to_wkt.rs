use geo_types::CoordNum;

use crate::types::{
    Coord, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon,
};
use crate::{ToWkt, Wkt};

/// # Examples
/// ```
/// use geo_types::{point, Geometry};
/// use wkt::ToWkt;
///
/// let geometry: Geometry<f64> = Geometry::Point(point!(x: 1., y: 2., z: 3.));
///
/// assert_eq!(geometry.wkt_string(), "POINT Z(1 2 3)");
/// ```
impl<T> ToWkt<T> for geo_types::Geometry<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        match self {
            geo_types::Geometry::Point(g) => g.to_wkt(),
            geo_types::Geometry::Line(g) => g.to_wkt(),
            geo_types::Geometry::LineString(g) => g.to_wkt(),
            geo_types::Geometry::Polygon(g) => g.to_wkt(),
            geo_types::Geometry::MultiPoint(g) => g.to_wkt(),
            geo_types::Geometry::MultiLineString(g) => g.to_wkt(),
            geo_types::Geometry::MultiPolygon(g) => g.to_wkt(),
            geo_types::Geometry::GeometryCollection(g) => g.to_wkt(),
            geo_types::Geometry::Rect(g) => g.to_wkt(),
            geo_types::Geometry::Triangle(g) => g.to_wkt(),
        }
    }
}

/// # Examples
/// ```
/// use geo_types::{point, Point};
/// use wkt::ToWkt;
///
/// let point: Point<f64> = point!(x: 1., y: 2., z: 3.);
///
/// assert_eq!(point.wkt_string(), "POINT Z(1 2 3)");
/// ```
impl<T> ToWkt<T> for geo_types::Point<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        Wkt::Point(g_point_to_w_point(self))
    }
}

/// # Examples
/// ```
/// use geo_types::{coord, Line};
/// use wkt::ToWkt;
///
/// let line = Line::<f64>::new(coord!(x: 1., y: 2., z: 3.), coord!(x: 4., y: 5., z: 6.));
///
/// assert_eq!(line.wkt_string(), "LINESTRING Z(1 2 3,4 5 6)");
/// ```
impl<T> ToWkt<T> for geo_types::Line<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_line_to_w_linestring(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{line_string, LineString};
/// use wkt::ToWkt;
///
/// let line_string: LineString<f64> = line_string![(x: 1., y: 2., z: 3.), (x: 3., y: 4., z: 4.), (x: 5., y: 6., z: 5.)];
///
/// assert_eq!(line_string.wkt_string(), "LINESTRING Z(1 2 3,3 4 4,5 6 5)");
/// ```
impl<T> ToWkt<T> for geo_types::LineString<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_linestring_to_w_linestring(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{polygon, Polygon};
/// use wkt::ToWkt;
///
/// let polygon: Polygon<f64> = polygon![(x: 0., y: 0., z: 0.), (x: 4., y: 0., z: -4.), (x: 2., y: 4., z: -2.), (x: 0., y: 0., z: 0.)];
///
/// assert_eq!(polygon.wkt_string(), "POLYGON Z((0 0 0,4 0 -4,2 4 -2,0 0 0))");
/// ```
impl<T> ToWkt<T> for geo_types::Polygon<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_polygon_to_w_polygon(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{point, MultiPoint};
/// use wkt::ToWkt;
///
/// let multi_point: MultiPoint<f64> = MultiPoint::new(vec![point!(x: 0., y: 0., z: 0.), point!(x: 4., y: 0., z: -4.), point!(x: 2., y: 4., z: -2.)]);
///
/// assert_eq!(multi_point.wkt_string(), "MULTIPOINT Z((0 0 0),(4 0 -4),(2 4 -2))");
/// ```
impl<T> ToWkt<T> for geo_types::MultiPoint<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_mpoint_to_w_mpoint(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{line_string, LineString, MultiLineString};
/// use wkt::ToWkt;
///
/// let line_string_1: LineString<f64> = line_string![(x: 1., y: 2., z: 3.), (x: 4., y: 5., z: 6.), (x: 7., y: 8., z: 9.)];
/// let line_string_2: LineString<f64> = line_string![(x: 7., y: 8., z: 9.), (x: 10., y: 11., z: 12.)];
/// let multi_line_string: MultiLineString<f64> = MultiLineString::new(vec![line_string_1, line_string_2]);
///
/// assert_eq!(multi_line_string.wkt_string(), "MULTILINESTRING Z((1 2 3,4 5 6,7 8 9),(7 8 9,10 11 12))");
/// ```
impl<T> ToWkt<T> for geo_types::MultiLineString<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_mline_to_w_mline(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{polygon, Polygon, MultiPolygon};
/// use wkt::ToWkt;
///
/// // triangle
/// let polygon_1: Polygon<f64> = polygon![(x: 0., y: 0., z: 0.), (x: 4., y: 0., z: -4.), (x: 2., y: 4., z: -2.), (x: 0., y: 0., z: 0.)];
/// // square
/// let polygon_2: Polygon<f64> = polygon![(x: 4., y: 4., z: 4.), (x: 8., y: 4., z: -8.), (x: 8., y: 8., z: 8.), (x: 4., y: 8., z: -4.), (x: 4., y: 4., z: 4.)];
/// let multi_polygon: MultiPolygon<f64> = MultiPolygon::new(vec![polygon_1, polygon_2]);
///
/// assert_eq!(multi_polygon.wkt_string(), "MULTIPOLYGON Z(((0 0 0,4 0 -4,2 4 -2,0 0 0)),((4 4 4,8 4 -8,8 8 8,4 8 -4,4 4 4)))");
/// ```
impl<T> ToWkt<T> for geo_types::MultiPolygon<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_mpolygon_to_w_mpolygon(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{line_string, LineString, polygon, Polygon, GeometryCollection};
/// use wkt::ToWkt;
///
/// let polygon: Polygon<f64> = polygon![(x: 0., y: 0., z: 0.), (x: 4., y: 0., z: -4.), (x: 2., y: 4., z: 6.), (x: 0., y: 0., z: 0.)];
/// let line_string: LineString<f64> = line_string![(x: 1., y: 2., z: 3.), (x: 4., y: 5., z: 6.), (x: 7., y: 8., z: 9.)];
/// let geometry_collection: GeometryCollection<f64> = GeometryCollection::new(vec![polygon.into(), line_string.into()]);
///
/// assert_eq!(geometry_collection.wkt_string(), "GEOMETRYCOLLECTION Z(POLYGON Z((0 0 0,4 0 -4,2 4 6,0 0 0)),LINESTRING Z(1 2 3,4 5 6,7 8 9))");
/// ```
impl<T> ToWkt<T> for geo_types::GeometryCollection<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_geocol_to_w_geocol(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{coord, Rect};
/// use wkt::ToWkt;
///
/// let rect: Rect<f64> = Rect::new(coord!(x: 4., y: 4., z: 4.), coord!(x: 8., y: 8., z: 8.));
///
/// assert_eq!(rect.wkt_string(), "POLYGON Z((4 4 8,4 8 8,8 8 8,8 4 8,8 4 4,4 4 4,4 8 4,8 8 4,4 4 8))");
/// ```
impl<T> ToWkt<T> for geo_types::Rect<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_rect_to_w_polygon(self).into()
    }
}

/// # Examples
/// ```
/// use geo_types::{coord, Triangle};
/// use wkt::ToWkt;
///
/// let triangle: Triangle<f64> = Triangle::new(coord!(x: 0., y: 0., z: 0.), coord!(x: 4., y: 0., z: 4.), coord!(x: 2., y: 4., z: 2.));
///
/// assert_eq!(triangle.wkt_string(), "POLYGON Z((0 0 0,4 0 4,2 4 2,0 0 0))");
/// ```
impl<T> ToWkt<T> for geo_types::Triangle<T>
where
    T: CoordNum + std::fmt::Display + Default,
{
    fn to_wkt(&self) -> Wkt<T> {
        g_triangle_to_w_polygon(self).into()
    }
}

fn g_point_to_w_coord<T>(g_point: &geo_types::Coord<T>) -> Coord<T>
where
    T: CoordNum + Default,
{
    Coord {
        x: g_point.x,
        y: g_point.y,
        z: g_point.z,
    }
}

fn g_point_to_w_point<T>(g_point: &geo_types::Point<T>) -> Point<T>
where
    T: CoordNum + Default,
{
    let coord = g_point_to_w_coord(&g_point.0);
    Point(Some(coord))
}

fn g_points_to_w_coords<T>(g_points: &[geo_types::Coord<T>]) -> Vec<Coord<T>>
where
    T: CoordNum + Default,
{
    g_points.iter().map(g_point_to_w_coord).collect()
}

fn g_points_to_w_points<T>(g_points: &[geo_types::Point<T>]) -> Vec<Point<T>>
where
    T: CoordNum + Default,
{
    g_points
        .iter()
        .map(|p| &p.0)
        .map(g_point_to_w_coord)
        .map(|c| Point(Some(c)))
        .collect()
}

fn g_line_to_w_linestring<T>(g_line: &geo_types::Line<T>) -> LineString<T>
where
    T: CoordNum + Default,
{
    g_points_to_w_linestring(&[g_line.start, g_line.end])
}

fn g_linestring_to_w_linestring<T>(g_linestring: &geo_types::LineString<T>) -> LineString<T>
where
    T: CoordNum + Default,
{
    let geo_types::LineString(g_points) = g_linestring;
    g_points_to_w_linestring(g_points)
}

fn g_points_to_w_linestring<T>(g_coords: &[geo_types::Coord<T>]) -> LineString<T>
where
    T: CoordNum + Default,
{
    let w_coords = g_points_to_w_coords(g_coords);
    LineString(w_coords)
}

fn g_lines_to_w_lines<T>(g_lines: &[geo_types::LineString<T>]) -> Vec<LineString<T>>
where
    T: CoordNum + Default,
{
    let mut w_lines = vec![];
    for g_line in g_lines {
        let geo_types::LineString(g_points) = g_line;
        w_lines.push(g_points_to_w_linestring(g_points));
    }
    w_lines
}

fn g_triangle_to_w_polygon<T>(g_triangle: &geo_types::Triangle<T>) -> Polygon<T>
where
    T: CoordNum + Default,
{
    let polygon = g_triangle.to_polygon();
    g_polygon_to_w_polygon(&polygon)
}

fn g_rect_to_w_polygon<T>(g_rect: &geo_types::Rect<T>) -> Polygon<T>
where
    T: CoordNum + Default,
{
    let polygon = g_rect.to_polygon();
    g_polygon_to_w_polygon(&polygon)
}

fn g_polygon_to_w_polygon<T>(g_polygon: &geo_types::Polygon<T>) -> Polygon<T>
where
    T: CoordNum + Default,
{
    let outer_line = g_polygon.exterior();
    let inner_lines = g_polygon.interiors();
    let mut poly_lines = vec![];

    // Outer
    let geo_types::LineString(outer_points) = outer_line;
    if !outer_points.is_empty() {
        poly_lines.push(g_points_to_w_linestring(outer_points));
    }

    // Inner
    let inner = g_lines_to_w_lines(inner_lines);
    poly_lines.extend(inner);

    Polygon(poly_lines)
}

fn g_mpoint_to_w_mpoint<T>(g_mpoint: &geo_types::MultiPoint<T>) -> MultiPoint<T>
where
    T: CoordNum + Default,
{
    let geo_types::MultiPoint(g_points) = g_mpoint;
    let w_points = g_points_to_w_points(g_points);
    MultiPoint(w_points)
}

fn g_mline_to_w_mline<T>(g_mline: &geo_types::MultiLineString<T>) -> MultiLineString<T>
where
    T: CoordNum + Default,
{
    let geo_types::MultiLineString(g_lines) = g_mline;
    let w_lines = g_lines_to_w_lines(g_lines);
    MultiLineString(w_lines)
}

fn g_polygons_to_w_polygons<T>(g_polygons: &[geo_types::Polygon<T>]) -> Vec<Polygon<T>>
where
    T: CoordNum + Default,
{
    let mut w_polygons = vec![];
    for g_polygon in g_polygons {
        w_polygons.push(g_polygon_to_w_polygon(g_polygon));
    }
    w_polygons
}

fn g_mpolygon_to_w_mpolygon<T>(g_mpolygon: &geo_types::MultiPolygon<T>) -> MultiPolygon<T>
where
    T: CoordNum + Default,
{
    let geo_types::MultiPolygon(g_polygons) = g_mpolygon;
    let w_polygons = g_polygons_to_w_polygons(g_polygons);
    MultiPolygon(w_polygons)
}

fn g_geocol_to_w_geocol<T>(g_geocol: &geo_types::GeometryCollection<T>) -> GeometryCollection<T>
where
    T: CoordNum + Default,
{
    let geo_types::GeometryCollection(g_geoms) = g_geocol;
    let mut w_geoms = vec![];
    for g_geom in g_geoms {
        let w_geom = g_geom_to_w_geom(g_geom);
        w_geoms.push(w_geom);
    }
    GeometryCollection(w_geoms)
}

fn g_geom_to_w_geom<T: CoordNum + Default>(g_geom: &geo_types::Geometry<T>) -> Wkt<T> {
    match *g_geom {
        geo_types::Geometry::Point(ref g_point) => g_point_to_w_point(g_point).into(),

        geo_types::Geometry::Line(ref g_line) => g_line_to_w_linestring(g_line).into(),

        geo_types::Geometry::LineString(ref g_line) => g_linestring_to_w_linestring(g_line).into(),

        geo_types::Geometry::Triangle(ref g_triangle) => g_triangle_to_w_polygon(g_triangle).into(),

        geo_types::Geometry::Rect(ref g_rect) => g_rect_to_w_polygon(g_rect).into(),

        geo_types::Geometry::Polygon(ref g_polygon) => g_polygon_to_w_polygon(g_polygon).into(),

        geo_types::Geometry::MultiPoint(ref g_mpoint) => g_mpoint_to_w_mpoint(g_mpoint).into(),

        geo_types::Geometry::MultiLineString(ref g_mline) => g_mline_to_w_mline(g_mline).into(),

        geo_types::Geometry::MultiPolygon(ref g_mpolygon) => {
            g_mpolygon_to_w_mpolygon(g_mpolygon).into()
        }

        geo_types::Geometry::GeometryCollection(ref g_geocol) => {
            g_geocol_to_w_geocol(g_geocol).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ToWkt;

    #[test]
    fn float_geom() {
        let point = geo_types::Point::new(1f32, 2f32, 3f32);
        assert_eq!("POINT Z(1 2 3)", &point.wkt_string());

        let point = geo_types::Point::new(1.1, 2.9, 3.8);
        assert_eq!("POINT Z(1.1 2.9 3.8)", &point.wkt_string());
    }
}
