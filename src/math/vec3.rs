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
      use std::{ cmp, ptr };

      pub struct $Type
      {
        x: $Component,
        y: $Component,
        z: $Component
      }

      impl $Type
      {
        pub fn new(nx: $Component, ny: $Component, nz: $Component) -> $Type
        { $Type{ x: nx, y: ny, z: nz } }

        pub fn zero() -> $Type
        { $Type{ x: 0 as $Component, y: 0 as $Component, z: 0 as $Component } }

        pub fn new_normalized(vec: &$Type) -> $Type
        {
          let mut n = *vec;
          n.normalize();

          n
        }

        pub fn normalize(&mut self)
        {
          let len = self.length();

          if (len as f32).approx_eq(&0.0)
          { return; }

          self.x /= len;
          self.y /= len;
          self.z /= len;
        }

        pub fn length(&self) -> $Component
        { (((self.x * self.x) + 
            (self.y * self.y) + 
            (self.z * self.z)) as float).sqrt() as $Component }

        pub fn cross(&self, rhs: &$Type) -> $Type
        {
          $Type { x: (self.y * rhs.z) - (self.z * rhs.y),
                  y: (self.z * rhs.x) - (self.x * rhs.z),
                  z: (self.x * rhs.y) - (self.y * rhs.x) }
        }

        pub fn dot(&self, rhs: &$Type) -> $Component
        { (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) }

        pub unsafe fn to_ptr(&self) -> *$Type
        { ptr::to_unsafe_ptr(self) } 

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

      impl cmp::Ord for $Type
      {
        fn lt(&self, other: &$Type) -> bool
        {
          if self.x < other.x
          { return true; }
          else if self.x > other.x
          { return false; }
          else if self.y < other.y
          { return true; }
          else if self.y > other.y
          { return false; }
          else if self.z < other.z
          { return true; }
          else if self.z > other.z
          { return false; }

          /* equal */
          false
        }
        fn le(&self, other: &$Type) -> bool
        { (self == other) || (self < other) }
        fn ge(&self, other: &$Type) -> bool
        { (self == other) || (self > other) }
        fn gt(&self, other: &$Type) -> bool
        { !(self == other) && !(self < other) }
      }

      impl cmp::TotalOrd for $Type
      {
        fn cmp(&self, other: &$Type) -> cmp::Ordering
        {
          if self < other
          { cmp::Less }
          else if self > other
          { cmp::Greater }
          else
          { cmp::Equal }
        }
      }

      impl cmp::Eq for $Type
      {
        fn eq(&self, other: &$Type) -> bool
        {
          (self.x as f32).approx_eq(&(other.x as f32)) && 
          (self.y as f32).approx_eq(&(other.y as f32)) && 
          (self.z as f32).approx_eq(&(other.z as f32))
        }
        fn ne(&self, other: &$Type) -> bool
        { !(self == other) }
      }

      impl cmp::TotalEq for $Type
      {
        fn equals(&self, other: &$Type) -> bool
        { self == other }
      }
    }
  );
)

declare!(Vec3f, vecf, f32)
declare!(Vec3i, veci, i32)
declare!(Vec3i8, veci8, i8)
declare!(Vec3u8, vecu8, u8)

