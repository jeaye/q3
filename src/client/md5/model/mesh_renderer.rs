/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: client/md5/model/mesh_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Renders an MD5 mesh.
*/

use std::mem;
use gl2 = opengles::gl2;
use gfx;
use math;
use super::Mesh;
use log::Log;

#[macro_escape]
#[path = "../../gfx/check.rs"]
mod check;

#[macro_escape]
#[path = "../../../shared/log/macros.rs"]
mod macros;

struct Mesh_Renderer<'self>
{
  mesh: &'self Mesh,

  vao: gl2::GLuint,
  position_vbo: gl2::GLuint, 
  tex_vbo: gl2::GLuint, 
  ibo: gl2::GLuint, 
  tex0_loc: gl2::GLint,

  shader: @mut gfx::Shader, 
  texture: Option<gfx::Texture>,
}

impl<'self> Mesh_Renderer<'self>
{
  pub fn new(m: &'self Mesh, sh: @mut gfx::Shader) -> Mesh_Renderer<'self>
  {
    let mut mr = Mesh_Renderer
    {
      mesh: m,

      vao: 0,
      position_vbo: 0,
      tex_vbo: 0,
      ibo: 0,
      tex0_loc: 0,

      shader: sh,
      texture: None,
    };

    if m.texture.len() > 0
    { mr.texture = Some(gfx::Texture::new(gl2::TEXTURE_2D, m.texture)); }

    mr.shader.bind();
    mr.tex0_loc = mr.shader.get_uniform_location("tex0");
    mr.shader.update_uniform_i32(mr.tex0_loc, 0);

    mr.upload();

    mr
  }

  fn upload(&mut self)
  {
    let name = check!(gl2::gen_vertex_arrays(1));
    log_assert!(name.len() == 1);
    self.vao = name[0];

    let name = check!(gl2::gen_buffers(3));
    log_assert!(name.len() == 3);
    self.position_vbo = name[0];
    self.tex_vbo = name[1];
    self.ibo = name[2];

    /* Upload data. */
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.position_vbo));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, self.mesh.positions, gl2::DYNAMIC_DRAW));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.tex_vbo));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, self.mesh.tex_coords, gl2::STATIC_DRAW));

    check!(gl2::bind_buffer(gl2::ELEMENT_ARRAY_BUFFER, self.ibo));
    check!(gl2::buffer_data(gl2::ELEMENT_ARRAY_BUFFER, self.mesh.indices, gl2::STATIC_DRAW));

    /* Setup vertex attribs. */
    check!(gl2::bind_vertex_array(self.vao));

    check!(gl2::enable_vertex_attrib_array(0));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.position_vbo));
    check!(gl2::vertex_attrib_pointer_f32(0, 3, false, mem::size_of::<math::Vec3f>() as i32, 0));

    check!(gl2::enable_vertex_attrib_array(1));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.tex_vbo));
    check!(gl2::vertex_attrib_pointer_f32(1, 2, false, mem::size_of::<math::Vec2f>() as i32, 0));

  }

  pub fn update(&mut self, _dt: f32)
  {
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.position_vbo));
    check!(gl2::buffer_sub_data(gl2::ARRAY_BUFFER, 0, self.mesh.positions));
  }

  pub fn render(&self)
  {
    match self.texture
    {
      Some(ref tex) => { tex.bind(gl2::TEXTURE_2D); },
      None => { }
    }

    check!(gl2::bind_vertex_array(self.vao));

    check!(gl2::bind_buffer(gl2::ELEMENT_ARRAY_BUFFER, self.ibo));
    check!(gl2::draw_elements(gl2::TRIANGLES, self.mesh.indices.len() as i32, gl2::UNSIGNED_INT, None));

    check!(gl2::bind_buffer(gl2::ELEMENT_ARRAY_BUFFER, 0));
    check!(gl2::bind_vertex_array(0));
  }
}

#[unsafe_destructor]
impl<'self> Drop for Mesh_Renderer<'self>
{
  fn drop(&mut self)
  {
    check!(gl2::delete_vertex_arrays(&[self.vao]));
    check!(gl2::delete_buffers(&[self.position_vbo, self.tex_vbo, self.ibo]));
  }
}

