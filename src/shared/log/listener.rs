/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/log/listener.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A trait to be implemented for objects that want
      to be notified about logging messages.
*/

use log::Verbosity;

/* These functions may be called by different threads
 * concurrently. */
pub trait Listener
{
  /* Returns whether or not the logger should output to stdout. */
  fn log(&mut self, module: &str, message: &str, verbosity: Verbosity) -> bool;
}

