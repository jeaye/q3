/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/vec3.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A 3D vector with X, Y, and Z components.
*/

pub use self::vecf::Vec3f;
pub use self::veci::Vec3i;
pub use self::veci8::Vec3i8;
pub use self::vecu8::Vec3u8;

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
        z: $Component
      }

      impl $Type
      {
        #[inline(always)]
        pub fn new(nx: $Component, ny: $Component, nz: $Component) -> $Type
        { $Type{ x: nx, y: ny, z: nz } }

        #[inline(always)]
        pub fn zero() -> $Type
        { $Type{ x: 0 as $Component, y: 0 as $Component, z: 0 as $Component } }

        #[inline(always)]
        pub fn normalize(&mut self)
        {
          let mut len = self.length();

          if (len == 0 as $Component) || (len < 0.0001 as $Component && len > -0.0001 as $Component) /* TODO: Egh, hack. */
          { len = 1 as $Component; } /* TODO: Return? */

          self.x /= len;
          self.y /= len;
          self.z /= len;
        }

        #[inline(always)]
        pub fn length(&self) -> $Component
        { float::sqrt(( (self.x * self.x) + 
                        (self.y * self.y) + 
                        (self.z * self.z)) as float) as $Component }

        #[inline(always)]
        pub fn cross(&self, rhs: &$Type) -> $Type
        {
          $Type { x: (self.y * rhs.z) - (self.z * rhs.y),
                  y: (self.z * rhs.x) - (self.x * rhs.z),
                  z: (self.x * rhs.y) - (self.y * rhs.x) }
        }

        #[inline(always)]
        pub fn dot(&self, rhs: &$Type) -> $Component
        { (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) }

        #[inline(always)]
        pub unsafe fn to_ptr(&self) -> *$Type
        { ptr::to_unsafe_ptr(self) } 

        #[inline(always)]
        pub fn to_str(&self) -> ~str
        { fmt!("(%?, %?, %?)", self.x, self.y, self.z) }
      }

      /***** Operator Overloads *****/
      impl Add<$Type, $Type> for $Type
      {
        fn add(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x + rhs.x ),
                y: ( self.y + rhs.y ),
                z: ( self.z + rhs.z ) }
        }
      }

      impl Sub<$Type, $Type> for $Type
      {
        fn sub(&self, rhs: &$Type) -> $Type
        {
          $Type{x: ( self.x - rhs.x ),
                y: ( self.y - rhs.y ),
                z: ( self.z - rhs.z ) }
        }
      }

      impl Mul<$Component, $Type> for $Type
      {
        fn mul(&self, rhs: &$Component) -> $Type
        {
          $Type{x: ( self.x * *rhs ),
                y: ( self.y * *rhs ),
                z: ( self.z * *rhs ) }
        }
      }

      impl Neg<$Type> for $Type
      {
        fn neg(&self) -> $Type
        {
          $Type{x: ( -self.x ),
                y: ( -self.y ),
                z: ( -self.z ) }
        }
      }

      impl Index<uint, $Component> for $Type
      {
        fn index(&self, rhs: &uint) -> $Component
        {
          match rhs
          {
            &0 => { self.x }
            &1 => { self.y }
            &2 => { self.z }
            _ => { fail!(~"Invalid index to Vec3"); }
          }
        }
      }
    }
  );
)

declare!(Vec3f, vecf, f32)
declare!(Vec3i, veci, i32)
declare!(Vec3i8, veci8, i8)
declare!(Vec3u8, vecu8, u8)

