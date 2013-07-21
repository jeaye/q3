/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/quaternion.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Quaternions are magic mathematical objects
      which can represent an orientation.
*/

use std::{ f32, cmp };
use math;

type Component = f32;

#[deriving(Clone)]
struct Quaternion
{
  x: Component,
  y: Component,
  z: Component,
  w: Component,
}

impl Quaternion
{
  pub fn new(x: Component, y: Component, z: Component, w: Component) -> Quaternion
  { Quaternion { x: x, y: y, z: z, w: w } }

  pub fn new_from_vec(vec: &math::Vec3f) -> Quaternion
  { Quaternion { x: vec.x, y: vec.y, z: vec.z, w: 0.0 } }

  pub fn zero() -> Quaternion
  { Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 } }

  pub fn new_from_axis(axis: &math::Vec3f, angle: f32) -> Quaternion
  {
    let sin_angle = angle.sin();
    let norm_axis = math::Vec3f::new_normalized(axis);

    Quaternion {  x: (norm_axis.x * sin_angle),
                  y: (norm_axis.y * sin_angle),
                  z: (norm_axis.z * sin_angle),
                  w: angle.cos() }
  }

  pub fn new_from_euler(yaw: f32, pitch: f32, roll: f32) -> Quaternion
  {
    let p = pitch * (f32::consts::pi / 180.0) / 2.0;
    let y = yaw * (f32::consts::pi / 180.0) / 2.0;
    let r = roll * (f32::consts::pi / 180.0) / 2.0;

    let sinp = p.sin();
    let siny = y.sin();
    let sinr = r.sin();
    let cosp = p.cos();
    let cosy = y.cos();
    let cosr = r.cos();

    let mut q = Quaternion::new
    (
      sinr * cosp * cosy - cosr * sinp * siny,
      cosr * sinp * cosy + sinr * cosp * siny,
      cosr * cosp * siny - sinr * sinp * cosy,
      cosr * cosp * cosy + sinr * sinp * siny
    );
    q.normalize();

    q
  }

  pub fn get_conjugate(&self) -> Quaternion
  { Quaternion { x: -self.x, y: -self.y, z: -self.z, w: -self.w } }

  pub fn normalize(&mut self)
  {
    let mag2 = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
    if mag2.abs() > 0.00001 && (mag2 - 1.0).abs() > 0.00001
    {
      let mag = mag2.sqrt();
      self.x /= mag;
      self.y /= mag;
      self.z /= mag;
      self.w /= mag;
    }
  }

  pub fn compute_w(&mut self)
  {
    let t = 1.0 - (self.x * self.x) - (self.y * self.y) - (self.z * self.z);

    if t < 0.0
    { self.w = 0.0; }
    else
    { self.w = -t.sqrt(); }
  }

  pub fn translate_vec(&self, vec: &math::Vec3f) -> math::Vec3f
  {
    let vn = math::Vec3f::new_normalized(vec);
    let vecq = Quaternion::new(vn.x, vn.y, vn.z, 0.0);
    let mut resq = vecq;
    resq * self.get_conjugate();
    resq = resq * *self;

    math::Vec3f::new(resq.x, resq.y, resq.z)
  }

  pub fn scale(&mut self, scalar: Component)
  {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
    self.w *= scalar;
  }

  pub fn to_vec(&self) -> math::Vec3f
  { math::Vec3f::new(self.x, self.y, self.z) }

  pub fn to_mat(&self) -> math::Mat4x4
  {
    let x2 = self.x * self.x;
    let y2 = self.y * self.y;
    let z2 = self.z * self.z;
    let xy = self.x * self.y;
    let xz = self.x * self.z;
    let yz = self.y * self.z;
    let wx = self.w * self.x;
    let wy = self.w * self.y;
    let wz = self.w * self.z;

    /* Magic. */
    math::Mat4x4
    {
      data:
      [
        [ 1.0 - 2.0 * (y2 + z2), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0 ],
        [ 2.0 * (xy + wz), 1.0 - 2.0 * (x2 + z2), 2.0 * (yz - wx), 0.0 ],
        [ 2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (x2 + y2), 0.0 ],
        [ 0.0, 0.0, 0.0, 1.0 ]
      ]
    }
  }
}

impl Mul<Quaternion, Quaternion> for Quaternion
{
  fn mul(&self, rhs: &Quaternion) -> Quaternion
  {
    Quaternion
    {
      x: self.w * rhs.x + self.x * self.w + self.y * rhs.z - self.z * rhs.y,
      y: self.w * rhs.y + self.y * self.w + self.z * rhs.x - self.x * rhs.z,
      z: self.w * rhs.z + self.z * self.w + self.x * rhs.y - self.y * rhs.x,
      w: self.w * rhs.w - self.x * self.x - self.y * rhs.y - self.z * rhs.z
    }
  }
}

impl cmp::Eq for Quaternion
{
  fn eq(&self, other: &Quaternion) -> bool
  {
    self.x.approx_eq(&other.x) && 
    self.y.approx_eq(&other.y) && 
    self.z.approx_eq(&other.z) &&
    self.w.approx_eq(&other.w)
  }

  fn ne(&self, other: &Quaternion) -> bool
  { !(self == other) }
}

impl cmp::TotalEq for Quaternion
{
  fn equals(&self, other: &Quaternion) -> bool
  { self == other }
}

