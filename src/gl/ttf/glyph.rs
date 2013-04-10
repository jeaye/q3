/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/glyph.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A single character stored in a font's texture atlas.
*/

use math::Vec2;

pub struct Glyph
{
  tex: Vec2<f32>,
  advance: Vec2<f32>,
  dimensions: Vec2<f32>,
  offset: Vec2<f32>
}

impl Glyph
{
  pub fn new() -> Glyph
  {
    Glyph { tex: Vec2::zero::<f32>(),
            advance: Vec2::zero::<f32>(),
            dimensions: Vec2::zero::<f32>(),
            offset: Vec2::zero::<f32>() }
  }
}

