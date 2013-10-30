/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: shared/obj/map.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Loader and handler of BSP maps.
*/

use std::{ vec, cmp, ptr, mem, cast };
use std::rt::io;
use std::rt::io::{ Reader, Seek };
use std::rt::io::file::{ FileReader, FileInfo };
use math;
use super::lump;
use primitive::{ Triangle, Vertex_PC };
use log::Log;

#[macro_escape]
#[path = "../../log/macros.rs"]
mod macros;

pub struct Map
{
  header: lump::Header,
  entity: lump::Entity,
  tris: ~[Triangle],
  verts: ~[lump::Vertex],
  faces: ~[lump::Face],
  mesh_verts: ~[lump::Mesh_Vert], 
  position: math::Vec3f,
  bb: math::BB3,
  error: ~str,
}

impl Map
{
  pub fn new(file: &str) -> Result<Map, ~str>
  {
    let mut map = Map
    {
      header: lump::Header::new(),
      entity: lump::Entity::new(),
      tris: ~[],
      verts: ~[],
      faces: ~[],
      mesh_verts: ~[],
      position: math::Vec3f::zero(),
      bb: math::BB3::zero(),
      error: ~"",
    };

    let fio = Path::new(file).open_reader(io::Open);
    if fio.is_none()
    { return Err(format!("Failed to read file: {}", file)); }
    let mut fio = fio.unwrap();
    let mut buff = vec::from_elem(mem::size_of::<lump::Header>(), 0u8);
    unsafe
    {
      let len = buff.len();
      fio.read(buff.mut_slice(0, len));
      ptr::copy_nonoverlapping_memory::<u8, *u8>(cast::transmute(&mut map.header),
                                                 vec::raw::to_ptr(buff), len);
    }

    log_assert!( map.header.magic[0] == 'I' as i8 &&
             map.header.magic[1] == 'B' as i8 &&
             map.header.magic[2] == 'S' as i8 &&
             map.header.magic[3] == 'P' as i8);

    if !map.read_verts(&mut fio)
    { return Err(map.error); }
    if !map.read_faces(&mut fio)
    { return Err(map.error); }
    if !map.read_mesh_verts(&mut fio)
    { return Err(map.error); }

    map.triangulate();
    
    Ok(map)
  }

  fn read_verts(&mut self, fio: &mut io::file::FileReader) -> bool
  {
    fio.seek(self.header.lumps[lump::Vertex_Type as int].offset as i64, io::SeekSet);
    let num_verts = (self.header.lumps[lump::Vertex_Type as int].length) /
                    (mem::size_of::<lump::Vertex>() as i32);
    if !(num_verts > 0)
    { self.error = ~"Invalid vertex count"; return false; }

    let mut vert = lump::Vertex::new();
    let mut buff = vec::from_elem(mem::size_of::<lump::Vertex>(), 0u8);
    for i in range(0, num_verts)
    {
      unsafe
      {
        let len = buff.len();
        fio.read(buff.mut_slice(0, len));
        ptr::copy_nonoverlapping_memory::<u8, *u8>(cast::transmute(&mut vert),
                                                   vec::raw::to_ptr(buff), len);
      }
      
      /* BSP likes Z to be up; we like Y to be up. */
      let temp = vert.position.y;
      vert.position.y = vert.position.z;
      vert.position.z = -temp;

      /* XXX: Color hack -- avoid pure black and pure white. */
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
    let mut min = math::Vec3f::new( self.verts[0].position.x,
                              self.verts[0].position.y, 
                              self.verts[0].position.z);
    let mut max = math::Vec3f::new( self.verts[0].position.x,
                              self.verts[0].position.y,
                              self.verts[0].position.z);
    for v in self.verts.iter()
    {
      min.x = cmp::min(min.x, v.position.x);
      min.y = cmp::min(min.y, v.position.y);
      min.z = cmp::min(min.z, v.position.z);

      max.x = cmp::max(max.x, v.position.x);
      max.y = cmp::max(max.y, v.position.y);
      max.z = cmp::max(max.z, v.position.z);
    }
    let center = math::Vec3f::new(max.x - ((max.x - min.x) / 2.0),
                            max.y - ((max.y - min.y) / 2.0),
                            max.z - ((max.z - min.z) / 2.0));

    /* Move the mesh by the center to the origin (easier to voxelize). */
    for v in self.verts.mut_iter()
    { v.position = v.position - center; }

    true
  }

  fn read_faces(&mut self, fio: &mut io::file::FileReader) -> bool
  {
    fio.seek(self.header.lumps[lump::Face_Type as int].offset as i64, io::SeekSet);
    let num_faces = (self.header.lumps[lump::Face_Type as int].length) /
                    (mem::size_of::<lump::Face>() as i32);
    if !(num_faces > 0)
    { self.error = ~"Invalid face count"; return false; }

    let mut face = lump::Face::new();
    let mut buff = vec::from_elem(mem::size_of::<lump::Face>(), 0u8);
    for _ in range(0, num_faces)
    {
      unsafe
      {
        let len = buff.len();
        fio.read(buff.mut_slice(0, len));
        ptr::copy_nonoverlapping_memory::<u8, *u8>(cast::transmute(&mut face),
                                                   vec::raw::to_ptr(buff), len);
      }
      self.faces.push(face);
    }

    true
  }

  fn read_mesh_verts(&mut self, fio: &mut io::file::FileReader) -> bool
  {
    fio.seek(self.header.lumps[lump::Mesh_Vert_Type as int].offset as i64, io::SeekSet);
    let num_obj = (self.header.lumps[lump::Mesh_Vert_Type as int].length) /
                    (mem::size_of::<lump::Mesh_Vert>() as i32);
    if !(num_obj > 0)
    { self.error = ~"Invalid object count"; return false; }

    let mut obj = lump::Mesh_Vert::new();
    let mut buff = vec::from_elem(mem::size_of::<lump::Mesh_Vert>(), 0u8);
    for _ in range(0, num_obj)
    {
      unsafe
      {
        let len = buff.len();
        fio.read(buff.mut_slice(0, len));
        ptr::copy_nonoverlapping_memory::<u8, *u8>(cast::transmute(&mut obj),
                                                   vec::raw::to_ptr(buff), len);
      }
      self.mesh_verts.push(obj);
    }

    true
  }

  fn triangulate(&mut self)
  {
    let mut verts: ~[lump::Vertex] = ~[];
    for face in self.faces.iter()
    {
      if face.kind != 1 { continue; }

      match face.num_vertices
      {
        n if n >= 3 =>
        {
          for i in range(0, n - 2)
          {
            verts.push(self.verts[face.start_vertex]);
            verts.push(self.verts[face.start_vertex + i + 2]);
            verts.push(self.verts[face.start_vertex + i + 1]);

            self.tris.push(Triangle::new( 
                        Vertex_PC::new(
                            self.verts[face.start_vertex].position,
                            math::Vec3f::new( self.verts[face.start_vertex].color.x as f32,
                                        self.verts[face.start_vertex].color.y as f32,
                                        self.verts[face.start_vertex].color.z as f32)),
                        Vertex_PC::new(
                            self.verts[face.start_vertex + i + 2].position,
                            math::Vec3f::new( self.verts[face.start_vertex + i + 2].color.x as f32,
                                        self.verts[face.start_vertex + i + 2].color.y as f32,
                                        self.verts[face.start_vertex + i + 2].color.z as f32)),
                        Vertex_PC::new(
                            self.verts[face.start_vertex + i + 1].position,
                            math::Vec3f::new( self.verts[face.start_vertex + i + 1].color.x as f32,
                                        self.verts[face.start_vertex + i + 1].color.y as f32,
                                        self.verts[face.start_vertex + i + 1].color.z as f32))));
          }
        }
        /* Something else. */
        n => { log_info!("Invalid face: {}", n); }
      }
    };

    self.verts = verts;
    log_debug!("Trianglulated to {} faces", self.verts.len());
  }
}

