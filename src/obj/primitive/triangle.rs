/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/triangle.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of primitive geometric items.
*/

use math::Vec3f;
use Vert = primitive::Vertex_PC;

#[packed]
struct Triangle /* TODO: Template this? */
{
  verts: ([Vert, ..3]),
}
impl Triangle
{
  pub fn new(v1: Vert, v2: Vert, v3: Vert) -> Triangle
  { Triangle { verts: ([ v1, v2, v3 ]) } }
  pub fn new_with_position(v1: Vec3f, v2: Vec3f, v3: Vec3f) -> Triangle
  { Triangle { verts: ([ Vert::new_with_position(v1), Vert::new_with_position(v2), Vert::new_with_position(v3) ]) } }
  pub fn zero() -> Triangle
  { Triangle { verts: ([ Vert::zero(), ..3 ]) } }

  pub fn get_normal(&self) -> Vec3f
  {
    let mut a = Vec3f::zero(), b = Vec3f::zero(), res = Vec3f::zero();

    /* First edge. */
    a.x = self.verts[0].position.x - self.verts[1].position.x;
    a.y = self.verts[0].position.y - self.verts[1].position.y;
    a.z = self.verts[0].position.z - self.verts[1].position.z;

    /* Second edge. */
    b.x = self.verts[1].position.x - self.verts[2].position.x;
    b.y = self.verts[1].position.y - self.verts[2].position.y;
    b.z = self.verts[1].position.z - self.verts[2].position.z;

    res = a.cross(&b); /* TODO: Other way? */
    res.normalize();
    res
  }
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

