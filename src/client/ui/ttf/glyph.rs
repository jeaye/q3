/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gl/ttf/glyph.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A single character stored in a font's texture atlas.
*/

use math;

#[packed]
pub struct Glyph
{
  tex: math::Vec2f,
  advance: math::Vec2f,
  dimensions: math::Vec2f,
  offset: math::Vec2f,
  buffer: ~[u8]
}

impl Glyph
{
  pub fn new() -> Glyph
  {
    Glyph
    {
      tex: math::Vec2f::zero(),
      advance: math::Vec2f::zero(),
      dimensions: math::Vec2f::zero(),
      offset: math::Vec2f::zero(),
      buffer: ~[]
    }
  }
}

