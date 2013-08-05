/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: gl/worker.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      A utility group of functionality
      for running tasks in the background.
*/

use std::{ comm, task, local_data };
use glfw;
use gl2 = opengles::gl2;
use util;

#[macro_escape]
mod check;

static tls_key: local_data::Key<@Worker> = &local_data::Key;

/* The bool represents whether or not the worker should exit.
 * Use false for most everything. */
type GL_Func = ~fn() -> bool;

struct Worker
{
  task_channel: comm::Chan<GL_Func>,
}

impl Worker
{
  pub fn initialize(window: glfw::Window) -> comm::Port<bool>
  {
    /* The task port and channel are used to tell the worker thread
     * when to run new functions. */
    let (task_port, task_channel): (comm::Port<GL_Func>, comm::Chan<GL_Func>) = comm::stream();

    /* The exit port is used only once: When the main thread signals
     * the GL worker thread to exit. The main thread will then block
     * until the GL thread sends data through the exit channel, signaling
     * that it has finished all of its work. */
    let (exit_port, exit_channel): (comm::Port<bool>, comm::Chan<bool>) = comm::stream();
    task::spawn_with((task_port, exit_channel, window), Worker::thread);

    /* Store the worker in task-local storage. (singleton) */
    let worker = @Worker
    {
      task_channel: task_channel,
    };
    local_data::set(tls_key, worker);

    /* The main thread will need to listen on the exit port for when
     * the GL worker is finished. */
    exit_port
  }

  priv fn get() -> @Worker
  {
    local_data::get(tls_key, 
    |opt|
    {
      match opt
      {
        Some(x) => *x,
        None => fail!("Singleton not available")
      }
    })
  }

  pub fn new_task(func: GL_Func)
  {
    let worker = Worker::get();
    worker.task_channel.send(func);
  }

  priv fn thread(data: (comm::Port<GL_Func>, comm::Chan<bool>, glfw::Window))
  {
    let (task_port, exit_channel, window) = data;
    window.make_context_current();
    util::Log::initialize(); /* Done for every thread. */

    loop
    {
      let func = task_port.recv();

      /* Run the func. If it returns true, exit
       * the worker thread. */
      let ret = func();
      if ret
      { break; }

      check!(gl2::flush());
    }

    /* Let the main thread know we're done. */
    glfw::detach_current_context();
    exit_channel.send(true);
  }
}

