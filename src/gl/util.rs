/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/util.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A collection of helpful OpenGL items.
*/

use gl2 = opengles::gl2;

pub fn get_err_str(err: u32) -> ~str
{
   match err
   {
     gl2::INVALID_ENUM => { ~"Invalid enum" },
     gl2::INVALID_VALUE => { ~"Invalid value" },
     gl2::INVALID_OPERATION => { ~"Invalid operation" },
     gl2::INVALID_FRAMEBUFFER_OPERATION => { ~"Invalid frame buffer operation" },
     gl2::OUT_OF_MEMORY => { ~"Out of memory" },
     gl2::STACK_UNDERFLOW => { ~"Stack underflow" },
     gl2::STACK_OVERFLOW => { ~"Stack overflow" },
     _ => { ~"Unknown error" }
   }
}

