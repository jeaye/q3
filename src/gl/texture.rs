/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/texture.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A wrapper of arbitrary OpenGL textures.
*/

extern mod std;
extern mod opengles;
extern mod stb_image;
use std::{ vec, cast };
use gl = opengles::gl2;
use math::Vec2i;

mod util;

#[macro_escape]
mod check_internal;

struct Texture
{
  target: gl::GLenum,
  obj: gl::GLuint,
  filename: @str,
  size: Vec2i,
}

impl Texture
{
  #[inline(always)]
  pub fn new(targ: gl::GLenum, file: &str) -> Texture
  {
    let mut tex = Texture
    {
      target: targ,
      obj: 0,
      filename: file.to_managed(),
      size: Vec2i::zero(),
    };

    let name = check!(gl::gen_textures(1));
    assert!(name.len() == 1);
    tex.obj = name[0];
    tex.bind(0);

    check!(gl::pixel_store_i(gl::UNPACK_ALIGNMENT, 4));
    check!(gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::GLint));
    check!(gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::GLint));
    check!(gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::GLint));
    check!(gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as gl::GLint));

    match stb_image::image::load(file.to_owned())
    {
      stb_image::image::ImageU8(ref image) => 
      {
        debug!(fmt!("Loaded image %s with %?x%?:%?", 
                    tex.filename, image.width, image.height, image.depth));

        tex.size = Vec2i::new(image.width as i32, image.height as i32);
        let format = match image.depth
        {
          3 => { gl::RGB },
          4 => { gl::RGBA },
          x => { error!(fmt!("Invalid texture depth %?", x)); gl::RGBA }
        };

        let data = copy image.data;
        unsafe {
          check!(gl::glTexImage2D
          (
            /* target */ gl::TEXTURE_2D, 
            /* mipmap */ 0, 
            /* internal */ gl::RGBA8 as gl::GLint, 
            /* size */ tex.size.x as gl::GLsizei, tex.size.y as gl::GLsizei, 
            /* border */ 0, 
            /* external */ format, 
            /* size type */ gl::UNSIGNED_BYTE, 
            /* data */ cast::transmute(vec::raw::to_ptr(data))
          ));
        }
      }
      _ => fail!(fmt!("Failed to load texture %s", tex.filename))
    }

    tex
  }

  #[inline(always)]
  pub fn bind(&self, unit: gl::GLenum)
  {
    check!(gl::active_texture(gl::TEXTURE0 + unit));
    check!(gl::bind_texture(gl::TEXTURE_2D, self.obj));
  }

  #[inline(always)]
  pub fn unbind(&self)
  { check!(gl::bind_texture(gl::TEXTURE_2D, 0)); }
}
 

