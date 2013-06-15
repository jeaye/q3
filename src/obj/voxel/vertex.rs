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
use math::{ Vec3i, Vec3u8 };

#[packed]
pub struct Vertex
{
  position: Vec3i,
  color: Vec3u8,
  unused: u8,
}

impl Vertex
{
  #[inline(always)]
  pub fn new(pos: Vec3i, col: Vec3u8) -> Vertex
  { Vertex { position: pos, color: col, unused: 0 } }
  #[inline(always)]
  pub fn new_with_position(pos: Vec3i) -> Vertex
  { Vertex { position: pos, color: Vec3u8::new(pos.x as u8, pos.y as u8, pos.z as u8), unused: 0 } }
  #[inline(always)]
  pub fn zero() -> Vertex
  { Vertex { position: Vec3i::zero(), color: Vec3u8::new(1, 1, 1), unused: 0 } }
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

