/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/cube.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of primitive geometric items.
*/

use math;
use primitive::{ Triangle, Triangle_Index };

#[packed]
struct Cube
{
  tris: ([Triangle, ..12]),
}
impl Cube
{
  pub fn new(size: f32, center: math::Vec3f) -> Cube
  {
    let half = size / 2.0;
    Cube
    {
      tris:
      ([
          Triangle::new_with_position(math::Vec3f::new(-half,-half,-half) + center, 
                                      math::Vec3f::new(-half,-half, half) + center,
                                      math::Vec3f::new(-half, half, half) + center),
          Triangle::new_with_position(math::Vec3f::new(half, half,-half) + center,
                                      math::Vec3f::new(-half,-half,-half) + center,
                                      math::Vec3f::new(-half, half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(half,-half, half) + center,
                                      math::Vec3f::new(-half,-half,-half) + center,
                                      math::Vec3f::new(half,-half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(half, half,-half) + center,
                                      math::Vec3f::new(half,-half,-half) + center,
                                      math::Vec3f::new(-half,-half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(-half,-half,-half) + center,
                                      math::Vec3f::new(-half, half, half) + center,
                                      math::Vec3f::new(-half, half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(half,-half, half) + center,
                                      math::Vec3f::new(-half,-half, half) + center,
                                      math::Vec3f::new(-half,-half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(-half, half, half) + center,
                                      math::Vec3f::new(-half,-half, half) + center,
                                      math::Vec3f::new(half,-half, half) + center),
          Triangle::new_with_position(math::Vec3f::new(half, half, half) + center,
                                      math::Vec3f::new(half,-half,-half) + center,
                                      math::Vec3f::new(half, half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(half,-half,-half) + center,
                                      math::Vec3f::new(half, half, half) + center,
                                      math::Vec3f::new(half,-half, half) + center),
          Triangle::new_with_position(math::Vec3f::new(half, half, half) + center,
                                      math::Vec3f::new(half, half,-half) + center,
                                      math::Vec3f::new(-half, half,-half) + center),
          Triangle::new_with_position(math::Vec3f::new(half, half, half) + center,
                                      math::Vec3f::new(-half, half,-half) + center,
                                      math::Vec3f::new(-half, half, half) + center),
          Triangle::new_with_position(math::Vec3f::new(half, half, half) + center,
                                      math::Vec3f::new(-half, half, half) + center,
                                      math::Vec3f::new(half,-half, half) + center),
    ])
    }
  }
}

#[packed]
struct Cube_Index
{
  indices: ([Triangle_Index, ..12]),
}
impl Cube_Index
{
  pub fn new(start: u32) -> Cube_Index
  {
    let adjusted_start = start * 36;
    Cube_Index
    {
      indices:
      [
        Triangle_Index::new(adjusted_start),
        Triangle_Index::new(adjusted_start + 3),
        Triangle_Index::new(adjusted_start + 6),
        Triangle_Index::new(adjusted_start + 9),
        Triangle_Index::new(adjusted_start + 12),
        Triangle_Index::new(adjusted_start + 15),
        Triangle_Index::new(adjusted_start + 18),
        Triangle_Index::new(adjusted_start + 21),
        Triangle_Index::new(adjusted_start + 24),
        Triangle_Index::new(adjusted_start + 27),
        Triangle_Index::new(adjusted_start + 30),
        Triangle_Index::new(adjusted_start + 33),
      ]
    }
  }
}

