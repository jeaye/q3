/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/util.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Misc math utils.
*/

pub fn next_power_of_2(num: i32) -> i32
{
  /* Magic. */
  let mut ret = num - 1;
  ret |= ret >> 1;
  ret |= ret >> 2;
  ret |= ret >> 4;
  ret |= ret >> 8;
  ret |= ret >> 16;
  ret + 1
}

