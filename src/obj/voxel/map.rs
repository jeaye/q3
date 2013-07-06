/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/voxel/map.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A voxelization of arbitrary triangles
      into OpenGL-ready cubes.
*/

use std::{ i32, f32, uint, vec, cmp };
use extra;
use math;
use primitive::Triangle;
use super::{ Vertex, Behavior, Invisible, Default };
use ui;
use gl2 = opengles::gl2;

#[path = "../../gl/check.rs"]
mod check;

struct Map
{
  resolution: u32,
  voxel_size: f32,

  vao: gl2::GLuint,
  vox_vbo: gl2::GLuint,
  offset_tex_vbo: gl2::GLuint,
  offset_tex: gl2::GLuint,
  ibo: gl2::GLuint,

  states: ~[Behavior],
  voxels: ~[Vertex],
  visible_voxels: ~[i32],

  wireframe: bool,
}

impl Map
{
  pub fn new(tris: &[Triangle], res: u32) -> @mut Map
  {
    let map = @mut Map
    {
      resolution: res,
      voxel_size: 0.0,
      vao: 0,
      vox_vbo: 0,
      offset_tex_vbo: 0,
      offset_tex: 0,
      ibo: 0,

      states: ~[],
      voxels: ~[],
      visible_voxels: ~[],

      wireframe: false,
    };

    map.voxelize(tris);

    /* Single voxel that will be instance-rendered. */
    let h: f32 = map.voxel_size / 2.0;
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
    map.vao = names[0];

    let names = check!(gl2::gen_buffers(3));
    assert!(names.len() == 3);
    map.vox_vbo = names[0];
    map.offset_tex_vbo = names[1];
    map.ibo = names[2];

    check!(gl2::bind_vertex_array(map.vao));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, map.vox_vbo));
    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, voxel, gl2::STATIC_DRAW));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, map.ibo));
    map.update_visibility(&math::Vec3f::zero());

    check!(gl2::bind_buffer(gl2::TEXTURE_BUFFER, map.offset_tex_vbo));
    check!(gl2::buffer_data(gl2::TEXTURE_BUFFER, map.voxels, gl2::STATIC_DRAW));

    let name = check!(gl2::gen_textures(1));
    assert!(name.len() == 1);
    map.offset_tex = name[0];
    check!(gl2::bind_texture(gl2::TEXTURE_BUFFER, map.offset_tex));
    check!(gl2::tex_buffer(gl2::TEXTURE_BUFFER, 0x8815 /* RGB32F */, map.offset_tex_vbo));

    /* Console functions. */
    ui::Console_Activator::get().add_accessor("map.wireframe", |_|
    { map.wireframe.to_str() });
    ui::Console_Activator::get().add_mutator("map.wireframe", |p, x|
    {
      let mut error = ~"";
      if x == "true"
      { map.wireframe = true; }
      else if x == "false"
      { map.wireframe = false; }
      else
      { error = fmt!("Invalid value for %s (use 'true' or 'false')", p); }

      if error.len() == 0
      { None }
      else
      { Some(error) }
    });

    map
  }

  pub fn draw(&mut self)
  {
    check!(gl2::bind_vertex_array(self.vao));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.vox_vbo));
    check!(gl2::vertex_attrib_pointer_f32(0, 3, false, 0, 0));
    check!(gl2::enable_vertex_attrib_array(0));

    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, self.ibo));
    check!(gl2::vertex_attrib_i_pointer_i32(1, 1, 0, 0));
    check!(gl2::enable_vertex_attrib_array(1));
    check!(gl2::vertex_attrib_divisor(1, 1));

    check!(gl2::bind_texture(gl2::TEXTURE_BUFFER, self.offset_tex));

    if self.wireframe
    { check!(gl2::polygon_mode(gl2::FRONT_AND_BACK, gl2::LINE)); }

    check!(gl2::draw_arrays_instanced(gl2::TRIANGLE_STRIP, 0, 24, self.visible_voxels.len() as i32));

    if self.wireframe
    { check!(gl2::polygon_mode(gl2::FRONT_AND_BACK, gl2::FILL)); }

    check!(gl2::disable_vertex_attrib_array(0));
    check!(gl2::disable_vertex_attrib_array(1));
    check!(gl2::bind_vertex_array(0));
    check!(gl2::bind_buffer(gl2::ARRAY_BUFFER, 0));
  }

  pub fn update_visibility(&mut self, _cam_pos: &math::Vec3f)
  {
    self.visible_voxels.clear();

    if self.visible_voxels.len() == 0
    {
      for self.voxels.iter().enumerate().advance |(i, _v)|
      { self.visible_voxels.push(i as i32);}
    }

    check!(gl2::buffer_data(gl2::ARRAY_BUFFER, self.visible_voxels, gl2::STREAM_DRAW));
  }

  priv fn voxelize(&mut self, tris: &[Triangle])
  {
    /* Require at least one triangle. */
    assert!(tris.len() >= 1);
    debug!("VOXEL: Incoming triangles: %?", tris.len());

    /* Bounding box of vert dimensions. */
    let mut min = math::Vec3f::new( tris[0].verts[0].position.x,
                              tris[0].verts[0].position.y, 
                              tris[0].verts[0].position.z);
    let mut max = math::Vec3f::new( tris[0].verts[0].position.x,
                              tris[0].verts[0].position.y,
                              tris[0].verts[0].position.z);
    for tris.iter().advance |curr|
    {
      for curr.verts.iter().advance |vert|
      {
        min.x = cmp::min(min.x, vert.position.x);
        min.y = cmp::min(min.y, vert.position.y);
        min.z = cmp::min(min.z, vert.position.z);

        max.x = cmp::max(max.x, vert.position.x);
        max.y = cmp::max(max.y, vert.position.y);
        max.z = cmp::max(max.z, vert.position.z);
      }
    }
    debug!("VOXEL: Min: %s Max: %s", min.to_str(), max.to_str());
    let center = math::Vec3f::new(max.x - ((max.x - min.x) / 2.0),
                            max.y - ((max.y - min.y) / 2.0),
                            max.z - ((max.z - min.z) / 2.0));
    debug!("VOXEL: Center of mesh is %s", center.to_str());

    /* Calculate, given resolution (how many states across), the dimensions of a voxel. */
    self.voxel_size = cmp::max( max.x - min.x,
                                cmp::max(max.y - min.y, max.z - min.z)) / (self.resolution as f32);
    debug!("VOXEL: Voxel size is %?", self.voxel_size);

    /* World space mid point of the grid. */
    let mid_offset = (((self.resolution as f32) / 2.0) * self.voxel_size); 
    debug!("VOXEL: Midpoint offset is %?", mid_offset);

    /* Create 3D array of states. */
    self.states = vec::with_capacity((f32::pow((self.resolution + 1) as f32, 3.0)) as uint);
    self.voxels = vec::with_capacity(self.states.len() / 2); 
    for uint::range(0, self.resolution as uint) |_z| 
    { for uint::range(0, self.resolution as uint) |_y|
      { for uint::range(0, self.resolution as uint) |_x|
        {
          self.states.push(Invisible);
        }
      }
    }

    for tris.iter().advance |tri|
    {
      /* Calculate bounding box of the triangle. */
      min = math::Vec3f::new(tri.verts[0].position.x, tri.verts[0].position.y, tri.verts[0].position.z);
      max = math::Vec3f::new(tri.verts[0].position.x, tri.verts[0].position.y, tri.verts[0].position.z);
      for tri.verts.iter().advance |vert|
      {
        /* Adjust by half of a voxel to account for voxel centering. */
        min.x = cmp::min(min.x, vert.position.x - (self.voxel_size / 2.0));
        min.y = cmp::min(min.y, vert.position.y - (self.voxel_size / 2.0));
        min.z = cmp::min(min.z, vert.position.z - (self.voxel_size / 2.0));

        max.x = cmp::max(max.x, vert.position.x - (self.voxel_size / 2.0));
        max.y = cmp::max(max.y, vert.position.y - (self.voxel_size / 2.0));
        max.z = cmp::max(max.z, vert.position.z - (self.voxel_size / 2.0));
      }

      /* The dimensions (in voxels) of the triangle's bounding box. */
      let mut vox_amount = math::Vec3i::new(f32::ceil(((max.x - min.x) / self.voxel_size)) as i32,
                                      f32::ceil(((max.y - min.y) / self.voxel_size)) as i32,
                                      f32::ceil(((max.z - min.z) / self.voxel_size)) as i32);
      if vox_amount.x < 1
      { vox_amount.x = 1; }
      if vox_amount.y < 1
      { vox_amount.y = 1; }
      if vox_amount.z < 1
      { vox_amount.z = 1; }
      //debug!("VOXEL: [Per voxel] Checking %s surrounding states with SAT", vox_amount.to_str());

      /* Get the starting indices of the triangle's bounding box. */
      let start_voxels = math::Vec3i::new( ((min.x - -mid_offset) / self.voxel_size) as i32, 
                                      ((min.y - -mid_offset) / self.voxel_size) as i32,
                                      ((min.z - -mid_offset) / self.voxel_size) as i32);

      /* Test intersection with each accepted voxel. */
      for i32::range(start_voxels.z, start_voxels.z + vox_amount.z) |z|
      { for i32::range(start_voxels.y, start_voxels.y + vox_amount.y) |y|
        { for i32::range(start_voxels.x, start_voxels.x + vox_amount.x) |x|
          {
            /* Check for intersection. */
            let c = math::Vec3f::new( ((x as f32 - (self.resolution as f32 / 2.0)) * self.voxel_size) + (self.voxel_size / 2.0), 
                                ((y as f32 - (self.resolution as f32 / 2.0)) * self.voxel_size) + (self.voxel_size / 2.0),
                                ((z as f32 - (self.resolution as f32 / 2.0)) * self.voxel_size) + (self.voxel_size / 2.0));
            if tri_cube_intersect(c, self.voxel_size, tri)
            {
              /* Calculate the average color from all three verts. */
              let av_color = math::Vec3f::new
              (
                ((tri.verts[0].color.x + tri.verts[1].color.x + tri.verts[2].color.x) / 3.0) as f32 / 255.0,
                ((tri.verts[0].color.y + tri.verts[1].color.y + tri.verts[2].color.y) / 3.0) as f32 / 255.0,
                ((tri.verts[0].color.z + tri.verts[1].color.z + tri.verts[2].color.z) / 3.0) as f32 / 255.0
              );

              /* Update the state of the voxel. */
              let index = (z * ((self.resolution * self.resolution) as i32)) + (y * (self.resolution as i32)) + x;
              self.states[index] = Default;

              /* Enable some debug rendering of invalid voxels. */
              let col = if x >= self.resolution as i32 || y >= self.resolution as i32 || z >= self.resolution as i32
              { math::Vec3f::new(1.0, 0.0, 0.0) }
              else if x < 0 || y < 0 || z < 0
              { math::Vec3f::new(1.0, 0.0, 0.0) }
              else
              { av_color };

              /* We have intersection; add a reference to this voxel to the index map. */
              self.voxels.push(Vertex
              {
                position: math::Vec3f::new( x as f32 - (self.resolution / 2) as f32, 
                                      y as f32 - (self.resolution / 2) as f32,
                                      z as f32 - (self.resolution / 2) as f32), 
                color: col
              });
            }
          }
        }
      }
    }

    /* Remove duplicates. */
    let len = self.voxels.len();
    extra::sort::quick_sort3(self.voxels);
    self.voxels.dedup();
    let new_len = self.voxels.len();
    debug!("VOXEL: New voxel count is %?, down from %?", new_len, len);

    debug!("VOXEL: Enabled %? of %? voxels", self.voxels.len(), self.states.len());
  }
}

macro_rules! find_min_max
(
  ($x0:expr, $x1:expr, $x2:expr) =>
  (
    {
      _min = $x0;
      _max = $x0;

      if($x1 < _min){ _min = $x1; }
      if($x1 > _max){ _max = $x1; }
      if($x2 < _min){ _min = $x2; }
      if($x2 > _max){ _max = $x2; }
    }
  )
)

/*======================== X-tests ========================*/
macro_rules! axis_test_x01
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      _p0 = $a * _v0.y - $b * _v0.z;
      _p2 = $a * _v2.y - $b * _v2.z;
      if _p0 < _p2  { _min = _p0; _max = _p2; } else { _min = _p2; _max = _p0; }
      _rad = $fa * box_size + $fb * box_size;
      if _min > _rad || _max < -_rad  { return false; }
    }
  )
)

macro_rules! axis_test_x2
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      _p0 = $a * _v0.y - $b * _v0.z;
      _p1 = $a * _v1.y - $b * _v1.z;
      if _p0 < _p1 { _min = _p0; _max = _p1; } else { _min = _p1; _max = _p0; }
      _rad = $fa * box_size + $fb * box_size;
      if _min > _rad || _max < -_rad { return false; }
    }
  )
)

/*======================== Y-tests ========================*/

macro_rules! axis_test_y02
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      _p0 = -$a * _v0.x + $b * _v0.z;
      _p2 = -$a * _v2.x + $b * _v2.z;
      if _p0 < _p2 { _min = _p0; _max = _p2; } else { _min = _p2; _max = _p0; }
      _rad = $fa * box_size + $fb * box_size;
      if _min > _rad || _max < -_rad { return false; }
    }
  )
)

macro_rules! axis_test_y1
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      _p0 = -$a * _v0.x + $b * _v0.z;
      _p1 = -$a * _v1.x + $b * _v1.z;
      if _p0 < _p1 { _min = _p0; _max = _p1; } else { _min = _p1; _max = _p0; }
      _rad = $fa * box_size + $fb * box_size;
      if _min > _rad || _max < -_rad { return false; }
    }
  )
)

/*======================== Z-tests ========================*/

macro_rules! axis_test_z12
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      _p1 = $a * _v1.x - $b * _v1.y;
      _p2 = $a * _v2.x - $b * _v2.y;
      if _p2 < _p1 { _min = _p2; _max = _p1;} else { _min = _p1; _max = _p2; }
      _rad = $fa * box_size + $fb * box_size;
      if _min > _rad || _max < -_rad { return false; }
    }
  )
)


macro_rules! axis_test_z0
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      _p0 = $a * _v0.x - $b * _v0.y;
      _p1 = $a * _v1.x - $b * _v1.y;
      if _p0 < _p1 { _min = _p0; _max = _p1; } else { _min = _p1; _max = _p0; }
      _rad = $fa * box_size + $fb * box_size;
      if _min > _rad || _max < -_rad { return false; }
    }
  )
)

#[inline(always)]
priv fn tri_cube_intersect(box_center: math::Vec3f, box_size: f32, tri: &Triangle) -> bool
{
  let _v0;
  let _v1;
  let _v2;
  let mut _min;
  let mut _max;
  let mut _p0 = 0.0;
  let mut _p1 = 0.0;
  let mut _p2 = 0.0;
  let mut _rad;
  let mut _fex;
  let mut _fey;
  let mut _fez;
  let _normal;
  let _e0;
  let _e1;
  let _e2;

  /* Move everything so that the box's center is in (0, 0, 0). */
  _v0 = tri.verts[0].position - box_center;
  _v1 = tri.verts[1].position - box_center;
  _v2 = tri.verts[2].position - box_center;

  /* Computer triangle edges. */
  _e0 = _v1 - _v0; /* Edge 0. */
  _e1 = _v2 - _v1; /* Edge 1. */
  _e2 = _v0 - _v2; /* Edge 2. */

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 3 edge 0");
  /* Bullet 3. */
  _fex = f32::abs(_e0.x);
  _fey = f32::abs(_e0.y);
  _fez = f32::abs(_e0.z);
  axis_test_x01!(_e0.z, _e0.y, _fez, _fey);
  axis_test_y02!(_e0.z, _e0.x, _fez, _fex);
  axis_test_z12!(_e0.y, _e0.x, _fey, _fex);

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 3 edge 1");
  _fex = f32::abs(_e1.x);
  _fey = f32::abs(_e1.y);
  _fez = f32::abs(_e1.z);
  axis_test_x01!(_e1.z, _e1.y, _fez, _fey);
  axis_test_y02!(_e1.z, _e1.x, _fez, _fex);
  axis_test_z0!(_e1.y, _e1.x, _fey, _fex);

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 3 edge 2");
  _fex = f32::abs(_e2.x);
  _fey = f32::abs(_e2.y);
  _fez = f32::abs(_e2.z);
  axis_test_x2!(_e2.z, _e2.y, _fez, _fey);
  axis_test_y1!(_e2.z, _e2.x, _fez, _fex);
  axis_test_z12!(_e2.y, _e2.x, _fey, _fex);

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 1");
  /* Bullet 1. */
  /* Test in X-direction */
  find_min_max!(_v0.x, _v1.x, _v2.x);
  if _min > box_size || _max < -box_size { return false; }

  /* Test in Y-direction */
  find_min_max!(_v0.y, _v1.y, _v2.y);
  if _min > box_size || _max < -box_size { return false; }

  /* Test in Z-direction */
  find_min_max!(_v0.z, _v1.z, _v2.z);
  if _min > box_size || _max < -box_size { return false; }

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 2");
  /* Bullet 2. */
  _normal = _e0.cross(&_e1);
  plane_cube_intersect(&_normal, &_v0, box_size)
}

#[inline(always)]
priv fn plane_cube_intersect(normal: &math::Vec3f, vert: &math::Vec3f, box_size: f32) -> bool
{
  let mut vmin: [f32, ..3] = [0.0, 0.0, 0.0];
  let mut vmax: [f32, ..3] = [0.0, 0.0, 0.0];
  let mut v = 0.0;

  for uint::range(0, 3) |q|
  {
    v = vert[q];
    if normal[q] > 0.0
    {
      vmin[q] = -box_size - v;
      vmax[q] = box_size - v;
    }
    else
    {
      vmin[q] = box_size - v;
      vmax[q] = -box_size - v;
    }
  }
  if (normal[0]*vmin[0]+normal[1]*vmin[1]+normal[2]*vmin[2]) > 0.0 { return false; }
  if (normal[0]*vmax[0]+normal[1]*vmax[1]+normal[2]*vmax[2]) >= 0.0 { return true; }

  false
}

