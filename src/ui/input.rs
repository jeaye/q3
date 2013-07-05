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
  /*  Returns true when the event has been captured. If the event is not
      captured, it's set to the next lower state. Rinse and repeat. */
  pub fn key_action(&mut self, key: i32, action: i32, mods: i32) -> bool;
  pub fn key_char(&mut self, ch: char) -> bool;
  pub fn mouse_action(&mut self, button: i32, action: i32, mods: i32) -> bool;
  pub fn mouse_moved(&mut self, x: f32, y: f32) -> bool;
}

