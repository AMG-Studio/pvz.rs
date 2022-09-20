//! This module contains related contents about polygon's calculations.

use super::{line::Line, Point};

/// The polygon trait.
pub trait Polygon {
  /// Should returns all the points of the polygon in the stroking order.
  /// For example, for a rectangle it should return the following points in the following order: left-top, right-top, right-bottom, left-bottom.
  fn points(&self) -> Vec<Point>;

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

    wn_pn_poly(p, &self.lines()) != 0
  }

  /// Return the lines of the polygon in the stroking order.
  fn lines(&self) -> Vec<Line> {
    let points = self.points();
    if points.len() <= 1 {
      // Check if the points is more than 1.
      return vec![];
    }
    let mut result = vec![];
    let mut last = None;
    for i in &points {
      if last.is_none() {
        // The first point.
        last = Some(i);
        continue;
      } else {
        // Connect the current point with the previous point.
        result.push(Line::new(&i, &last.unwrap().clone()));
        last = Some(i);
      }
    }
    if points.len() >= 3 {
      // Connect the last point with the first point.
      result.push(Line::new(points.last().unwrap(), &points[0]));
    }
    result
  }
}
