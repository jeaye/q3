/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/check_internal.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Provides a handy macro to check the outcome
      of an OpenGL call for errors -- use it everywhere.
*/

#[cfg(check_gl)]
#[macro_escape]
#[path = "../log/macros.rs"]
mod macros;

#[cfg(check_gl)]
pub fn check_gl(func: &str)
{
  use gl2 = opengles::gl2;
  use log::Log;

  let err = gl2::get_error();
  if err != gl2::NO_ERROR
  {
    log_error!(func);
    log_fail!(util::get_err_str(err)); 
  }
}

#[cfg(not(check_gl))]
pub fn check_gl(_func: &str)
{ }

macro_rules! check
(
  ($func:expr) => 
  ({
    let ret = $func;

    check::check_gl(stringify!($func));

    ret
  });
)

macro_rules! check_unsafe
(
  ($func:expr) => 
  ({
    unsafe { check!($func) }
  });
)
