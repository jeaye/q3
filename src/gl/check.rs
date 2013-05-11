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

//#[cfg(check_gl)]
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
      fail!(gl::get_err_str(err)); 
    }

    ret
  });
)

//#[cfg(no_check_gl)]
//macro_rules! check
//(
//  ($func:expr) => 
//  (
//    $func
//  );
//)

