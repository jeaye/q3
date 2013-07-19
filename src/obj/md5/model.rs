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

use std::{ io, path };
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
  pub fn new(mesh_file: ~str) -> Model
  {
    let mut model = Model
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

    model.load(mesh_file);

    model
  }

  priv fn load(&mut self, file: ~str) -> bool
  {
    let fior = io::file_reader(&path::Path(file));
    if fior.is_err()
    { error!("Failed to open model file %s", file); return false; }

    /* Clear existing data. */
    self.joints.clear();
    self.meshes.clear();

    let fio = fior.get();
    let mut param = ~"";
    let read_param = ||
    {
      param = ~""; /* TODO: clear? */
      let mut ch = fio.read_char();
      while ch.is_whitespace() && !fio.eof() /* Find the next word. */
      { ch = fio.read_char(); }

      if !fio.eof()
      { 
        param.push_char(ch);
        ch = fio.read_char();
        while !ch.is_whitespace() && !fio.eof() /* Read the next word. */
        { param.push_char(ch); ch = fio.read_char(); }
      }
    };
    let read_i32 = |_val: &mut i32|
    {
      let name = param.clone(); /* TODO: Ouch -- lots of cloning. */
      read_param();
      let num = FromStr::from_str(param);
      if num.is_some()
      { *_val = num.get(); }
      else
      { error!("Invalid %s in %s", name, file); }
    };

    /* Read the first param and jump into the parsing. */
    read_param();
    while !fio.eof()
    {
      match param
      {
        ~"MD5Version" =>
        {
          /* Read version. */
          read_i32(&mut self.version);
          println(fmt!("Version: %?", self.version));
        }
        ~"commandline" =>
        { fio.read_line(); /* Ignore this line. */ }
        ~"numJoints" =>
        {
          read_i32(&mut self.num_joints);
          self.joints = vec::with_capacity(self.num_joints);
        }
        ~"numMeshes" =>
        {
          read_i32(&mut self.num_meshes);
          self.meshes = vec::with_capacity(self.num_meshes);
        }
        _ => { } 
      }

      /* Move to the next param. */
      read_param();
    }

    true
  }
}

