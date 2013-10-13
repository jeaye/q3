/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/mod.rs
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

extern mod log;
extern mod math;

pub use BSP_Map = self::bsp::map::Map;
pub use Voxel_Map = self::voxel::map::Map;

pub mod bsp;
pub mod primitive;
pub mod voxel;

