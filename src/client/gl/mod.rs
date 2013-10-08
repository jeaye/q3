/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gl/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of OpenGL items.
*/

#[link(name = "gl", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

extern mod opengles;
extern mod glfw;
extern mod stb_image;

extern mod log;
extern mod math;
extern mod console;

pub use self::camera::Camera;
pub use self::shader::{ Shader, Shader_Builder };
pub use self::texture::Texture;
pub use self::worker::Worker;

pub mod camera;
pub mod shader;
pub mod texture;
pub mod worker;

