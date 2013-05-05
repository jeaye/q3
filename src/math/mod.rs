/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of math items.
*/

pub use self::matrix::Mat4x4;
pub use self::vec2::{ Vec2i, Vec2f };
pub use self::vec3::{ Vec3i, Vec3i8, Vec3u8, Vec3f };
pub use self::vec4::{ Vec4u8, Vec4f };
pub use self::bb3::BB3;
pub use self::util::{ next_power_of_2 };

mod matrix;
mod vec2;
mod vec3;
mod vec4;
mod bb3;
mod util;

