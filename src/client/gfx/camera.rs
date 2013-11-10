/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/camera.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The game's camera that handles 
      projection, viewport sizing, etc.
      Movement and input-related business
      is handled in the camera state instead.
*/

use std::local_data;
use glfw;
use gl;
use math;
use console;
use log::Log;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../../shared/log/macros.rs"]
mod macros;

static tls_key: local_data::Key<@mut Camera> = &local_data::Key;

/* TODO: This should just be a trait. */
pub struct Camera
{
  position: math::Vec3f,
  angles: math::Vec2f,
  
  /* Projection. */
  projection: math::Mat4x4,
  near_far: math::Vec2f,
  fov: f32,
  view: math::Mat4x4,

  /* Mouse. */
  look_speed: f32,

  /* Keyboard. */
  move_to: u8,
  move_speed: f32,

  /* Frame rate. */ /* TODO: Frame rate regulator? */
  target_frame_rate: f32,
  frame_rate: f32,
  frames_this_sec: f32,
  this_sec: f32,

  /* Window. */
  window: @glfw::Window,
  window_size: math::Vec2i,
  show_fps: bool,
  vsync: bool,
}

impl Camera
{
  pub fn new(win: @glfw::Window) -> @mut Camera
  {
    let c = @mut Camera
    {
      position: math::Vec3f::zero(),
      angles: math::Vec2f::zero(),
      projection: math::Mat4x4::new(),
      near_far: math::Vec2f::new(0.1, 70.0),
      fov: 100.0,
      view: math::Mat4x4::new(),
      look_speed: 0.001,
      move_to: 0,
      move_speed: 10.0,
      target_frame_rate: 60.0,
      frame_rate: 0.0,
      frames_this_sec: 0.0,
      this_sec: 0.0,

      window: win,
      window_size: math::Vec2i::zero(),
      show_fps: true,
      vsync: true,
    };

    console::Console::get().add_accessor("camera.fov", c as @console::Accessor);
    console::Console::get().add_accessor("camera.vsync", c as @console::Accessor);
    console::Console::get().add_accessor("ui.show_fps", c as @console::Accessor);

    console::Console::get().add_mutator("camera.fov", c as @mut console::Mutator);
    console::Console::get().add_mutator("camera.vsync", c as @mut console::Mutator);
    console::Console::get().add_mutator("ui.show_fps", c as @mut console::Mutator);

    /* Set some defaults. */
    console::Console::run_function(~"set camera.vsync true");

    c
  }

  pub fn set_active(cam: @mut Camera)
  { local_data::set(tls_key, cam); }

  pub fn get_active() -> @mut Camera
  { 
    local_data::get(tls_key, 
    |opt|
    {
      match opt
      {
        Some(x) => *x,
        None => log_fail!("Singleton not available")
      }
    })
  }

  pub fn resize(&mut self, new_width: i32, new_height: i32)
  {
    /* Avoid division by zero if the window is being fondled. */
    if new_width == 0 || new_height == 0
    { return; }

    self.window_size.x = new_width;
    self.window_size.y = new_height;

    check!(gl::Viewport(0, 0, self.window_size.x, self.window_size.y));

    self.projection = math::Mat4x4::new_perspective(
                                      self.fov,
                                      (self.window_size.x / self.window_size.y) as f32,
                                      self.near_far.x,
                                      self.near_far.y);
  }

  pub fn reset(&mut self)
  { self.position = math::Vec3f::zero(); }
}

impl console::Accessor for Camera
{
  fn access(&self, name: &str) -> ~str
  {
    match name
    {
      "camera.fov" =>
      { self.fov.to_str() },
      "camera.vsync" =>
      { self.vsync.to_str() },
      "ui.show_fps" =>
      { self.show_fps.to_str() },

      _ => { ~"ERROR" },
    }
  }
}

impl console::Mutator for Camera
{
  fn mutate(&mut self, name: &str, val: &str) -> Option<~str>
  {
    match name
    {
      "camera.fov" =>
      {
        let res = console::Util::parse_f32(name, val);
        match res
        {
          Ok(val) =>
          {
            self.fov = val;
            self.resize(self.window_size.x, self.window_size.y);
            None
          }
          Err(msg) => { Some(msg) }
        }
      },
      "ui.show_fps" =>
      {
        let res = console::Util::parse_bool(name, val);
        match res
        {
          Ok(val) => { self.show_fps = val; None },
          Err(msg) => { Some(msg) }
        }
      },
      "camera.vsync" =>
      {
        let res = console::Util::parse_bool(name, val);
        match res
        {
          Ok(val) =>
          {
            self.vsync = val;
            glfw::set_swap_interval(if self.vsync { 1 } else { 0 });
            None
          },
          Err(msg) => { Some(msg) }
        }
      },
      _ => { Some(~"Invalid property name") }
    }
  }
}

