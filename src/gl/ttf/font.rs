/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/font.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TrueType font atlas.
*/

use core::hashmap::HashMap;
use core::libc::{ c_uint };
use gl = opengles::gl2;
use math::Vec2;
use self::glyph::Glyph;

#[macro_escape]
#[path = "../check.rs"]
mod check;

mod glyph;

#[path = "./ft.rs"]
mod ft;

struct Font
{
  library: ft::Library,
  face: ft::Face,
  texuture_atlas: gl::GLuint,
  atlas_dimensions: Vec2<i32>,
  glyphs: HashMap<u8, Glyph>
}

impl Font /* TODO: Check macro for Freetype. */
{
  pub fn new(file: &str, size: i32) -> Font
  {
    let mut font = Font { library: ptr::null(),
                          face: ptr::null(),
                          texuture_atlas: 0,
                          atlas_dimensions: Vec2::zero::<i32>(),
                          glyphs: HashMap::new::<u8, Glyph>()
                        };

    unsafe
    {
      ft::FT_Init_FreeType(&font.library);

      do str::as_c_str(file) |c_str|
      { ft::FT_New_Face(font.library, c_str, 0, &font.face); }
   
      ft::FT_Set_Pixel_Sizes(font.face, 0, size as c_uint);

      let mut glyph = (*font.face).glyph;
      let max_width = 1024;
      let mut row_width = 0, row_height = 0;

      let chars: ~str = ~"!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^abcdefghijklmnopqrstuvwxyz{|}~";

      for chars.each |curr|
      {
        if ft::FT_Load_Char(font.face, curr as u32, ft::LOAD_RENDER) != 0
        { loop; }

        /* If we've exhausted the width for this row, add another. */
        if row_width + (*glyph).bitmap.width + 1 > max_width
        {
          font.atlas_dimensions.x = 
            if font.atlas_dimensions.x > row_width
            { font.atlas_dimensions.x }
            else
            { row_width };
          font.atlas_dimensions.y += row_height;
          row_width = 0; row_height = 0;
        }
      }
    }


    font
  }
}
impl Drop for Font
{
  fn finalize(&self)
  {
    unsafe
    { ft::FT_Done_FreeType(self.library); }
  }
}

