/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/font.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TrueType font atlas.
*/

use core::libc::{ c_uint };
use gl = opengles::gl2;
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
  texuture_atlas: gl::GLuint
}

impl Font /* TODO: Check macro for Freetype. */
{
  pub fn new(file: &str, size: i32) -> Font
  {
    let mut font = Font { library: ptr::null(),
                          face: ptr::null(),
                          texuture_atlas: 0 
                        };

    unsafe
    {
      ft::FT_Init_FreeType(&font.library);

      do str::as_c_str(file) |c_str|
      { ft::FT_New_Face(font.library, c_str, 0, &font.face); }
   
      ft::FT_Set_Pixel_Sizes(font.face, 0, size as c_uint);
    }

    let mut glyph = Glyph::new();
    glyph.offset.x = 0.0;

    font
  }
}
impl Drop for Font
{
  fn finalize(&self)
  {
    unsafe
    { ft::FT_Done_FreeType(self.library);; }
  }
}

