/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/vbo.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A wrapper of arbitrary OpenGL vertex buffer objects.
*/

use std::{ vec, mem, cast };
use gl, gl::types::*;
use log::Log;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

pub struct VBO
{
  priv name: GLuint,
  priv target: GLenum,
  priv is_bound: bool,
}

impl VBO
{
  pub fn new(target: GLenum) -> VBO
  {
    let mut name: GLuint = 0;
    check_unsafe!(gl::GenBuffers(1, &mut name));
    log_assert!(name > 0);

    VBO
    {
      name: name,
      target: target,
      is_bound: false,
    }
  }

  pub fn zero() -> VBO
  {
    VBO
    {
      name: 0,
      target: 0,
      is_bound: false,
    }
  }

  pub fn bind(&self)
  {
    log_assert!(self.target != 0 && self.name != 0);

    check!(gl::BindBuffer(self.target, self.name));
    unsafe { cast::transmute_mut(self).is_bound = true; }
  }

  pub fn unbind(&self)
  {
    log_assert!(self.target != 0 && self.name != 0);
    log_assert!(self.is_bound, "VBO is not bound");

    check!(gl::BindBuffer(self.target, 0));
    unsafe { cast::transmute_mut(self).is_bound = false; }
  }

  pub fn buffer_data<T>(&mut self, data: &[T], usage: GLenum)
  {
    log_assert!(self.is_bound, "VBO is not bound");

    check_unsafe!(gl::BufferData(self.target, (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                                 vec::raw::to_ptr(data) as *GLvoid, usage));
  }

  pub fn buffer_data_ptr(&mut self, size: GLsizeiptr, data: *GLvoid, usage: GLenum)
  {
    log_assert!(self.is_bound, "VBO is not bound");

    unsafe { check!(gl::BufferData(self.target, size, data, usage)); }
  }

  pub fn buffer_sub_data<T>(&mut self, offset: GLintptr, data: &[T])
  {
    log_assert!(self.is_bound, "VBO is not bound");

    let size = mem::size_of::<T>() as GLintptr;
    check_unsafe!(gl::BufferSubData(self.target, (offset * size),
                                    (data.len() as GLintptr * size),
                                    vec::raw::to_ptr(data) as *GLvoid));
  }

  pub fn buffer_sub_data_ptr(&mut self, offset: GLintptr, size: GLsizeiptr, data: *GLvoid)
  {
    log_assert!(self.is_bound, "VBO is not bound");

    check_unsafe!(gl::BufferSubData(self.target, offset, size, data));
  }
}

impl Drop for VBO
{
  fn drop(&mut self)
  {
    if(self.name != 0)
    { check_unsafe!(gl::DeleteBuffers(1, &self.name)); }
  }
}
