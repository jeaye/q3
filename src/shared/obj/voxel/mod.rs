/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/voxel/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of voxel-oriented items.
*/

pub use self::map::Map;
pub use self::vertex::Vertex;
pub use self::behavior::{ Visible };

pub mod map;
pub mod vertex;
pub mod behavior;

