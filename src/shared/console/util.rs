/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/console/util.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Utility functions for the in-game console.
*/

use std::ascii::*;

pub struct Util;
impl Util
{
  pub fn parse_bool(name: &str, val: &str) -> Result<bool, ~str>
  {
    match val.to_ascii_lower()
    {
      ~"true" => Ok(true),
      ~"on" => Ok(true),
      ~"yes" => Ok(true),

      ~"false" => Ok(false),
      ~"off" => Ok(false),
      ~"no" => Ok(false),

      _ => Err(fmt!("Invalid value for %s (bool)", name))
    }
  }

  pub fn parse_f32(name: &str, val: &str) -> Result<f32, ~str>
  {
    match from_str::<f32>(val)
    {
      Some(x) => Ok(x),
      None => Err(fmt!("Invalid value for %s (number)", name))
    }
  }
}

