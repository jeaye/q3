/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/state/camera/camera.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The camera state, which processes input
      and other user events to adjust the camera.
*/

use glfw;
use gl;
use gl2 = opengles::gl2;
use std::f32;
use math;
use super::State;
use log::Log;

#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

#[macro_escape]
#[path = "../../../shared/log/macros.rs"]
mod macros;

pub static MOVE_LEFT: u8 = 1;
pub static MOVE_RIGHT: u8 = 2;
pub static MOVE_FORWARD: u8 = 4;
pub static MOVE_BACKWARD: u8 = 8;
pub static MOVE_UP: u8 = 16;
pub static MOVE_DOWN: u8 = 32;

impl State for gl::Camera
{
  fn get_key(&self) -> &str
  { &"camera" }

  fn load(&mut self)
  {
    log_debug!("Loading camera state");

    check!(gl2::enable(gl2::CULL_FACE)); 
    check!(gl2::enable(gl2::DEPTH_TEST));
    check!(gl2::depth_func(gl2::LEQUAL));
    check!(gl2::blend_func(gl2::SRC_ALPHA, gl2::ONE_MINUS_SRC_ALPHA));
    check!(gl2::clear_color(0.0, 0.0, 0.0, 1.0));

    match self.window.get_size()
    { (width, height) => self.resize(width as i32, height as i32) }
  }

  fn update(&mut self, dt: f32) -> bool
  {
    /* Avoid division by zero if the window is being fondled. */
    if self.window_size.x == 0 || self.window_size.y == 0
    { return false; }

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

    false
  }

  fn key_action(&mut self, key: glfw::Key, action: glfw::Action, _mods: glfw::Modifiers) -> bool
  {
    let mut captured = true;

    if action == glfw::Press 
    {
      match key
      {
        glfw::KeyW => { self.move_to |= MOVE_FORWARD; }
        glfw::KeyA => { self.move_to |= MOVE_LEFT; }
        glfw::KeyS => { self.move_to |= MOVE_BACKWARD; }
        glfw::KeyD => { self.move_to |= MOVE_RIGHT; }
        glfw::KeyLeftControl => { self.move_to |= MOVE_DOWN; }
        glfw::KeySpace => { self.move_to |= MOVE_UP; }
        _ => { captured = false; }
      }
    }
    else if action == glfw::Release
    {
      match key
      {
        glfw::KeyW => { self.move_to &= !MOVE_FORWARD; }
        glfw::KeyA => { self.move_to &= !MOVE_LEFT; }
        glfw::KeyS => { self.move_to &= !MOVE_BACKWARD; }
        glfw::KeyD => { self.move_to &= !MOVE_RIGHT; }
        glfw::KeyLeftControl => { self.move_to &= !MOVE_DOWN; }
        glfw::KeySpace => { self.move_to &= !MOVE_UP; }
        _ => { captured = false; }
      }
    }

    captured
  }
  fn key_char(&mut self, _ch: char) -> bool
  { false }
  fn mouse_action(&mut self, _button: i32, _action: i32, _mods: i32) -> bool
  { false }
  fn mouse_moved(&mut self, x: f32, y: f32) -> bool
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

    self.window.set_cursor_pos( (self.window_size.x / 2) as f64, 
                                (self.window_size.y / 2) as f64);

    true
  }
}

