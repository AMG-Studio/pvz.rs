//! This module contains some geometric calculation utilities.
//! 

/// The type to represent a point. Exported from `math::point::Point`(which is actually `math::vec2::Vec2<f64>`).
pub use crate::point::Point;

pub mod line;
pub mod polygon;
