/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/model/joint.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Represents the intersection
      of two bones.
*/

use math;

#[deriving(Clone)]
struct Joint
{
  name: ~str,
  parent: i32,
  position: math::Vec3f,
  orientation: math::Quaternion,
}

impl Joint
{
  pub fn new() -> Joint
  {
    Joint
    {
      name: ~"Default",
      parent: 0,
      position: math::Vec3f::zero(),
      orientation: math::Quaternion::zero(),
    }
  }
}

