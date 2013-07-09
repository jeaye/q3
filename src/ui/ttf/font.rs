/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/font.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TrueType font atlas.
*/

use std::{ str, vec, cmp, ptr };
use std::hashmap::HashMap;
use std::libc::{ c_uint };
use std::iterator::IteratorUtil;
use math;
use self::glyph::Glyph;
use gl2 = opengles::gl2;

#[path = "../../gl/check.rs"]
mod check;

mod glyph;
mod ft;

struct Font
{
  file: ~str,
  library: ft::Library,
  face: ft::Face,
  texture_atlas: gl2::GLuint,
  atlas_dimensions: math::Vec2i,
  glyphs: HashMap<u8, Glyph>,
  height: i32,
}

impl Font
{
  pub fn new(filename: &str, size: i32) -> Font
  {
    let mut font = Font
    {
      file: filename.to_owned(),
      library: ptr::null(),
      face: ptr::null(),
      texture_atlas: 0,
      atlas_dimensions: math::Vec2i::zero(),
      glyphs: HashMap::new::<u8, Glyph>(),
      height: 0,
    };

    unsafe
    {
      ft::FT_Init_FreeType(&font.library);

      do str::as_c_str(filename) |c_str|
      {
        if ft::FT_New_Face(font.library, c_str, 0, &font.face) != 0
        { fail!(~"Failed to create TTF face."); }
      }
   
      ft::FT_Set_Pixel_Sizes(font.face, 0, size as c_uint);
      check!(gl2::active_texture(gl2::TEXTURE0));

      let ft_glyph = (*font.face).glyph;
      let max_width = 1024;
      let mut row_width = 0;
      let mut row_height = 0;

      let chars = &" !\"#$%&'`()*+,-_./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^abcdefghijklmnopqrstuvwxyz{|}~";

      for chars.iter().advance |curr|
      {
        if ft::FT_Load_Char(font.face, curr as u32, ft::LOAD_RENDER) != 0
        { loop; }

        /* If we've exhausted the width for this row, add another. */
        if row_width + (*ft_glyph).bitmap.width + 1 >= max_width
        {
          font.atlas_dimensions.x = cmp::max(font.atlas_dimensions.x, row_width);
          font.atlas_dimensions.y += row_height;
          row_width = 0; row_height = 0;
        }

        let mut glyph = Glyph::new();
        glyph.advance.x = ((*ft_glyph).advance.x >> 6) as f32;
        glyph.advance.y = ((*ft_glyph).advance.y >> 6) as f32;
        glyph.dimensions.x = (*ft_glyph).bitmap.width as f32;
        glyph.dimensions.y = (*ft_glyph).bitmap.rows as f32;
        glyph.offset.x = (*ft_glyph).bitmap_left as f32;
        glyph.offset.y = (*ft_glyph).bitmap_top as f32;
        glyph.buffer = vec::from_buf( (*ft_glyph).bitmap.buffer,
                                      (glyph.dimensions.x * glyph.dimensions.y) as uint);

        row_width += (glyph.dimensions.x + 1.0) as i32;
        row_height = cmp::max(row_height, (*ft_glyph).bitmap.rows);
        font.height = cmp::max(font.height, row_height);

        font.glyphs.insert(curr as u8, glyph);
      }

      font.atlas_dimensions.x = math::next_power_of_2(cmp::max(font.atlas_dimensions.x, row_width));
      font.atlas_dimensions.y = math::next_power_of_2(font.atlas_dimensions.y + row_height);

      /* We're using 1 byte alignment buffering. */
      check!(gl2::pixel_store_i(gl2::UNPACK_ALIGNMENT, 1));
      
      let name = check!(gl2::gen_textures(1));
      assert!(name.len() == 1);
      font.texture_atlas = name[0];
      check!(gl2::bind_texture(gl2::TEXTURE_2D, font.texture_atlas));
      check!(gl2::tex_image_2d(gl2::TEXTURE_2D, 0, gl2::RGB as gl2::GLint,
                              font.atlas_dimensions.x, font.atlas_dimensions.y,
                              0, gl2::RED, gl2::UNSIGNED_BYTE, None));

      /* Clamp to the edge to avoid artifacts when scaling. */
      check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_WRAP_S, gl2::CLAMP_TO_EDGE as i32));
      check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_WRAP_T, gl2::CLAMP_TO_EDGE as i32));

      /* Linear filtering usually looks best for text. */
      check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_MIN_FILTER, gl2::LINEAR as i32));
      check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_MAG_FILTER, gl2::LINEAR as i32));

      /* Copy all glyphs into the texture atlas. */
      let mut offset = math::Vec2i::zero();
      row_height = 0;
      for chars.iter().advance |curr|
      {
        let glyph = match font.glyphs.find_mut(&(curr as u8))
        {
          Some(g) => g,
          None => fail!(fmt!("Invalid char (%?) in font %?", curr, filename))
        };

        if offset.x + (glyph.dimensions.x as i32) + 1 >= max_width
        {
          offset.y += row_height;
          row_height = 0; offset.x = 0;
        }

        { /* temp has a short scope. */
          let temp: &[u8] = glyph.buffer;
          check!(gl2::tex_sub_image_2d(
                      gl2::TEXTURE_2D, 0, offset.x, offset.y,
                      glyph.dimensions.x as i32, glyph.dimensions.y as i32,
                      gl2::RED, gl2::UNSIGNED_BYTE, Some(temp)));
        }

        /* Calculate the position in the texture. */
        glyph.tex.x = (offset.x as f32 / (font.atlas_dimensions.x as f32));
        glyph.tex.y = (offset.y as f32 / (font.atlas_dimensions.y as f32));

        offset.x += glyph.dimensions.x as i32;
        row_height = cmp::max(row_height, glyph.dimensions.y as i32);
      }
    }

    /* Reset the state. */
    check!(gl2::pixel_store_i(gl2::UNPACK_ALIGNMENT, 4));

    assert!(font.height > 0);

    font
  }
}

impl Drop for Font
{
  fn drop(&self)
  {
    unsafe
    { ft::FT_Done_FreeType(self.library); }
  }
}

