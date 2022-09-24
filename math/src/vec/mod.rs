pub mod vec2;
pub mod vec3;
pub mod vec4;

#[macro_export]
macro_rules! impl_vec {
  { $name:ident, $( $m:ident ),* } => {
    impl<T> $name<T> {
      pub fn new($( $m: T ),*) -> Self {
        $name {
          $( $m ),*
        }
      }

      pub fn map<U>(
        self,
        map_fn: impl Fn($name<T>) -> $name<U>
      ) -> $name<U> {
        map_fn(self)
      }
    }

    impl<T: PartialEq + PartialOrd> PartialEq for $name<T> {
      fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).expect("Failed to compare").is_eq()
      }
    }

    impl<T: PartialEq + PartialOrd> PartialOrd for $name<T> {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        $(
          if self.$m > other.$m { // Original is comparing by ">=" but will have bugs when testing equality
            return Some(Ordering::Greater);
          }
        )*
        $(
          if self.$m < other.$m { // Original is comparing by "!=" but will have bugs when testing equality
            return Some(Ordering::Less);
          }
        )*
        Some(Ordering::Equal)
      }
    }

    impl<T: Add<Output = T>> Add<Self> for $name<T> {
      type Output = $name<T>;

      fn add(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m + rhs.$m
          ),*
        }
      }
    }

    impl<T: Sub<Output = T>> Sub<Self> for $name<T> {
      type Output = $name<T>;

      fn sub(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m - rhs.$m
          ),*
        }
      }
    }

    impl<T: Mul<Output = T>> Mul<Self> for $name<T> {
      type Output = $name<T>;

      fn mul(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m * rhs.$m
          ),*
        }
      }
    }

    impl<T: Mul<Output = T> + Copy> Mul<T> for $name<T> {
      type Output = $name<T>;

      fn mul(self, rhs: T) -> Self::Output {
        $name {
          $(
            $m: self.$m * rhs
          ),*
        }
      }
    }

    impl<T: Div<Output = T>> Div<Self> for $name<T> {
      type Output = $name<T>;

      fn div(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m / rhs.$m
          ),*
        }
      }
    }

    impl<T: Div<Output = T> + Copy> Div<T> for $name<T> {
      type Output = $name<T>;

      fn div(self, rhs: T) -> Self::Output {
        $name {
          $(
            $m: self.$m / rhs
          ),*
        }
      }
    }

    impl<T: Add<Output = T> + Copy> Add<Self> for &$name<T> {
      type Output = $name<T>;

      fn add(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m + rhs.$m
          ),*
        }
      }
    }

    impl<T: Sub<Output = T> + Copy> Sub<Self> for &$name<T> {
      type Output = $name<T>;

      fn sub(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m - rhs.$m
          ),*
        }
      }
    }

    impl<T: Mul<Output = T> + Copy> Mul<Self> for &$name<T> {
      type Output = $name<T>;

      fn mul(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m * rhs.$m
          ),*
        }
      }
    }

    impl<T: Mul<Output = T> + Copy> Mul<T> for &$name<T> {
      type Output = $name<T>;

      fn mul(self, rhs: T) -> Self::Output {
        $name {
          $(
            $m: self.$m * rhs
          ),*
        }
      }
    }

    impl<T: Div<Output = T> + Copy> Div<Self> for &$name<T> {
      type Output = $name<T>;

      fn div(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $m: self.$m / rhs.$m
          ),*
        }
      }
    }

    impl<T: Div<Output = T> + Copy> Div<T> for &$name<T> {
      type Output = $name<T>;

      fn div(self, rhs: T) -> Self::Output {
        $name {
          $(
            $m: self.$m / rhs
          ),*
        }
      }
    }
  }
}
