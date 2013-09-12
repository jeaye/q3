/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/md5/animation/animation.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      TODO
*/

use std::{ io, path };
use super::{ Joint_Info, Bound, Base_Frame, Frame_Data, Frame_Skeleton };
use util::Log;

#[macro_escape]
#[path = "../../../util/log_macros.rs"]
mod log_macros;

struct Animation
{
  joint_infos: ~[Joint_Info],
  bounds: ~[Bound],

  base_frames: ~[Base_Frame],
  frames: ~[Frame_Data],

  /* All skeletons for all frames. */
  skeletons: ~[Frame_Skeleton],
  animated_skeleton: Frame_Skeleton,

  /* Header data */
  version: i32,
  num_frames: i32,
  num_joints: i32,
  frame_rate: i32,
  num_animated_components: i32,

  /* Time data. */
  total_duration: f32,
  frame_duration: f32,
  time: f32,
}

impl Animation
{
  pub fn new(filename: ~str) -> Option<Animation>
  {
    if filename.len() == 0
    {
      log_error!("Invalid (empty) animation to load");
      return None;
    }

    let mut anim = Animation
    {
      joint_infos: ~[],
      bounds: ~[],
      base_frames: ~[],
      frames: ~[],

      skeletons: ~[],
      animated_skeleton: Frame_Skeleton::new(),

      version: 0,
      num_frames: 0,
      num_joints: 0,
      frame_rate: 0,
      num_animated_components: 0,

      total_duration: 0.0,
      frame_duration: 0.0,
      time: 0.0,
    };

    if anim.load(filename)
    { return Some(anim); }
    else
    { return None; }
  }

  fn load(&mut self, file: ~str) -> bool
  {
    let fior = io::file_reader(&path::Path(file));
    if fior.is_err()
    { log_error!("Failed to open animation file %s", file); return false; }

    /* Clear existing data. */
    self.joint_infos.clear();
    self.bounds.clear();
    self.base_frames.clear();
    self.frames.clear();
    self.skeletons.clear();

    let fio = fior.unwrap();
    let mut param;
    macro_rules! read_param
    (
      () =>
      ({
        param = ~"";
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
      });
    )
    macro_rules! read_junk
    (() => ({ read_param!(); });)
    macro_rules! ignore_line
    (() => ({ fio.read_line(); });)
    macro_rules! read_type
    (
      ($var:expr) =>
      ({
        let name = param.clone();
        read_param!();
        let num = FromStr::from_str(param);
        if num.is_some()
        { $var = num.unwrap(); }
        else
        { log_error!("Invalid %s in %s", name, file); }
      });
    )

    /* Read the first param and jump into the parsing. */
    log_debug!("Parsing animation %s", file);
    log_push!();
    read_param!();
    while !fio.eof()
    {
      match param
      {
        ~"MD5Version" =>
        {
          read_type!(self.version);
          log_debug!("Version: %d", self.version as int);
        }
        ~"commandline" =>
        { ignore_line!(); }
        ~"numFrames" =>
        {
          read_type!(self.num_frames);
          ignore_line!();
          log_debug!("Num frames: %d", self.num_frames as int);
        }
        ~"numJoints" =>
        {
          read_type!(self.num_joints);
          ignore_line!();
          log_debug!("Num joints: %d", self.num_frames as int);
        }
        ~"frameRate" =>
        {
          read_type!(self.frame_rate);
          ignore_line!();
          log_debug!("Framerate: %d", self.num_frames as int);
        }
        ~"numAnimatedComponents" =>
        {
          read_type!(self.num_animated_components);
          ignore_line!();
          log_debug!("Num animated components: %d", self.num_frames as int);
        }
        _ =>
        { ignore_line!(); }
      }

      read_param!();
    }
    log_pop!();

    /* Calculate timing. */
    self.frame_duration = 1.0 / (self.frame_rate as f32);
    self.total_duration = (self.frame_duration * (self.num_frames as f32));
    self.time = 0.0;

    /* Ensure everything went well. */
    self.joint_infos.len() as i32 == self.num_joints &&
    self.bounds.len() as i32 == self.num_frames &&
    self.base_frames.len() as i32 == self.num_joints &&
    self.frames.len() as i32 == self.num_frames &&
    self.skeletons.len() as i32 == self.num_frames
  }
}

