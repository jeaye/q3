/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of voxel-oriented items.
*/

pub use self::map::Map;
pub use self::vertex::Vertex;
pub use self::behavior::{ Behavior, Invisible, Default };

mod map;
mod vertex;
mod behavior;

