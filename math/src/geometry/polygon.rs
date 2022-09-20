//! This module contains related contents about polygon's calculations.

use super::{Point, line::Line};

/// The polygon trait.
pub trait Polygon {

    /// Should returns all the points of the polygon in the stroking order.
    /// For example, for a rectangle it should return the following points in the following order: left-top, right-top, right-bottom, left-bottom.
    fn points(&self) -> Vec<Point>;

    /// Check if the given point is inside the polygon.
    /// The default algorithm is **Winding Numbers**.
    fn is_point_inside(&self, p: &Point) -> bool {
        // Unimplemented yet.
        todo!()
    }

}

impl dyn Polygon {

    /// Returns the lines of the polygon.
    pub fn lines(&self) -> Vec<Line> {
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
