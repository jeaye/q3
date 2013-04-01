/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/traits.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A trait representing movable
      objects that can translate, rotate,
      and scale.
*/

#[path = "../math/math.rs"]
mod math;

pub trait Movable
{
  /* Relative translate. */
  pub fn translate(&mut self, new_position: math::Vec3<f32>);
  /* Absolute translate. */
  pub fn translate_to(&mut self, new_position: math::Vec3<f32>);
}

pub trait Rotatable
{
  /* Relative translate. */
  pub fn rotate(&mut self, new_rotation: math::Vec3<f32>);
  /* Absolute translate. */
  pub fn rotate_to(&mut self, new_rotation: math::Vec3<f32>);
}

pub trait Scalable
{
  /* Relative scale. */
  pub fn scale(&mut self, new_scale: math::Vec3<f32>);
}

