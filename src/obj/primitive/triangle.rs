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
use primitive::Vertex_PC;
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
}

