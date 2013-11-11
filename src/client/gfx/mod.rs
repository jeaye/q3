/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gfx/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of OpenGL and graphics items.
*/

#[link(name = "gfx", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];
#[feature(managed_boxes)];

extern mod opengles;
extern mod gl;
extern mod glfw;
extern mod stb_image;

extern mod log;
extern mod math;
extern mod console;

pub use self::camera::Camera;
pub use self::shader::{ Shader, Shader_Builder };
pub use self::texture::Texture;
pub use self::worker::Worker;
pub use self::vbo::VBO;
pub use self::vao::VAO;

pub mod camera;
pub mod shader;
pub mod texture;
pub mod worker;
pub mod vbo;
pub mod vao;
