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

extern mod ncurses;

use ncurses::*;

static NICKWIN_WIDTH: i32 = 20;
static STATUSWIN_HEIGHT: i32 = 5;

struct Server
{
  logwin: WINDOW,
  statuswin: WINDOW,
  nickwin: WINDOW,

  screen_height: i32,
  screen_width: i32,
}

impl Server
{
  pub fn new() -> Server
  {
    Server
    {
      logwin: 0 as WINDOW,
      statuswin: 0 as WINDOW,
      nickwin: 0 as WINDOW,

      screen_height: 0,
      screen_width: 0,
    }
  }

  pub fn initialize(&mut self)
  {
    initscr();
    raw();
    keypad(stdscr, true);
    echo();
    curs_set(CURSOR_VISIBLE);
    refresh();

    getmaxyx(stdscr, &mut self.screen_height, &mut self.screen_width);

    self.logwin = newwin(self.screen_height - STATUSWIN_HEIGHT - 1, self.screen_width - NICKWIN_WIDTH, STATUSWIN_HEIGHT, 0);
    mvwprintw(self.logwin, 1, 1, "[0:43:12] INFO => Something blah blah foo\n");
    mvwprintw(self.logwin, 2, 1, "[0:43:12] INFO => This is a logging message\n");
    mvwprintw(self.logwin, 3, 1, "[0:43:12] INFO => Very neat things are happening\n");
    mvwprintw(self.logwin, 4, 1, "[0:43:12] ERROR => Look out!\n");
    mvwprintw(self.logwin, 5, 1, "[0:43:12] INFO => Foo bar spam baz\n");
    mvwprintw(self.logwin, 6, 1, "[0:43:12] INFO => This is a logging message\n");
    mvwprintw(self.logwin, 7, 1, "[0:43:12] INFO => Very neat things are happening\n");
    mvwprintw(self.logwin, 8, 1, "[0:43:12] INFO => This is a logging message\n");
    mvwprintw(self.logwin, 9, 1, "[0:43:12] INFO => Foo bar spam baz\n");
    mvwprintw(self.logwin, 0, 1, "[0:43:12] INFO => Very neat things are happening\n");
    box(self.logwin, 0, 0);
    wrefresh(self.logwin);

    self.statuswin = newwin(STATUSWIN_HEIGHT, self.screen_width - NICKWIN_WIDTH, 0, 0);
    box(self.statuswin, 0, 0);
    wrefresh(self.statuswin);

    self.nickwin = newwin(self.screen_height - 1, NICKWIN_WIDTH, 0, self.screen_width - NICKWIN_WIDTH);
    mvwprintw(self.nickwin, 1, 1, "jeaye");
    mvwprintw(self.nickwin, 2, 1, "lame");
    mvwprintw(self.nickwin, 3, 1, "n1ck");
    mvwprintw(self.nickwin, 4, 1, "goes");
    mvwprintw(self.nickwin, 5, 1, "here117");
    box(self.nickwin, 0, 0);
    wrefresh(self.nickwin);

    mvprintw(self.screen_height - 1, 0, "");
  }
}

impl Drop for Server
{
  fn drop(&mut self)
  {
    refresh();
    endwin();
  }
}

fn main()
{
  let mut server = Server::new();
  server.initialize();

  while getch() != KEY_F(1)
  { }
}


