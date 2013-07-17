/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/model.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Loads, parses, and represents
      the model (as in MVC) side of
      MD5 animated models.
*/

use super::{ Joint, Mesh };
use math;

struct Model
{
  version: i32,
  num_joints: i32,
  num_meshes: i32,
  is_animated: bool,

  joints: ~[Joint],
  meshes: ~[Mesh],

  //animation: Option<Animation>,
  
  local_to_world: math::Mat4x4,
}

impl Model
{
  pub fn new() -> Model
  {
    let model = Model
    {
      version: 0,
      num_joints: 0,
      num_meshes: 0,
      is_animated: false,

      joints: ~[],
      meshes: ~[],

      //animation: None,

      local_to_world: math::Mat4x4::new(),
    };

    model
  }
}

