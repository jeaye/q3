/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: server/ui/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of UI items.
*/

#[link(name = "ui", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

extern mod ncurses;

pub use term::root::initialize;
pub use driver::Driver;

pub mod term;
pub mod driver;

