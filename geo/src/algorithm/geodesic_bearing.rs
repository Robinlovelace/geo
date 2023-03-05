use crate::{Point};
use geographiclib_rs::{Geodesic, InverseGeodesic};
use geo_types::CoordNum;

/// Returns the bearing to another Point in degrees on a geodesic.
///
/// This uses the geodesic methods given by [Karney (2013)].
///
/// [Karney (2013)]:  https://arxiv.org/pdf/1109.4448.pdf
pub trait GeodesicBearing<T: CoordNum> {
    /// Returns the bearing to another Point in degrees, where North is 0° and East is 90°.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// #
    /// use geo::GeodesicBearing;
    /// use geo::Point;
    ///
    /// let p_1 = Point::new(9.177789688110352, 48.776781529534965);
    /// let p_2 = Point::new(9.274410083250379, 48.84033282787534);
    /// let bearing = p_1.bearing(p_2);
    /// assert_relative_eq!(bearing, 45., epsilon = 0.1);
    /// ```
    fn bearing(&self, point: Point<T>) -> T;

    /// Returns the bearing and distance to another Point in a (bearing, distance) tuple.
    /// Bearing is reported in degrees, where North is 0° and East is 90°. Distance is reported in meters.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// #
    /// use geo::Bearing;
    /// use geo::Point;
    ///
    /// let p_1 = Point::new(9.177789688110352, 48.776781529534965);
    /// let p_2 = Point::new(9.274410083250379, 48.84033282787534);
    /// let bearing = p_1.bearing(p_2);
    /// assert_relative_eq!(bearing, 45., epsilon = 1.0e-6);
    /// ```
    fn bearing_distance(&self, point: Point<T>) -> (T, T);
}

impl GeodesicBearing<f64> for Point<f64> {
    fn bearing(&self, rhs: Point<f64>) -> f64 {
        let (azi1, _, _) = Geodesic::wgs84().inverse(self.y(), self.x(), rhs.y(), rhs.x());
        azi1
    }

    fn bearing_distance(&self, rhs: Point<f64>) -> (f64, f64) {
        let (distance, azi1, _, _) = Geodesic::wgs84().inverse(self.y(), self.x(), rhs.y(), rhs.x());
        (azi1, distance)
    }
}

#[cfg(test)]
mod test {
    use crate::point;
    use crate::GeodesicBearing;
    use crate::HaversineDestination;

    #[test]
    fn geodesic_bearing() {
        let p_1 = point!(x: 9.177789688110352f64, y: 48.776781529534965);
        let p_2 = p_1.haversine_destination(45., 10000.);
        let b_1 = p_1.bearing(p_2);
        assert_relative_eq!(b_1, 45., epsilon = 0.1);

        let p_3 = point!(x: 9., y: 47.);
        let p_4 = point!(x: 9., y: 48.);
        let b_2 = p_3.bearing(p_4);
        assert_relative_eq!(b_2, 0., epsilon = 0.1);
    }
}
