/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/ui/term/view.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      UI view trait.
*/

pub trait View
{
  fn initialize(&mut self) -> bool;
  fn shutdown(&mut self);
}

