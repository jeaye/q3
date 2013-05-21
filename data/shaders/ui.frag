/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: data/shaders/ui.frag
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      The vertex shader program for rendering textured
      UI elements.
*/

#version 330

uniform sampler2D texture0;

in vec2 trans_coord;
in float trans_alpha;

out vec4 frag_color;

void main()
{
  frag_color = texture2D(texture0, trans_coord);
  frag_color.a *= trans_alpha;
}

