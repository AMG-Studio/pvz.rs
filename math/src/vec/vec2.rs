use crate::impl_vec;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

impl<T: Copy> Clone for Vec2<T> {
  fn clone(&self) -> Self {
    Vec2 {
      x: self.x,
      y: self.y,
    }
  }
}

impl<T> From<(T, T)> for Vec2<T> {
  fn from((x, y): (T, T)) -> Self {
    Vec2 { x, y }
  }
}

impl From<Vec2<f32>> for Vec2<usize> {
  fn from(origin: Vec2<f32>) -> Self {
    Vec2 {
      x: origin.x as usize,
      y: origin.y as usize,
    }
  }
}

impl From<Vec2<f32>> for Vec2<isize> {
  fn from(origin: Vec2<f32>) -> Self {
    Vec2 {
      x: origin.x as isize,
      y: origin.y as isize,
    }
  }
}

impl From<Vec2<isize>> for Vec2<f32> {
  fn from(origin: Vec2<isize>) -> Self {
    Vec2 {
      x: origin.x as f32,
      y: origin.y as f32,
    }
  }
}

impl From<Vec2<usize>> for Vec2<f32> {
  fn from(origin: Vec2<usize>) -> Self {
    Vec2 {
      x: origin.x as f32,
      y: origin.y as f32,
    }
  }
}

impl_vec! {
  Vec2, x, y
}
