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

  mouse_speed: f32,
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
              window_size: math::Vec2::zero::<i32>()
    }
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
}

impl obj::traits::Movable for Camera
{
  pub fn translate(&mut self, new_position: math::Vec3<f32>)
  { }
  pub fn translate_to(&mut self, new_position: math::Vec3<f32>)
  { }
}

