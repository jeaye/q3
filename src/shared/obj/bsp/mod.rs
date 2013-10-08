/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/bsp/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of Quake BSP items.
*/

pub use self::map::Map;
pub use self::lump::Lump;

pub mod map;
pub mod lump;

