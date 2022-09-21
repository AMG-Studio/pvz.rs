//! This module contains syntax sugars for the whole math library.
//! This sugar can be used in constructing lots of objects, especially geometric objects.

mod vec_sugar;
mod macro_sugar;

pub use vec_sugar::*;
pub use macro_sugar::*;
