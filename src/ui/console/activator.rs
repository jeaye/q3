/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: ui/console/activator.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An input listener to open/close the console.
*/

use std::{ str, cast, local_data };
use glfw::{ PRESS, KEY_GRAVE_ACCENT, KEY_ENTER, KEY_BACKSPACE };
use ui::Input_Listener;
use super::Console;

pub struct Console_Activator
{
  console: @mut Console,
}

impl Console_Activator
{
  /*  Key function used to index our singleton in
      task-local storage. */
  priv fn tls_key(_: @@Console_Activator) { }

  pub fn new(new_console: @mut Console) -> @mut Console_Activator
  {
    let ca = @mut Console_Activator
    {
      console: new_console,
    };

    /* Store the activator in task-local storage. (singleton) */
    unsafe
    {
      local_data::local_data_set
      (
        Console_Activator::tls_key,
        @cast::transmute::<@mut Console_Activator, @Console_Activator>(ca)
      );
    }

    ca
  }

  /* Accesses the singleton activator from task-local storage. */
  pub fn get() -> @mut Console_Activator
  {
    unsafe 
    {
      cast::transmute::<@Console_Activator, @mut Console_Activator>
      (
        *local_data::local_data_get(Console_Activator::tls_key).get()
      )
    }
  }
}

impl Input_Listener for Console_Activator
{
  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  {
    if action == PRESS
    {
      if key == KEY_GRAVE_ACCENT
      {
        self.console.velocity *= -1.0;
        return true;
      }

      /* The following only apply if the console is enabled. */
      if self.console.velocity > 0.0
      {
        if key == KEY_ENTER
        {
          /* TODO: Have the console do something here. */
          self.console.body = copy self.console.input;
          self.console.input = ~"";
          return true;
        }
        else if key == KEY_BACKSPACE
        {
          if self.console.input.len() > 0
          { str::pop_char(&mut self.console.input); }
          return true;
        }
        /* Non-whitespace. */
        else if key >= 32 && key <= 93
        {
          /* This will be handled when we receive it as a char. */
          return true;
        }
      }
    }

    false
  }
  pub fn key_char(&mut self, ch: char) -> bool
  {
    /* Check if the console is enabled. */
    if self.console.velocity > 0.0
    {
      /* Non-whitespace and not ` or ~ */
      if ch >= 0x20 as char && ch <= 0x7D as char && ch != 0x60 as char
      {
        self.console.input.push_char(ch);
        return true;
      }
    }

    false
  }
  pub fn mouse_action(&mut self, _button: i32, _action: i32, _mods: i32) -> bool
  { false }
  pub fn mouse_moved(&mut self, _x: f32, _y: f32) -> bool
  { false }
}

