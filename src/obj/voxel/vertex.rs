/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/vertex.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A vertex specific to voxels.
*/

use std::cmp;
use math::{ Vec3f };

#[packed]
pub struct Vertex
{
  position: Vec3f,
  color: Vec3f,
}

impl Vertex
{
  #[inline(always)]
  pub fn new(pos: Vec3f, col: Vec3f) -> Vertex
  { Vertex { position: pos, color: col } }
  #[inline(always)]
  pub fn new_with_position(pos: Vec3f) -> Vertex
  { Vertex { position: pos, color: Vec3f::new(pos.x, pos.y, pos.z) } }
  #[inline(always)]
  pub fn zero() -> Vertex
  { Vertex { position: Vec3f::zero(), color: Vec3f::new(1.0, 1.0, 1.0) } }
}

impl cmp::Ord for Vertex
{
  fn lt(&self, other: &Vertex) -> bool
  {
    if self.position.x < other.position.x
    { return true; }
    else if self.position.x > other.position.x
    { return false; }

    if self.position.y < other.position.y
    { return true; }
    else if self.position.y > other.position.y
    { return false; }

    if self.position.z < other.position.z
    { return true; }
    else if self.position.z > other.position.z
    { return false; }
    else
    { false }
  }

  /* I don't care about these. */
  fn le(&self, _other: &Vertex) -> bool
  { fail!("Unsupported") }
  fn ge(&self, _other: &Vertex) -> bool
  { fail!("Unsupported") }
  fn gt(&self, _other: &Vertex) -> bool
  { fail!("Unsupported") }
}

impl cmp::Eq for Vertex
{
  fn eq(&self, other: &Vertex) -> bool
  {
    self.position.x == other.position.x &&
    self.position.y == other.position.y &&
    self.position.z == other.position.z
  }
  fn ne(&self, other: &Vertex) -> bool
  { !(self == other) }
}

