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

extern mod ui;

use std::os;

struct Server
{
  log: ~str,
  ui_driver: ~ui::Driver,
}

impl Server
{
  pub fn new() -> Server
  {
    let args = os::args();
    if args.contains(&~"--help")
    { Server::show_help(); }

    /* Determine the UI driver. */
    let driver =
    if args.contains(&~"--gui")
    { fail!("GUI mode is not yet implemented"); }
    else
    { ui::term::initialize().expect("Unable to create terminal UI") };

    let server = Server
    {
      log: ~"",
      ui_driver: driver,
    };

    server
  }

  fn show_help()
  {
    let args = os::args();

    println!("Q³ Server {}.{}", env!("VERSION"), env!("COMMIT"));
    println!("Usage:\n\t{} [options...]", args[0]);
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
  let _server = Server::new();
}

