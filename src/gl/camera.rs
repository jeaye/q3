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

use glfw;
use gl = opengles::gl2;
use math::vec2::{ Vec2f, Vec2i };
use math::vec3::Vec3f;
use math::matrix::Mat4x4;

mod util;

#[macro_escape]
mod check_internal;

static Move_Left: u8 = 1;
static Move_Right: u8 = 2;
static Move_Forward: u8 = 4;
static Move_Backward: u8 = 8;
static Move_Up: u8 = 16;
static Move_Down: u8 = 32;

pub struct Camera
{
  position: Vec3f,
  angles: Vec2f,
  
  /* Projection. */
  projection: @Mat4x4,
  near_far: Vec2f,
  fov: f32,
  view: @Mat4x4,

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
  window_size: Vec2i,
}
impl Camera
{
  #[inline(always)]
  pub fn new(win: @glfw::Window) -> Camera
  {
    Camera {  position: Vec3f::zero(),
              angles: Vec2f::zero(),
              projection: @Mat4x4::new(),
              near_far: Vec2f::new(0.1, 1000.0),
              fov: 100.0,
              view: @Mat4x4::new(), /* TODO: s/new/identity/g */
              look_speed: 0.001,
              move_to: 0,
              move_speed: 0.0001,
              target_frame_rate: 60.0,
              frame_rate: 0.0,
              frames_this_sec: 0.0,
              this_sec: 0.0,
              window: win,
              window_size: Vec2i::zero(),
    }
  }

  #[inline(always)]
  pub fn init(&mut self)
  {
    check!(gl::enable(gl::CULL_FACE)); 
    check!(gl::enable(gl::DEPTH_TEST));
    check!(gl::depth_func(gl::LEQUAL));
    check!(gl::clear_color(0.0, 0.0, 0.0, 1.0));

    match self.window.get_size()
    { (width, height) => self.resize(width as i32, height as i32) }
  }

  #[inline(always)]
  pub fn resize(&mut self, new_width: i32, new_height: i32)
  {
    self.window_size.x = new_width;
    self.window_size.y = new_height;

    check!(gl::viewport(0, 0, self.window_size.x, self.window_size.y));
  }

  pub fn mouse_moved(&mut self, x: float, y: float) 
  {
    let dx = x - (self.window_size.x / 2) as float;
    let dy = y - (self.window_size.y / 2) as float;

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
  }

  pub fn key_action(&mut self, key: libc::c_int, action: libc::c_int) 
  {
    if action == glfw::PRESS /* TODO: Clean up by creating a closure and running that on all. */
    {
      match key
      {
        glfw::KEY_W => { self.move_to |= Move_Forward; }
        glfw::KEY_A => { self.move_to |= Move_Left; }
        glfw::KEY_S => { self.move_to |= Move_Backward; }
        glfw::KEY_D => { self.move_to |= Move_Right; }
        glfw::KEY_LEFT_CONTROL => { self.move_to |= Move_Down; }
        glfw::KEY_SPACE => { self.move_to |= Move_Up; }
        _ => { }
      }
    }
    else if action == glfw::RELEASE
    {
      match key
      {
        glfw::KEY_W => { self.move_to &= !Move_Forward; }
        glfw::KEY_A => { self.move_to &= !Move_Left; }
        glfw::KEY_S => { self.move_to &= !Move_Backward; }
        glfw::KEY_D => { self.move_to &= !Move_Right; }
        glfw::KEY_LEFT_CONTROL => { self.move_to &= !Move_Down; }
        glfw::KEY_SPACE => { self.move_to &= !Move_Up; }
        glfw::KEY_F => { io::println(fmt!("FPS: %?", self.frame_rate)); }
        _ => { }
      }
    }
  }

  pub fn update(&mut self, dt: f32)
  {
    /* Avoid division by zero if the window is being fondled. */
    if self.window_size.x == 0 || self.window_size.y == 0
    { return; }

    /* Frame rate. */
    self.this_sec += dt;
    if self.this_sec >= 100000f32
    {
      self.frame_rate = self.frames_this_sec;
      self.frames_this_sec = 0f32;
      self.this_sec -= 100000f32;
    } else
    { self.frames_this_sec += 1f32; }


    self.projection = @Mat4x4::new_perspective( 
                                      self.fov,
                                      (self.window_size.x / self.window_size.y) as f32,
                                      self.near_far.x,
                                      self.near_far.y);

    /* Update where the camera is looking. */
    let mut lookat = Vec3f::zero();
    lookat.x = f32::sin(self.angles.x) * f32::cos(self.angles.y);
    lookat.y = f32::sin(self.angles.y);
    lookat.z = f32::cos(self.angles.x) * f32::cos(self.angles.y);
    self.view = @Mat4x4::new_lookat(self.position,
                                    self.position + lookat, /* TODO: * focus for zoom */
                                    Vec3f::new(0.0, 1.0, 0.0));

    /* Move based on the keyboard input. */
    let forward = self.view.forward();
    let right = self.view.right();
    if self.move_to & Move_Left > 0
    { self.position -= right * self.move_speed * dt; }
    if self.move_to & Move_Right  > 0
    { self.position += right * self.move_speed * dt; }
    if self.move_to & Move_Forward > 0
    { self.position += forward * self.move_speed * dt; }
    if self.move_to & Move_Backward > 0
    { self.position -= forward * self.move_speed * dt; }
    if self.move_to & Move_Up > 0
    { self.position.y += self.move_speed * dt; }
    if self.move_to & Move_Down > 0
    { self.position.y -= self.move_speed * dt; }
  }
}

