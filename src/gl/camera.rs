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

extern mod glfw;
extern mod opengles;
use gl = opengles::gl2;

mod util;
#[path = "../math/math.rs"]
mod math;

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
  position: math::Vec3<f32>,
  angles: math::Vec2<f32>,
  
  /* Projection. */
  projection: @math::Mat4x4,
  near_far: math::Vec2<f32>,
  fov: f32,

  /* Mouse. */
  mouse_speed: f32,
  mouse_position: math::Vec2<i32>,

  /* Keyboard. */
  move_to: u8,

  window_size: math::Vec2<i32>
}
impl Camera
{
  pub fn new() -> Camera
  {
    Camera {  position: math::Vec3::zero::<f32>(),
              angles: math::Vec2::zero::<f32>(),
              projection: @math::Mat4x4::new(),
              near_far: math::Vec2::new::<f32>(1.0, 1000.0),
              fov: 100.0,
              mouse_speed: 0.001,
              mouse_position: math::Vec2::zero::<i32>(),
              move_to: 0,
              window_size: math::Vec2::zero::<i32>()
    }
  }

  pub fn init(&mut self)
  {
    check!(gl::enable(gl::CULL_FACE));
    check!(gl::clear_color(0.0, 0.0, 0.0, 1.0));
  }

  pub fn resize(&mut self, new_width: i32, new_height: i32)
  {
    self.window_size.x = new_width;
    self.window_size.y = new_height;

    self.refresh();
  }
  
  pub fn refresh(&mut self)
  {
    check!(gl::viewport(0, 0, self.window_size.x, self.window_size.y));

    self.projection = @math::Mat4x4::new_perspective_projection( 
                                      self.fov,
                                      (self.window_size.x / self.window_size.y) as f32,
                                      self.near_far.x,
                                      self.near_far.y);
  }

  pub fn mouse_moved(&mut self, x: i32, y: i32) /* TODO: dx, dy */
  {
    self.mouse_position.x = x;
    self.mouse_position.y = y;

    self.angles.x += x as f32 * self.mouse_speed;
    self.angles.y += y as f32 * self.mouse_speed;
    
    /* Wrap X. */
    if self.angles.x < -f32::consts::pi
    { self.angles.x += f32::consts::pi * 2.0; }
    else if self.angles.x > f32::consts::pi
    { self.angles.x -= f32::consts::pi * 2.0; }

    /* Clamp Y. */
    if self.angles.y < -f32::consts::pi / 2.0
    { self.angles.y = -f32::consts::pi / 2.0; }
    else if self.angles.y > f32::consts::pi / 2.0
    { self.angles.y = f32::consts::pi / 2.0; }

    let lookat = math::Vec3::zero::<f32>();
    lookat.x = f32::sin(self.angles.x) * f32::cos(self.angles.y);
    lookat.y = f32::sin(self.angles.y);
    lookat.z = f32::cos(self.angles.x) * f32::cos(self.angles.y);
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
      }
    }
    else
    {
      match key
      {
        glfw::KEY_W => { self.move_to &= !Forward; }
        glfw::KEY_A => { self.move_to &= !Left; }
        glfw::KEY_S => { self.move_to &= !Backward; }
        glfw::KEY_D => { self.move_to &= !Right; }
        glfw::KEY_Q => { self.move_to &= !Down; }
        glfw::KEY_E => { self.move_to &= !Up; }
      }
    }
  }
}

impl obj::traits::Movable for Camera
{
  pub fn translate(&mut self, _new_position: math::Vec3<f32>) /* TODO: wtf */
  { }
  pub fn translate_to(&mut self, _new_position: math::Vec3<f32>)
  { }
}

