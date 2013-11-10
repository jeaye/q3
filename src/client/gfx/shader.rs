/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/shader.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Abstracts loading, compiling, linking, and
      the setup of GLSL shaders. For debug builds,
      the Debug_Shader is used, which allows shaders
      loaded from files to be dynamically reloaded if
      the file changes. Release_Shaders drop this
      functionality for performance.
*/

use std::str;
use std::rt::io::Reader;
use std::rt::io::File;
use gl;
use log::Log;
use math::*;

pub use Shader = self::Shaderable;

/* TODO: Type for uniform location that is GLint on release,
   but a lazily-updating custom type on debug that follows the changing
   of shaders during runtime. */

#[cfg(debug_shader)] /* TODO: Can I use one of these for multiple lines? { } */
pub use Shader_Builder = self::Debug_Shader;

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
  fn get_uniform_location(&self, uniform: &str) -> i32;
  fn update_uniform_i32(&self, location: i32, i: i32);
  fn update_uniform_f32(&self, location: i32, i: f32);
  fn update_uniform_mat(&self, location: i32, mat: &Mat4x4);
}

#[cfg(debug_shader)]
struct Debug_Shader
{
  prog: u32,
  vert_obj: u32,
  frag_obj: u32,
  vert_file: ~str,
  frag_file: ~str,
  vert_file_time: u64,
  frag_file_time: u64,
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
    shader.vert_file_time = Path::new(new_vert_file).stat().modified;
    shader.frag_file_time = Path::new(new_frag_file).stat().modified;

    let mut fio = File::open(&Path::new(new_vert_file)).expect("Unable to open reader");
    let vert_src = str::from_utf8(fio.read_to_end());

    let mut fio = File::open(&Path::new(new_frag_file)).expect("Unable to open reader");
    let frag_src = str::from_utf8(fio.read_to_end());

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
    let vert_time = Path::new(self.vert_file.clone()).stat().modified;
    let frag_time = Path::new(self.frag_file.clone()).stat().modified;

    /* Check if the files are newer than before. */
    if vert_time > self.vert_file_time || frag_time > self.frag_file_time
    {
      let mut fio = File::open(&Path::new(self.vert_file.clone())).expect("Unable to open reader");
      let vert_src = str::from_utf8(fio.read_to_end());

      let mut fio = File::open(&Path::new(self.frag_file.clone())).expect("Unable to open reader");
      let frag_src = str::from_utf8(fio.read_to_end());

      self.valid = shared::load(self, vert_src, frag_src);

      self.vert_file_time = vert_time;
      self.frag_file_time = frag_time;
    }

    if self.valid
    { shared::bind(self); }
  }

  fn get_uniform_location(&self, uniform: &str) -> i32
  { if self.valid { return shared::get_uniform_location(self, uniform); } -1 }

  fn update_uniform_i32(&self, location: i32, i: i32)
  { if self.valid { shared::update_uniform_i32(location, i); } }

  fn update_uniform_f32(&self, location: i32, i: f32)
  { if self.valid { shared::update_uniform_f32(location, i); } }

  fn update_uniform_mat(&self, location: i32, mat: &Mat4x4)
  { if self.valid { shared::update_uniform_mat(location, mat); } }
}

#[cfg(debug_shader)]
impl Drop for Debug_Shader
{
  fn drop(&mut self)
  {
    check!(gl::DeleteShader(self.vert_obj));
    check!(gl::DeleteShader(self.frag_obj));
    check!(gl::DeleteProgram(self.prog));
  }
}
 
#[cfg(release_shader)]
struct Release_Shader
{
  prog: u32,
  vert_obj: u32,
  frag_obj: u32,
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

    let mut fio = File::open(&Path::new(vert_file)).expect("Unable to open reader");
    let vert_src = str::from_utf8(fio.read_to_end());

    let mut fio = File::open(&Path::new(frag_file)).expect("Unable to open reader");
    let frag_src = str::from_utf8(fio.read_to_end());

    log_assert!(shared::load(shader, vert_src, frag_src));

    shader as @mut Shaderable
  }
}

#[cfg(release_shader)]
impl Shader for Release_Shader
{
  fn bind(&mut self)
  { shared::bind(self); }

  fn get_uniform_location(&self, uniform: &str) -> i32
  { shared::get_uniform_location(self, uniform) }

  fn update_uniform_i32(&self, location: i32, i: i32)
  { shared::update_uniform_i32(location, i) }

  fn update_uniform_f32(&self, location: i32, i: f32)
  { shared::update_uniform_f32(location, i); }

  fn update_uniform_mat(&self, location: i32, mat: &Mat4x4)
  { shared::update_uniform_mat(location, mat) }
}

#[cfg(release_shader)]
impl Drop for Release_Shader
{
  fn drop(&mut self)
  {
    check!(gl::DeleteShader(self.vert_obj));
    check!(gl::DeleteShader(self.frag_obj));
    check!(gl::DeleteProgram(self.prog));
  }
}

pub mod shared
{
  use gl;
  use gl::types::*;
  use std::{ str, vec, ptr, cast };
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
    if check!(gl::IsProgram(shader.prog)) != 0
    { check!(gl::DeleteProgram(shader.prog)); }

    shader.prog = check!(gl::CreateProgram());

    let compile_check = |obj| -> bool
    {
      /* Error check. */
      let mut result: GLint = 0;
      check_unsafe!(gl::GetShaderiv(obj, gl::COMPILE_STATUS, &mut result));
      if result == 0
      {
        let mut len = 0;
        check_unsafe!(gl::GetShaderiv(obj, gl::INFO_LOG_LENGTH, &mut len));

        /* Subtract one to ignore the trailing null. */
        let mut buf = vec::from_elem(len as uint - 1, 0u8); 
        check_unsafe!(gl::GetShaderInfoLog(obj, len, ptr::mut_null(),
                                           vec::raw::to_mut_ptr(buf) as *mut GLchar));
        log_error!(unsafe { str::raw::from_utf8(buf) });
      }
      result != 0
    };

    /* Compile the provided shaders. */
    if vert_src.len() > 0
    {
      shader.vert_obj = check!(gl::CreateShader(gl::VERTEX_SHADER));
      log_assert!(shader.vert_obj != 0);

      vert_src.with_c_str(|ptr| check_unsafe!(gl::ShaderSource(shader.vert_obj, 1, &ptr, ptr::null())));
      check!(gl::CompileShader(shader.vert_obj));

      /* Error checking. */
      if !compile_check(shader.vert_obj)
      { check!(gl::DeleteShader(shader.vert_obj)); return false; }
    }
    if frag_src.len() > 0
    {
      shader.frag_obj = check!(gl::CreateShader(gl::FRAGMENT_SHADER));
      log_assert!(shader.frag_obj != 0);

      frag_src.with_c_str(|ptr| check_unsafe!(gl::ShaderSource(shader.frag_obj, 1, &ptr, ptr::null())));
      check!(gl::CompileShader(shader.frag_obj));

      /* Error checking. */
      if !compile_check(shader.frag_obj)
      { check!(gl::DeleteShader(shader.frag_obj)); return false; }
    }

    /* Check if one of the shaders was properly compiled. */
    if shader.vert_obj > 0 
    { check!(gl::AttachShader(shader.prog, shader.vert_obj)); }
    if shader.frag_obj > 0
    { check!(gl::AttachShader(shader.prog, shader.frag_obj)); }

    check!(gl::LinkProgram(shader.prog));

    /* Error check. */
    let mut result: GLint = 0;
    check_unsafe!(gl::GetProgramiv(shader.prog, gl::LINK_STATUS, &mut result));
    if result == 0
    {
      let mut len = 0;
      check_unsafe!(gl::GetShaderiv(shader.prog, gl::INFO_LOG_LENGTH, &mut len));

      /* Subtract one to ignore the trailing null. */
      let mut buf = vec::from_elem(len as uint - 1, 0u8); 
      check_unsafe!(gl::GetShaderInfoLog(shader.prog, len, ptr::mut_null(),
      vec::raw::to_mut_ptr(buf) as *mut GLchar));
      log_error!(unsafe { str::raw::from_utf8(buf) });

      /* Delete shaders. */
      check!(gl::DetachShader(shader.prog, shader.vert_obj));
      check!(gl::DeleteShader(shader.vert_obj));

      check!(gl::DetachShader(shader.prog, shader.frag_obj));
      check!(gl::DeleteShader(shader.frag_obj));

      check!(gl::DeleteProgram(shader.prog));

      return false;
    }

    true
  }

  pub fn bind(shader: &mut super::Shader_Builder)
  { check!(gl::UseProgram(shader.prog)); }

  pub fn get_uniform_location(shader: &super::Shader_Builder, uniform: &str) -> i32
  {
    let name = uniform.with_c_str(|ptr| check_unsafe!(gl::GetUniformLocation(shader.prog, ptr)));
    match name
    {
      -1 => { log_error!("Uniform '{}' not found!", uniform); name }
      _ => { name }
    }
  }

  pub fn update_uniform_i32(location: i32, i: i32)
  { check!(gl::Uniform1i(location, i)); }

  pub fn update_uniform_f32(location: i32, i: f32)
  { check!(gl::Uniform1f(location, i)); }

  pub fn update_uniform_mat(location: i32, mat: &Mat4x4)
  { 
    check_unsafe!(gl::UniformMatrix4fv(
                 location, 
                 1,
                 0u8, 
                 cast::transmute(vec::raw::to_ptr(mat.data))))
  }
}

