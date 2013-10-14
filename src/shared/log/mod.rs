/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/log/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of logging items.
*/

#[link(name = "log", vers = "0.2")];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

extern mod extra;

pub use self::log::{ Log, Verbosity };
pub use self::log::{ VERBOSITY_DEBUG, VERBOSITY_INFO, VERBOSITY_ERROR, VERBOSITY_NONE };
pub use self::listener::Listener;

#[macro_escape]
pub mod log;
pub mod listener;

