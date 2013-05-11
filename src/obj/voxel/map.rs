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

use math::{ Vec3f, Vec3i, Vec3i8, Vec3u8 };
use primitive::Vertex_PC;
use primitive::Triangle;
use primitive::{ Cube, Cube_Index };
use super::{ Vertex, Behavior, Default };

#[path = "../../gl/mod.rs"]
mod gl;
#[path = "../../gl/util.rs"]
mod util;
#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

struct Map
{
  resolution: u32,
  voxel_size: f32,

  vao: gl::GLuint,
  vbo: gl::GLuint,
  ibo: gl::GLuint,

  voxels: ~[Behavior],
  indices: ~[Vertex],
}

impl Map
{
  pub fn new(tris: &[Triangle], res: u32) -> Map
  {
    let mut map = Map
    {
      resolution: res,
      voxel_size: 0.0,
      vao: 0,
      vbo: 0,
      ibo: 0,
      voxels: ~[],
      indices: ~[],
    };

    map.voxelize(tris);

    /* Single voxel that will be instance-rendered. */
    let mut voxel = ~[];
    voxel.push(Cube::new(map.voxel_size, Vec3f::zero()));

    map.vao = check!(gl::gen_vertex_arrays(1))[0]; /* TODO: Check these. */
    map.vbo = check!(gl::gen_buffers(1))[0];
    map.ibo = check!(gl::gen_buffers(1))[0];
    check!(gl::bind_vertex_array(map.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, map.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, voxel, gl::STATIC_DRAW));

    check!(gl::bind_buffer(gl::ARRAY_BUFFER, map.ibo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, map.indices, gl::STATIC_DRAW));

    map
  }

  pub fn draw(&self)
  {
    check!(gl::bind_vertex_array(self.vao));

    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));
    check!(gl::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vertex_PC>()) as i32, 0));
    check!(gl::enable_vertex_attrib_array(0));

    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.ibo));

    check!(gl::vertex_attrib_pointer_i32(1, 3, false, (sys::size_of::<Vertex>()) as i32, 0));
    check!(gl::enable_vertex_attrib_array(1));
    check!(gl::vertex_attrib_divisor(1, 1));

    check!(gl::vertex_attrib_pointer_u8(2, 3, true, (sys::size_of::<Vertex>()) as i32, 
                                        (sys::size_of::<Vec3i>()) as u32));
    check!(gl::enable_vertex_attrib_array(2));
    check!(gl::vertex_attrib_divisor(2, 1));

    //check!(gl::polygon_mode(gl::FRONT_AND_BACK, gl::LINE));
    check!(gl::draw_arrays_instanced(gl::TRIANGLES, 0, 36, self.indices.len() as i32));
    //check!(gl::polygon_mode(gl::FRONT_AND_BACK, gl::FILL));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
    check!(gl::disable_vertex_attrib_array(2));
    check!(gl::bind_vertex_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
  }

  priv fn voxelize(&mut self, tris: &[Triangle])
  {
    /* Require at least one triangle. */
    assert!(tris.len() >= 1);
    debug!("VOXEL: Incoming triangles: %?", tris.len());

    /* Bounding box of vert dimensions. */
    let mut min = Vec3f::new( tris[0].verts[0].position.x,
                              tris[0].verts[0].position.y, 
                              tris[0].verts[0].position.z);
    let mut max = Vec3f::new( tris[0].verts[0].position.x,
                              tris[0].verts[0].position.y,
                              tris[0].verts[0].position.z);
    for tris.each |curr|
    {
      for curr.verts.each |vert|
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
    let center = Vec3f::new(max.x - ((max.x - min.x) / 2.0),
                            max.y - ((max.y - min.y) / 2.0),
                            max.z - ((max.z - min.z) / 2.0));
    debug!("VOXEL: Center of mesh is %s", center.to_str());

    /* Calculate, given resolution (how many voxels across), the dimensions of a voxel. */
    self.voxel_size = cmp::max( max.x - min.x,
                                cmp::max(max.y - min.y, max.z - min.z)) / (self.resolution as f32);
    debug!("VOXEL: Voxel size is %?", self.voxel_size);

    /* Create 3D array of voxels. */
    let mid_offset = (((self.resolution as f32) / 2.0) * self.voxel_size);
    debug!("VOXEL: Midpoint offset is %?", mid_offset);

    self.voxels = vec::with_capacity((f32::pow((self.resolution + 1) as f32, 3.0)) as uint);
    self.indices = vec::with_capacity((f32::pow((self.resolution + 1) as f32, 3.0)) as uint);
    for uint::range(0, self.resolution as uint) |z| 
    { for uint::range(0, self.resolution as uint) |y|
      { for uint::range(0, self.resolution as uint) |x|
        {
          self.voxels.push(Default);
        }
      }
    }
    assert!(self.voxels.len() == (f32::pow((self.resolution) as f32, 3.0)) as uint);

    for tris.each |tri|
    {
      /* Calculate bounding box of the triangle. */
      min = Vec3f::new(tri.verts[0].position.x, tri.verts[0].position.y, tri.verts[0].position.z);
      max = Vec3f::new(tri.verts[0].position.x, tri.verts[0].position.y, tri.verts[0].position.z);
      for tri.verts.each |vert|
      {
        min.x = cmp::min(min.x, vert.position.x);
        min.y = cmp::min(min.y, vert.position.y);
        min.z = cmp::min(min.z, vert.position.z);

        max.x = cmp::max(max.x, vert.position.x);
        max.y = cmp::max(max.y, vert.position.y);
        max.z = cmp::max(max.z, vert.position.z);
      }

      /* Determine what voxels lie in the bounding box. */
      let mut vox_amount = Vec3i::new(f32::ceil(((max.x - min.x) / self.voxel_size)) as i32,
                                      f32::ceil(((max.y - min.y) / self.voxel_size)) as i32,
                                      f32::ceil(((max.z - min.z) / self.voxel_size)) as i32);
      if vox_amount.x < 1
      { vox_amount.x = 1; }
      if vox_amount.y < 1
      { vox_amount.y = 1; }
      if vox_amount.z < 1
      { vox_amount.z = 1; }
      //debug!("VOXEL: [Per voxel] Checking %s surrounding voxels with SAT", vox_amount.to_str());

      let start_indices = Vec3i::new( ((min.x - -mid_offset) / self.voxel_size) as i32, 
                                      ((min.y - -mid_offset) / self.voxel_size) as i32,
                                      ((min.z - -mid_offset) / self.voxel_size) as i32);
      //debug!("VOXEL: [Per voxel] Starting indices are %s", start_indices.to_str());

      /* Test intersection with each accepted voxel. */
      /* TODO: Better loop syntax. */
      let mut z = start_indices.z;
      'collision: loop
      {
        if z == start_indices.z + vox_amount.z
        { break; }

        let mut y = start_indices.y;
        loop
        {
          if y == start_indices.y + vox_amount.y
          { break; }

          let mut x = start_indices.x;
          loop
          {
            if x == start_indices.x + vox_amount.x
            { break; }

            let index = (z * ((self.resolution * self.resolution) as i32)) + 
                        (y * (self.resolution as i32)) + x;

            /* Check for intersection. */
            let c = Vec3f::new( ((x as f32 - (self.resolution as f32 / 2.0)) * self.voxel_size) + (self.voxel_size / 2.0), 
                                ((y as f32 - (self.resolution as f32 / 2.0)) * self.voxel_size) + (self.voxel_size / 2.0),
                                ((z as f32 - (self.resolution as f32 / 2.0)) * self.voxel_size) + (self.voxel_size / 2.0));
            if tri_cube_intersect(c, self.voxel_size, tri)
            {
              /* We have intersection; add a reference to this voxel to the index map. */
              self.indices.push(Vertex
              {
                position: Vec3i::new( x - (self.resolution / 2) as i32, /* TODO: Remove duplicates. */
                                      y - (self.resolution / 2) as i32,
                                      z - (self.resolution / 2) as i32), 
                color: Vec3u8::new(tri.verts[0].color.x as u8, tri.verts[0].color.y as u8, tri.verts[0].color.z as u8), /* TODO: Conversion between Vec types. */
              });
            }
            
            x += 1;
          }
          y += 1;
        }
        z += 1;
      }
    }
    debug!("VOXEL: Enabled %? of %? voxels", self.indices.len(), self.voxels.len());
  }
}

macro_rules! find_min_max
(
  ($x0:expr, $x1:expr, $x2:expr) =>
  (
    {
      min = $x0;
      max = $x0;

      if($x1 < min){ min = $x1; }
      if($x1 > max){ max = $x1; }
      if($x2 < min){ min = $x2; }
      if($x2 > max){ max = $x2; }
    }
  )
)

/*======================== X-tests ========================*/
macro_rules! axis_test_x01
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      p0 = $a * v0.y - $b * v0.z;
      p2 = $a * v2.y - $b * v2.z;
      if p0 < p2  { min = p0; max = p2;} else { min = p2; max = p0; }
      rad = $fa * box_size + $fb * box_size;
      if min > rad || max < -rad  { return false; }
    }
  )
)

macro_rules! axis_test_x2
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      p0 = $a * v0.y - $b * v0.z;
      p1 = $a * v1.y - $b * v1.z;
      if p0 < p1 { min = p0; max = p1; } else { min = p1; max = p0; }
      rad = $fa * box_size + $fb * box_size;
      if min > rad || max < -rad { return false; }
    }
  )
)

/*======================== Y-tests ========================*/

macro_rules! axis_test_y02
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      p0 = -$a * v0.x + $b * v0.z;
      p2 = -$a * v2.x + $b * v2.z;
      if p0 < p2 { min = p0; max = p2; } else { min = p2; max = p0; }
      rad = $fa * box_size + $fb * box_size;
      if min > rad || max < -rad { return false; }
    }
  )
)

macro_rules! axis_test_y1
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      p0 = -$a * v0.x + $b * v0.z;
      p1 = -$a * v1.x + $b * v1.z;
      if p0 < p1 { min = p0; max = p1; } else { min = p1; max = p0; }
      rad = $fa * box_size + $fb * box_size;
      if min > rad || max < -rad { return false; }
    }
  )
)

/*======================== Z-tests ========================*/

macro_rules! axis_test_z12
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      p1 = $a * v1.x - $b * v1.y;
      p2 = $a * v2.x - $b * v2.y;
      if p2 < p1 { min = p2; max = p1;} else { min = p1; max = p2; }
      rad = $fa * box_size + $fb * box_size;
      if min > rad || max < -rad { return false; }
    }
  )
)


macro_rules! axis_test_z0
(
  ($a:expr, $b:expr, $fa:expr, $fb:expr) =>
  (
    {
      p0 = $a * v0.x - $b * v0.y;
      p1 = $a * v1.x - $b * v1.y;
      if p0 < p1 { min = p0; max = p1; } else { min = p1; max = p0; }
      rad = $fa * box_size + $fb * box_size;
      if min > rad || max < -rad { return false; }
    }
  )
)

#[inline(always)]
priv fn tri_cube_intersect(box_center: Vec3f, box_size: f32, tri: &Triangle) -> bool
{
  let mut v0, v1, v2;
  let mut min, max, p0, p1, p2, rad, fex, fey, fez;
  let mut normal, e0, e1, e2;

  /* Move everything so that the box's center is in (0, 0, 0). */
  v0 = tri.verts[0].position - box_center;
  v1 = tri.verts[1].position - box_center;
  v2 = tri.verts[2].position - box_center;

  /* Computer triangle edges. */
  e0 = v1 - v0; /* Edge 0. */
  e1 = v2 - v1; /* Edge 1. */
  e2 = v0 - v2; /* Edge 2. */

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 3 edge 0");
  /* Bullet 3. */
  fex = f32::abs(e0.x);
  fey = f32::abs(e0.y);
  fez = f32::abs(e0.z);
  axis_test_x01!(e0.z, e0.y, fez, fey);
  axis_test_y02!(e0.z, e0.x, fez, fex);
  axis_test_z12!(e0.y, e0.x, fey, fex);

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 3 edge 1");
  fex = f32::abs(e1.x);
  fey = f32::abs(e1.y);
  fez = f32::abs(e1.z);
  axis_test_x01!(e1.z, e1.y, fez, fey);
  axis_test_y02!(e1.z, e1.x, fez, fex);
  axis_test_z0!(e1.y, e1.x, fey, fex);

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 3 edge 2");
  fex = f32::abs(e2.x);
  fey = f32::abs(e2.y);
  fez = f32::abs(e2.z);
  axis_test_x2!(e2.z, e2.y, fez, fey);
  axis_test_y1!(e2.z, e2.x, fez, fex);
  axis_test_z12!(e2.y, e2.x, fey, fex);

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 1");
  /* Bullet 1. */
  /* Test in X-direction */
  find_min_max!(v0.x, v1.x, v2.x);
  if min > box_size || max < -box_size { return false; }

  /* Test in Y-direction */
  find_min_max!(v0.y, v1.y, v2.y);
  if min > box_size || max < -box_size { return false; }

  /* Test in Z-direction */
  find_min_max!(v0.z, v1.z, v2.z);
  if min > box_size || max < -box_size { return false; }

  //debug!("VOXEL: [Per voxel SAT] Testing bullet 2");
  /* Bullet 2. */
  normal = e0.cross(&e1);
  plane_cube_intersect(&normal, &v0, box_size)
}

#[inline(always)]
priv fn plane_cube_intersect(normal: &Vec3f, vert: &Vec3f, box_size: f32) -> bool
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

