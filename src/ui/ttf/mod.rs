/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/ttf/mod.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      An aggregator of TTF items.
*/

pub use self::font::Font;
pub use self::renderer::Renderer;

mod font;
mod renderer;

