/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/triangle.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A voxel-specific triangle.
*/

use math;
use Vert = super::Vertex;

#[packed]
struct Triangle 
{
  verts: ([Vert, ..3]),
}
impl Triangle
{
  pub fn new(v1: Vert, v2: Vert, v3: Vert) -> Triangle
  { Triangle { verts: ([ v1, v2, v3 ]) } }
  pub fn new_with_position(v1: math::Vec3f, v2: math::Vec3f, v3: math::Vec3f) -> Triangle
  { Triangle { verts: ([ Vert::new_with_position(v1), Vert::new_with_position(v2), Vert::new_with_position(v3) ]) } }
  pub fn zero() -> Triangle
  { Triangle { verts: ([ Vert::zero(), ..3 ]) } }
}

#[packed]
struct Triangle_Index
{
  indices: ([u32, ..3]),
}
impl Triangle_Index
{
  pub fn new(start: u32) -> Triangle_Index
  {
    Triangle_Index
    {
      indices:
      [
        start, start + 1, start + 2,
      ]
    }
  }
}

