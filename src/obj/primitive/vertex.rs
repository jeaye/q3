/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/primitive/vertex.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A collection base vertex types.
*/

use math::Vec3f;

#[packed]
pub struct Vertex_P
{
  position: Vec3f,
}
impl Vertex_P
{
  #[inline(always)]
  pub fn new(pos: Vec3f) -> Vertex_P
  { Vertex_P { position: pos } }
  #[inline(always)]
  pub fn zero() -> Vertex_P
  { Vertex_P { position: Vec3f::zero() } }
}

#[packed]
pub struct Vertex_PC
{
  position: Vec3f,
  color: Vec3f,
}
impl Vertex_PC
{
  #[inline(always)]
  pub fn new(pos: Vec3f, col: Vec3f) -> Vertex_PC
  { Vertex_PC { position: pos, color: col } }
  #[inline(always)]
  pub fn new_with_position(pos: Vec3f) -> Vertex_PC
  { Vertex_PC { position: pos, color: pos } }
  #[inline(always)]
  pub fn zero() -> Vertex_PC
  { Vertex_PC { position: Vec3f::zero(), color: Vec3f::new(1.0, 1.0, 1.0) } }
}

#[packed]
pub struct Vertex_PN
{
  position: Vec3f,
  normal: Vec3f,
}
impl Vertex_PN
{
  #[inline(always)]
  pub fn new(pos: Vec3f, norm: Vec3f) -> Vertex_PN
  { Vertex_PN { position: pos, normal: norm } }
  #[inline(always)]
  pub fn new_with_position(pos: Vec3f) -> Vertex_PN
  { Vertex_PN { position: pos, normal: Vec3f::zero() } }
  #[inline(always)]
  pub fn zero() -> Vertex_PN
  { Vertex_PN { position: Vec3f::zero(), normal: Vec3f::zero() } }
}

#[packed]
pub struct Vertex_PCN
{
  position: Vec3f,
  color: Vec3f,
  normal: Vec3f,
}
pub type Vertex_PNC = self::Vertex_PCN;
impl Vertex_PCN
{
  #[inline(always)]
  pub fn new(pos: Vec3f, col: Vec3f, norm: Vec3f) -> Vertex_PCN
  { Vertex_PCN { position: pos, color: col, normal: norm } }
  #[inline(always)]
  pub fn new_with_position(pos: Vec3f) -> Vertex_PCN
  { Vertex_PCN { position: pos, normal: Vec3f::zero(), color: Vec3f::new(1.0, 1.0, 1.0) } }
  #[inline(always)]
  pub fn zero() -> Vertex_PCN
  { Vertex_PCN { position: Vec3f::zero(), color: Vec3f::new(1.0, 1.0, 1.0), normal: Vec3f::zero() } }
}

