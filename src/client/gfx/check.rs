/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/check.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Provides a handy macro to check the outcome
      of an OpenGL call for errors -- use it everywhere.
*/

#[cfg(check_gl)]
#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

#[cfg(check_gl)]
pub fn get_err_str(err: u32) -> ~str
{
  use gl2 = opengles::gl2;

  match err
  {
    gl2::INVALID_ENUM => { ~"Invalid enum" },
    gl2::INVALID_VALUE => { ~"Invalid value" },
    gl2::INVALID_OPERATION => { ~"Invalid operation" },
    gl2::INVALID_FRAMEBUFFER_OPERATION => { ~"Invalid frame buffer operation" },
    gl2::OUT_OF_MEMORY => { ~"Out of memory" },
    gl2::STACK_UNDERFLOW => { ~"Stack underflow" },
    gl2::STACK_OVERFLOW => { ~"Stack overflow" },
    _ => { ~"Unknown error" }
  }
}

#[cfg(check_gl)]
pub fn check_gl(func: &str)
{
  use gl2 = opengles::gl2;
  use log::Log;

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

