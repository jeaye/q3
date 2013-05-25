/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: ui/renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A UI component renderer.
*/

use gl::{ Shader, Shader_Builder, Texture, Camera };
use math::{ Vec2f, Mat4x4 };
use TTF_Renderer = super::ttf::Renderer;
use TTF_Font = super::ttf::Font;

#[path = "../gl/mod.rs"]
mod gl;
#[path = "../gl/util.rs"]
mod util;
#[path = "../gl/check.rs"]
mod check;

struct Renderer
{
  vao: gl::GLuint,
  vbo: gl::GLuint,

  /* Shader uniforms. */
  shader: @Shader,
  world: Mat4x4,
  tex_world: Mat4x4,

  /* Shader uniform locations. */
  proj_loc: gl::GLint,
  world_loc: gl::GLint,
  alpha_loc: gl::GLint,
  tex_world_loc: gl::GLint,
  texture0_loc: gl::GLint,

  /* Font support. */
  font_renderer: TTF_Renderer,
}

impl Renderer
{
  pub fn new() -> Renderer
  {
    let mut renderer = Renderer
    {
      vao: 0,
      vbo: 0,
      shader: Shader_Builder::new_with_files("data/shaders/ui.vert", "data/shaders/ui.frag"),
      world: Mat4x4::new(),
      tex_world: Mat4x4::new(),
      proj_loc: 0,
      world_loc: 0,
      alpha_loc: 0,
      tex_world_loc: 0,
      texture0_loc: 0,
      font_renderer: TTF_Renderer::new(),
    };

    renderer.proj_loc = renderer.shader.get_uniform_location("proj");
    renderer.world_loc = renderer.shader.get_uniform_location("world");
    renderer.alpha_loc = renderer.shader.get_uniform_location("alpha");
    renderer.tex_world_loc = renderer.shader.get_uniform_location("tex_world");
    renderer.texture0_loc = renderer.shader.get_uniform_location("texture0"); 
    renderer.shader.bind();
    renderer.shader.update_uniform_i32(renderer.texture0_loc, 0);

    /* VAO */
    let name = check!(gl::gen_vertex_arrays(1));
    assert!(name.len() == 1);
    renderer.vao = name[0];
    check!(gl::bind_vertex_array(renderer.vao));

    /* VBO */
    let name = check!(gl::gen_buffers(1));
    assert!(name.len() == 1);
    renderer.vbo = name[0];
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, renderer.vbo));

    struct Point
    {
      x: f32, y: f32,
      u: f32, v: f32
    }
    impl Point
    {
      #[inline(always)]
      pub fn new(nx: f32, ny: f32, nu: f32, nv: f32) -> Point
      { Point { x: nx, y: ny, u: nu, v: nv } }
    }
    let data =
    [
      /*(X , Y) (U , V)*/
      Point::new(0.0, 0.0, 0.0, 0.0),
      Point::new(0.0, 1.0, 0.0, 1.0),
      Point::new(1.0, 1.0, 1.0, 1.0),
      Point::new(1.0, 0.0, 1.0, 0.0),
    ];
    check!(gl::buffer_data(gl::ARRAY_BUFFER, data, gl::STATIC_DRAW));
    check!(gl::enable_vertex_attrib_array(0));

    renderer
  }

  #[inline(always)]
  pub fn begin(&mut self, camera: &Camera)
  {
    check!(gl::disable(gl::DEPTH_TEST));

    /* Enable transparency. */
    check!(gl::enable(gl::BLEND));
    check!(gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));

    /* Update the projection information. */
    let proj = Mat4x4::new_orthographic(0.0, camera.window_size.x as f32, camera.window_size.y as f32, 0.0,  1.0, 100.0);

    self.font_renderer.shader.bind();
    self.font_renderer.shader.update_uniform_mat(self.font_renderer.proj_loc, &proj);
    
    self.shader.bind();
    self.shader.update_uniform_mat(self.proj_loc, &proj);
  }

  #[inline(always)]
  pub fn end(&mut self)
  {
    check!(gl::enable(gl::DEPTH_TEST));
    check!(gl::disable(gl::BLEND));
  }

  pub fn render_texture(&mut self, tex: &Texture, pos: &Vec2f)
  {
    self.world = Mat4x4::new_scale(tex.dimensions.x as f32, tex.dimensions.y as f32, 1.0);
    self.world *= Mat4x4::new_translation(pos.x, pos.y, 0.0);
    self.shader.update_uniform_mat(self.world_loc, &self.world);

    self.tex_world.identity();
    self.shader.update_uniform_mat(self.tex_world_loc, &self.tex_world);

    self.render(tex);
  }

  pub fn render_font(&mut self, text: &str, pos: Vec2f, font: &TTF_Font)
  {
    self.font_renderer.shader.bind();
    self.font_renderer.render(text, pos, font);
    self.shader.bind();
  }

  priv fn render(&mut self, tex: &Texture)
  {
    tex.bind(0);

    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::vertex_attrib_pointer_f32(0, 4, false, 0, 0));

    check!(gl::draw_arrays(gl::TRIANGLE_STRIP, 0, 6));

    tex.unbind();

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
    check!(gl::bind_vertex_array(0));
  }
}

