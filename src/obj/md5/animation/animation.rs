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
use super::{ Joint_Info, Bound, Base_Frame, Frame_Data, Frame_Skeleton, Skeleton_Joint };
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

  /* Frame/Time data. */
  total_duration: f32,
  frame_duration: f32,
  time: f32,
  curr_frame: i32,
  next_frame: i32,
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
      curr_frame: 0,
      next_frame: 0,
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
          log_debug!("Num joints: %d", self.num_joints as int);
        }
        ~"frameRate" =>
        {
          read_type!(self.frame_rate);
          ignore_line!();
          log_debug!("Framerate: %d", self.frame_rate as int);
        }
        ~"numAnimatedComponents" =>
        {
          read_type!(self.num_animated_components);
          ignore_line!();
          log_debug!("Num animated components: %d", self.num_animated_components as int);
        }
        ~"hierarchy" =>
        {
          log_debug!("Reading hierarchy");
          log_push!();

          read_junk!(); /* { */
          for _ in range(0, self.num_joints)
          {
            let mut joint = Joint_Info::new();
            read_param!();
            joint.name = param.clone();
            //str::replace(joint.name, "\"", "");

            read_type!(joint.parent_id);
            read_type!(joint.flags);
            read_type!(joint.start_index);

            printfln!("%s %d %d %d", joint.name, joint.parent_id as int, joint.flags as int, joint.start_index as int);
            log_debug!("Joint: %s", joint.name);
            self.joint_infos.push(joint);

            ignore_line!();
          }

          read_junk!(); /* } */
          log_pop!();
        }
        ~"bounds" =>
        {
          log_debug!("Reading bounds");
          log_push!();

          read_junk!(); /* { */
          ignore_line!();
          for _ in range(0, self.num_frames)
          {
            let mut bound = Bound::new();
            read_junk!(); /* ( */
            read_type!(bound.min.x);
            read_type!(bound.min.y);
            read_type!(bound.min.z);
            read_junk!(); /* ) */
            read_junk!(); /* ( */
            read_type!(bound.max.x);
            read_type!(bound.max.y);
            read_type!(bound.max.z);

            log_debug!("Bound %s %s", bound.min.to_str(), bound.max.to_str());
            self.bounds.push(bound);

            ignore_line!();
          }

          read_junk!(); /* } */
          ignore_line!();
          log_pop!();
        }
        ~"baseframe" =>
        {
          log_debug!("Reading base frame");
          log_push!();

          read_junk!(); /* { */
          ignore_line!();
          for _ in range(0, self.num_joints)
          {
            let mut base_frame = Base_Frame::new();
            read_junk!(); /* ( */
            read_type!(base_frame.position.x);
            read_type!(base_frame.position.y);
            read_type!(base_frame.position.z);
            read_junk!(); /* ) */
            read_junk!(); /* ( */
            read_type!(base_frame.orientation.x);
            read_type!(base_frame.orientation.y);
            read_type!(base_frame.orientation.z);
            ignore_line!();

            base_frame.orientation.compute_w();

            printfln!("%s", base_frame.position.to_str());
            printfln!("%s\n", base_frame.orientation.to_str());

            log_debug!("Base frame %s %s",
                        base_frame.position.to_str(),
                        base_frame.orientation.to_str());
            self.base_frames.push(base_frame);
          }
          read_junk!(); /* } */
          ignore_line!();
          log_pop!();
        }
        ~"frame" =>
        {
          log_debug!("Reading frame");
          let mut frame = Frame_Data::new();
          read_type!(frame.id);
          read_junk!(); /* { */
          ignore_line!();

          printfln!("frame: %d", frame.id as int);
          for _ in range(0, self.num_animated_components)
          {
            let mut frameData: f32 = 0.0;
            read_type!(frameData);
            printfln!("%f", frameData as float);
            frame.data.push(frameData);
          }

          self.build_frame_skeleton(&frame);

          self.frames.push(frame);

          read_junk!(); /* { */
          ignore_line!();
        }
        _ =>
        { ignore_line!(); }
      }

      read_param!();
    }
    log_pop!();

    /* Ensure there are enough joints for the skeleton. */
    while self.animated_skeleton.joints.len() < self.num_joints as uint
    { self.animated_skeleton.joints.push(Skeleton_Joint::new()); }

    /* Calculate timing. */
    self.frame_duration = 1.0 / (self.frame_rate as f32);
    self.total_duration = (self.frame_duration * (self.num_frames as f32));
    self.time = 0.0;
    self.curr_frame = 1;
    self.next_frame = 2;

    /* Ensure everything went well. */
    self.joint_infos.len() as i32 == self.num_joints &&
    self.bounds.len() as i32 == self.num_frames &&
    self.base_frames.len() as i32 == self.num_joints &&
    self.frames.len() as i32 == self.num_frames &&
    self.skeletons.len() as i32 == self.num_frames
  }

  fn build_frame_skeleton(&mut self, frame: &Frame_Data)
  {
    let mut skeleton = Frame_Skeleton::new();

    for i in range(0, self.joint_infos.len())
    {
      let mut j = 0;
      let joint_info = &self.joint_infos[i];
      let mut joint = Skeleton_Joint::new_from_base_frame(&self.base_frames[i]);

      joint.parent = joint_info.parent_id;

      printfln!("top p: %d", joint.parent as int);
      printfln!("top jp: %s", joint.position.to_str());
      printfln!("top jo: %s\n", joint.orientation.to_str());

      if (joint_info.flags & 1) != 0 /* position.x */
      {
        joint.position.x = frame.data[joint_info.start_index + j];
        printfln!("p.x %f", joint.position.x as float);
        j += 1;
      }
      if (joint_info.flags & 2) != 0 /* position.y */
      {
        joint.position.y = frame.data[joint_info.start_index + j];
        printfln!("p.y %f", joint.position.y as float);
        j += 1;
      }
      if (joint_info.flags & 4) != 0 /* position.z */
      {
        joint.position.z = frame.data[joint_info.start_index + j];
        printfln!("p.z %f", joint.position.z as float);
        j += 1;
      }
      if (joint_info.flags & 8) != 0 /* orientation.x */
      {
        joint.orientation.x = frame.data[joint_info.start_index + j];
        printfln!("o.x %f", joint.orientation.x as float);
        j += 1;
      }
      if (joint_info.flags & 16) != 0 /* orientation.y */
      {
        joint.orientation.y = frame.data[joint_info.start_index + j];
        printfln!("o.y %f", joint.orientation.y as float);
        j += 1;
      }
      if (joint_info.flags & 32) != 0 /* orientation.z */
      {
        joint.orientation.z = frame.data[joint_info.start_index + j];
        printfln!("o.z %f", joint.orientation.z as float);
      }

      joint.orientation.compute_w();
      printfln!("o.w %f", joint.orientation.w as float);

      /* If the joint has a parent. */
      if joint.parent >= 0 
      {
        let parent = &skeleton.joints[joint.parent];
        let rot_pos = parent.orientation.rotate_vec(&joint.position);

        printfln!("po: %s", parent.orientation.to_str());
        printfln!("jp: %s", joint.position.to_str());
        printfln!("rp: %s", rot_pos.to_str());

        joint.position = parent.position + rot_pos;
        joint.orientation = parent.orientation * joint.orientation;
        joint.orientation.normalize();
      }

      printfln!("p: %d", joint.parent as int);
      printfln!("%s", joint.position.to_str());
      printfln!("%s\n", joint.orientation.to_str());

      skeleton.joints.push(joint);
    }

    self.skeletons.push(skeleton);
  }

  pub fn update(&mut self, dt: f32)
  {
    if self.num_frames < 1
    { return; }

    /* Progress time. */
    self.time += dt;

    if self.time >= self.frame_duration
    {
      self.curr_frame += 1;
      self.next_frame += 1;
      self.time = 0.0;

      if self.curr_frame > (self.num_frames - 1)
      { self.curr_frame = 0; }
      if self.next_frame > (self.num_frames - 1)
      { self.next_frame = 0; }
    }

    /* Keep it within bounds. */
 //   while self.time > self.total_duration
 //   { self.time -= self.total_duration; }
 //   while self.time < 0.0
 //   { self.time += self.total_duration; }

 //   /* Determine which frame we're on. */
 //   let frame_num = (self.time * (self.frame_rate as f32));
 //   let mut frame0 = frame_num.floor() as i32;
 //   let mut frame1 = frame_num.ceil() as i32;
 //   frame0 = frame0 % self.num_frames;
 //   frame1 = frame1 % self.num_frames;

 //   //log_assert!(frame0 != frame1);

 //   let interpolate = if self.frame_duration.approx_eq(&0.0)
 //   { 0.0 } /* Avoid division by zero. */
 //   else
 //   { (self.time % self.frame_duration) / self.frame_duration };

    printfln!("CF: %d", self.curr_frame as int);
    printfln!("NF: %d", self.next_frame as int);

    self.animated_skeleton.interpolate( &self.skeletons[self.curr_frame],
                                        &self.skeletons[self.next_frame],
                                        self.time * (self.frame_rate as f32));
  }
}

