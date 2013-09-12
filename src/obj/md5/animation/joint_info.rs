/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/animation/joint_info.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TODO
*/

struct Joint_Info
{
  name: ~str,
  parent_id: i32,
  flags: i32,
  start_index: i32,
}

impl Joint_Info
{
  pub fn new() -> Joint_Info
  {
    Joint_Info
    {
      name: ~"",
      parent_id: 0,
      flags: 0,
      start_index: 0,
    }
  }
}

