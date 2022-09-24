use std::ops::{
  Add,
  Sub,
  Mul,
  Div,
};
use std::cmp::Ordering;
use crate::impl_vec;

#[derive(Debug)]
pub struct Vec4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl_vec! {
  Vec4, x, y, z, w
}
