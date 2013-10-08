/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gl/shader.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Abstracts loading, compiling, linking, and
      the setup of GLSL shaders. For debug builds,
      the Debug_Shader is used, which allows shaders
      loaded from files to be dynamically reloaded if
      the file changes. Release_Shaders drop this
      functionality for performance.
*/

use std::{ str, io };
use gl2 = opengles::gl2;
use log::Log;
use math::*;

pub use Shader = self::Shaderable;

/* TODO: Type for uniform location that is GLint on release,
   but a lazily-updating custom type on debug that follows the changing
   of shaders during runtime. */

#[cfg(debug_shader)] /* TODO: Can I use one of these for multiple lines? { } */
pub use Shader_Builder = self::Debug_Shader;
#[cfg(debug_shader)]
use std::libc;

#[cfg(release_shader)]
pub use Shader_Builder = self::Release_Shader;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

pub trait Shaderable
{
  fn bind(&mut self);
  fn get_uniform_location(&self, uniform: &str) -> gl2::GLint;
  fn update_uniform_i32(&self, location: gl2::GLint, i: i32);
  fn update_uniform_f32(&self, location: gl2::GLint, i: f32);
  fn update_uniform_mat(&self, location: gl2::GLint, mat: &Mat4x4);
}

#[cfg(debug_shader)]
struct Debug_Shader
{
  prog: gl2::GLuint,
  vert_obj: gl2::GLuint,
  frag_obj: gl2::GLuint,
  vert_file: ~str,
  frag_file: ~str,
  vert_file_time: libc::time_t,
  frag_file_time: libc::time_t,
  valid: bool, /* Whether or not the last compilation succeeded. */
}

#[cfg(debug_shader)]
impl Debug_Shader
{
  pub fn new(vert_src : &str, frag_src : &str) -> @mut Shaderable
  {
    let shader = @mut Debug_Shader
    {
      prog: 0,
      vert_obj: 0,
      frag_obj: 0,
      vert_file: ~"",
      frag_file: ~"",
      vert_file_time: 0,
      frag_file_time: 0,
      valid: false,
    };

    log_assert!(shared::load(shader, vert_src, frag_src));
    shader.valid = true;

    shader as @mut Shaderable
  }

  pub fn new_with_files(new_vert_file : &str, new_frag_file : &str) -> @mut Shaderable
  {
    let shader = @mut Debug_Shader
    {
      prog: 0,
      vert_obj: 0,
      frag_obj: 0,
      vert_file: new_vert_file.to_owned(),
      frag_file: new_frag_file.to_owned(),
      vert_file_time: 0,
      frag_file_time: 0,
      valid: false,
    };
    shader.vert_file_time = match Path(new_vert_file).stat()
    {
      Some(ref st) => st.st_mtime,
      None => 0
    };
    shader.frag_file_time = match Path(new_frag_file).stat()
    {
      Some(ref st) => st.st_mtime,
      None => 0
    };

    let fio = io::file_reader(&Path(new_vert_file)).unwrap();
    let vert_src = str::from_utf8(fio.read_whole_stream());

    let fio = io::file_reader(&Path(new_frag_file)).unwrap();
    let frag_src = str::from_utf8(fio.read_whole_stream());

    log_assert!(shared::load(shader, vert_src, frag_src));
    shader.valid = true;

    shader as @mut Shaderable
  }
}

#[cfg(debug_shader)]
impl Shader for Debug_Shader
{
  fn bind(&mut self)
  {
    /* Get the time stamp on the files. */
    let vert_time = match Path(self.vert_file).stat()
    {
      Some(ref st) => st.st_mtime,
      None => 0
    };
    let frag_time = match Path(self.frag_file).stat()
    {
      Some(ref st) => st.st_mtime,
      None => 0
    };

    /* Check if the files are newer than before. */
    if vert_time > self.vert_file_time || frag_time > self.frag_file_time
    {
      let fio = io::file_reader(&Path(self.vert_file)).unwrap();
      let vert_src = str::from_utf8(fio.read_whole_stream());

      let fio = io::file_reader(&Path(self.frag_file)).unwrap();
      let frag_src = str::from_utf8(fio.read_whole_stream());

      self.valid = shared::load(self, vert_src, frag_src);

      self.vert_file_time = vert_time;
      self.frag_file_time = frag_time;
    }

    if self.valid
    { shared::bind(self); }
  }

  fn get_uniform_location(&self, uniform: &str) -> gl2::GLint
  { if self.valid { return shared::get_uniform_location(self, uniform); } -1 }

  fn update_uniform_i32(&self, location: gl2::GLint, i: i32)
  { if self.valid { shared::update_uniform_i32(location, i); } }

  fn update_uniform_f32(&self, location: gl2::GLint, i: f32)
  { if self.valid { shared::update_uniform_f32(location, i); } }

  fn update_uniform_mat(&self, location: gl2::GLint, mat: &Mat4x4)
  { if self.valid { shared::update_uniform_mat(location, mat); } }
}

#[cfg(debug_shader)]
impl Drop for Debug_Shader
{
  fn drop(&mut self)
  {
    check!(gl2::delete_shader(self.vert_obj));
    check!(gl2::delete_shader(self.frag_obj));
    check!(gl2::delete_program(self.prog));
  }
}
 
#[cfg(release_shader)]
struct Release_Shader
{
  prog: gl2::GLuint,
  vert_obj: gl2::GLuint,
  frag_obj: gl2::GLuint,
}

#[cfg(release_shader)]
impl Release_Shader
{
  pub fn new(vert_src : &str, frag_src : &str) -> @mut Shaderable
  {
    let shader = @mut Release_Shader{ prog: 0, vert_obj: 0, frag_obj: 0 };

    log_assert!(shared::load(shader, vert_src, frag_src));

    shader as @mut Shaderable
  }

  pub fn new_with_files(vert_file : &str, frag_file : &str) -> @mut Shaderable
  {
    let shader = @mut Release_Shader{ prog: 0, vert_obj: 0, frag_obj: 0 };

    let fio = io::file_reader(&Path(vert_file)).unwrap();
    let vert_src = str::from_utf8(fio.read_whole_stream());

    let fio = io::file_reader(&Path(frag_file)).unwrap();
    let frag_src = str::from_utf8(fio.read_whole_stream());

    log_assert!(shared::load(shader, vert_src, frag_src));

    shader as @mut Shaderable
  }
}

#[cfg(release_shader)]
impl Shader for Release_Shader
{
  fn bind(&mut self)
  { shared::bind(self); }

  fn get_uniform_location(&self, uniform: &str) -> gl2::GLint
  { shared::get_uniform_location(self, uniform) }

  fn update_uniform_i32(&self, location: gl2::GLint, i: i32)
  { shared::update_uniform_i32(location, i) }

  fn update_uniform_f32(&self, location: gl2::GLint, i: f32)
  { shared::update_uniform_f32(location, i); }

  fn update_uniform_mat(&self, location: gl2::GLint, mat: &Mat4x4)
  { shared::update_uniform_mat(location, mat) }
}

#[cfg(release_shader)]
impl Drop for Release_Shader
{
  fn drop(&mut self)
  {
    check!(gl2::delete_shader(self.vert_obj));
    check!(gl2::delete_shader(self.frag_obj));
    check!(gl2::delete_program(self.prog));
  }
}

pub mod shared
{
  use gl2 = opengles::gl2;
  use std::cast;
  use log::Log;
  use math::*;

  #[macro_escape]
  #[path = "../check.rs"]
  mod check;

  #[macro_escape]
  #[path = "../../../shared/log/macros.rs"]
  mod macros;

  pub fn load(shader: &mut super::Shader_Builder, vert_src: &str, frag_src: &str) -> bool
  {
    if check!(gl2::is_program(shader.prog))
    { check!(gl2::delete_program(shader.prog)); }

    shader.prog = check!(gl2::create_program());

    let compile_check = |obj| -> bool
    {
      /* Error check. */
      let result = check!(gl2::get_shader_iv(obj, gl2::COMPILE_STATUS));
      if result == 0 as gl2::GLint
      {
        let err = check!(gl2::get_shader_info_log(obj));
        log_error!(err);
      }
      result != 0
    };

    /* Compile the provided shaders. */
    if vert_src.len() > 0
    {
      shader.vert_obj = check!(gl2::create_shader(gl2::VERTEX_SHADER));
      log_assert!(shader.vert_obj != 0);

      let src = [vert_src];
      check!(gl2::shader_source(shader.vert_obj, src.map(|x| (*x).as_bytes().to_owned())));
      check!(gl2::compile_shader(shader.vert_obj));

      /* Error checking. */
      if !compile_check(shader.vert_obj)
      { check!(gl2::delete_shader(shader.vert_obj)); return false; }
    }
    if frag_src.len() > 0
    {
      shader.frag_obj = check!(gl2::create_shader(gl2::FRAGMENT_SHADER));
      log_assert!(shader.frag_obj != 0);

      let src = [frag_src];
      check!(gl2::shader_source(shader.frag_obj, src.map(|x| (*x).as_bytes().to_owned())));
      check!(gl2::compile_shader(shader.frag_obj));

      /* Error checking. */
      if !compile_check(shader.frag_obj)
      { check!(gl2::delete_shader(shader.frag_obj)); return false; }
    }

    /* Check if one of the shaders was properly compiled. */
    if shader.vert_obj > 0 
    { check!(gl2::attach_shader(shader.prog, shader.vert_obj)); }
    if shader.frag_obj > 0
    { check!(gl2::attach_shader(shader.prog, shader.frag_obj)); }

    check!(gl2::link_program(shader.prog));

    /* Error check. */
    let result = check!(gl2::get_program_iv(shader.prog, gl2::LINK_STATUS));
    if result == 0 as gl2::GLint
    {
      let err = check!(gl2::get_program_info_log(shader.prog));
      log_error!(err);

      /* Delete shaders. */
      check!(gl2::detach_shader(shader.prog, shader.vert_obj));
      check!(gl2::delete_shader(shader.vert_obj));

      check!(gl2::detach_shader(shader.prog, shader.frag_obj));
      check!(gl2::delete_shader(shader.frag_obj));

      check!(gl2::delete_program(shader.prog));

      return false;
    }

    true
  }

  pub fn bind(shader: &mut super::Shader_Builder)
  { check!(gl2::use_program(shader.prog)); }

  pub fn get_uniform_location(shader: &super::Shader_Builder, uniform: &str) -> gl2::GLint
  {
    let name = check!(gl2::get_uniform_location(shader.prog, uniform.to_owned()));
    match name
    {
      -1 => { log_error!("Uniform '%s' not found!", uniform); name }
      _ => { name }
    }
  }

  pub fn update_uniform_i32(location: gl2::GLint, i: i32)
  { check!(gl2::uniform_1i(location, i)); }

  pub fn update_uniform_f32(location: gl2::GLint, i: f32)
  { check!(gl2::uniform_1f(location, i)); }

  pub fn update_uniform_mat(location: gl2::GLint, mat: &Mat4x4)
  { 
    unsafe
    {
      check!(gl2::uniform_matrix_4fv(
                   location, 
                   false, 
                   cast::transmute::<[[f32, ..4], ..4], [f32, ..16]>(mat.data))) 
    }; 
  }
}

