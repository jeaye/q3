/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/weight.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A biased attraction to a bone.
      Weights are assigned to vertices
      to influence their final position.
*/

use math;

struct Weight
{
  joint_id: i32,
  bias: f32,
  position: math::Vec3f,
}

impl Weight
{
  pub fn new() -> Weight
  {
    Weight
    {
      joint_id: 0,
      bias: 0.0,
      position: math::Vec3f::zero(),
    }
  }
}

