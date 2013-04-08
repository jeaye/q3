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

use math::Vec3f;

pub trait Movable
{
  /* Relative translate. */
  pub fn translate(&mut self, new_position: Vec3f);
  /* Absolute translate. */
  pub fn translate_to(&mut self, new_position: Vec3f);
}

pub trait Rotatable
{
  /* Relative translate. */
  pub fn rotate(&mut self, new_rotation: Vec3f);
  /* Absolute translate. */
  pub fn rotate_to(&mut self, new_rotation: Vec3f);
}

pub trait Scalable
{
  /* Relative scale. */
  pub fn scale(&mut self, new_scale: Vec3f);
}

