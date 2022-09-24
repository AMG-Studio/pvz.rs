//! This module contains related contents about polygon's calculations.

use super::{line::Line, Point};

/// The polygon trait.
pub trait Polygon {
  /// Returns all the points of the polygon.
  fn points(&self) -> Vec<Point>;

  /// Returns all the edges of the polygon.
  fn edges(&self) -> Vec<Line>;

  /// Check if the given point is inside the polygon.
  /// The default algorithm is **Winding Numbers**.
  fn is_point_inside(&self, p: &Point) -> bool {
    // Assistant functions.

    #[inline(always)]
    fn is_left(l: &Line, p: &Point) -> f64 {
      (l.end.x - l.start.x) * (p.y - l.start.y) - (p.x - l.start.x) * (l.end.y - l.start.y)
    }

    #[inline(always)]
    fn wn_pn_poly(p: &Point, v: &Vec<Line>) -> i32 {
      let mut wn = 0;

      // Iterate over all the lines.
      for i in v {
        if i.start.y <= p.y {
          if i.end.y > p.y {
            if is_left(i, p) > 0.0 {
              wn += 1;
            }
          }
        } else {
          if i.end.y <= p.y {
            if is_left(i, p) < 0.0 {
              wn -= 1;
            }
          }
        }
      }

      wn
    }

    wn_pn_poly(p, &self.edges()) != 0
  }
}
