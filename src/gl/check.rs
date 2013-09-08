/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/check.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Provides a handy macro to check the outcome
      of an OpenGL call for errors -- use it everywhere.
*/

#[cfg(check_gl)]
use util::Log;

#[cfg(check_gl)]
#[macro_escape]
#[path = "../util/log_macros.rs"]
mod log_macros;

#[cfg(check_gl)]
pub fn check_gl(func: &str)
{
  use gl::get_err_str;
  use gl2 = opengles::gl2;

  let err = gl2::get_error();
  if err != gl2::NO_ERROR
  {
    log_error!(func);
    log_fail!(get_err_str(err)); 
  }
}

#[cfg(not(check_gl))]
pub fn check_gl(_func: &str)
{ }

macro_rules! check
(
  ($func:expr) => 
  ({
    //log_debug!("%s -> ", stringify!($func));
    let ret = $func;
    //log_debug!("%?", ret);

    check::check_gl(stringify!($func));

    ret
  });
)

