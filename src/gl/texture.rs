/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/texture.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A wrapper of arbitrary OpenGL textures.
*/

use std::{ vec, cast };
use gl2 = opengles::gl2;
use stb_image;
use math;
use util::Log;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../util/log_macros.rs"]
mod log_macros;

struct Texture
{
  target: gl2::GLenum,
  obj: gl2::GLuint,
  filename: @str,
  size: math::Vec2i,
}

impl Texture
{
  #[fixed_stack_segment]
  pub fn new(targ: gl2::GLenum, file: &str) -> Texture
  {
    let mut tex = Texture
    {
      target: targ,
      obj: 0,
      filename: file.to_managed(),
      size: math::Vec2i::zero(),
    };

    let name = check!(gl2::gen_textures(1));
    assert!(name.len() == 1);
    tex.obj = name[0];
    tex.bind(0);

    check!(gl2::pixel_store_i(gl2::UNPACK_ALIGNMENT, 4));
    check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_MIN_FILTER, gl2::LINEAR as gl2::GLint));
    check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_MAG_FILTER, gl2::LINEAR as gl2::GLint));
    check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_WRAP_S, gl2::CLAMP_TO_EDGE as gl2::GLint));
    check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_WRAP_T, gl2::CLAMP_TO_EDGE as gl2::GLint));

    match stb_image::image::load(file.to_owned())
    {
      stb_image::image::ImageU8(ref image) => 
      {
        log_debug!("Loaded image %s with %ux%u:%u", 
                    tex.filename, image.width, image.height, image.depth);

        tex.size = math::Vec2i::new(image.width as i32, image.height as i32);
        let format = match image.depth
        {
          3 => { gl2::RGB },
          4 => { gl2::RGBA },
          x => { log_error!("Invalid texture depth %u", x); gl2::RGBA }
        };

        let data = image.data.clone();
        unsafe {
          check!(gl2::glTexImage2D
          (
            /* target */ gl2::TEXTURE_2D, 
            /* mipmap */ 0, 
            /* internal */ gl2::RGBA8 as gl2::GLint, 
            /* size */ tex.size.x as gl2::GLsizei, tex.size.y as gl2::GLsizei, 
            /* border */ 0, 
            /* external */ format, 
            /* size type */ gl2::UNSIGNED_BYTE, 
            /* data */ cast::transmute(vec::raw::to_ptr(data))
          ));
        }
      }
      _ => fail!(fmt!("Failed to load texture %s", tex.filename))
    }

    tex
  }

  pub fn bind(&self, _unit: gl2::GLenum)
  {
    //check!(gl2::active_texture(gl2::TEXTURE0 + unit));
    check!(gl2::bind_texture(gl2::TEXTURE_2D, self.obj));
  }

  pub fn unbind(&self)
  { check!(gl2::bind_texture(gl2::TEXTURE_2D, 0)); }
}
 

