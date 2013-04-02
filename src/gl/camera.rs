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
              window_size: math::Vec2::zero::<i32>()
    }
  }

  pub fn init_gl(&mut self)
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

  pub fn mouse_moved(&mut self, x: i32, y: i32)
  {
    self.mouse_position.x = x;
    self.mouse_position.y = y;
  }

  pub fn key_action(&mut self, _key: libc::c_int, _action: libc::c_int) /* TODO: Param names */
  {
    if _action == glfw::PRESS
    {

    }
    else
    {

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

