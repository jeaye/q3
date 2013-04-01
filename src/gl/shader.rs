/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/shader.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Abstracts loading, compiling, linking, and
      the setup of GLSL shaders.
*/

extern mod std;
extern mod opengles;
use gl = opengles::gl2;

mod util;
#[path = "../math/math.rs"]
mod math;

#[macro_escape]
mod check_internal;

struct Shader
{
  prog: gl::GLuint,
  vert_obj: gl::GLuint,
  frag_obj: gl::GLuint
}

impl Shader
{
  pub fn new(vert_src : &str, frag_src : &str) -> @mut Shader
  {
    let mut shader = @mut Shader{ prog: 0, vert_obj: 0, frag_obj: 0 };
    
    /* TODO: Error checking. */

    /* Create the shader program. */
    shader.prog = check!(gl::create_program());

    /* Compile the provided shaders. */
    if vert_src.len() > 0
    {
      shader.vert_obj = check!(gl::create_shader(gl::VERTEX_SHADER));
      
      let src = [vert_src];
      check!(gl::shader_source(shader.vert_obj, src.map(|x| str::to_bytes(*x))));
      check!(gl::compile_shader(shader.vert_obj));
    }
    if frag_src.len() > 0
    {
      shader.frag_obj = check!(gl::create_shader(gl::FRAGMENT_SHADER));
      
      let src = [frag_src];
      check!(gl::shader_source(shader.frag_obj, src.map(|x| str::to_bytes(*x))));
      check!(gl::compile_shader(shader.frag_obj));
    }

    /* Check if one of the shaders was properly compiled. */
    if shader.vert_obj > 0 
    { check!(gl::attach_shader(shader.prog, shader.vert_obj)); }
    if shader.frag_obj > 0
    { check!(gl::attach_shader(shader.prog, shader.frag_obj)); }

    check!(gl::link_program(shader.prog));
    check!(gl::validate_program(shader.prog));
      
    return shader;
  }

  pub fn bind(&self)
  { check!(gl::use_program(self.prog)); }

  pub fn get_uniform_location(&self, uniform: &str) -> gl::GLint
  { check!(gl::get_uniform_location(self.prog, uniform.to_owned())) }

  pub fn update_uniform(&self, location: gl::GLint, mat: &math::Mat4x4)
  { 
    unsafe
    {
      check!(gl::uniform_matrix_4fv(
                   location, 
                   false, 
                   cast::transmute::<[[f32, ..4], ..4], [f32, ..16]>(mat.data))) 
    }; 
  }
}
 
