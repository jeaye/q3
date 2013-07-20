/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/mesh.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A collection of vertices, weights, and
      joints that make up one renderable entity.
*/

use gl2 = opengles::gl2;
use super::{ Vertex, Triangle, Weight };
use math;

struct Mesh
{
  texture: ~str,
  verts: ~[Vertex],
  triangles: ~[Triangle],
  weights: ~[Weight],

  tex_id: gl2::GLuint,

  positions: ~[math::Vec3f],
  normals: ~[math::Vec3f],
  tex_coords: ~[math::Vec2f],
  indices: ~[u32],
}

impl Mesh
{
  pub fn new() -> Mesh
  {
    Mesh
    {
      texture: ~"",
      verts: ~[],
      triangles: ~[],
      weights: ~[],

      tex_id: 0,

      positions: ~[],
      normals: ~[],
      tex_coords: ~[],
      indices: ~[],
    }
  }
}

