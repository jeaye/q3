/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: main.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Entry point.
*/

extern mod extra;
extern mod opengles;
extern mod glfw;
extern mod stb_image;

/* Q³ */
extern mod log;
extern mod console;
extern mod math;
extern mod gl;
extern mod ui;
extern mod obj;
extern mod state;

use std::rt;
use std::unstable::finally::Finally;

pub mod client;

#[start]
fn main(argc: int, argv: **u8) -> int
{
  do rt::start_on_main_thread(argc, argv)
  {
    log::Log::initialize(); 

    (||
    {
      let client = client::Client::new();
      client.run();
    }).finally(glfw::terminate);
  }
}


