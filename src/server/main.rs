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

struct Server
{
  log: ~str,
  ui_driver: ~ui::Driver,
}

impl Server
{
  pub fn new() -> Server
  {
    let server = Server
    {
      log: ~"",
      ui_driver: ui::term::initialize().expect("Unable to create UI"),
    };

    server
  }
}

fn main()
{
  let _server = Server::new();
}

