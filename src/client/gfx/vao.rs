/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/vao.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A wrapper of arbitrary OpenGL vertex array objects.
*/

use std::{ cast };
use gl, gl::types::*;
use log::Log;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

pub struct VAO
{
  priv name: GLuint,
  priv is_bound: bool,
}

impl VAO
{
  pub fn new() -> VAO
  {
    let mut name: GLuint = 0;
    check_unsafe!(gl::GenVertexArrays(1, &mut name));
    log_assert!(name > 0);

    VAO
    {
      name: name,
      is_bound: false
    }
  }

  pub fn zero() -> VAO
  {
    VAO
    {
      name: 0,
      is_bound: false
    }
  }

  pub fn bind(&self)
  {
    log_assert!(self.name != 0, "Invalid name for VAO");

    check!(gl::BindVertexArray(self.name));
    unsafe { cast::transmute_mut(self).is_bound = true; }
  }

  pub fn unbind(&self)
  {
    log_assert!(self.name != 0, "Invalid name for VAO");

    check!(gl::BindVertexArray(0));
    unsafe { cast::transmute_mut(self).is_bound = false; }
  }

  pub fn enable_vertex_attrib_array(&self, attrib: GLuint)
  {
    log_assert!(self.is_bound, "VAO is not bound");

    check!(gl::EnableVertexAttribArray(attrib));
  }

  pub fn vertex_attrib_pointer_f32(&self, index: GLuint, size: GLint,
                                   normalized: bool, stride: GLsizei, ptr: *GLvoid)
  {
    log_assert!(self.is_bound, "VAO is not bound");

    check_unsafe!(gl::VertexAttribPointer(index, size, gl::FLOAT, normalized as u8, stride, ptr));
  }
}

impl Drop for VAO
{
  fn drop(&mut self)
  {
    if(self.name != 0)
    { check_unsafe!(gl::DeleteVertexArrays(1, &self.name)); }
  }
}

