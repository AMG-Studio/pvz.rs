use std::ops::{
  Add,
  Sub,
  Mul,
  Div,
};
use crate::def_vec;

def_vec! {
  (UVec2, usize, x, y),
  (UVec3, usize, x, y, z),
  (UVec4, usize, x, y, z, w)
}
