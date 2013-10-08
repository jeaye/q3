/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Aggregator for modules related to
      game objects.
*/

#[link(name = "obj", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

extern mod extra;
extern mod opengles;

extern mod log;
extern mod math;
extern mod gl;

pub mod bsp;
pub mod md5;
pub mod primitive;
pub mod voxel;

