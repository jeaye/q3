/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/font.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TrueType font atlas.
*/

#[path = "./ft.rs"]
mod ft;

struct Font
{
  library: ft::Library
}

impl Font /* TODO: Check macro for Freetype. */
{
  pub fn new() -> Font
  {
    let mut font = Font { library: ptr::null() };

    unsafe
    { ft::FT_Init_FreeType(&font.library); }

    font
  }
}

