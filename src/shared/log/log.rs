/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/log/log.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A general logging collection
      with differen verbosity levels.
      Use Q3_LOG=num where num matches:

      0 => Disable all logging
      1 => Enable only error logging
      2 => Enable info logging (default)
      3 => Enable debug logging (very verbose)

      Usage of logging utilities is generally
      by means of the macros: log_debug!() and
      log_error!(). Logs can be owned/indented
      using a log_push!(), which must have an
      associated log_pop!(). Assertions and
      program halting can be accomplished with
      log_assert!() and log_fail!().
*/

use std::{ io, local_data, os };
use extra::term;
use extra::term::color;

/* Default is error. */
type Verbosity = u8;
static VERBOSITY_NONE: Verbosity = 0;
static VERBOSITY_ERROR: Verbosity = 1;
static VERBOSITY_INFO: Verbosity = 2;
static VERBOSITY_DEBUG: Verbosity = 3;

static tls_key: local_data::Key<@mut Log> = &local_data::Key;

#[macro_escape]
pub mod macros;

pub struct Log
{
  verbosity: Verbosity,
  push_level: u8, /* The indentation level, for nested logs. */
  terminal: term::Terminal,
}

impl Log
{
  pub fn initialize()
  {
    let logger = @mut Log
    {
      verbosity: match os::getenv("Q3_LOG").take()
                 {
                   Some(val) => match val
                   {
                     ~"0" => VERBOSITY_NONE,
                     ~"1" => VERBOSITY_ERROR,
                     ~"2" => VERBOSITY_INFO,
                     ~"3" => VERBOSITY_DEBUG,
                     _ => VERBOSITY_INFO, /* default */
                   },
                   None => VERBOSITY_INFO /* default */
                 },
      push_level: 0,
      terminal: term::Terminal::new(io::stdout()).unwrap(),
    };

    local_data::set(tls_key, logger);

    log_debug!("Logging system initialized");
  }

  fn get() -> @mut Log
  {
    local_data::get(tls_key, 
    |opt|
    {
      match opt
      {
        Some(x) => *x,
        None => log_fail!("Singleton not available")
      }
    })
  }

  pub fn debug(module: &str, message: &str)
  {
    let logger = Log::get();
    if logger.verbosity >= VERBOSITY_DEBUG
    { Log::log(module, message, VERBOSITY_DEBUG); }
  }

  pub fn info(module: &str, message: &str)
  {
    let logger = Log::get();
    if logger.verbosity >= VERBOSITY_INFO
    { Log::log(module, message, VERBOSITY_INFO); }
  }

  pub fn error(module: &str, message: &str)
  {
    let logger = Log::get();
    if logger.verbosity >= VERBOSITY_ERROR
    { Log::log(module, message, VERBOSITY_ERROR); }
  }

  pub fn get_module(file: &str) -> ~str
  {
    let i = file.find_str("src").unwrap();
    file.slice_from(i + 4).replace(".rs", "") /* 4 is strlen("src/") */
  }

  pub fn push()
  {
    let logger = Log::get();
    logger.push_level += 1;
  }

  pub fn pop()
  {
    let logger = Log::get();

    log_assert!(logger.push_level > 0);
    logger.push_level -= 1;
  }

  fn log(module: &str, message: &str, verbosity: Verbosity)
  {
    let logger = Log::get();

    /* Display the current module. */
    logger.terminal.fg(color::BRIGHT_WHITE);
    print(module); 
    logger.terminal.reset();

    /* Indent as per the push level. */
    for _ in range(0, logger.push_level)
    { print("  "); }

    match verbosity
    {
      VERBOSITY_DEBUG =>
      {
        logger.terminal.fg(color::BRIGHT_GREEN);
        print(" debug => ");
      },
      VERBOSITY_INFO =>
      {
        logger.terminal.fg(color::BRIGHT_YELLOW);
        print(" info => ");
      },
      VERBOSITY_ERROR =>
      {
        logger.terminal.fg(color::BRIGHT_RED);
        print(" error => ");
      },
      val => log_fail!("Invalid verbosity for logging: %d", val as int)
    }
    logger.terminal.reset();
    println(message);
  }
}

