use crate::impl_vec;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl_vec! {
  Vec3, x, y, z
}
