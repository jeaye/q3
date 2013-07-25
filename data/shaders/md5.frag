/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/md5.frag
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The fragment shader for MD5 models.
*/

#version 330

uniform sampler2D tex0;

in vec2 trans_coord;
out vec4 out_color;

void main()
{
  out_color = texture(tex0, trans_coord); 
}

