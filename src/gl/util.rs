/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/util.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A collection of helpful OpenGL items.
*/

extern mod opengles;
use gl = opengles::gl2;

pub fn get_err_str(err: u32) -> ~str
{
   match err
   {
     gl::INVALID_ENUM => { ~"Invalid enum" },
     gl::INVALID_VALUE => { ~"Invalid value" },
     gl::INVALID_OPERATION => { ~"Invalid operation" },
     gl::INVALID_FRAMEBUFFER_OPERATION => { ~"Invalid frame buffer operation" },
     gl::OUT_OF_MEMORY => { ~"Out of memory" },
     gl::STACK_UNDERFLOW => { ~"Stack underflow" },
     gl::STACK_OVERFLOW => { ~"Stack overflow" },
     _ => { ~"Unknown error" }
   }
}

