/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/text.frag
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The fragment shader program for rendering colored
      TTF text.
*/

#version 330

uniform sampler2D tex0;
uniform vec4 color0;

in vec2 trans_coord;
out vec4 out_color;

void main()
{
  //out_color = vec4(1, 1, 1, texture2D(tex0, trans_coord).r) * color0;
  out_color = vec4(1, 1, 1, texture(tex0, trans_coord).r); 
}

