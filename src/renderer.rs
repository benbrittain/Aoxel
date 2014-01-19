// renderer.rs
// render in a unique task

extern mod glfw;
extern mod gl;

use gl::types::*;
use gl::*;

pub struct Renderer {
  // OpenGL Buffers
  vao: GLuint,
  vbo: GLuint
//ebo: int,

  // Window
//  window: glfw::Window

}


impl Renderer {
//  pub fn new(window: @glfw::Window) -> Renderer {
  pub fn new() -> Renderer {
    let mut renderer = Renderer {
      vao: 0,
      vbo: 0,
    };
    // Vertice Array Object
    unsafe {
      gl::GenVertexArrays(1, &mut renderer.vao);
      gl::BindVertexArray(renderer.vao);

    // Vertice Buffer Object
      gl::GenBuffers(1, &mut renderer.vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);
    }
    renderer
  }
}
