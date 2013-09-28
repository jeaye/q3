/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: math/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of math items.
*/

#[link(name = "math", vers = "0.2")];
#[crate_type = "lib"];

extern mod log;

pub use self::quaternion::*;
pub use self::matrix::*;
pub use self::vec2::*;
pub use self::vec3::*;
pub use self::vec4::*;
pub use self::bb3::*;
pub use self::util::*;

pub mod quaternion;
pub mod matrix;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod bb3;
pub mod util;

