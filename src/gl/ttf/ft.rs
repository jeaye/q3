/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/ft.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Wrappers for Freetype2 font loading.
*/

use core::libc::{ c_void, c_char, c_int, c_uint, c_short, c_ushort, c_long };
pub use self::ll::*;

#[nolink]
#[link_args="-lfreetype"]
#[cfg(target_os = "linux")] /* TODO: Fix for other platforms. */
extern mod linkhack { }

/* Types. */
pub type Error = int;
pub type Face = *c_void;
pub type Glyph_Slot = *c_void;
pub type Library = *c_void; /* NOTE: Each thread should have its own library. */
pub type Size = *c_void;
pub type Char_Map = *c_void;
pub type Driver = *c_void;
pub type Memory = *c_void;
pub type Stream = *c_void;
pub type Face_Internal = *c_void;
struct Generic
{
  data: *c_void,
  finalizer: *c_void
}
struct BBox
{
  x_min: c_long, y_min: c_long,
  x_max: c_long, y_max: c_long
}
struct List_Rec
{
  head: *c_void,
  tail: *c_void
}
struct Face_Rec
{
  num_faces: c_long,
  face_index: c_long,
  face_flags: c_long,
  style_flags: c_long,
  num_glyphs: c_long,
  family_name: *c_char,
  style_name: *c_char,
  num_fixed_sizes: c_int,
  available_sizes: *c_void,
  num_charmaps: c_int,
  charmaps: *c_void,
  generic: Generic,
  bbox: BBox,
  units_per_EM: c_ushort,
  ascender: c_short,
  descender: c_short,
  height: c_short,
  max_advance_width: c_short,
  max_advance_height: c_short,
  underline_position: c_short,
  underline_thickness: c_short,
  glyph: Glyph_Slot, /* Literally the only one we care about. */
  size: Size,
  charmap: Char_Map,
  driver: Driver,
  memory: Memory,
  stream: Stream,
  sizes_list: List_Rec,
  autohint: Generic,
  extensions: *c_void,
  internal: Face_Internal
}

/* Constants. */
pub static LOAD_DEFAULT: i32                      = 0x0;
pub static LOAD_NO_SCALE: i32                     = (1 << 0);
pub static LOAD_NO_HINTING: i32                   = (1 << 1);
pub static LOAD_RENDER: i32                       = (1 << 2);
pub static LOAD_NO_BITMAP: i32                    = (1 << 3);
pub static LOAD_VERTICAL_LAYOUT: i32              = (1 << 4);
pub static LOAD_FORCE_AUTOHINT: i32               = (1 << 5);
pub static LOAD_CROP_BITMAP: i32                  = (1 << 6);
pub static LOAD_PEDANTIC: i32                     = (1 << 7);
pub static LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH: i32  = (1 << 9);
pub static LOAD_NO_RECURSE: i32                   = (1 << 10);
pub static LOAD_IGNORE_TRANSFORM: i32             = (1 << 11);
pub static LOAD_MONOCHROME: i32                   = (1 << 12);
pub static LOAD_LINEAR_DESIGN: i32                = (1 << 13);
pub static LOAD_NO_AUTOHINT: i32                  = (1 << 15);

// http://freetype.sourceforge.net/freetype2/docs/reference/ft2-base_interface.html
#[nolink]
pub extern mod ll 
{
  pub fn FT_Init_FreeType(++library: *Library) -> Error;
  pub fn FT_Done_FreeType(++library: Library) -> Error;

  pub fn FT_New_Face(++library: Library, ++file_name: *c_char, ++face_index: c_int, ++face: *Face);
  pub fn FT_Set_Pixel_Sizes(++face: Face, ++pixel_width: c_uint, ++pixel_height: c_uint) -> Error;
  pub fn FT_Load_Char(++face: Face, ++char_code: c_uint, ++load_flags: c_int) -> Error;
}

