use std::ops::{
  Add,
  Sub,
  Mul,
  Div,
};
use std::cmp::Ordering;
use crate::impl_vec;

#[derive(Clone, Debug)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl_vec! {
  Vec3, x, y, z
}

