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
use gl = opengles::gl2;

mod util;
#[path = "../math/math.rs"]
mod math;

#[macro_escape]
mod check;

/* TODO: Find a texture library. */
struct Texture
{
  target: gl::GLenum,
  obj: ~[gl::GLuint],
  filename: @str
}

impl Texture
{
  pub fn new(targ: gl::GLenum, file: &str) -> Texture
  {
    let tex = Texture{ target: targ, obj: check!(gl::gen_textures(1)), filename: file };

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

  pub fn bind(&self, unit: gl::GLenum)
  {
    check!(gl::active_texture(unit));
    check!(gl::bind_texture(self.obj));
  }
}
 
