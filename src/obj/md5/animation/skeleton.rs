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

struct Skeleton_Joint
{
  parent: i32,
  position: math::Vec3f,
  orientation: math::Quaternion,
}

struct Frame_Skeleton
{
  joints: ~[Skeleton_Joint],
}

impl Skeleton_Joint
{
  pub fn new() -> Skeleton_Joint
  {
    Skeleton_Joint
    {
      parent: -1,
      position: math::Vec3f::zero(),
      orientation: math::Quaternion::zero(),
    }
  }
}

impl Frame_Skeleton
{
  pub fn new() -> Frame_Skeleton
  {
    Frame_Skeleton
    {
      joints: ~[],
    }
  }
}

