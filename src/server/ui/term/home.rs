/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/ui/term/home.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The default/main terminal UI.
*/

use ncurses;
use super::view;

static NICKWIN_WIDTH: i32 = 20;
static STATUSWIN_HEIGHT: i32 = 5;

pub struct Home
{
  logwin: ncurses::WINDOW,
  statuswin: ncurses::WINDOW,
  nickwin: ncurses::WINDOW,

  screen_height: i32,
  screen_width: i32,
}

impl Home
{
  pub fn new() -> ~Home
  {
    ~Home
    {
      logwin: 0 as ncurses::WINDOW,
      statuswin: 0 as ncurses::WINDOW,
      nickwin: 0 as ncurses::WINDOW,

      screen_height: 0,
      screen_width: 0,
    }
  }

}

impl view::View for Home
{
  fn initialize(&mut self) -> bool
  {
    ncurses::getmaxyx(ncurses::stdscr, &mut self.screen_height, &mut self.screen_width);

    self.logwin = ncurses::newwin(self.screen_height - STATUSWIN_HEIGHT - 1, self.screen_width - NICKWIN_WIDTH, STATUSWIN_HEIGHT, 0);
    ncurses::mvwprintw(self.logwin, 1, 1, "[0:43:12] INFO => Something blah blah foo\n");
    ncurses::mvwprintw(self.logwin, 2, 1, "[0:43:12] INFO => This is a logging message\n");
    ncurses::mvwprintw(self.logwin, 3, 1, "[0:43:12] INFO => Very neat things are happening\n");
    ncurses::mvwprintw(self.logwin, 4, 1, "[0:43:12] ERROR => Look out!\n");
    ncurses::mvwprintw(self.logwin, 5, 1, "[0:43:12] INFO => Foo bar spam baz\n");
    ncurses::mvwprintw(self.logwin, 6, 1, "[0:43:12] INFO => This is a logging message\n");
    ncurses::mvwprintw(self.logwin, 7, 1, "[0:43:12] INFO => Very neat things are happening\n");
    ncurses::mvwprintw(self.logwin, 8, 1, "[0:43:12] INFO => This is a logging message\n");
    ncurses::mvwprintw(self.logwin, 9, 1, "[0:43:12] INFO => Foo bar spam baz\n");
    ncurses::mvwprintw(self.logwin, 0, 1, "[0:43:12] INFO => Very neat things are happening\n");
    ncurses::box(self.logwin, 0, 0);
    ncurses::wrefresh(self.logwin);

    self.statuswin = ncurses::newwin(STATUSWIN_HEIGHT, self.screen_width - NICKWIN_WIDTH, 0, 0);
    ncurses::box(self.statuswin, 0, 0);
    ncurses::wrefresh(self.statuswin);

    self.nickwin = ncurses::newwin(self.screen_height - 1, NICKWIN_WIDTH, 0, self.screen_width - NICKWIN_WIDTH);
    ncurses::mvwprintw(self.nickwin, 1, 1, "jeaye");
    ncurses::mvwprintw(self.nickwin, 2, 1, "lame");
    ncurses::mvwprintw(self.nickwin, 3, 1, "n1ck");
    ncurses::mvwprintw(self.nickwin, 4, 1, "goes");
    ncurses::mvwprintw(self.nickwin, 5, 1, "here117");
    ncurses::box(self.nickwin, 0, 0);
    ncurses::wrefresh(self.nickwin);

    ncurses::mvprintw(self.screen_height - 1, 0, "");

    ncurses::getch();

    true
  }

  fn shutdown(&mut self)
  {
  }
}

