/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/model/triangle.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A three-sided polygon that
      reference vertices stored
      elsewhere by index.
*/

struct Triangle
{
  indices: [i32, ..3],
}

impl Triangle
{
  pub fn new() -> Triangle
  {
    Triangle
    {
      indices: [0, ..3],
    }
  }
}

