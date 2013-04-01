/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec3.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 3D vector with X, Y, and Z components.
*/

pub struct Vec3<T>
{
  x: T,
  y: T,
  z: T
}

impl<T: num::Zero + Add<T, T> + Mul<T, T>> Vec3<T>
{
  pub fn new(nx: T, ny: T, nz: T) -> Vec3<T>
  { Vec3{ x: nx, y: ny, z: nz } }

  pub fn zero() -> Vec3<T>
  { Vec3{ x: num::Zero::zero(), y: num::Zero::zero(), z: num::Zero::zero() } }

  pub unsafe fn to_ptr(&self) -> *Vec3<T>
  { ptr::addr_of(self) }
}

/***** Operator Overloads *****/
impl<T: Add<T, T>> Add<Vec3<T>, Vec3<T>> for Vec3<T>
{
  fn add(&self, rhs: &Vec3<T>) -> Vec3<T>
  {
    Vec3{ x: ( self.x + rhs.x ),
          y: ( self.y + rhs.y ),
          z: ( self.z + rhs.z ) }
  }
}

impl<T: Sub<T, T>> Sub<Vec3<T>, Vec3<T>> for Vec3<T>
{
  fn sub(&self, rhs: &Vec3<T>) -> Vec3<T>
  {
    Vec3{ x: ( self.x - rhs.x ),
          y: ( self.y - rhs.y ),
          z: ( self.z - rhs.z ) }
  }
}

impl<T: Neg<T>> Neg<Vec3<T>> for Vec3<T>
{
  fn neg(&self) -> Vec3<T>
  {
    Vec3{ x: ( -self.x ),
          y: ( -self.y ),
          z: ( -self.z ) }
  }
}

