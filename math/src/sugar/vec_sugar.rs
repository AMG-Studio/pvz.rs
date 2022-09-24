use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use crate::vec::vec4::Vec4;

/// Extension used to consturct a 2d vector.
pub trait Vec2ConstructionSugarExt<T> {
  /// Construct a vector2 from the object.
  fn to_vec2(&self) -> Vec2<T>;
}

impl<T> Vec2ConstructionSugarExt<T> for (T, T)
where
  T: Clone,
{
  /// Construct a vector2 from a tuple.
  /// ```
  /// let a = (3, 5).to_vec2();
  /// assert!(a.x == 3 && a.y == 5);
  /// ```
  fn to_vec2(&self) -> Vec2<T> {
    Vec2::<T>::new(self.0.clone(), self.1.clone())
  }
}

/// Extension used to consturct a 3d vector.
pub trait Vec3ConstructionSugarExt<T> {
  /// Construct a vector3 from the object.
  fn to_vec3(&self) -> Vec3<T>;
}

impl<T> Vec3ConstructionSugarExt<T> for (T, T, T)
where
  T: Clone,
{
  /// Construct a vector3 from a tuple.
  /// ```
  /// let a = (3, 5, 7).to_vec3();
  /// assert!(a.x == 3 && a.y == 5 && a.z == 7);
  /// ```
  fn to_vec3(&self) -> Vec3<T> {
    Vec3::<T>::new(self.0.clone(), self.1.clone(), self.2.clone())
  }
}

/// Extension used to consturct a 4d vector.
pub trait Vec4ConstructionSugarExt<T> {
  /// Construct a vector4 from the object.
  fn to_vec4(&self) -> Vec4<T>;
}

impl<T> Vec4ConstructionSugarExt<T> for (T, T, T, T)
where
  T: Clone,
{
  /// Construct a vector4 from a tuple.
  /// ```
  /// let a = (3, 5, 10, 2).to_vec4();
  /// assert!(a.x == 3 && a.y == 5 && a.z == 10 && a.w == 2);
  /// ```
  fn to_vec4(&self) -> Vec4<T> {
    Vec4::<T>::new(
      self.0.clone(),
      self.1.clone(),
      self.2.clone(),
      self.3.clone(),
    )
  }
}

#[allow(unused_imports)]
mod test {
  use super::*;
  use crate::prelude::*;

  #[test]
  fn test_sugar() {
    let a = (3, 5).to_vec2();
    assert!(a.x == 3 && a.y == 5);

    let b = (3, 5, 7).to_vec3();
    assert!(b.x == 3 && b.y == 5 && b.z == 7);
  }

  #[test]
  fn test_vec_equality() {
    assert_eq!((3, 5).to_vec2(), (3, 5).to_vec2());
  }
}
