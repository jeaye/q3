/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: ui/console/activator.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An input listener to open/close the console.
*/

use glfw::{ PRESS, KEY_GRAVE_ACCENT };
use ui::Input_Listener;
use super::Console;

pub struct Console_Activator
{
  console: @mut Console,
}

impl Console_Activator
{
  pub fn new(new_console: @mut Console) -> Console_Activator
  {
    Console_Activator
    {
      console: new_console,
    }
  }
}

impl Input_Listener for Console_Activator
{
  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  {
    if action == PRESS && key == KEY_GRAVE_ACCENT
    { io::println("Toggling console..."); return true; }

    false
  }
  pub fn key_char(&mut self, _ch: char) -> bool
  { false }
  pub fn mouse_action(&mut self, _button: i32, _action: i32, _mods: i32) -> bool
  { false }
  pub fn mouse_moved(&mut self, _x: f32, _y: f32) -> bool
  { false }
}

