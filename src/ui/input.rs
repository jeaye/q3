/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: ui/input.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The stack-based input state.
*/

pub trait Input_Listener
{
  /* Return true when the event has been captured. */
  pub fn key_action(&mut self, key: i32, action: i32, mods: i32) -> bool;
  pub fn key_char(&mut self, ch: char) -> bool;
  pub fn mouse_action(&mut self, button: i32, action: i32, mods: i32) -> bool;
  pub fn mouse_moved(&mut self, x: f32, y: f32) -> bool;
}

pub struct Input_State
{
  listeners: ~[@mut Input_Listener],
}

impl Input_State
{
  pub fn new() -> Input_State
  {
    Input_State
    {
      listeners: ~[],
    }
  }

  pub fn push(&mut self, list: @mut Input_Listener)
  { self.listeners.unshift(list); }

  pub fn pop(&mut self)
  { self.listeners.shift(); }

  pub fn key_action(&mut self, key: i32, action: i32, mods: i32)
  {
    assert!(self.listeners.len() > 0);

    for self.listeners.mut_iter().advance |x|
    {
      if x.key_action(key, action, mods)
      { break; }
    }
  }

  pub fn key_char(&mut self, ch: char)
  {
    assert!(self.listeners.len() > 0);

    for self.listeners.mut_iter().advance |x|
    {
      if x.key_char(ch)
      { break; }
    }
  }

  pub fn mouse_action(&mut self, button: i32, action: i32, mods: i32)
  {
    assert!(self.listeners.len() > 0);

    for self.listeners.mut_iter().advance |x|
    {
      if x.mouse_action(button, action, mods)
      { break; }
    }
  }

  pub fn mouse_moved(&mut self, x: f32, y: f32)
  {
    assert!(self.listeners.len() > 0);

    for self.listeners.mut_iter().advance |list|
    {
      if list.mouse_moved(x, y)
      { break; }
    }
  }
}

