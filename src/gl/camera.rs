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
use math::vec2::Vec2;
use math::vec3::Vec3f;
use math::matrix::Mat4x4;

mod util;

#[macro_escape]
mod check_internal;

#[path = "../obj/mod.rs"]
mod obj;

static Left: u8 = 1;
static Right: u8 = 2;
static Forward: u8 = 4;
static Backward: u8 = 8;
static Up: u8 = 16;
static Down: u8 = 32;

pub struct Camera
{
  position: Vec3f,
  angles: Vec2<f32>,
  
  /* Projection. */
  projection: @Mat4x4,
  near_far: Vec2<f32>,
  fov: f32,
  view: @Mat4x4,

  /* Mouse. */
  look_speed: f32,

  /* Keyboard. */
  move_to: u8,
  move_speed: f32,

  /* Window. */
  window: @glfw::Window,
  window_size: Vec2<i32>
}
impl Camera
{
  pub fn new(win: @glfw::Window) -> Camera
  {
    Camera {  position: Vec3f::zero(),
              angles: Vec2::zero::<f32>(),
              projection: @Mat4x4::new(),
              near_far: Vec2::new::<f32>(1.0, 1000.0),
              fov: 100.0,
              view: @Mat4x4::new(), /* TODO: s/new/identity/g */
              look_speed: 0.001,
              move_to: 0,
              move_speed: 0.001,
              window: win,
              window_size: Vec2::zero::<i32>()
    }
  }

  pub fn init(&mut self)
  {
    //check!(gl::enable(gl::CULL_FACE));
    check!(gl::enable(gl::DEPTH_TEST));
    check!(gl::depth_func(gl::LEQUAL));
    check!(gl::clear_color(0.0, 0.0, 0.0, 1.0));
  }

  pub fn resize(&mut self, new_width: i32, new_height: i32)
  {
    self.window_size.x = new_width;
    self.window_size.y = new_height;

    check!(gl::viewport(0, 0, self.window_size.x, self.window_size.y));
  }

  pub fn mouse_moved(&mut self, x: i32, y: i32) 
  {
    let dx = x - (self.window_size.x / 2);
    let dy = y - (self.window_size.y / 2);

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

    self.window.set_cursor_pos((self.window_size.x / 2) as int, (self.window_size.y / 2) as int);
  }

  pub fn key_action(&mut self, key: libc::c_int, action: libc::c_int) 
  {
    if action == glfw::PRESS /* TODO: Clean up by creating a closure and running that on all. */
    {
      match key
      {
        glfw::KEY_W => { self.move_to |= Forward; }
        glfw::KEY_A => { self.move_to |= Left; }
        glfw::KEY_S => { self.move_to |= Backward; }
        glfw::KEY_D => { self.move_to |= Right; }
        glfw::KEY_Q => { self.move_to |= Down; }
        glfw::KEY_E => { self.move_to |= Up; }
        _ => { }
      }
    }
    else if action == glfw::RELEASE
    {
      match key
      {
        glfw::KEY_W => { self.move_to &= !Forward; }
        glfw::KEY_A => { self.move_to &= !Left; }
        glfw::KEY_S => { self.move_to &= !Backward; }
        glfw::KEY_D => { self.move_to &= !Right; }
        glfw::KEY_Q => { self.move_to &= !Down; }
        glfw::KEY_E => { self.move_to &= !Up; }
        _ => { }
      }
    }
  }

  pub fn update(&mut self, dt: f32)
  {
    if self.window_size.x == 0 || self.window_size.y == 0
    { return; }

    self.projection = @Mat4x4::new_perspective_projection( 
                                      self.fov,
                                      (self.window_size.x / self.window_size.y) as f32,
                                      self.near_far.x,
                                      self.near_far.y);

    let mut lookat = Vec3f::zero();
    lookat.x = f32::sin(self.angles.x) * f32::cos(self.angles.y);
    lookat.y = f32::sin(self.angles.y);
    lookat.z = f32::cos(self.angles.x) * f32::cos(self.angles.y);

    self.view = @Mat4x4::new_lookat(self.position,
                                    self.position + lookat, /* TODO: * focus for zoom */
                                    Vec3f::new(0.0, 1.0, 0.0));
    let forward = self.view.forward();
    let right = self.view.right();

    if self.move_to & Left > 0
    { self.position -= right * self.move_speed * dt; }
    if self.move_to & Right  > 0
    { self.position += right * self.move_speed * dt; }
    if self.move_to & Forward > 0
    { self.position += forward * self.move_speed * dt; }
    if self.move_to & Backward > 0
    { self.position -= forward * self.move_speed * dt; }
    if self.move_to & Up > 0
    { self.position.y += self.move_speed * dt; }
    if self.move_to & Down > 0
    { self.position.y -= self.move_speed * dt; }

  }
}

impl obj::traits::Movable for Camera
{
  pub fn translate(&mut self, _new_position: Vec3f) /* TODO: wtf */
  { }
  pub fn translate_to(&mut self, _new_position: Vec3f)
  { }
}

