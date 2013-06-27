/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: main.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Entry point.
*/

extern mod extra;
extern mod opengles;
extern mod glfw;

use std::libc;

#[nolink]
#[link_args="-lX11 -lXrandr -lXi -lXxf86vm"]
extern { }

#[path = "gl/mod.rs"]
mod gl;

#[path = "math/mod.rs"]
mod math;

#[macro_escape]
#[path = "gl/check.rs"]
mod check;

#[path = "obj/bsp/mod.rs"]
mod bsp; 

#[path = "ui/mod.rs"]
mod ui;

#[path = "obj/primitive/mod.rs"]
mod primitive;

#[path = "obj/voxel/mod.rs"]
mod voxel;

fn main()
{
  glfw::set_error_callback(error_callback);

  do glfw::spawn
  {
    glfw::window_hint::context_version(3, 2);
    glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
    glfw::window_hint::opengl_forward_compat(true);

    let window_res = glfw::Window::create(1024, 768, "Q^3", glfw::Windowed);
    let window = match window_res
    {
      Some(win) => { @win },
      None => { fail!("Failed to create window!") }
    };
    window.make_context_current();

    let console = @mut ui::Console::new();
    let console_activator = ui::Console_Activator::new(console);

    let camera = gl::Camera::new(window);
    camera.init();

    /* Setup UI elements. */
    let ui_input = @mut ui::Input_State::new();
    ui_input.push(camera as @mut ui::Input_Listener);
    ui_input.push(console_activator as @mut ui::Input_Listener);

    /* Setup callbacks. */ /* TODO: Crash on close with these callbacks. */
    window.set_cursor_mode(glfw::CURSOR_DISABLED);
    do window.set_size_callback |_, width, height|
    { camera.resize(width as i32, height as i32); }
    do window.set_cursor_pos_callback |_, x, y| 
    { ui_input.mouse_moved(x as f32, y as f32); }
    do window.set_char_callback |_, c|
    { ui_input.key_char(c); }
    do window.set_key_callback |window, key, _scancode, action, mods|
    {
      ui_input.key_action(key, action, mods);
      key_callback(window, key, action);
    }

    let ui_renderer = @mut ui::Renderer::new();

    let map = bsp::Map::new("data/maps/q3ctf1.bsp");

    let st = extra::time::precise_time_s();
    let vox_map = voxel::Map::new(map.tris, 200);
    let et = extra::time::precise_time_s();
    println(fmt!("Voxel map creation took %? seconds.", (et - st)));

    /* Temp test for font loading. */
    let font = ui::Font::new("data/fonts/test.ttf", 30);

    /* Shader Creation. */
    let vox_shader = @mut gl::Shader_Builder::new_with_files("data/shaders/voxel.vert", "data/shaders/voxel.frag");
    let color_shader = @mut gl::Shader_Builder::new_with_files("data/shaders/color.vert", "data/shaders/color.frag");
    vox_shader.bind();

    let proj_loc = vox_shader.get_uniform_location("proj");
    let world_loc = vox_shader.get_uniform_location("world");
    let voxel_size_loc = vox_shader.get_uniform_location("voxel_size");
    let offsets_loc = vox_shader.get_uniform_location("offsets");
    let color_proj_loc = color_shader.get_uniform_location("proj");
    let color_world_loc = color_shader.get_uniform_location("world");

    vox_shader.update_uniform_i32(offsets_loc, 0);

    let mut cur_time = (extra::time::precise_time_ns() / 10000) as f32; // Hundredth of a second
    let mut last_time = cur_time;

    /* Console functions. */
    console_activator.add_accessor("q3.version", |_|
    { fmt!("%s.%s", env!("VERSION"), env!("COMMIT")) });

    while !window.should_close()
    {
      glfw::poll_events();

      /* Delta time. */
      let delta = cur_time - last_time;
      last_time = cur_time;
      cur_time = (extra::time::precise_time_ns() / 10000) as f32;

      console.update(delta);
      camera.update(delta);

      vox_shader.bind();
      vox_shader.update_uniform_mat(proj_loc, &camera.projection);
      vox_shader.update_uniform_mat(world_loc, &camera.view);
      vox_shader.update_uniform_f32(voxel_size_loc, vox_map.voxel_size);

      color_shader.bind();
      color_shader.update_uniform_mat(color_proj_loc, &camera.projection);
      color_shader.update_uniform_mat(color_world_loc, &camera.view);

      let fps = camera.frame_rate;

      check!(gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));
      {
        //color_shader.bind();
        //map.draw();

        vox_shader.bind();
        vox_map.draw();

        ui_renderer.begin(camera);
        
        console.render(ui_renderer);

        if camera.show_fps
        { ui_renderer.render_font(fmt!("%?", fps), math::Vec2f::new(camera.window_size.x as f32 - 40.0, 0.0), &font); }

        ui_renderer.end();
      } window.swap_buffers();

      //extra::timer::sleep(@extra::uv::global_loop::get(), 1000 / (camera.target_frame_rate as uint));
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

