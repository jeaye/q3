/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/main.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Main engine controller and entry point
      for the server.
*/

#[feature(globs)];
#[feature(macro_rules)];
#[feature(managed_boxes)];

extern mod log;
extern mod console;
extern mod ui;

use std::os;
use log::Log;

#[macro_escape]
#[path = "../shared/log/macros.rs"]
mod macros;

struct Server
{
  ui_driver: Option<~ui::Driver>,
}

impl Server
{
  pub fn new() -> Server
  {
    let server = Server
    {
      ui_driver: None,
    };
    
    server
  }

  pub fn initialize(&mut self)
  {
    /* Create the console. */
    let _console = console::Console::new();

    self.parse_cmd_line();
  }

  fn parse_cmd_line(&mut self)
  {
    let args = os::args();
    if args.contains(&~"--help")
    { Server::show_help(); }
    if args.contains(&~"--version")
    { Server::show_version(); }

    /* Determine the UI driver. */
    let driver =
    if args.contains(&~"--gui")
    { log_fail!("GUI mode is not yet implemented"); }
    else
    { ui::term::initialize() };
    log_assert!(driver.is_some(), "Unable to initialize UI");
    self.ui_driver = driver;
  }

  fn show_version()
  {
    println!("Q³ Server {}.{}", env!("VERSION"), env!("COMMIT"));
    fail!("Exiting");
  }

  fn show_help()
  {
    let args = os::args();

    println!("Q³ Server {}.{}", env!("VERSION"), env!("COMMIT"));
    println!("Usage:");
    println!("\t{} [options...]", args[0]);
    println!("");
    println!("Options:");
    println!("\t--help\tShows this help menu and exits");
    println!("\t--tui\tEnable textual user interface mode (default)");
    println!("\t--gui\tEnable graphical user interface mode");
    println!("");

    fail!("Exiting");
  }
}

fn main()
{
  log::Log::initialize(); 
  let mut server = Server::new();
  server.initialize();
}

