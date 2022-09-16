use std::ops::{
  Add,
  Sub,
  Mul,
  Div,
};
use crate::def_vec;

def_vec! {
  (IVec2, isize, x, y),
  (IVec3, isize, x, y, z),
  (IVec4, isize, x, y, z, w)
}
