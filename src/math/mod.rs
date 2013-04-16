/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of math items.
*/

pub use math::matrix::Mat4x4;
pub use math::vec2::{ Vec2i, Vec2f };
pub use math::vec3::Vec3f;
pub use math::vec4::{ Vec4u8, Vec4f };
pub use math::bb3::BB3;
pub use math::util::{ next_power_of_2 };

mod matrix;
mod vec2;
mod vec3;
mod vec4;
mod bb3;
mod util;

