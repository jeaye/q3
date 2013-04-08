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
pub use gl::camera::Camera;
pub use gl::shader::Shader;
pub use gl::util::*;

pub mod camera;
pub mod shader;
pub mod util;

