// main.rs
// Ben Brittain
//

#[feature(globs)];

extern mod glfw;
extern mod gl;
extern mod green;
extern mod cgmath;
extern mod noise;

// standard libraries and such
use std::libc;
use std::gc;

// parts of this project
use chunk::*;
use renderer::*;
use world::*;
mod chunk;
mod renderer;
mod world;

fn main() {
  println!("Starting Aoxel...");

  //initialize GLFW
  do glfw::start {
    debug!("Setting up GL context");
    glfw::window_hint::context_version_major(3);
    glfw::window_hint::context_version_minor(2);
    glfw::window_hint::opengl_profile(glfw::OpenGlCoreProfile);

    let window = glfw::Window::create(800,600, "Aoxel", glfw::Windowed).unwrap();
    window.make_context_current();
    gl::load_with(glfw::get_proc_address);

    debug!("Initialize Callbacks");
    window.set_key_callback(~KeyContext);
//    window.set_cursor_pos_callback(~KeyContext);
//    window.set_focus_callback(


    // Create the World which manag:s Chunks
    let mut world: World = World::new();

    //initialize chunk 
//    let mut chunk:Chunk = Chunk::new_with_random();


    //initialize renderer
//    let renderer:Renderer = Renderer::new();

    while !window.should_close() {
      glfw::poll_events();

      // render
 //     renderer.update(&mut chunk);
      world.render();
      window.swap_buffers();
    }

  }
}

struct KeyContext;
impl glfw::KeyCallback for KeyContext {
  fn call (&self, window: &glfw::Window, key: glfw::Key,
          _scancode: libc::c_int, action: glfw::Action,
          _mods: glfw::Modifiers) {
    match (key, action) {
      (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),
      _ => println!("{}", key.to_str())
    }
  }
}


#[start]
fn start(argc: int, argv: **u8) -> int {
  //TODO initialize more advanced logging
  do green::start(argc, argv) {
    main();
  }
}
