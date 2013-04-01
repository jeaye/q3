/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec4.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 4D vector with X, Y, Z, and W components.
*/

pub struct Vec4<T>
{
  x: T,
  y: T,
  z: T,
  w: T
}

impl<T: num::Zero + Add<T, T> + Mul<T, T>> Vec4<T>
{
  pub fn new(nx: T, ny: T, nz: T, nw: T) -> Vec4<T>
  { Vec4{ x: nx, y: ny, z: nz, w: nw } }

  pub fn zero() -> Vec4<T>
  { Vec4{ x: num::Zero::zero(), y: num::Zero::zero(), z: num::Zero::zero(), w: num::Zero::zero() } }

  pub unsafe fn to_ptr(&self) -> *Vec4<T>
  {
    ptr::addr_of(self)
  }
}

/***** Operator Overloads *****/
impl<T: Add<T, T>> Add<Vec4<T>, Vec4<T>> for Vec4<T>
{
  fn add(&self, rhs: &Vec4<T>) -> Vec4<T>
  {
    Vec4{ x: ( self.x + rhs.x ),
          y: ( self.y + rhs.y ),
          z: ( self.z + rhs.z ),
          w: ( self.w + rhs.w )}
  }
}

impl<T: Sub<T, T>> Sub<Vec4<T>, Vec4<T>> for Vec4<T>
{
  fn sub(&self, rhs: &Vec4<T>) -> Vec4<T>
  {
    Vec4{ x: ( self.x - rhs.x ),
          y: ( self.y - rhs.y ),
          z: ( self.z - rhs.z ),
          w: ( self.w - rhs.w )}
  }
}

impl<T: Neg<T> + Copy> Neg<Vec4<T>> for Vec4<T>
{
  fn neg(&self) -> Vec4<T>
  {
    Vec4{ x: ( -self.x ),
          y: ( -self.y ),
          z: ( -self.z ),
          w: ( -self.w )}
  }
}

