/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/ui/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of UI items.
*/

#[link(name = "ui", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];
#[feature(managed_boxes)];

extern mod opengles;
extern mod glfw;
extern mod stb_image;

extern mod log;
extern mod math;
extern mod gfx;

pub use self::renderer::Renderer;
pub use self::ttf::Font;
pub use self::input::{ Input_Listener };

pub mod renderer;
pub mod ttf;
pub mod input;

