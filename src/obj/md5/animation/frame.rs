/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/animation/frame.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TODO
*/

use math;

struct Base_Frame
{
  position: math::Vec3f,
  orientation: math::Quaternion,
}

struct Frame_Data
{
  frame_id: i32,
  frame_data: ~[f32],
}

impl Base_Frame
{
  pub fn new() -> Base_Frame
  {
    Base_Frame
    {
      position: math::Vec3f::zero(),
      orientation: math::Quaternion::zero(),
    }
  }
}

impl Frame_Data
{
  pub fn new() -> Frame_Data
  {
    Frame_Data
    {
      frame_id: 0,
      frame_data: ~[],
    }
  }
}

