/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: util/log_macros.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Usage of logging utilities is generally
      by means of the macros: log_debug!() and
      log_error!().
*/

macro_rules! log_debug
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::debug(fmt!("[%s]:%ud", module, line!()), $message);
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::debug(fmt!("[%s]:%ud", module, line!()), fmt!($($message),+));
  });
)

macro_rules! log_info
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::info(fmt!("[%s]:%ud", module, line!()), $message);
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::info(fmt!("[%s]:%ud", module, line!()), fmt!($($message),+));
  });
)

macro_rules! log_error
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::error(fmt!("[%s]:%ud", module, line!()), $message);
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::error(fmt!("[%s]:%ud", module, line!()), fmt!($($message),+));
  });
)

macro_rules! log_push
(
  () => 
  ({
    Log::push();
  });
)

macro_rules! log_pop
(
  () => 
  ({
    Log::pop();
  });
)

