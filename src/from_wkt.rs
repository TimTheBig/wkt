/// Create geometries from WKT.
///
/// A default implementation exists for [geo-types](../geo-types), or you can implement this trait
/// for your own types.
pub trait TryFromWkt<T>: Sized {
    type Error;

    /// # Examples
    /// ```
    /// // This example requires the geo-types feature (on by default).
    /// use wkt::TryFromWkt;
    /// use geo_types::Point;
    /// let point: Point<f64> = Point::try_from_wkt_str("POINT Z(10 20 30)").unwrap();
    /// assert_eq!(point.y(), 20.0);
    /// ```
    fn try_from_wkt_str(wkt_str: &str) -> Result<Self, Self::Error>;

    /// # Examples
    /// ```
    /// // This example requires the geo-types feature (on by default).
    /// use wkt::TryFromWkt;
    /// use geo_types::Point;
    ///
    /// let fake_file = "POINT Z(10 20 30)".as_bytes().to_vec();
    /// let point: Point<f64> = Point::try_from_wkt_reader(&*fake_file).unwrap();
    /// assert_eq!(point.y(), 20.0);
    /// ```
    fn try_from_wkt_reader(wkt_reader: impl std::io::Read) -> Result<Self, Self::Error>;
}
