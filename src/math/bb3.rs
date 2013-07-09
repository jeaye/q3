/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/bb3.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 3D bounding box.
*/

use math;

pub struct BB3 /* TODO: Check math on this shit. */
{
  top_left: math::Vec3f,
  bottom_right: math::Vec3f,
}

impl BB3
{
  pub fn new(t_left: math::Vec3f, b_right: math::Vec3f) -> BB3
  {
    BB3{ top_left: t_left, bottom_right: b_right }
  }

  pub fn zero() -> BB3
  {
    BB3{  top_left: math::Vec3f::zero(), bottom_right: math::Vec3f::zero() } 
  }

  pub fn center(&self) -> math::Vec3f
  { math::Vec3f::new( (self.bottom_right.x - self.top_left.x) / 2.0,
                      (self.top_left.y - self.bottom_right.y) / 2.0,
                      (self.top_left.z - self.bottom_right.z) / 2.0 )
  }

  pub fn center_with_offset(&self, offset: math::Vec3f) -> math::Vec3f
  { math::Vec3f::new( ((self.bottom_right.x - self.top_left.x) / 2.0) + offset.x,
                      ((self.top_left.y - self.bottom_right.y) / 2.0) + offset.y,
                      ((self.top_left.z - self.bottom_right.z) / 2.0) + offset.z )
  }
}

