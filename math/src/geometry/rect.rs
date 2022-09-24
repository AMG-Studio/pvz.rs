//! This module contains calculations about rectangles.

use super::{polygon::Polygon, Point};
use crate::geometry::Line;
use crate::{vec::vec2::*, vec::vec4::*};

/// A rectangle.
#[derive(Clone, Debug)]
pub struct Rect {
  /// The start point.
  point: Point,

  /// The width and the height of the rectangle.
  size: Vec2<f64>,
}

/// A assistant struct for constructing a rectangle with builder mode.
pub struct RectBuilder {
  r: Rect,
}

impl RectBuilder {
  /// Set the position of the rectangle.
  pub fn pos(&mut self, x: f64, y: f64) -> &mut RectBuilder {
    self.r.point.x = x;
    self.r.point.y = y;
    self
  }

  /// Set the left-top position of the rectangle, same as `pos()`.
  pub fn pos_lt(&mut self, x: f64, y: f64) -> &mut RectBuilder {
    self.pos(x, y);
    self
  }

  /// Set the right-bottom position of the rectangle.
  pub fn pos_rb(&mut self, x: f64, y: f64) -> &mut RectBuilder {
    self.r.size.x = (x - self.r.point.x).abs();
    self.r.size.y = (y - self.r.point.y).abs();
    self
  }

  /// Set the size of the rectangle.
  pub fn size(&mut self, w: f64, h: f64) -> &mut RectBuilder {
    self.r.size.x = w;
    self.r.size.y = h;
    self
  }

  /// Set the x-pos of the rectangle.
  pub fn x(&mut self, x: f64) -> &mut RectBuilder {
    self.r.point.x = x;
    self
  }

  /// Set the y-pos of the rectangle.
  pub fn y(&mut self, y: f64) -> &mut RectBuilder {
    self.r.point.y = y;
    self
  }

  /// Set the width of the rectangle.
  pub fn w(&mut self, w: f64) -> &mut RectBuilder {
    self.r.size.x = w;
    self
  }

  /// Set the height of the rectangle.
  pub fn h(&mut self, h: f64) -> &mut RectBuilder {
    self.r.size.y = h;
    self
  }

  /// Get the built rectangle.
  pub fn unwrap(self) -> Rect {
    self.r
  }
}

impl Default for Rect {
  /// Construct an empty rectangle.
  fn default() -> Rect {
    Rect {
      point: Point::new(0.0, 0.0),
      size: Vec2::new(0.0, 0.0),
    }
  }
}

impl Rect {
  /// Construct a rectangle from builder.
  pub fn builder() -> RectBuilder {
    RectBuilder { r: Self::default() }
  }

  /// Get the left-top position of the rectangle.
  pub fn left_top(&self) -> Point {
    Point::new(self.point.x, self.point.y)
  }

  /// Get the right-top position of the rectangle.
  pub fn right_top(&self) -> Point {
    Point::new(self.point.x + self.size.x, self.point.y)
  }

  /// Get the right-bottom position of the rectangle.
  pub fn right_bottom(&self) -> Point {
    Point::new(self.point.x + self.size.x, self.point.y + self.size.y)
  }

  /// Get the left-bottom position of the rectangle.
  pub fn left_bottom(&self) -> Point {
    Point::new(self.point.x, self.point.y + self.size.y)
  }

  /// Represent the rectangle by using a `Vec4`.
  pub fn to_vec(&self) -> Vec4<f64> {
    Vec4::new(self.point.x, self.point.y, self.size.x, self.size.y)
  }
}

impl Polygon for Rect {
  fn points(&self) -> Vec<Point> {
    vec![
      self.left_top(),
      self.right_top(),
      self.right_bottom(),
      self.left_bottom(),
    ]
  }

  fn edges(&self) -> Vec<Line> {
    todo!()
  }

  /// An optimized hit test especially implemented for rectangles.
  fn is_point_inside(&self, p: &Point) -> bool {
    ((self.point.x)..=(self.point.x + self.size.x)).contains(&p.x)
      && ((self.point.y)..=(self.point.y + self.size.y)).contains(&p.y)
  }
}
