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
use gl = opengles::gl2;

mod util;

#[macro_escape]
mod check_internal;

struct Texture
{
  target: gl::GLenum,
  obj: gl::GLuint,
  filename: @str,
}

impl Texture
{
  #[inline(always)]
  pub fn new(targ: gl::GLenum, file: &str) -> Texture
  {
    let mut tex = Texture
    {
      target: targ,
      obj: check!(gl::gen_textures(1))[0],
      filename: file.to_managed(),
    };

    check!(gl::bind_texture(tex.target, tex.obj));

    match stb_image::image::load(file.to_owned())
    {
      stb_image::image::ImageU8(ref image) => 
      {
        debug!(fmt!("Loaded image %s with %?x%?:%?", 
                    tex.filename, image.width, image.height, image.depth));
        unsafe {
          check!(gl::tex_image_2d
          (
            gl::TEXTURE_2D, 0,
            gl::RGB as gl::GLint,
            image.width as gl::GLsizei,
            image.height as gl::GLsizei,
            0, gl::RGB, gl::UNSIGNED_BYTE,
            Some(cast::transmute((copy image.data, image.data.len())))
            //(cast::transmute(&image.data[0]))
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
    //check!(gl::active_texture(unit));
    check!(gl::bind_texture(gl::TEXTURE0 + unit, self.obj));
  }
}
 

