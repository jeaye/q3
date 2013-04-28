/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: src/obj/voxel/map.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A voxelization of arbitrary triangles
      into OpenGL-ready cubes.
*/

use math::{ Vec3f };
use primitive::Vertex_PC;
use primitive::Triangle;
use primitive::{ Cube, Cube_Index };

#[path = "../../gl/mod.rs"]
mod gl;
#[path = "../../gl/util.rs"]
mod util;
#[macro_escape]
#[path = "../../gl/check.rs"]
mod check;

macro_rules! index
(
  ($arr:ident[$x:expr][$y:expr][$z:expr]) => 
  (
    $arr[($z * self.resolution * self.resolution) + (y * self.resolution) + x]
  )
)

struct Map
{
  resolution: u32,

  vao: gl::GLuint,
  vbo: gl::GLuint,
  ibo: gl::GLuint,

  voxels: ~[Cube],
  indices: ~[Cube_Index],
  //types: ~[Voxel_Type], /* TODO: Different types of voxels. */
}

impl Map
{
  pub fn new(tris: &[Triangle], res: u32) -> Map
  {
    let mut map = Map
    {
      resolution: res,
      vao: 0,
      vbo: 0,
      ibo: 0,
      voxels: ~[],
      indices: ~[],
      //types: ~[],
    };

    map.voxelize(tris);

    map.vao = check!(gl::gen_vertex_arrays(1))[0]; /* TODO: Check these. */
    map.vbo = check!(gl::gen_buffers(1))[0];
    map.ibo = check!(gl::gen_buffers(1))[0];
    check!(gl::bind_vertex_array(map.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, map.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, map.voxels, gl::STATIC_DRAW));

    check!(gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, map.ibo));
    check!(gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, map.indices, gl::STATIC_DRAW));

    map
  }

  pub fn draw(&self)
  {
    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));
    check!(gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo));

    check!(gl::vertex_attrib_pointer_f32(0, 3, false, (sys::size_of::<Vertex_PC>()) as i32, 0));
    check!(gl::vertex_attrib_pointer_f32(1, 3, false, (sys::size_of::<Vertex_PC>()) as i32, sys::size_of::<Vec3f>() as u32));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));

    check!(gl::polygon_mode(gl::FRONT_AND_BACK, gl::LINE));
    check!(gl::draw_elements(gl::TRIANGLES, self.indices.len() as i32 * 36, gl::UNSIGNED_INT, None));
    check!(gl::polygon_mode(gl::FRONT_AND_BACK, gl::FILL));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
    check!(gl::bind_vertex_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
    check!(gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, 0));
  }

  priv fn voxelize(&mut self, tris: &[Triangle])
  {
    /* Require at least one triangle. */
    assert!(tris.len() >= 1);

    /* Bounding box of vert dimensions. */
    let mut min = Vec3f::new(tris[0].verts[0].position.x, tris[0].verts[0].position.y, tris[0].verts[0].position.z);
    let mut max = Vec3f::new(tris[0].verts[0].position.x, tris[0].verts[0].position.y, tris[0].verts[0].position.z);
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
    let center = Vec3f::new(max.x - ((max.x - min.x) / 2.0), max.y - ((max.y - min.y) / 2.0), max.z - ((max.z - min.z) / 2.0));

    /* Calculate, given resolution (how many voxels across), the dimensions of a voxel. */
    let size = cmp::max(max.x - min.x, cmp::max(max.y - min.y, max.z - min.z)) / (self.resolution as f32);

    /* Create 3D array of voxels. */
    let mid_offset = (((self.resolution  as f32) / 2.0) * size);
    self.voxels = vec::with_capacity((f32::pow((self.resolution + 1) as f32, 3.0)) as uint);
    let adjusted_resolution: uint = self.resolution as uint + 1;
    for uint::range(0, adjusted_resolution) |z| 
    { for uint::range(0, adjusted_resolution) |y|
      { for uint::range(0, adjusted_resolution) |x|
        {
          let c = Vec3f::new((x as f32 * size) - mid_offset, (y as f32 * size) - mid_offset, (z as f32 * size) - mid_offset) - center;
          let cube = Cube::new(size, c);
          self.voxels.push(cube);

          /* Check if this cube intersects with any triangles. */
          for tris.each |tri|
          {
            if tri_cube_intersect(c, size, tri, x)
            {
              self.indices.push(Cube_Index::new(((z * adjusted_resolution * adjusted_resolution) + (y * adjusted_resolution) + x) as u32));
              break;
            }
          }
        }
      }
    }
    assert!(self.voxels.len() == (f32::pow((adjusted_resolution) as f32, 3.0)) as uint);
  }
}

priv fn tri_cube_intersect(box_center: Vec3f, box_size: f32, tri: &Triangle, x: uint) -> bool
{

  /* There is an overlap. */
  true
}

