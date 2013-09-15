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
use super::Base_Frame;

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

  pub fn new_from_base_frame(base_frame: &Base_Frame) -> Skeleton_Joint
  {
    Skeleton_Joint
    {
      parent: -1,
      position: base_frame.position,
      orientation: base_frame.orientation,
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

  pub fn interpolate(&mut self, skele0: &Frame_Skeleton, skele1: &Frame_Skeleton, interp: f32)
  {
    /* Assumes all skeletons have the same number of joints. */
    for i in range(0, self.joints.len())
    {
      let final = &mut self.joints[i];
      let joint0 = &skele0.joints[i];
      let joint1 = &skele1.joints[i];

      final.parent = joint0.parent;

      final.position = math::Vec3f::new_lerp(&joint0.position, &joint1.position, interp);
      final.orientation = math::Quaternion::new_slerp(&joint0.orientation, &joint1.orientation, interp);
    }
  }
}

