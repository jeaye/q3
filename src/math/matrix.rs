/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/matrix.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 16 component (4x4) matrix of floats
      for representing orientational data.
*/

use std::ptr;
use math;

type Component = f32;

struct Mat4x4
{
  data: [[f32, ..4], ..4]
}

impl Mat4x4
{

  pub fn new() -> Mat4x4
  {
    Mat4x4{data:[ [1.0, 0.0, 0.0, 0.0],
                  [0.0, 1.0, 0.0, 0.0],
                  [0.0, 0.0, 1.0, 0.0],
                  [0.0, 0.0, 0.0, 1.0]]}
  }

  pub fn new_perspective(fov: Component, aspect_ratio: Component, near: Component, far: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();

    let rad = (3.1415 * fov) / 180.0;
    let range = (rad / 2.0).tan() * near;
    let left = -range * aspect_ratio;
    let right = range * aspect_ratio;
    let bottom = -range;
    let top = range;

    mat.data[0][0] = (2.0 * near) / (right - left); mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = 0.0; 
    mat.data[0][1] = 0.0; mat.data[1][1] = (2.0 * near) / (top - bottom); mat.data[2][1] = 0.0; mat.data[3][1] = 0.0; 
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = -(far + near) / (far - near); mat.data[3][2] = -(2.0 * far * near) / (far - near); 
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = -1.0; mat.data[3][3] = 0.0;

    mat
  }

  pub fn new_orthographic(left: Component, right: Component, bottom: Component, top: Component,
                          near: Component, far: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = 2.0 / (right - left); mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] =  -(right + left) / (right - left); 
    mat.data[0][1] = 0.0; mat.data[1][1] = 2.0 / (top - bottom); mat.data[2][1] = 0.0; mat.data[3][1] = -(top + bottom) / (top - bottom); 
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = -2.0 / (far - near); mat.data[3][2] = -(far + near) / (far - near);
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;

    mat
  }
  
  pub fn new_scale(x: Component, y: Component, z: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = x;   mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = 0.0;
    mat.data[0][1] = 0.0; mat.data[1][1] = y;   mat.data[2][1] = 0.0; mat.data[3][1] = 0.0;
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = z;   mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;
  
    mat
  }
    
  pub fn new_translation(x: Component, y: Component, z: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = 1.0; mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = x; 
    mat.data[0][1] = 0.0; mat.data[1][1] = 1.0; mat.data[2][1] = 0.0; mat.data[3][1] = y; 
    mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = 1.0; mat.data[3][2] = z; 
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;
  
    mat
  }
  
  pub fn new_rotation_x(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = 1.0; mat.data[1][0] = 0.0;            mat.data[2][0] = 0.0;           mat.data[3][0] = 0.0;
    mat.data[0][1] = 0.0; mat.data[1][1] = rad.cos();  mat.data[2][1] = rad.sin(); mat.data[3][1] = 0.0;
    mat.data[0][2] = 0.0; mat.data[1][2] = -rad.sin(); mat.data[2][2] = rad.cos(); mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0; mat.data[1][3] = 0.0;            mat.data[2][3] = 0.0;           mat.data[3][3] = 1.0;
  
    mat
  }

  pub fn new_rotation_y(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = rad.cos();  mat.data[1][0] = 0.0; mat.data[2][0] = rad.sin(); mat.data[3][0] = 0.0;  
    mat.data[0][1] = 0.0;            mat.data[1][1] = 1.0; mat.data[2][1] = 0.0;           mat.data[3][1] = 0.0;
    mat.data[0][2] = -rad.sin(); mat.data[1][2] = 0.0; mat.data[2][2] = rad.cos(); mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0;            mat.data[1][3] = 0.0; mat.data[2][3] = 0.0;           mat.data[3][3] = 1.0;
    
    mat
  }

  pub fn new_rotation_z(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = rad.cos();  mat.data[1][0] = rad.sin(); mat.data[2][0] = 0.0; mat.data[3][0] = 0.0;
    mat.data[0][1] = -rad.sin(); mat.data[1][1] = rad.cos(); mat.data[2][1] = 0.0; mat.data[3][1] = 0.0;
    mat.data[0][2] = 0.0;            mat.data[1][2] = 0.0;           mat.data[2][2] = 1.0; mat.data[3][2] = 0.0;
    mat.data[0][3] = 0.0;            mat.data[1][3] = 0.0;           mat.data[2][3] = 0.0; mat.data[3][3] = 1.0;
  
    mat
  }

  pub fn new_lookat(position: math::Vec3f, target: math::Vec3f, up: math::Vec3f) -> Mat4x4
  {
    let mut forward = target - position;
    forward.normalize();

    let mut side = forward.cross(&up);
    side.normalize();

    let mut proper_up = side.cross(&forward);
    proper_up.normalize();

    let mut mat = Mat4x4::new();
    mat.data[0][0] = side.x;      mat.data[1][0] = side.y;      mat.data[2][0] = side.z;      mat.data[3][0] = -side.dot(&position); 
    mat.data[0][1] = proper_up.x; mat.data[1][1] = proper_up.y; mat.data[2][1] = proper_up.z; mat.data[3][1] = -proper_up.dot(&position);
    mat.data[0][2] = -forward.x;  mat.data[1][2] = -forward.y;  mat.data[2][2] = -forward.z;  mat.data[3][2] = forward.dot(&position);
    mat.data[0][3] = 0.0;         mat.data[1][3] = 0.0;         mat.data[2][3] = 0.0;         mat.data[3][3] = 1.0;

    mat
  }

  pub fn get_width(&self) -> uint
  { return 4; }
  pub fn get_height(&self) -> uint
  { return 4; }

  pub fn get_up(&self) -> math::Vec3f
  { math::Vec3f::new(self.data[0][1], self.data[1][1], self.data[2][1]) }
  pub fn get_down(&self) -> math::Vec3f
  { math::Vec3f::new(-self.data[0][1], -self.data[1][1], -self.data[2][1]) }
  pub fn get_left(&self) -> math::Vec3f
  { math::Vec3f::new(-self.data[0][0], -self.data[1][0], -self.data[2][0]) }
  pub fn get_right(&self) -> math::Vec3f
  { math::Vec3f::new(self.data[0][0], self.data[1][0], self.data[2][0]) }
  pub fn get_forward(&self) -> math::Vec3f
  { math::Vec3f::new(-self.data[0][2], -self.data[1][2], -self.data[2][2]) }
  pub fn get_backward(&self) -> math::Vec3f
  { math::Vec3f::new(self.data[0][2], self.data[1][2], self.data[2][2]) }

  pub fn get_position(&self) -> math::Vec3f
  { math::Vec3f::new(self.data[3][0], self.data[3][1], self.data[3][2]) }

  pub fn identity(&mut self)
  {
    self.data = [ [1.0, 0.0, 0.0, 0.0],
                  [0.0, 1.0, 0.0, 0.0],
                  [0.0, 0.0, 1.0, 0.0],
                  [0.0, 0.0, 0.0, 1.0]];
  }

  pub unsafe fn to_ptr(&self) -> *Mat4x4
  { ptr::to_unsafe_ptr(self) }
}

/***** Operator Overloads *****/
impl Mul<Mat4x4, Mat4x4> for Mat4x4
{
  pub fn mul(&self, rhs: &Mat4x4) -> Mat4x4
  {
    let mut mat = Mat4x4::new();

    for i in range(0, 4)
    {
      for k in range(0, 4)
      {
        mat.data[i][k] =  self.data[i][0] * rhs.data[0][k] +
                          self.data[i][1] * rhs.data[1][k] +
                          self.data[i][2] * rhs.data[2][k] +
                          self.data[i][3] * rhs.data[3][k];
      }
    }

    mat
  }
}

