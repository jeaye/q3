/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/primitive/vertex.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A collection base vertex types.
*/

use math;

#[packed]
pub struct Vertex_P
{
  position: math::Vec3f,
}
impl Vertex_P
{
  pub fn new(pos: math::Vec3f) -> Vertex_P
  { Vertex_P { position: pos } }
  pub fn zero() -> Vertex_P
  { Vertex_P { position: math::Vec3f::zero() } }
}

#[packed]
pub struct Vertex_PC
{
  position: math::Vec3f,
  color: math::Vec3f,
}
impl Vertex_PC
{
  pub fn new(pos: math::Vec3f, col: math::Vec3f) -> Vertex_PC
  { Vertex_PC { position: pos, color: col } }
  pub fn new_with_position(pos: math::Vec3f) -> Vertex_PC
  { Vertex_PC { position: pos, color: pos } }
  pub fn zero() -> Vertex_PC
  { Vertex_PC { position: math::Vec3f::zero(), color: math::Vec3f::new(1.0, 1.0, 1.0) } }
}

#[packed]
pub struct Vertex_PN
{
  position: math::Vec3f,
  normal: math::Vec3f,
}
impl Vertex_PN
{
  pub fn new(pos: math::Vec3f, norm: math::Vec3f) -> Vertex_PN
  { Vertex_PN { position: pos, normal: norm } }
  pub fn new_with_position(pos: math::Vec3f) -> Vertex_PN
  { Vertex_PN { position: pos, normal: math::Vec3f::zero() } }
  pub fn zero() -> Vertex_PN
  { Vertex_PN { position: math::Vec3f::zero(), normal: math::Vec3f::zero() } }
}

#[packed]
pub struct Vertex_PCN
{
  position: math::Vec3f,
  color: math::Vec3f,
  normal: math::Vec3f,
}
pub type Vertex_PNC = self::Vertex_PCN;
impl Vertex_PCN
{
  pub fn new(pos: math::Vec3f, col: math::Vec3f, norm: math::Vec3f) -> Vertex_PCN
  { Vertex_PCN { position: pos, color: col, normal: norm } }
  pub fn new_with_position(pos: math::Vec3f) -> Vertex_PCN
  { Vertex_PCN { position: pos, normal: math::Vec3f::zero(), color: math::Vec3f::new(1.0, 1.0, 1.0) } }
  pub fn zero() -> Vertex_PCN
  { Vertex_PCN { position: math::Vec3f::zero(), color: math::Vec3f::new(1.0, 1.0, 1.0), normal: math::Vec3f::zero() } }
}

