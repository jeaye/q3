/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/triangle.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A voxel-specific triangle.
*/

use math::{ Vec3i, Vec3u8 };
use Vert = super::Vertex;

#[packed]
struct Triangle 
{
  verts: ([Vert, ..3]),
}
impl Triangle
{
  #[inline(always)]
  pub fn new(v1: Vert, v2: Vert, v3: Vert) -> Triangle
  { Triangle { verts: ([ v1, v2, v3 ]) } }
  #[inline(always)]
  pub fn new_with_position(v1: Vec3i, v2: Vec3i, v3: Vec3i) -> Triangle
  { Triangle { verts: ([ Vert::new_with_position(v1), Vert::new_with_position(v2), Vert::new_with_position(v3) ]) } }
  #[inline(always)]
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
  #[inline(always)]
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

