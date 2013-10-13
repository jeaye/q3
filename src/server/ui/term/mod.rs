/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/ui/term/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of terminal UI items.
*/

pub use self::root::initialize;

pub mod root;
pub mod view;
pub mod home;

