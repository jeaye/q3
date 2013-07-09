/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec2.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 2D vector with X and Y components.
*/

pub use self::vecf::Vec2f;
pub use self::veci::Vec2i;
pub use self::vecu8::Vec2u8;

macro_rules! declare
(
  ($Type:ident, $Mod:ident, $Component:ty) =>
  (
    mod $Mod
    {
      use std::{ float, ptr };

      pub struct $Type
      {
        x: $Component,
        y: $Component,
      }

      impl $Type
      {
        pub fn new(nx: $Component, ny: $Component) -> $Type
        { $Type{ x: nx, y: ny } }

        pub fn zero() -> $Type
        { $Type{ x: 0 as $Component, y: 0 as $Component } }

        pub fn normalize(&mut self)
        {
          let len = self.length();

          if (len as f32).approx_eq(&0.0)
          { return; }

          self.x /= len;
          self.y /= len;
        }

        pub fn length(&self) -> $Component
        { float::sqrt(( (self.x * self.x) + 
                        (self.y * self.y)) as float) as $Component }

        pub unsafe fn to_ptr(&self) -> *$Type
        { ptr::to_unsafe_ptr(self) } 

        pub fn to_str(&self) -> ~str
        { fmt!("(%?, %?)", self.x, self.y) }
      }

      /***** Operator Overloads *****/
      impl Add<$Type, $Type> for $Type
      {
        fn add(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x + rhs.x ),
                y: ( self.y + rhs.y ) }
        }
      }

      impl Sub<$Type, $Type> for $Type
      {
        fn sub(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x - rhs.x ),
                y: ( self.y - rhs.y ) }
        }
      }

      impl Mul<$Component, $Type> for $Type
      {
        fn mul(&self, rhs: &$Component) -> $Type
        {
          $Type{x: ( self.x * *rhs ),
                y: ( self.y * *rhs ) }
        }
      }

      impl Neg<$Type> for $Type
      {
        fn neg(&self) -> $Type
        {
          $Type{x: ( -self.x ),
                y: ( -self.y ) }
        }
      }
    }
  );
)

declare!(Vec2f, vecf, f32)
declare!(Vec2i, veci, i32)
declare!(Vec2u8, vecu8, u8)

