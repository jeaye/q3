/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/gl/ttf/renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A TTF font renderer.
*/

use std::{ vec, sys };
use gl;
use gl2 = opengles::gl2;
use super::Font;
use math;
use log::Log;

#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

#[macro_escape]
#[path = "../../../shared/log/macros.rs"]
mod macros;

#[packed]
struct Point
{
  x: f32, y: f32,
  u: f32, v: f32,
  r: f32, g: f32, b: f32,
}
impl Point
{
  pub fn new(nx: f32, ny: f32, nu: f32, nv: f32, nr: f32, ng: f32, nb: f32) -> Point
  { Point { x: nx, y: ny, u: nu, v: nv, r: nr, g: ng, b: nb } }
}

struct Renderer
{
  vao: gl2::GLuint,
  vbo: gl2::GLuint,
  shader: @mut gl::Shader,
  proj_loc: gl2::GLint,
}

impl Renderer
{
  pub fn new() -> Renderer
  {
    let mut renderer = Renderer
    {
        vao: 0,
        vbo: 0,
        shader: gl::Shader_Builder::new_with_files("data/shaders/text.vert", "data/shaders/text.frag"),
        proj_loc: 0,
    };
    renderer.proj_loc = renderer.shader.get_uniform_location("proj");
    let tex_loc = renderer.shader.get_uniform_location("tex0"); 
    renderer.shader.bind();
    renderer.shader.update_uniform_i32(tex_loc, 0);

    let name = check!(gl2::gen_vertex_arrays(1));
    log_assert!(name.len() == 1);
    renderer.vao = name[0];
    check!(gl2::bind_vertex_array(renderer.vao));

    let name = check!(gl2::gen_buffers(1));
    log_assert!(name.len() == 1);
    renderer.vbo = name[0];
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, renderer.vbo));

    let data: ~[u8] = ~[];
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, data, gl2::STREAM_DRAW));

    /* Setup vertex attribs. */
    check!(gl2::enable_vertex_attrib_array(0));
    check!(gl2::vertex_attrib_pointer_f32(0, 4, false, sys::size_of::<Point>() as i32, 0));
    check!(gl2::enable_vertex_attrib_array(1));
    check!(gl2::vertex_attrib_pointer_f32(1, 3, false, sys::size_of::<Point>() as i32, (sys::size_of::<f32>() * 4) as u32));

    renderer
  }

  pub fn begin(&mut self)
  {
    check!(gl2::disable(gl2::DEPTH_TEST));

    check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_WRAP_S, gl2::CLAMP_TO_EDGE as i32));
    check!(gl2::tex_parameter_i(gl2::TEXTURE_2D, gl2::TEXTURE_WRAP_T, gl2::CLAMP_TO_EDGE as i32));

    /* Enable transparency. */
    check!(gl2::enable(gl2::BLEND));
    check!(gl2::blend_func(gl2::SRC_ALPHA, gl2::ONE_MINUS_SRC_ALPHA));

    self.shader.bind();
    let camera = gl::Camera::get_active();
    let proj = math::Mat4x4::new_orthographic(0.0, camera.window_size.x as f32, camera.window_size.y as f32, 0.0,  1.0, 100.0);
    self.shader.update_uniform_mat(self.proj_loc, &proj);
  }

  pub fn end(&mut self)
  {
    check!(gl2::enable(gl2::DEPTH_TEST));
    check!(gl2::disable(gl2::BLEND));
  }

  pub fn render(&mut self, text: &str, pos: math::Vec2f, font: &Font)
  {
    check!(gl2::bind_texture(gl2::TEXTURE_2D, font.texture_atlas));

    check!(gl2::bind_vertex_array(self.vao));

    let mut color = math::Vec3f::new(1.0, 1.0, 1.0);
    let mut color_str = ~"";
    let mut expecting_color = false;

    /* Render each line separately. */
    let mut line_count = 0;
    for line in text.line_iter()
    {
      line_count += 1;
      let mut coords = vec::with_capacity::<Point>(line.len());
      let mut temp_pos = pos;
      temp_pos.y += (font.height * line_count) as f32;

      let mut count = 0;
      for curr in line.iter()
      {
        let glyph = match font.glyphs.find(&(curr as u8))
        {
          Some(g) => g,
          None => log_fail!("Invalid char (%c) in font %s len %u", curr, font.file, font.glyphs.len())
        };

        /* Parse color. */
        if expecting_color
        {
          /* Build the number string. */
          if curr.is_digit()
          {
            color_str.push_char(curr);
            continue;
          }
          expecting_color = false;

          /* Convert char to int. */
          let num = FromStr::from_str(color_str);
          match num
          {
            Some(val) => { color = self.get_color(val); },
            None => { }
          }
        }
        if curr == '\\'
        {
          expecting_color = true;
          color_str.clear();
          color_str.reserve(2);
          continue;
        }

        let end_x = temp_pos.x + glyph.offset.x;
        let end_y = -temp_pos.y - (glyph.dimensions.y - glyph.offset.y);
        let end_w = glyph.dimensions.x;
        let end_h = glyph.dimensions.y;

        temp_pos.x += glyph.advance.x; 
        temp_pos.y += glyph.advance.y; 

        /* Skip empty glyphs. */
        if end_w <= 0.1 || end_h <= 0.1
        { continue; }

        coords.push(Point::new(end_x, -end_y - end_h, glyph.tex.x, glyph.tex.y, color.x, color.y, color.z));
        coords.push(Point::new(end_x, -end_y, glyph.tex.x, glyph.tex.y + (end_h / (font.atlas_dimensions.y as f32)), color.x, color.y, color.z));
        coords.push(Point::new(end_x + end_w, -end_y, glyph.tex.x + (end_w / (font.atlas_dimensions.x as f32)), glyph.tex.y + (end_h / (font.atlas_dimensions.y as f32)), color.x, color.y, color.z));
        coords.push(Point::new(end_x, -end_y - end_h, glyph.tex.x, glyph.tex.y, color.x, color.y, color.z));
        coords.push(Point::new(end_x + end_w, -end_y, glyph.tex.x + (end_w / (font.atlas_dimensions.x as f32)), glyph.tex.y + (end_h / (font.atlas_dimensions.y as f32)), color.x, color.y, color.z));
        coords.push(Point::new(end_x + end_w, -end_y - end_h, glyph.tex.x + (end_w / (font.atlas_dimensions.x as f32)), glyph.tex.y, color.x, color.y, color.z));
        count += 6;
      }

      check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.vbo));
      check!(gl2::buffer_data(gl2::ARRAY_BUFFER, coords, gl2::STREAM_DRAW)); 
      check!(gl2::draw_arrays(gl2::TRIANGLES, 0, count));
      check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, 0));
    }

    check!(gl2::bind_vertex_array(0));
  }

  /* Standard Quake colors. */
  fn get_color(&self, name: u32) -> math::Vec3f
  {
    match name
    {
      /* Black */
      0 => { math::Vec3f::new(0.0, 0.0, 0.0) },
      /* White */
      1 => { math::Vec3f::new(1.0, 1.0, 1.0) },
      /* Red */
      2 => { math::Vec3f::new(1.0, 0.0, 0.0) },
      /* Lime */
      3 => { math::Vec3f::new(0.0, 1.0, 0.0) },
      /* Blue */
      4 => { math::Vec3f::new(0.0, 0.0, 1.0) },
      /* Yellow */
      5 => { math::Vec3f::new(1.0, 1.0, 0.0) },
      /* Cyan */
      6 => { math::Vec3f::new(0.0, 1.0, 1.0) },
      /* Magenta */
      7 => { math::Vec3f::new(1.0, 0.0, 1.0) },
      /* Silver */
      8 => { math::Vec3f::new(0.75, 0.75, 0.75) },
      /* Gray */
      9 => { math::Vec3f::new(0.5, 0.5, 0.5) },
      /* Maroon */
      10 => { math::Vec3f::new(0.5, 0.0, 0.0) },
      /* Olive */
      11 => { math::Vec3f::new(0.5, 0.5, 0.0) },
      /* Green */
      12 => { math::Vec3f::new(0.0, 0.5, 0.0) },
      /* Purple */
      13 => { math::Vec3f::new(0.5, 0.0, 0.5) },
      /* Teal */
      14 => { math::Vec3f::new(0.0, 0.5, 0.5) },
      /* Navy */
      15 => { math::Vec3f::new(0.0, 0.0, 0.5) },
      /* Default */
      _ => { math::Vec3f::new(1.0, 1.0, 1.0) },
    }
  }
}

#[unsafe_destructor]
impl Drop for Renderer
{
  fn drop(&mut self)
  {
    check!(gl2::delete_vertex_arrays(&[self.vao]));
    check!(gl2::delete_buffers(&[self.vbo]));
  }
}

