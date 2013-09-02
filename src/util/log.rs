/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: util/log.rs
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
      associated log_pop!().
*/

use std::{ io, local_data, os };
use extra::term;
use extra::term::color;

/* Default is error. */
type Verbosity = u8;
static Verbosity_None: Verbosity = 0;
static Verbosity_Error: Verbosity = 1;
static Verbosity_Info: Verbosity = 2;
static Verbosity_Debug: Verbosity = 3;

static tls_key: local_data::Key<@mut Log> = &local_data::Key;

#[macro_escape]
mod log_macros;

struct Log
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
                     ~"0" => Verbosity_None,
                     ~"1" => Verbosity_Error,
                     ~"2" => Verbosity_Info,
                     ~"3" => Verbosity_Debug,
                     _ => Verbosity_Info, /* default */
                   },
                   None => Verbosity_Info /* default */
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
        None => fail!("Singleton not available")
      }
    })
  }

  pub fn debug(module: &str, message: &str)
  {
    let logger = Log::get();
    if logger.verbosity >= Verbosity_Debug
    { Log::log(module, message, Verbosity_Debug); }
  }

  pub fn info(module: &str, message: &str)
  {
    let logger = Log::get();
    if logger.verbosity >= Verbosity_Info
    { Log::log(module, message, Verbosity_Info); }
  }

  pub fn error(module: &str, message: &str)
  {
    let logger = Log::get();
    if logger.verbosity >= Verbosity_Error
    { Log::log(module, message, Verbosity_Error); }
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

    assert!(logger.push_level > 0);
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
      Verbosity_Debug =>
      {
        logger.terminal.fg(color::BRIGHT_GREEN);
        print(" debug => ");
      },
      Verbosity_Info =>
      {
        logger.terminal.fg(color::BRIGHT_YELLOW);
        print(" info => ");
      },
      Verbosity_Error =>
      {
        logger.terminal.fg(color::BRIGHT_RED);
        print(" error => ");
      },
      val => fail!("Invalid verbosity for logging: {}", val)
    }
    logger.terminal.reset();
    println(message);
  }
}

