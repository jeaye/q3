/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of OpenGL items.
*/

extern mod glfw;
extern mod opengles;

pub use glfw::*;
pub use opengles::gl2::*;
pub use self::camera::Camera;
pub use self::shader::Shader;
pub use self::util::*;

mod camera;
mod shader;
mod util;

