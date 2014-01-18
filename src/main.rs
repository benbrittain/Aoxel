// main.rs
// Ben Brittain
//

extern mod glfw;
extern mod gl;
extern mod green;

use std::libc;

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

    //TODO initialize world
  }
}

struct KeyContext;
impl glfw::KeyCallback for KeyContext {
  fn call (&self, window: &glfw::Window, key: glfw::Key,
          scancode: libc::c_int, action: glfw::Action,
          mods: glfw::Modifiers) {
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
