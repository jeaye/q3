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
mod check;

struct Texture
{
  target: gl::GLenum,
  obj: ~[gl::GLuint],
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
      obj: check!(gl::gen_textures(1)),
      filename: file
    };

    check!(gl::bind_texture(tex.target, tes.obj));

    match stb_image::image::load_with_depth("resources/sample.png", 3)
    {
      Some(ref image) => 
      {
        unsafe {
          gl::tex_image_2d(
              gl::TEXTURE_2D, 0,
              gl::RGB as gl::GLint,
              image.width as gl::GLsizei,
              image.height as gl::GLsizei,
              0, gl::RGB, gl::UNSIGNED_BYTE,
              &cast::transmute(&image.data))
            );
        }
      }
      None => fail!(~"Failed to load texture.")
    }

    tex
  }

  #[inline(always)]
  pub fn bind(&self, unit: gl::GLenum)
  {
    check!(gl::active_texture(unit));
    check!(gl::bind_texture(self.obj));
  }
}
 

