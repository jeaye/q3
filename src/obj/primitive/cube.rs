/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/cube.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of primitive geometric items.
*/

use math::Vec3f;
use primitive::Triangle;

#[packed]
struct Cube
{
  triangles: ([Triangle, ..12]),
}
impl Cube
{
  pub fn new(size: f32, center: Vec3f) -> Cube
  {
    let half = size / 2.0;
    Cube
    {
      triangles:
      ([
          Triangle::new_with_position(Vec3f::new(-half,-half,-half) + center, 
                                      Vec3f::new(-half,-half, half) + center,
                                      Vec3f::new(-half, half, half) + center),
          Triangle::new_with_position(Vec3f::new(half, half,-half) + center,
                                      Vec3f::new(-half,-half,-half) + center,
                                      Vec3f::new(-half, half,-half) + center),
          Triangle::new_with_position(Vec3f::new(half,-half, half) + center,
                                      Vec3f::new(-half,-half,-half) + center,
                                      Vec3f::new(half,-half,-half) + center),
          Triangle::new_with_position(Vec3f::new(half, half,-half) + center,
                                      Vec3f::new(half,-half,-half) + center,
                                      Vec3f::new(-half,-half,-half) + center),
          Triangle::new_with_position(Vec3f::new(-half,-half,-half) + center,
                                      Vec3f::new(-half, half, half) + center,
                                      Vec3f::new(-half, half,-half) + center),
          Triangle::new_with_position(Vec3f::new(half,-half, half) + center,
                                      Vec3f::new(-half,-half, half) + center,
                                      Vec3f::new(-half,-half,-half) + center),
          Triangle::new_with_position(Vec3f::new(-half, half, half) + center,
                                      Vec3f::new(-half,-half, half) + center,
                                      Vec3f::new(half,-half, half) + center),
          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
                                      Vec3f::new(half,-half,-half) + center,
                                      Vec3f::new(half, half,-half) + center),
          Triangle::new_with_position(Vec3f::new(half,-half,-half) + center,
                                      Vec3f::new(half, half, half) + center,
                                      Vec3f::new(half,-half, half) + center),
          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
                                      Vec3f::new(half, half,-half) + center,
                                      Vec3f::new(-half, half,-half) + center),
          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
                                      Vec3f::new(-half, half,-half) + center,
                                      Vec3f::new(-half, half, half) + center),
          Triangle::new_with_position(Vec3f::new(half, half, half) + center,
                                      Vec3f::new(-half, half, half) + center,
                                      Vec3f::new(half,-half, half) + center),
    ])
    }
  }
}

