#[macro_export]
macro_rules! vec2 {
  [ $x: expr, $y: expr ] => {
    $crate::vec::vec2::Vec2::new($x, $y)
  };
}

#[macro_export]
macro_rules! vec3 {
  [ $x: expr, $y: expr, $z: expr ] => {
    $crate::vec3::Vec3::new($x, $y, $z)
  };
}

#[macro_export]
macro_rules! vec4 {
  [ $x: expr, $y: expr, $z: expr, $w: expr ] => {
    $crate::vec::vec4::Vec4::new($x, $y, $z, $w)
  };
}

#[macro_export]
macro_rules! line {
  [$pstart: expr => $pend: expr] => {
    $crate::geometry::line::Line::new(&$pstart, &$pend)
  };
}

#[allow(unused_imports)]
mod test {
  use crate::sugar::{Vec2ConstructionSugarExt, Vec4ConstructionSugarExt};

  #[test]
  fn test_sugar() {
    let a = vec2![ 1, 2 ];
    let b = vec4![ 1, 2, 3, 4 ];
    assert_eq!(a, (1, 2).to_vec2());
    assert_eq!(b, (1, 2, 3, 4).to_vec4());
    let c = line![ vec2![ 1.0, 2.0 ] => vec2![ 3.0, 4.0 ] ];
    println!("{:?}", c);
  }
}
