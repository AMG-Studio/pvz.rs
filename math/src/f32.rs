use std::ops::{
  Add,
  Sub,
  Mul,
  Div,
};
use crate::def_vec;

def_vec! {
  (Vec2, f32, x, y),
  (Vec3, f32, x, y, z),
  (Vec4, f32, x, y, z, w)
}
