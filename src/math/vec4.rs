/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec4.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 4D vector with X, Y, Z, and W components.
*/

pub use self::vecf::Vec4f;
pub use self::vecu8::Vec4u8;

macro_rules! declare
(
  ($Type:ident, $Mod:ident, $Component:ty) =>
  (
    mod $Mod
    {
      use std::{ ptr };

      pub struct $Type
      {
        x: $Component,
        y: $Component,
        z: $Component,
        w: $Component,
      }

      impl $Type
      {
        pub fn new(nx: $Component, ny: $Component, nz: $Component, nw: $Component) -> $Type
        { $Type{ x: nx, y: ny, z: nz, w: nw } }

        pub fn zero() -> $Type
        { $Type{ x: 0 as $Component, y: 0 as $Component, z: 0 as $Component, w: 0 as $Component } }

        pub fn normalize(&mut self)
        {
          let len = self.length();

          if (len as f32).approx_eq(&0.0)
          { return; }

          self.x /= len;
          self.y /= len;
          self.z /= len;
          self.w /= len;
        }

        pub fn length(&self) -> $Component
        { (((self.x * self.x) + 
            (self.y * self.y) + 
            (self.z * self.z) +
            (self.w * self.w)) as float).sqrt() as $Component }

        pub unsafe fn to_ptr(&self) -> *$Type
        { ptr::to_unsafe_ptr(self) } 

        pub fn to_str(&self) -> ~str
        { format!("({}, {}, {}, {})", self.x, self.y, self.z, self.w) }
      }

      /***** Operator Overloads *****/
      impl Add<$Type, $Type> for $Type
      {
        fn add(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x + rhs.x ),
                y: ( self.y + rhs.y ),
                z: ( self.z + rhs.z ),
                w: ( self.w + rhs.w ) }
        }
      }

      impl Sub<$Type, $Type> for $Type
      {
        fn sub(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x - rhs.x ),
                y: ( self.y - rhs.y ),
                z: ( self.z - rhs.z ),
                w: ( self.w - rhs.w ) }
        }
      }

      impl Mul<$Component, $Type> for $Type
      {
        fn mul(&self, rhs: &$Component) -> $Type
        {
          $Type{x: ( self.x * *rhs ),
                y: ( self.y * *rhs ),
                z: ( self.z * *rhs ),
                w: ( self.w * *rhs ) }
        }
      }

      impl Neg<$Type> for $Type
      {
        fn neg(&self) -> $Type
        {
          $Type{x: ( -self.x ),
                y: ( -self.y ),
                z: ( -self.z ),
                w: ( -self.w ) }
        }
      }
    }
  );
)

declare!(Vec4f, vecf, f32)
declare!(Vec4u8, vecu8, u8)

