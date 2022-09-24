//! The point module supports more advanced operations for 2d points in a 2d coordinate system.

/// The point type refers to an 2d vector with f64.
/// We use the left-up as the origin of the coordinate system.
/// [changes from wgxh-cli]: Use of left-bottom origin is convenient.
pub type Point = crate::vec::vec2::Vec2<f64>;

impl Point {

  /// Transform the point to a rotated coordinate system.
  /// Parameter `rad` are in radians. The new coordinate system will be rotate from its origin to angle `rad`.
  fn rotate(&self, rad: f64) -> Point {
    let new_x = self.x * rad.cos() - self.y * rad.sin();
    let new_y = self.x * rad.sin() + self.y * rad.cos();
    Point::new(new_x, new_y)
  }

  /// Transform the point to a translated coordinate system.
  /// Parameter `dx` and `dy` describe how the new coordinate system translate.
  fn translate(&self, dx: f64, dy: f64) -> Point {
    let new_x = self.x - dx;
    let new_y = self.y - dy;
    Point::new(new_x, new_y)
  }

  /// Transform the point to a scaled coordinate system.
  fn scale(&self, sx: f64, sy: f64) -> Point {
    let new_x = self.x * sx;
    let new_y = self.y * sy;
    Point::new(new_x, new_y)
  }

}
