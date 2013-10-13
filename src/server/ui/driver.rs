/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/ui/driver.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A trait that each UI driver will implement.
*/

pub trait Driver
{
  fn new() -> ~Self;
}

