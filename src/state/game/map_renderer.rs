/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: state/game/map_renderer.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A client-only state that depends on
      the shared game state. This state is
      used only to render the voxel map.
*/

use std::{ i32, vec, ptr, sys, cast };
use state::State;
use gl2 = opengles::gl2;
use gl;
use ui;
use math;
use voxel;

#[path = "../../gl/check.rs"]
mod check;

pub struct Map_Renderer
{
  map: @mut voxel::Map,
  camera: @mut gl::Camera,

  vao: gl2::GLuint,
  vox_vbo: gl2::GLuint,
  offset_tex_vbo: gl2::GLuint,
  offset_tex: gl2::GLuint,
  ibos: ~[gl2::GLuint],
  curr_ibo: u32,
  visible_voxels: ~[u32],
  prev_visible_voxel_count: u32,

  wireframe: bool,

  shader: @gl::Shader,
  proj_loc: gl2::GLint,
  world_loc: gl2::GLint,
  voxel_size_loc: gl2::GLint,
  offsets_loc: gl2::GLint,
}

impl Map_Renderer
{
  pub fn new(map: @mut voxel::Map, cam: @mut gl::Camera) -> @mut Map_Renderer
  {
    let mr = @mut Map_Renderer
    {
      map: map,
      camera: cam,

      vao: 0,
      vox_vbo: 0,
      offset_tex_vbo: 0,
      offset_tex: 0,
      ibos: vec::from_elem(2, 2u32),
      curr_ibo: 0,
      visible_voxels: vec::from_elem((map.resolution * map.resolution * map.resolution) as uint, 0u32),
      prev_visible_voxel_count: 0,

      wireframe: false,

      shader: gl::Shader_Builder::new_with_files("data/shaders/voxel.vert", "data/shaders/voxel.frag"),
      proj_loc: 0,
      world_loc: 0,
      voxel_size_loc: 0,
      offsets_loc: 0,
    };

    /* Single voxel that will be instance-rendered. */
    let h: f32 = mr.map.voxel_size / 2.0;
    let voxel: ~[f32] = /* TRIANGLE_STRIP style. */
    ~[
      -h,-h,h,  h,-h,h,   
      -h,h,h,   h,h,h,    

      h,-h,h,   h,-h,-h,  
      h,h,h,    h,h,-h,   

      h,-h,-h,  -h,-h,-h, 
      h,h,-h,   -h,h,-h,  

      -h,-h,-h, -h,-h,h,  
      -h,h,-h,  -h,h,h,   

      -h,-h,-h, h,-h,-h,  
      -h,-h,h,  h,-h,h,   

      -h,h,h,   h,h,h,    
      -h,h,-h,  h,h,-h,   
    ];

    let names = check!(gl2::gen_vertex_arrays(1));
    assert!(names.len() == 1);
    mr.vao = names[0];

    let names = check!(gl2::gen_buffers(4));
    assert!(names.len() == 4);
    mr.vox_vbo = names[0];
    mr.offset_tex_vbo = names[1];
    mr.ibos[0] = names[2];
    mr.ibos[1] = names[3];

    check!(gl2::bind_vertex_array(mr.vao));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, mr.vox_vbo));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, voxel, gl2::STATIC_DRAW));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, mr.ibos[0]));
    let ibo_buf = vec::from_elem((mr.map.resolution * mr.map.resolution * mr.map.resolution) as uint, 0);
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, ibo_buf, gl2::DYNAMIC_DRAW));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, mr.ibos[1]));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, ibo_buf, gl2::DYNAMIC_DRAW));

    check!(gl2::bind_buffer(gl2::TEXTURE_BUFFER, mr.offset_tex_vbo));
    check!(gl2::buffer_data(gl2::TEXTURE_BUFFER, mr.map.voxels, gl2::STATIC_DRAW));

    let name = check!(gl2::gen_textures(1));
    assert!(name.len() == 1);
    mr.offset_tex = name[0];
    check!(gl2::bind_texture(gl2::TEXTURE_BUFFER, mr.offset_tex));
    check!(gl2::tex_buffer(gl2::TEXTURE_BUFFER, 0x8815 /* RGB32F */, mr.offset_tex_vbo));

    /* Console functions. */
    ui::Console_Activator::get().add_accessor("map.wireframe", |_|
    { mr.wireframe.to_str() });
    ui::Console_Activator::get().add_mutator("map.wireframe", |p, x|
    {
      let mut error = ~"";
      if x == "true"
      { mr.wireframe = true; }
      else if x == "false"
      { mr.wireframe = false; }
      else
      { error = fmt!("Invalid value for %s (use 'true' or 'false')", p); }

      if error.len() == 0
      { None }
      else
      { Some(error) }
    });


    mr
  }

  pub fn update_visibility(&mut self)
  {
    self.prev_visible_voxel_count = self.visible_voxels.len() as u32;

    let cam = gl::Camera::get_active();
    let dist = (cam.near_far.y  / self.map.voxel_size) as i32; /* How far the camera can see. */
    let res = self.map.resolution as f32;
    let pos = math::Vec3f::new(cam.position.x / self.map.voxel_size,
                               cam.position.y / self.map.voxel_size,
                               cam.position.z / self.map.voxel_size)
                               + math::Vec3f::new(res / 2.0, res / 2.0, res / 2.0);
    let start = math::Vec3i::new
    (
      (pos.x - dist as f32).clamp(&0.0, &(res - 1.0)) as i32,
      (pos.y - dist as f32).clamp(&0.0, &(res - 1.0)) as i32,
      (pos.z - dist as f32).clamp(&0.0, &(res - 1.0)) as i32
    );
    let end = math::Vec3i::new
    (
      (pos.x + dist as f32).clamp(&0.0, &(res - 1.0)) as i32,
      (pos.y + dist as f32).clamp(&0.0, &(res - 1.0)) as i32,
      (pos.z + dist as f32).clamp(&0.0, &(res - 1.0)) as i32
    );

    self.visible_voxels.clear();

    for i32::range(start.z, end.z) |z|
    {
      for i32::range(start.y, end.y) |y|
      {
        for i32::range(start.x, end.x) |x|
        {
          let index = (z * ((self.map.resolution * self.map.resolution) as i32)) + (y * (self.map.resolution as i32)) + x;
          if (self.map.states[index] & voxel::Visible) != 0
          { self.visible_voxels.push(self.map.states[index] & !voxel::Visible); }
        }
      }
    }

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.ibos[self.curr_ibo]));
    //check!(gl2::buffer_sub_data(gl2::ARRAY_BUFFER, 0, self.visible_voxels));
    unsafe
    {
      let size = self.visible_voxels.len() * sys::size_of::<u32>();
      let mut mem = check!(gl2::glMapBufferRange(gl2::ARRAY_BUFFER, 0, size as i64, 2 | 32));
      ptr::copy_nonoverlapping_memory(cast::transmute(mem), vec::raw::to_ptr(self.visible_voxels), size);
      check!(gl2::glUnmapBuffer(gl2::ARRAY_BUFFER));
    }
  }
}

impl State for Map_Renderer
{
  pub fn load(&mut self)
  {
    debug!("Loading map renderer state.");

    self.shader.bind();
    self.proj_loc = self.shader.get_uniform_location("proj");
    self.world_loc = self.shader.get_uniform_location("world");
    self.voxel_size_loc = self.shader.get_uniform_location("voxel_size");
    self.offsets_loc = self.shader.get_uniform_location("offsets");

    self.shader.update_uniform_i32(self.offsets_loc, 0);
  }

  pub fn unload(&mut self)
  { debug!("Unloading map renderer state."); }

  pub fn update(&mut self, delta: f32) -> bool /* dt is in terms of seconds. */
  {
    if self.curr_ibo == 0
    { self.curr_ibo = 1; }
    else
    { self.curr_ibo = 0; }

    self.update_visibility();

    false      
  }

  pub fn render(&mut self) -> bool
  {
    self.shader.bind();
    self.shader.update_uniform_mat(self.proj_loc, &self.camera.projection);
    self.shader.update_uniform_mat(self.world_loc, &self.camera.view);
    self.shader.update_uniform_f32(self.voxel_size_loc, self.map.voxel_size);

    check!(gl2::bind_vertex_array(self.vao));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.vox_vbo));
    check!(gl2::vertex_attrib_pointer_f32(0, 3, false, 0, 0));
    check!(gl2::enable_vertex_attrib_array(0));

    if self.curr_ibo == 0
    { check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.ibos[1])); }
    else
    { check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.ibos[0])); }
    check!(gl2::vertex_attrib_i_pointer_i32(1, 1, 0, 0));
    check!(gl2::enable_vertex_attrib_array(1));
    check!(gl2::vertex_attrib_divisor(1, 1));

    check!(gl2::bind_texture(gl2::TEXTURE_BUFFER, self.offset_tex));

    if self.wireframe
    { check!(gl2::polygon_mode(gl2::FRONT_AND_BACK, gl2::LINE)); }

    check!(gl2::draw_arrays_instanced(gl2::TRIANGLE_STRIP, 0, 24, self.prev_visible_voxel_count as i32));

    if self.wireframe
    { check!(gl2::polygon_mode(gl2::FRONT_AND_BACK, gl2::FILL)); }

    check!(gl2::disable_vertex_attrib_array(0));
    check!(gl2::disable_vertex_attrib_array(1));
    check!(gl2::bind_vertex_array(0));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, 0));
    
    false
  }
}

