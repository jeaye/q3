/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: obj/map.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Loader and handler of BSP maps.
*/

use std::{ i32, cmp, path, io, sys, cast };
use math::{ Vec3f, Vec4u8, BB3 };
use primitive::{ Triangle, Vertex_PC };

#[path = "lump.rs"]
mod lump;
#[path = "../../gl/mod.rs"]
mod gl;

#[path = "../../gl/check.rs"]
mod check;

pub struct Map
{
  header: lump::Header,
  entity: lump::Entity,
  tris: ~[Triangle],
  verts: ~[lump::Vertex],
  faces: ~[lump::Face],
  mesh_verts: ~[lump::Mesh_Vert], 
  vao: gl::GLuint,
  vbo: gl::GLuint, 
  position: Vec3f, /* TODO: Trait for positional objects. */
  bb: BB3
}

impl Map
{
  pub fn new(file: &str) -> Map
  {
    let mut map = Map
    {
      header: lump::Header::new(),
      entity: lump::Entity::new(),
      tris: ~[],
      verts: ~[],
      faces: ~[],
      mesh_verts: ~[],
      vao: 0,
      vbo: 0,
      position: Vec3f::zero(),
      bb: BB3::zero(),
    };

    let fio = io::file_reader(@path::PosixPath(file)).unwrap();
    unsafe {  fio.read( cast::transmute((&map.header, sys::size_of::<lump::Header>())),
                        sys::size_of::<lump::Header>()); }

    assert!( map.header.magic[0] == 'I' as i8 &&
             map.header.magic[1] == 'B' as i8 &&
             map.header.magic[2] == 'S' as i8 &&
             map.header.magic[3] == 'P' as i8);

    map.read_verts(fio);
    map.read_faces(fio);
    map.read_mesh_verts(fio);

    map.triangulate();
    map.upload();
    
    map
  }


  /* TODO: Make this generic. */
  priv fn read_verts(&mut self, fio: @io::Reader)
  {
    fio.seek(self.header.lumps[lump::Vertex_Type as int].offset as int, io::SeekSet);
    let num_verts = (self.header.lumps[lump::Vertex_Type as int].length) /
                    (sys::size_of::<lump::Vertex>() as i32);
    assert!(num_verts > 0);

    let mut vert = lump::Vertex::new();
    for i32::range(0, num_verts) |i|
    {
      unsafe { fio.read( cast::transmute((&vert, sys::size_of::<lump::Vertex>())),
                sys::size_of::<lump::Vertex>()); }
      
      /* BSP likes Z to be up; we like Y to be up. */
      let temp = vert.position.y;
      vert.position.y = vert.position.z;
      vert.position.z = -temp;

      /* TODO: Color hack. */
      if vert.color.x == 0 { vert.color.x += 100; }
      if vert.color.y == 0 { vert.color.y += 100; }
      if vert.color.z == 0 { vert.color.z += 100; }

      if vert.color.x == 255 { vert.color.x -= 100; }
      if vert.color.y == 255 { vert.color.y -= 100; }
      if vert.color.z == 255 { vert.color.z -= 100; }
      vert.color.w = 1;

      /* Global size scale. */
      vert.position.x /= 32.0;
      vert.position.y /= 32.0;
      vert.position.z /= 32.0;

      /* Create bounding box based on first vert; this'll update as more come. */
      match i
      {
        0 =>
        { self.bb.top_left = vert.position; self.bb.bottom_right = vert.position; }
        _ =>
        {
          /* Keep track of bounds. */
          if vert.position.x < self.bb.top_left.x
          { self.bb.top_left.x = vert.position.x; }
          else if vert.position.x > self.bb.bottom_right.x
          { self.bb.bottom_right.x = vert.position.x }

          if vert.position.y > self.bb.top_left.y
          { self.bb.top_left.y = vert.position.y; }
          else if vert.position.y < self.bb.bottom_right.y
          { self.bb.bottom_right.y = vert.position.y }

          if vert.position.z > self.bb.top_left.z
          { self.bb.top_left.z = vert.position.z; }
          else if vert.position.z < self.bb.bottom_right.z
          { self.bb.bottom_right.z = vert.position.z }
        }
      }

      self.verts.push(vert);
    }

    /* Calculate the mesh's bounding box. */
    let mut min = Vec3f::new( self.verts[0].position.x,
                              self.verts[0].position.y, 
                              self.verts[0].position.z);
    let mut max = Vec3f::new( self.verts[0].position.x,
                              self.verts[0].position.y,
                              self.verts[0].position.z);
    for self.verts.iter().advance |v|
    {
      min.x = cmp::min(min.x, v.position.x);
      min.y = cmp::min(min.y, v.position.y);
      min.z = cmp::min(min.z, v.position.z);

      max.x = cmp::max(max.x, v.position.x);
      max.y = cmp::max(max.y, v.position.y);
      max.z = cmp::max(max.z, v.position.z);
    }
    let center = Vec3f::new(max.x - ((max.x - min.x) / 2.0),
                            max.y - ((max.y - min.y) / 2.0),
                            max.z - ((max.z - min.z) / 2.0));

    /* Move the mesh by the center to the origin (easier to voxelize). */
    for self.verts.mut_iter().advance |v|
    { v.position -= center; }
  }

  priv fn read_faces(&mut self, fio: @io::Reader)
  {
    fio.seek(self.header.lumps[lump::Face_Type as int].offset as int, io::SeekSet);
    let num_faces = (self.header.lumps[lump::Face_Type as int].length) /
                    (sys::size_of::<lump::Face>() as i32);
    assert!(num_faces > 0);

    let face = lump::Face::new();
    for i32::range(0, num_faces) |_|
    {
      unsafe { fio.read( cast::transmute((&face, sys::size_of::<lump::Face>())),
                sys::size_of::<lump::Face>()); }
      self.faces.push(face);
    }
  }

  priv fn read_mesh_verts(&mut self, fio: @io::Reader)
  {
    fio.seek(self.header.lumps[lump::Mesh_Vert_Type as int].offset as int, io::SeekSet);
    let num_obj = (self.header.lumps[lump::Mesh_Vert_Type as int].length) /
                    (sys::size_of::<lump::Mesh_Vert>() as i32);
    assert!(num_obj > 0);

    let obj = lump::Mesh_Vert::new();
    for i32::range(0, num_obj) |_|
    {
      unsafe { fio.read( cast::transmute((&obj, sys::size_of::<lump::Mesh_Vert>())),
                sys::size_of::<lump::Mesh_Vert>()); }
      self.mesh_verts.push(obj);
    }
  }

  priv fn triangulate(&mut self)
  {
    let mut verts: ~[lump::Vertex] = ~[];
    for self.faces.iter().advance |face|
    {
      if face.kind != 1 { loop; }

      match face.num_vertices
      {
        n if n >= 3 =>
        {
          for i32::range(0, n - 2) |i|
          {
            verts.push(self.verts[face.start_vertex]);
            verts.push(self.verts[face.start_vertex + i + 2]);
            verts.push(self.verts[face.start_vertex + i + 1]);

            self.tris.push(Triangle::new( 
                        Vertex_PC::new(
                            self.verts[face.start_vertex].position,
                            Vec3f::new( self.verts[face.start_vertex].color.x as f32,
                                        self.verts[face.start_vertex].color.y as f32,
                                        self.verts[face.start_vertex].color.z as f32)),
                        Vertex_PC::new(
                            self.verts[face.start_vertex + i + 2].position,
                            Vec3f::new( self.verts[face.start_vertex + i + 2].color.x as f32,
                                        self.verts[face.start_vertex + i + 2].color.y as f32,
                                        self.verts[face.start_vertex + i + 2].color.z as f32)),
                        Vertex_PC::new(
                            self.verts[face.start_vertex + i + 1].position,
                            Vec3f::new( self.verts[face.start_vertex + i + 1].color.x as f32,
                                        self.verts[face.start_vertex + i + 1].color.y as f32,
                                        self.verts[face.start_vertex + i + 1].color.z as f32))));
          }
        }
        /* Something else. */
        n => { warn!(fmt!("BSP: Invalid face: %?", n)); }
      }
    };

    self.verts = verts;
    debug!("BSP: Trianglulated to %? faces.", self.verts.len());
  }

  priv fn upload(&mut self)
  {
    let name = check!(gl::gen_vertex_arrays(1));
    assert!(name.len() == 1);
    self.vao = name[0];

    let name = check!(gl::gen_buffers(1));
    assert!(name.len() == 1);
    self.vbo = name[0];

    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));
    check!(gl::buffer_data(gl::ARRAY_BUFFER, self.verts, gl::STATIC_DRAW));
  }

  pub fn draw(&self)
  {
    check!(gl::bind_vertex_array(self.vao));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, self.vbo));
    check!(gl::enable_vertex_attrib_array(0));
    check!(gl::enable_vertex_attrib_array(1));

    check!(gl::vertex_attrib_pointer_f32(0, 3, false, 
                sys::size_of::<lump::Vertex>() as i32, 
                0));
    check!(gl::vertex_attrib_pointer_u8(1, 4, true, 
                sys::size_of::<lump::Vertex>() as i32, 
                sys::size_of::<lump::Vertex>() as u32 -
                sys::size_of::<Vec4u8>() as u32));
    check!(gl::draw_arrays(gl::TRIANGLES, 0, self.verts.len() as i32));

    check!(gl::disable_vertex_attrib_array(0));
    check!(gl::disable_vertex_attrib_array(1));
    check!(gl::bind_vertex_array(0));
    check!(gl::bind_buffer(gl::ARRAY_BUFFER, 0));
  }

  #[inline(always)]
  pub fn center(&self) -> Vec3f
  { self.bb.center_with_offset(self.position) }
}

