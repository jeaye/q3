/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/texture.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A wrapper of arbitrary OpenGL textures.
*/

use std::{ vec, cast };
use gl;
use gl::types::*;
use stb_image;
use math;
use log::Log;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

pub struct Texture
{
  target: GLenum,
  obj: GLuint,
  filename: @str,
  size: math::Vec2i,
}

impl Texture
{
  #[fixed_stack_segment]
  pub fn new(targ: GLenum, file: &str) -> Texture
  {
    let mut tex = Texture
    {
      target: targ,
      obj: 0,
      filename: file.to_managed(),
      size: math::Vec2i::zero(),
    };

    check_unsafe!(gl::GenTextures(1, &mut tex.obj));
    log_assert!(tex.obj > 0);
    tex.bind(0);

    check!(gl::PixelStorei(gl::UNPACK_ALIGNMENT, 4));
    check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint));
    check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint));
    check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint));
    check!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint));

    match stb_image::image::load(file.to_owned())
    {
      stb_image::image::ImageU8(ref image) => 
      {
        log_debug!("Loaded image {} with {}x{}:{}", 
                    tex.filename, image.width, image.height, image.depth);

        tex.size = math::Vec2i::new(image.width as i32, image.height as i32);
        let format = match image.depth
        {
          3 => { gl::RGB },
          4 => { gl::RGBA },
          x => { log_error!("Invalid texture depth {}", x); gl::RGBA }
        };

        let data = image.data.clone();
        unsafe {
          check!(gl::TexImage2D
          (
            /* target */ gl::TEXTURE_2D, 
            /* mipmap */ 0, 
            /* internal */ gl::RGBA8 as GLint, 
            /* size */ tex.size.x as GLsizei, tex.size.y as GLsizei, 
            /* border */ 0, 
            /* external */ format, 
            /* size type */ gl::UNSIGNED_BYTE, 
            /* data */ cast::transmute(vec::raw::to_ptr(data))
          ));
        }
      }
      _ => log_fail!("Failed to load texture {}", tex.filename)
    }

    tex
  }

  pub fn bind(&self, _unit: GLenum)
  { check!(gl::BindTexture(gl::TEXTURE_2D, self.obj)); }

  pub fn unbind(&self)
  { check!(gl::BindTexture(gl::TEXTURE_2D, 0)); }
}

#[unsafe_destructor]
impl Drop for Texture
{
  fn drop(&mut self)
  { check_unsafe!(gl::DeleteTextures(1, &self.obj)); }
}

