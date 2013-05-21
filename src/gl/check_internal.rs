/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/check_internal.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Provides a handy macro to check the outcome
      of an OpenGL call for errors -- used only
      within the gl module; use check.rs for the
      public version.
*/

macro_rules! check
(
  ($func:expr) => 
  ({
    //io::print(fmt!("%s -> ", stringify!($func)));
    let ret = $func;
    //io::println(fmt!("%?", ret));

    let err = gl::get_error();
    if err != gl::NO_ERROR
    {
      stringify!($func);
      fail!(util::get_err_str(err)); 
    }

    ret
  });
)

