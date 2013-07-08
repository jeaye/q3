/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/chunk.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A pageable 3D chunk of voxel data.
*/

use math;
use gl2 = opengles::gl2;

pub struct Chunk
{
  vbo: gl2::GLuint,
  dimensions: math::Vec3i8,
  voxels: ~[math::Vec3i8],
}

impl Chunk
{
  pub fn new(dim: &math::Vec3i8) -> Chunk
  {
    let chunk = Chunk
    {
      vbo: 0,
      dimensions: *dim,
      voxels: ~[],
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

