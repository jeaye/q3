/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/md5/model/vertex.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Represents a 3D point.
*/

use math;

struct Vertex
{
  position: math::Vec3f,
  normal: math::Vec3f,
  tex_coord: math::Vec2f,
  start_weight: i32,
  weight_count: i32,
}

impl Vertex
{
  pub fn new() -> Vertex
  {
    Vertex
    {
      position: math::Vec3f::zero(),
      normal: math::Vec3f::zero(),
      tex_coord: math::Vec2f::zero(),
      start_weight: 0,
      weight_count: 0,
    }
  }
}

