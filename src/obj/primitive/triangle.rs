/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/triangle.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A generic triangle of three vertices.
*/

use math;
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
  pub fn new_with_position(v1: math::Vec3f, v2: math::Vec3f, v3: math::Vec3f) -> Triangle
  { Triangle { verts: ([ Vert::new_with_position(v1), Vert::new_with_position(v2), Vert::new_with_position(v3) ]) } }
  pub fn zero() -> Triangle
  { Triangle { verts: ([ Vert::zero(), ..3 ]) } }

  pub fn get_normal(&self) -> math::Vec3f
  {
    let mut a = math::Vec3f::zero();
    let mut b = math::Vec3f::zero();
    let mut res;

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
struct Voxel_Triangle /* TODO: Template this? */
{
  verts: ([Vert, ..3]),
}
impl Voxel_Triangle
{
  pub fn new(v1: Vert, v2: Vert, v3: Vert) -> Voxel_Triangle
  { Voxel_Triangle { verts: ([ v1, v2, v3 ]) } }
  pub fn new_with_position(v1: math::Vec3f, v2: math::Vec3f, v3: math::Vec3f) -> Voxel_Triangle
  { Voxel_Triangle { verts: ([ Vert::new_with_position(v1), Vert::new_with_position(v2), Vert::new_with_position(v3) ]) } }
  pub fn zero() -> Voxel_Triangle
  { Voxel_Triangle { verts: ([ Vert::zero(), ..3 ]) } }

  pub fn get_normal(&self) -> math::Vec3f
  {
    let mut a = math::Vec3f::zero();
    let mut b = math::Vec3f::zero();
    let mut res;

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

