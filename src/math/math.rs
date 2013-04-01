/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/math.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of math items.
*/

pub use math::matrix::Mat4x4;
pub use math::vec2::Vec2;
pub use math::vec3::Vec3;
pub use math::vec4::Vec4;
pub use math::bb3::BB3;

pub mod matrix;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod bb3;

/* TODO: These are being fixed. https://github.com/mozilla/rust/issues/5635 */
//pub type Vec2f = math::vec2::Vec2<f32>;
//pub type Vec3f = math::vec3::Vec3<f32>;
//pub type Vec4f = math::vec4::Vec4<f32>;

