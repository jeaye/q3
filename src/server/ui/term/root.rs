/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/ui/term/root.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Manager of low-level terminal UI.
*/

use ncurses;
use driver;
use super::view;
use super::home;

pub fn initialize() -> Option<~driver::Driver>
{
  let mut x: ~Term_Driver = driver::Driver::new();
  if x.initialize()
  { Some(x as ~driver::Driver) }
  else
  { None }
}

struct Term_Driver
{
  view: ~view::View,
}

impl Term_Driver
{
  pub fn initialize(&mut self) -> bool
  {
    macro_rules! init
    (($func:expr) => ({ if $func != ncurses::OK { return false; } });)

    /* Be cautious while initializing. */
    if ncurses::initscr() == 0 as ncurses::WINDOW
    { return false; }
    init!(ncurses::raw());
    init!(ncurses::keypad(ncurses::stdscr, true));
    init!(ncurses::echo());
    ncurses::curs_set(ncurses::CURSOR_VISIBLE);
    init!(ncurses::refresh());

    self.view.initialize()
  }
}

impl driver::Driver for Term_Driver
{
  fn new() -> ~Term_Driver
  {
    let td = ~Term_Driver
    {
      view: home::Home::new() as ~view::View,
    };

    td
  }
}

impl Drop for Term_Driver
{
  fn drop(&mut self)
  {
    ncurses::clear();
    ncurses::printw("Shutting down...");
    ncurses::getch();
    ncurses::endwin();
  }
}

