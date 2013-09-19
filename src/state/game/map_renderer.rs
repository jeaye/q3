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

use std::{ vec, ptr, sys, cast, cell };
use extra;
use state::State;
use gl2 = opengles::gl2;
use gl;
use math;
use voxel;
use state;
use util::Log;

#[path = "../../gl/check.rs"]
mod check;

#[macro_escape]
#[path = "../../util/log_macros.rs"]
mod log_macros;

pub struct Map_Renderer
{
  map: @mut voxel::Map,

  vao: gl2::GLuint,
  vox_vbo: gl2::GLuint,
  offset_tex_vbo: gl2::GLuint,
  offset_tex: gl2::GLuint,
  ibos: ~[gl2::GLuint],
  curr_ibo: u32,
  visible_voxels: Option<~[u32]>,
  prev_visible_voxel_count: u32,

  /* states, visible */
  map_stream: extra::comm::DuplexStream<(cell::Cell<~[u32]>, cell::Cell<~[u32]>), (~[u32], ~[u32])>,

  wireframe: bool,

  shader: @mut gl::Shader,
  proj_loc: gl2::GLint,
  world_loc: gl2::GLint,
  voxel_size_loc: gl2::GLint,
  offsets_loc: gl2::GLint,
}

impl Map_Renderer
{
  pub fn new(map: @mut voxel::Map) -> @mut Map_Renderer
  {
    let (local_stream, _) = extra::comm::DuplexStream();

    let mr = @mut Map_Renderer
    {
      map: map,

      vao: 0,
      vox_vbo: 0,
      offset_tex_vbo: 0,
      offset_tex: 0,
      ibos: vec::from_elem(2, 2u32),
      curr_ibo: 0,
      visible_voxels: Some(vec::from_elem((map.resolution * map.resolution * map.resolution) as uint, 0u32)),
      prev_visible_voxel_count: 0,

      map_stream: local_stream,

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
    log_assert!(names.len() == 1);
    mr.vao = names[0];

    let names = check!(gl2::gen_buffers(4));
    log_assert!(names.len() == 4);
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
    log_assert!(name.len() == 1);
    mr.offset_tex = name[0];
    check!(gl2::bind_texture(gl2::TEXTURE_BUFFER, mr.offset_tex));
    check!(gl2::tex_buffer(gl2::TEXTURE_BUFFER, 0x8815 /* RGB32F */, mr.offset_tex_vbo));

    /* Console functions. */
    do state::Director::push_deferred ||
    {
      state::Console::get().registry.accessors.insert(~"map.wireframe", |_|
      { mr.wireframe.to_str() });
      state::Console::get().add_mutator("map.wireframe", |p, x|
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
    }

    mr
  }

  #[fixed_stack_segment]
  pub fn update_visibility(&mut self)
  {
    self.prev_visible_voxel_count = self.visible_voxels.get_ref().len() as u32;

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

    self.visible_voxels.get_mut_ref().clear();

    /* Updating visible voxels is an expensive task. To remedy this,
     * the work is done on a background thread that has a shared OpenGL
     * context. While that work is being done, the map renderer will
     * not have visible voxels or voxel states, since they're moved
     * to the task. Once the task is finished, however, the fields are
     * sent back. */
    let (local_stream, remote_stream) = extra::comm::DuplexStream();
    self.map_stream = local_stream;

    /* Send out the voxel states and visible voxels. */
    self.map_stream.send((cell::Cell::new(self.map.states.take_unwrap()), cell::Cell::new(self.visible_voxels.take_unwrap())));

    /* Start the new background task of culling far-away voxels. */
    let resolution = self.map.resolution;
    let ibo = self.ibos[self.curr_ibo];
    do gl::Worker::new_task
    {
      let (cell_states, cell_visible_voxels) = remote_stream.recv();
      let states = cell_states.take();
      let mut visible_voxels = cell_visible_voxels.take();

      for z in range(start.z, end.z)
      {
        for y in range(start.y, end.y)
        {
          for x in range(start.x, end.x)
          {
            let index = (z * ((resolution * resolution) as i32)) + (y * (resolution as i32)) + x;
            if (states[index] & voxel::Visible) != 0
            { visible_voxels.push(states[index] & !voxel::Visible); }
          }
        }
      }

      /* Upload the data to the inactive buffer. */
      check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, ibo));
      unsafe
      {
        let size = visible_voxels.len() * sys::size_of::<u32>();
        let mem = check!(gl2::map_buffer_range(gl2::ARRAY_BUFFER, 0, size as i64, gl2::MAP_WRITE_BIT));
        log_assert!(mem != ptr::null());
        ptr::copy_nonoverlapping_memory(cast::transmute(mem), vec::raw::to_ptr(visible_voxels), size);
        check!(gl2::unmap_buffer(gl2::ARRAY_BUFFER));
      }

      /* Send the member data back. */
      remote_stream.send((states, visible_voxels));

      false /* Don't kill the GL worker. */
    }
  }
}

impl State for Map_Renderer
{
  fn load(&mut self)
  {
    log_debug!("Loading map renderer state");

    self.shader.bind();
    self.proj_loc = self.shader.get_uniform_location("proj");
    self.world_loc = self.shader.get_uniform_location("world");
    self.voxel_size_loc = self.shader.get_uniform_location("voxel_size");
    self.offsets_loc = self.shader.get_uniform_location("offsets");

    self.shader.update_uniform_i32(self.offsets_loc, 0);

    self.update_visibility();
  }

  fn unload(&mut self)
  {
    log_debug!("Unloading map renderer state");
    
    /* Since the background worker is doing its thing, we'll
     * need to wait for it to finish so that it doesn't try
     * to update us when we're dead. */
    let (states, visible_voxels) = self.map_stream.recv();
    self.map.states = Some(states);
    self.visible_voxels = Some(visible_voxels);
  }

  fn get_key(&self) -> &str
  { &"map_renderer" }

  fn update(&mut self, _delta: f32) -> bool /* dt is in terms of seconds. */
  {
    /* Check if there is data available between the background
     * thread and us. The last thing it does is send back some
     * member data that we'll need to put back in place before
     * doing any more work. */
    if !self.map_stream.peek()
    { return false; }

    /* Extract the new data. */
    let (states, visible_voxels) = self.map_stream.recv();
    self.map.states = Some(states);
    self.visible_voxels = Some(visible_voxels);
    
    /* TODO: Work goes here. */

    /* Swap the current IBO and begin updating the old one. */
    if self.curr_ibo == 0
    { self.curr_ibo = 1; }
    else
    { self.curr_ibo = 0; }
    self.update_visibility();

    false      
  }

  fn render(&mut self) -> bool
  {
    let camera = gl::Camera::get_active();

    self.shader.bind();
    self.shader.update_uniform_mat(self.proj_loc, &camera.projection);
    self.shader.update_uniform_mat(self.world_loc, &camera.view);
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

#[unsafe_destructor]
impl Drop for Map_Renderer
{
  fn drop(&self)
  {
    check!(gl2::delete_vertex_arrays(&[self.vao]));
    check!(gl2::delete_buffers(&[self.vox_vbo, self.offset_tex_vbo,
                                 self.ibos[0], self.ibos[1]]));
  }
}

