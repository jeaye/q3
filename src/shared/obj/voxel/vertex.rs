/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/voxel/vertex.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A vertex specific to voxels.
*/

use std::cmp;
use math;

#[packed]
pub struct Vertex
{
  position: math::Vec3f,
  color: math::Vec3f,
}

impl Vertex
{
  pub fn new(pos: math::Vec3f, col: math::Vec3f) -> Vertex
  { Vertex { position: pos, color: col } }
  pub fn new_with_position(pos: math::Vec3f) -> Vertex
  { Vertex { position: pos, color: math::Vec3f::new(pos.x, pos.y, pos.z) } }
  pub fn zero() -> Vertex
  { Vertex { position: math::Vec3f::zero(), color: math::Vec3f::new(1.0, 1.0, 1.0) } }
}

impl cmp::Ord for Vertex
{
  fn lt(&self, other: &Vertex) -> bool
  { self.position < other.position }
  fn le(&self, other: &Vertex) -> bool
  { (self == other) || (self < other) }
  fn ge(&self, other: &Vertex) -> bool
  { (self == other) || (self > other) }
  fn gt(&self, other: &Vertex) -> bool
  { !(self == other) && !(self < other) }
}

impl cmp::TotalOrd for Vertex
{
  fn cmp(&self, other: &Vertex) -> cmp::Ordering
  {
    if self.position < other.position
    { cmp::Less }
    else if self.position > other.position
    { cmp::Greater }
    else
    { cmp::Equal }
  }
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

impl cmp::TotalEq for Vertex
{
  fn equals(&self, other: &Vertex) -> bool
  { self.position == other.position }
}

