/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/animation/bound.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TODO
*/

use math;

struct Bound
{
  min: math::Vec3f,
  max: math::Vec3f,
}

impl Bound
{
  pub fn new() -> Bound
  {
    Bound
    {
      min: math::Vec3f::zero(),
      max: math::Vec3f::zero(),
    }
  }
}

