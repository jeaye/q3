/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec2.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 2D vector with X and Y components.
*/

pub struct Vec2<T>
{
  x: T,
  y: T
}
impl<T: num::Zero> Vec2<T> 
{
  pub fn new(nx: T, ny: T) -> Vec2<T>
  { Vec2{ x: nx, y: ny } }

  pub fn zero() -> Vec2<T>
  { Vec2{ x: num::Zero::zero(), y: num::Zero::zero() } }

  pub unsafe fn to_ptr(&self) -> *Vec2<T>
  { ptr::addr_of(self) }
}

/***** Operator Overloads *****/
impl<T: Add<T, T>> Add<Vec2<T>, Vec2<T>> for Vec2<T>
{
  fn add(&self, rhs: &Vec2<T>) -> Vec2<T>
  {
    Vec2{ x: ( self.x + rhs.x ),
          y: ( self.y + rhs.y ) }
  }
}

impl<T: Sub<T, T>> Sub<Vec2<T>, Vec2<T>> for Vec2<T>
{
  fn sub(&self, rhs: &Vec2<T>) -> Vec2<T>
  {
    Vec2{ x: ( self.x - rhs.x ),
          y: ( self.y - rhs.y ) }
  }
}

impl<T: Neg<T>> Neg<Vec2<T>> for Vec2<T>
{
  fn neg(&self) -> Vec2<T>
  {
    Vec2{ x: ( -self.x ),
          y: ( -self.y ) }
  }
}

