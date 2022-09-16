#![allow(clippy::derive_partial_eq_without_eq)]

pub mod f32;
pub mod isize;
pub mod usize;

#[macro_export]
macro_rules! def_vec {
  { $( ( $name:ident, $ty:ty, $( $m:ident ),+ ) ),* } => {
    $(
      #[derive(Clone, Debug, PartialEq)]
      pub struct $name {
        $(
          $m: $ty
        ),+
      }

      impl $name {
        pub fn new($( $m: $ty ),+) -> Self {
          $name {
            $( $m ),+
          }
        }
      }

      impl Add<$name> for $name {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
          $name {
            $(
              $m: self.$m + rhs.$m
            ),+
          }
        }
      }

      impl Sub<$name> for $name {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
          $name {
            $(
              $m: self.$m - rhs.$m
            ),+
          }
        }
      }

      impl Mul<$ty> for $name {
        type Output = Self;

        fn mul(self, rhs: $ty) -> Self::Output {
          $name {
            $(
              $m: self.$m * rhs
            ),+
          }
        }
      }

      impl Div<$ty> for $name {
        type Output = Self;

        fn div(self, rhs: $ty) -> Self {
          $name {
            $(
              $m: self.$m / rhs
            ),+
          }
        }
      }
    )*
  }
}

