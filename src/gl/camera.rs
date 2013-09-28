/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/camera.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The game's camera that handles 
      projection, viewport sizing, etc.
      Movement and input-related business
      is handled in the camera state instead.
*/

use std::{ f32, local_data };
use glfw;
use gl2 = opengles::gl2;
use math;
use console;
use log::Log;

#[macro_escape]
mod check;

#[macro_escape]
#[path = "../log/macros.rs"]
mod macros;

static tls_key: local_data::Key<@mut Camera> = &local_data::Key;

/* TODO: This should just be a trait. */
struct Camera
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

    console::Console::get().add_accessor("camera.fov", |_|
    { c.fov.to_str() });
    console::Console::get().add_mutator("camera.fov", |p, fov|
    {
      let mut error = ~"";

      c.fov = match f32::from_str(fov)
      {
        Some(x) => { x },
        None => { error = fmt!("Invalid value for %s (use a floating point number)", p); c.fov }
      };

      /* Rebuild the projection info. */
      c.resize(c.window_size.x, c.window_size.y);

      if error.len() == 0
      { None }
      else
      { Some(error) }
    });
    console::Console::get().add_accessor("ui.show_fps", |_|
    { c.show_fps.to_str() });
    console::Console::get().add_mutator("ui.show_fps", |p, x|
    {
      let mut error = ~"";
      if x == "true"
      { c.show_fps = true; }
      else if x == "false"
      { c.show_fps = false; }
      else
      { error = fmt!("Invalid value for %s (use 'true' or 'false')", p); }

      if error.len() == 0
      { None }
      else
      { Some(error) }
    });
    console::Console::get().add_accessor("camera.vsync", |_|
    { c.vsync.to_str() });
    console::Console::get().add_mutator("camera.vsync", |p, x|
    {
      let mut error = ~"";
      if x == "true"
      {
        c.vsync = true;
        glfw::set_swap_interval(1); 
      }
      else if x == "false"
      {
        c.vsync = false;
        glfw::set_swap_interval(0);
      }
      else
      { error = fmt!("Invalid value for %s (use 'true' or 'false')", p); }

      if error.len() == 0
      { None }
      else
      { Some(error) }
    });

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

    check!(gl2::viewport(0, 0, self.window_size.x, self.window_size.y));

    self.projection = math::Mat4x4::new_perspective(
                                      self.fov,
                                      (self.window_size.x / self.window_size.y) as f32,
                                      self.near_far.x,
                                      self.near_far.y);
  }

  pub fn reset(&mut self)
  { self.position = math::Vec3f::zero(); }
}

