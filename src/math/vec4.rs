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
      use std::{ float, ptr };

      pub struct $Type
      {
        x: $Component,
        y: $Component,
        z: $Component,
        w: $Component,
      }

      impl $Type
      {
        #[inline(always)]
        pub fn new(nx: $Component, ny: $Component, nz: $Component, nw: $Component) -> $Type
        { $Type{ x: nx, y: ny, z: nz, w: nw } }

        #[inline(always)]
        pub fn zero() -> $Type
        { $Type{ x: 0 as $Component, y: 0 as $Component, z: 0 as $Component, w: 0 as $Component } }

        #[inline(always)]
        pub fn normalize(&mut self)
        {
          let mut len = self.length();

          if (len == 0 as $Component) || (len < 0.0001 as $Component && len > -0.0001 as $Component) /* TODO: Egh, hack. */
          { len = 1 as $Component; } /* TODO: Return? */

          self.x /= len;
          self.y /= len;
          self.z /= len;
          self.w /= len;
        }

        #[inline(always)]
        pub fn length(&self) -> $Component
        { float::sqrt(( (self.x * self.x) + 
                        (self.y * self.y) + 
                        (self.z * self.z) +
                        (self.w * self.w)) as float) as $Component }

        #[inline(always)]
        pub unsafe fn to_ptr(&self) -> *$Type
        { ptr::to_unsafe_ptr(self) } 

        #[inline(always)]
        pub fn to_str(&self) -> ~str
        { fmt!("(%?, %?, %?, %?)", self.x, self.y, self.z, self.w) }
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

