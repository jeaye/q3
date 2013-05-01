/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: main.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Entry point.
*/

extern mod std;
extern mod opengles;
extern mod glfw;

#[path = "gl/mod.rs"]
mod gl;

#[path = "math/mod.rs"]
mod math;

#[macro_escape]
#[path = "gl/check.rs"]
mod check;

#[path = "obj/bsp/mod.rs"]
mod bsp; 

#[path = "gl/ttf/mod.rs"]
mod ttf;

#[path = "obj/primitive/mod.rs"]
mod primitive;

#[path = "obj/voxel/mod.rs"]
mod voxel;

fn main() {
  glfw::set_error_callback(error_callback);

  do glfw::spawn {
    glfw::window_hint::context_version(3, 2);
    glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
    glfw::window_hint::opengl_forward_compat(true);

    let window_res = glfw::Window::create(1024, 768, "Q^3", glfw::Windowed);
    if window_res.is_err()
    { fail!(window_res.get_err()); }
    let window = @window_res.get();
    window.make_context_current();

    let camera = @mut gl::Camera::new(window);
    camera.init();

    /* Setup callbacks. */
    window.set_cursor_mode(glfw::CURSOR_CAPTURED);
    do window.set_size_callback |_, width, height|
    { camera.resize(width as i32, height as i32); }
    do window.set_cursor_pos_callback |_, x, y|
    { camera.mouse_moved(x, y); }
    do window.set_key_callback |window, key, action|
    {
      camera.key_action(key, action);
      key_callback(window, key, action);
    }

    let mut sphere = primitive::Sphere::new(100.0, 5);

    let st = std::time::precise_time_s();
    let vox_sphere = voxel::Map::new(sphere.tris, 30);
    let et = std::time::precise_time_s();
    io::println(fmt!("Voxelization took %? seconds.", (et - st)));

    /* Temp test for font loading. */
    let mut font_renderer = ttf::Renderer::new();
    let mut font = ttf::Font::new("data/test.ttf", 50);

    let map = bsp::Map::new("data/q3ctf1.bsp");
    //let map = map::Map::new("data/dk.bsp");

    /* Shader Creation. */
    let shader = @mut gl::Shader_Builder::new_with_files("data/shaders/color.vert", "data/shaders/color.frag");
    shader.bind();

    let proj_loc = shader.get_uniform_location(~"proj");
    let world_loc = shader.get_uniform_location(~"world");

    let mut cur_time = (std::time::precise_time_ns() / 10000) as f32; // Hundredth of a second
    let mut last_time = cur_time;

    while !window.should_close() {
      glfw::poll_events();

      /* Delta time. */
      let delta = cur_time - last_time;
      last_time = cur_time;
      cur_time = (std::time::precise_time_ns() / 10000) as f32;

      camera.update(delta);
      shader.bind();
      shader.update_uniform_mat(proj_loc, camera.projection);
      shader.update_uniform_mat(world_loc, camera.view);

      let fps = camera.frame_rate;

      check!(gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
      {
        //map.draw();
        //sphere.draw();
        vox_sphere.draw();
        font_renderer.begin(camera);
        font_renderer.render(fmt!("%?", fps), math::Vec2f::new(0.0, 0.0), &font);
        font_renderer.end();
      } window.swap_buffers();

      std::timer::sleep(@std::uv::global_loop::get(), 1000 / (camera.target_frame_rate as uint));
    }
  }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int)
{
  if action == glfw::PRESS && key == glfw::KEY_ESCAPE
  { window.set_should_close(true); }
}

fn error_callback(error: libc::c_int, description: ~str)
{ error!("GLFW Error %?: %s", error, description); }

