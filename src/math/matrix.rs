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

type Component = f32; /* TODO: Template. */

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

  pub fn new_perspective_projection(fov: Component, aspect_ratio: Component, near: Component, far: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();

    let z_range: Component = near - far;
    let tan_half_fov: Component = f32::tan((3.14159 * (fov / 2.0)) / 180.0);

    mat.data[0][0] = 1.0/(tan_half_fov * aspect_ratio); mat.data[0][1] = 0.0;mat.data[0][2] = 0.0;mat.data[0][3] = 0.0;
    mat.data[1][0] = 0.0;mat.data[1][1] = 1.0/tan_half_fov; mat.data[1][2] = 0.0;mat.data[1][3] = 0.0;
    mat.data[2][0] = 0.0;mat.data[2][1] = 0.0;mat.data[2][2] = (-near -far)/z_range ; mat.data[2][3] = 2.0 * far*near/z_range;
    mat.data[3][0] = 0.0;mat.data[3][1] = 0.0;mat.data[3][2] = 1.0;mat.data[3][3] = 0.0;

    mat
  }
  
  pub fn new_scale(x: Component, y: Component, z: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = x;      mat.data[0][1] = 0.0;   mat.data[0][2] = 0.0;   mat.data[0][3] = 0.0;
    mat.data[1][0] = 0.0;   mat.data[1][1] = y;      mat.data[1][2] = 0.0;   mat.data[1][3] = 0.0;
    mat.data[2][0] = 0.0;   mat.data[2][1] = 0.0;   mat.data[2][2] = z;      mat.data[2][3] = 0.0;
    mat.data[3][0] = 0.0;   mat.data[3][1] = 0.0;   mat.data[3][2] = 0.0;   mat.data[3][3] = 1.0;
    mat
  }
    
  pub fn new_translation(x: Component, y: Component, z: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    mat.data[0][0] = 1.0;      mat.data[0][1] = 0.0;   mat.data[0][2] = 0.0;   mat.data[0][3] = 0.0;
    mat.data[1][0] = 0.0;   mat.data[1][1] = 1.0;      mat.data[1][2] = 0.0;   mat.data[1][3] = 0.0;
    mat.data[2][0] = 0.0;   mat.data[2][1] = 0.0;   mat.data[2][2] = 1.0;      mat.data[2][3] = 0.0;
    mat.data[3][0] = x;   mat.data[3][1] = y;   mat.data[3][2] = z;   mat.data[3][3] = 1.0;
    mat
  }
  
  pub fn new_rotation_x(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = 1.0; mat.data[0][1] = 0.0   ; mat.data[0][2] = 0.0    ; mat.data[0][3] = 0.0;
    mat.data[1][0] = 0.0; mat.data[1][1] = f32::cos(rad); mat.data[1][2] = -f32::sin(rad); mat.data[1][3] = 0.0;
    mat.data[2][0] = 0.0; mat.data[2][1] = f32::sin(rad); mat.data[2][2] = f32::cos(rad) ; mat.data[2][3] = 0.0;
    mat.data[3][0] = 0.0; mat.data[3][1] = 0.0   ; mat.data[3][2] = 0.0    ; mat.data[3][3] = 1.0;

    mat
  }

  pub fn new_rotation_y(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = f32::cos(rad); mat.data[0][1] = 0.0; mat.data[0][2] = -f32::sin(rad); mat.data[0][3] = 0.0;
    mat.data[1][0] = 0.0   ; mat.data[1][1] = 1.0; mat.data[1][2] = 0.0    ; mat.data[1][3] = 0.0;
    mat.data[2][0] = f32::sin(rad); mat.data[2][1] = 0.0; mat.data[2][2] = f32::cos(rad) ; mat.data[2][3] = 0.0;
    mat.data[3][0] = 0.0   ; mat.data[3][1] = 0.0; mat.data[3][2] = 0.0    ; mat.data[3][3] = 1.0;

    mat
  }

  pub fn new_rotation_z(deg: Component) -> Mat4x4
  {
    let mut mat = Mat4x4::new();
    let rad: Component = (3.14159 * deg) / 180.0;

    mat.data[0][0] = f32::cos(rad); mat.data[0][1] = -f32::sin(rad); mat.data[0][2] = 0.0; mat.data[0][3] = 0.0;
    mat.data[1][0] = f32::sin(rad); mat.data[1][1] = f32::cos(rad) ; mat.data[1][2] = 0.0; mat.data[1][3] = 0.0;
    mat.data[2][0] = 0.0   ; mat.data[2][1] = 0.0    ; mat.data[2][2] = 1.0; mat.data[2][3] = 0.0;
    mat.data[3][0] = 0.0   ; mat.data[3][1] = 0.0    ; mat.data[3][2] = 0.0; mat.data[3][3] = 1.0;

    mat
  }

  pub fn get_width(&self) -> uint
  { return 4; }
  pub fn get_height(&self) -> uint
  { return 4; }

  pub fn identity(&mut self)
  {
    self.data = [ [1.0, 0.0, 0.0, 0.0],
                  [0.0, 1.0, 0.0, 0.0],
                  [0.0, 0.0, 1.0, 0.0],
                  [0.0, 0.0, 0.0, 1.0]];
  }

  pub unsafe fn to_ptr(&self) -> *Mat4x4
  { return ptr::addr_of(self); }

  pub fn show(&self)
  {
    let mut y = 0, x;
    io::println("----------");
    while y < 4
    {
      x = 0;
      io::print("|");
      while x < 4
      {
        io::print(f32::to_str(self.data[x][y]) + " ");
        x += 1;
      }
      io::println("|");
      y += 1;
    }
    io::println("----------");
  }
}

/***** Operator Overloads *****/
impl Mul<Mat4x4, Mat4x4> for Mat4x4
{
  pub fn mul(&self, rhs: &Mat4x4) -> Mat4x4
  {
    let mut mat = unsafe{ Mat4x4::new() };

    let mut i = 0, j;
    while i  < 4 /* TODO: f32::range */
    {
      j = 0;
      while j < 4
      {
        mat.data[i][j] =  self.data[i][0] * rhs.data[0][j] +
                          self.data[i][1] * rhs.data[1][j] +
                          self.data[i][2] * rhs.data[2][j] +
                          self.data[i][3] * rhs.data[3][j];
        j += 1;
      }
      i += 1;
    }

    mat
  }
}

