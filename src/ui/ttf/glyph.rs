/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/glyph.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A single character stored in a font's texture atlas.
*/

use math::Vec2f;

#[packed]
pub struct Glyph
{
  tex: Vec2f,
  advance: Vec2f,
  dimensions: Vec2f,
  offset: Vec2f,
  buffer: ~[u8]
}

impl Glyph
{
  #[inline(always)]
  pub fn new() -> Glyph
  {
    Glyph
    {
      tex: Vec2f::zero(),
      advance: Vec2f::zero(),
      dimensions: Vec2f::zero(),
      offset: Vec2f::zero(),
      buffer: ~[]
    }
  }
}

