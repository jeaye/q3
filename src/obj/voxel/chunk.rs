/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/chunk.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A pageable 3D chunk of voxel data.
*/

use math::{ Vec3i8 };
use primitive::{ Cube, Cube_Index };
use super::Behavior;

#[path = "../../gl/mod.rs"]
mod gl;
#[path = "../../gl/util.rs"]
mod util;
#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

pub struct Chunk
{
  vbo: gl::GLuint,
  dimensions: Vec3i8,
  voxels: ~[Cube],
  indices: ~[Cube_Index],
  behaviors: ~[Behavior],
}

impl Chunk
{
  pub fn new() -> Chunk
  {
    let chunk = Chunk
    {
      vbo: check!(gl::gen_buffers(1))[0],
      dimensions: Vec3i8::new(16, 16, 16),
      voxels: ~[],
      indices: ~[],
      behaviors: ~[],
    };

    chunk
  }

  //pub fn get(&self, x: i8, y: i8, z: i8) -> Behavior
  //{
  //  voxels[(z * self.dimensions.z * self.dimensions.z) + (y * self.dimensions.y) + x].get_behavior()
  //}

  pub fn draw(&self)
  {

  }
}

