//! This module contains calculation about lines.

use super::Point;

/// An abstraction of a geometric line.
/// Store with two vectors, which is the start and the end point of the line, respectively.
#[derive(Clone, Debug)]
pub struct Line {

  /// The start point of the line.
  pub start: Point,

  /// The end point of the line.
  pub end: Point,

}

/// An implementation of a line.
impl Line {

  /// Construct a line from two points. `pstart` is the start point and `pend` is the end point.
  pub fn new(pstart: &Point, pend: &Point) -> Line {
    Line { start: pstart.clone(), end: pend.clone() }
  }

  /// Calculate a line's length.
  pub fn len(&self) -> f64 {
    let xdis = (self.start.x - self.end.x).abs();
    let ydis = (self.start.y - self.end.y).abs();
    (xdis * xdis + ydis * ydis).sqrt()
  }

}
