/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/camera.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The game's camera that handles movement,
      projection, viewport sizing, etc.
*/

use std::local_data;
use glfw;
use gl2 = opengles::gl2;
use std::f32;
use math;
use ui;
use state;

#[macro_escape]
mod check;

static MOVE_LEFT: u8 = 1;
static MOVE_RIGHT: u8 = 2;
static MOVE_FORWARD: u8 = 4;
static MOVE_BACKWARD: u8 = 8;
static MOVE_UP: u8 = 16;
static MOVE_DOWN: u8 = 32;

static tls_key: local_data::Key<@mut Camera> = &local_data::Key;

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
      near_far: math::Vec2f::new(0.1, 50.0),
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

    state::Console::get().add_accessor("camera.fov", |_|
    { c.fov.to_str() });
    state::Console::get().add_mutator("camera.fov", |p, fov|
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
    state::Console::get().add_accessor("ui.show_fps", |_|
    { c.show_fps.to_str() });
    state::Console::get().add_mutator("ui.show_fps", |p, x|
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
    state::Console::get().add_accessor("camera.vsync", |_|
    { c.vsync.to_str() });
    state::Console::get().add_mutator("camera.vsync", |p, x|
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
    state::Console::run_function(~"set camera.vsync true");

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
        None => fail!("Singleton not available")
      }
    })
  }

  pub fn init(&mut self)
  {
    check!(gl2::enable(gl2::CULL_FACE)); 
    check!(gl2::enable(gl2::DEPTH_TEST));
    check!(gl2::depth_func(gl2::LEQUAL));
    check!(gl2::blend_func(gl2::SRC_ALPHA, gl2::ONE_MINUS_SRC_ALPHA));
    check!(gl2::clear_color(0.0, 0.0, 0.0, 1.0));

    match self.window.get_size()
    { (width, height) => self.resize(width as i32, height as i32) }
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

  pub fn update(&mut self, dt: f32)
  {
    /* Avoid division by zero if the window is being fondled. */
    if self.window_size.x == 0 || self.window_size.y == 0
    { return; }

    /* Frame rate. */
    self.this_sec += dt;
    if self.this_sec >= 1.0
    {
      self.frame_rate = self.frames_this_sec;
      self.frames_this_sec = 0.0;
      self.this_sec -= 1.0;
    } else
    { self.frames_this_sec += 1.0; }


    /* Update where the camera is looking. */
    let lookat = math::Vec3f::new
    (
      self.angles.x.sin() * self.angles.y.cos(),
      self.angles.y.sin(),
      self.angles.x.cos() * self.angles.y.cos()
    );
    self.view = math::Mat4x4::new_lookat(self.position, 
                                    self.position + lookat, /* TODO: * focus for zoom */
                                    math::Vec3f::new(0.0, 1.0, 0.0));

    /* Move based on the keyboard input. */
    let forward = self.view.get_forward();
    let right = self.view.get_right();
    if self.move_to & MOVE_LEFT > 0
    { self.position = self.position - right * self.move_speed * dt; }
    if self.move_to & MOVE_RIGHT  > 0
    { self.position = self.position + right * self.move_speed * dt; }
    if self.move_to & MOVE_FORWARD > 0
    { self.position = self.position + forward * self.move_speed * dt; }
    if self.move_to & MOVE_BACKWARD > 0
    { self.position = self.position - forward * self.move_speed * dt; }
    if self.move_to & MOVE_UP > 0
    { self.position.y = self.position.y + self.move_speed * dt; }
    if self.move_to & MOVE_DOWN > 0
    { self.position.y = self.position.y - self.move_speed * dt; }
  }
}

impl ui::Input_Listener for Camera
{
  pub fn key_action(&mut self, key: i32, action: i32, _mods: i32) -> bool
  {
    let mut captured = true;

    if action == glfw::PRESS 
    {
      match key
      {
        glfw::KEY_W => { self.move_to |= MOVE_FORWARD; }
        glfw::KEY_A => { self.move_to |= MOVE_LEFT; }
        glfw::KEY_S => { self.move_to |= MOVE_BACKWARD; }
        glfw::KEY_D => { self.move_to |= MOVE_RIGHT; }
        glfw::KEY_LEFT_CONTROL => { self.move_to |= MOVE_DOWN; }
        glfw::KEY_SPACE => { self.move_to |= MOVE_UP; }
        _ => { captured = false; }
      }
    }
    else if action == glfw::RELEASE
    {
      match key
      {
        glfw::KEY_W => { self.move_to &= !MOVE_FORWARD; }
        glfw::KEY_A => { self.move_to &= !MOVE_LEFT; }
        glfw::KEY_S => { self.move_to &= !MOVE_BACKWARD; }
        glfw::KEY_D => { self.move_to &= !MOVE_RIGHT; }
        glfw::KEY_LEFT_CONTROL => { self.move_to &= !MOVE_DOWN; }
        glfw::KEY_SPACE => { self.move_to &= !MOVE_UP; }
        _ => { captured = false; }
      }
    }

    captured
  }
  pub fn key_char(&mut self, _ch: char) -> bool
  { false }
  pub fn mouse_action(&mut self, _button: i32, _action: i32, _mods: i32) -> bool
  { false }
  pub fn mouse_moved(&mut self, x: f32, y: f32) -> bool
  {
    let dx = x - (self.window_size.x / 2) as f32;
    let dy = y - (self.window_size.y / 2) as f32;

    self.angles.x -= dx as f32 * self.look_speed;
    self.angles.y -= dy as f32 * self.look_speed;
    
    /* Wrap X. */
    if self.angles.x < -f32::consts::pi
    { self.angles.x += f32::consts::pi * 2.0; }
    else if self.angles.x > f32::consts::pi
    { self.angles.x -= f32::consts::pi * 2.0; }

    /* Clamp Y. */
    if self.angles.y < -f32::consts::pi * 0.49
    { self.angles.y = -f32::consts::pi * 0.49; }
    else if self.angles.y > f32::consts::pi * 0.49
    { self.angles.y = f32::consts::pi * 0.49; }

    self.window.set_cursor_pos( (self.window_size.x / 2) as float, 
                                (self.window_size.y / 2) as float);

    true
  }
}

