// renderer.rs
// render in a unique task

extern mod glfw;
extern mod gl;
extern mod cgmath;

use std::mem;
use std::cast;
use std::ptr;
use std::vec;
use std::rand;

use std::rand::*;
use gl::types::*;
use gl::*;

use cgmath::projection;
use cgmath::matrix::*;
use cgmath::ptr::*;
use cgmath::angle::*;
use cgmath::quaternion::*;
use cgmath::point::*;
use cgmath::vector::*;

use chunk::*;
use chunk::Block;

// TODO change lifetime
// TODO load into renderer
// TODO extract into data file
// TODO change variables to a more rust style

static VS_SRC: &'static str =
  "#version 150\n\
  in vec3 position;\n\

  in vec3 color;\n\
  out vec3 Color;\n\

  uniform vec3 offset;\n\
  uniform mat4 view;\n\
  uniform mat4 proj;\n\

  void main() {\n\
    Color = color;\n\
    vec4 camera_pos = vec4(position.x, position.y, position.z, 1.0) + vec4(offset.x, offset.y, offset.z, 0.0); \n\
    gl_Position = proj * view * camera_pos;\n\
  }";

static FS_SRC: &'static str =
  "#version 150\n\
  in vec3 Color;\n\
  out vec4 outColor;\n\
  void main(){\n\
      outColor = vec4(Color.r/4, Color.r/4, Color.r/4, 1.0); \n\
  }";

pub struct Renderer {
  // OpenGL Buffers
  vao: GLuint,
  vbo: GLuint,
  ebo: GLuint,
  program: GLuint

  // Window
//  window: glfw::Window

}


impl Renderer {
//  pub fn new(window: @glfw::Window) -> Renderer {
  pub fn new() -> Renderer {
    let mut renderer = Renderer {
      vao: 0,
      vbo: 0,
      ebo: 0,
      program: 0
    };


    gl::Enable(gl::DEPTH_TEST);
    //    TODO enable
//    gl::Enable(gl::CULL_FACE);

    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);

    renderer.program = gl::CreateProgram();

    // Make shaders
    gl::AttachShader(renderer.program, vs);
    gl::AttachShader(renderer.program, fs);

    unsafe {
      // Link Frag buffer
      "outColor".with_c_str(|ptr| gl::BindFragDataLocation(renderer.program, 0, ptr));
      gl::LinkProgram(renderer.program);

      // Vertice Array Object
      gl::GenVertexArrays(1, &mut renderer.vao);
      gl::BindVertexArray(renderer.vao);

      // Vertice Buffer Object
      gl::GenBuffers(1, &mut renderer.vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, renderer.vbo);

      // Element Buffer Object
//      gl::GenBuffers(1, &mut renderer.ebo);
//      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, renderer.ebo);
//      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
//                     (elements.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
//                     cast::transmute(&elements), gl::STATIC_DRAW);

    // Use Shader
    gl::UseProgram(renderer.program);

    let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(renderer.program, ptr));
    gl::EnableVertexAttribArray(pos_attr as GLuint);
    gl::VertexAttribPointer(pos_attr as u32, 3, gl::BYTE, gl::FALSE,
                            (4 * mem::size_of::<GLbyte>()) as i32, ptr::null());

    let col_attr = "color".with_c_str(|ptr| gl::GetAttribLocation(renderer.program, ptr));
    gl::EnableVertexAttribArray(col_attr as GLuint);
    gl::VertexAttribPointer(col_attr as u32, 1, gl::BYTE, gl::FALSE,
                            (4 *mem::size_of::<GLbyte>()) as GLsizei,
                            cast::transmute(3*mem::size_of::<GLbyte>() as uint));

    //glVertexAttribPointer(colAttrib, 3, GL_FLOAT, GL_FALSE, 8 * sizeof(GLfloat), (void*)(3 * sizeof(GLfloat)));

    }
    renderer
  }

  pub fn update(&self, chunk: &mut Chunk) {
    if chunk.update {
      let mut block_vertexes: ~[GLbyte] = ~[];
      chunk.reset_update();

    // loop over blocks in the chunk
      for z in range(0, chunk.len()) {
        for x in range(0, chunk.len()) {
          for y in range(0, chunk.len()) {
            match chunk.get_block(x, y, z) {
              Some(block) =>  {
                //TODO append instead vec::append(block_vertexes, gen_vertex(x,y,z));
//                let mut rng = rand::task_rng();
//                let r: int = rng.gen_range(0, 2);
//

                for i in gen_vertex(x, y, z, block, chunk).iter(){
                  block_vertexes.push(*i);
                }
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                unsafe{
                  gl::BufferData(gl::ARRAY_BUFFER,
                  (block_vertexes.len() * mem::size_of::<GLbyte>()) as GLsizeiptr,
                  cast::transmute(&block_vertexes[0]), gl::STATIC_DRAW);
                }
              }
              None => ()// println!("no block to render at ({},{},{})", x, y, z)
            }

          }
        }
      }


      unsafe {
        let view:Mat4<f32> = Mat4::look_at(&Point3::new(75.0 as f32, 75.0, 75.0),
        &Point3::new(0.0 as f32, 0.0, 0.0),
        &Vec3::new(0.0 as f32, 0.0, 1.0));
        let uni_view = "view".with_c_str(|ptr| gl::GetUniformLocation(self.program, ptr));
        gl::UniformMatrix4fv(uni_view , 1, gl::FALSE, view.ptr());

        let proj = projection::perspective(deg(45.0 as f32), 800.0/600.0, 1.0, 150.0);
        let uni_proj = "proj".with_c_str(|ptr| gl::GetUniformLocation(self.program, ptr));
        gl::UniformMatrix4fv(uni_proj, 1, gl::FALSE, proj.ptr());

        let uni_offset = "offset".with_c_str(|ptr| gl::GetUniformLocation(self.program, ptr));
        gl::Uniform3f(uni_offset, 0.0, 0.0, 0.0);

      }
      gl::ClearColor(0.1, 0.1, 0.1, 0.1);
      //TODO change to constant
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      //   glBindBuffer(gl::ARRAY_BUFFER, self.vbo);
      //   glVertexAttribPointer(attribute_coord, 4, GL_BYTE, GL_FALSE, 0, 0);

      // Draw to the screen
      gl::DrawArrays(gl::TRIANGLES, 0, block_vertexes.len() as i32);
    }
  }
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
  let shader = gl::CreateShader(ty);
  unsafe {
    // grab pointer for shader
    src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
    gl::CompileShader(shader);

    // compile status
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    if status != (gl::TRUE as GLint) {
      let mut len = 0;
      gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = vec::from_elem(len as uint, 0u8);
      gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
      print!("{}", src);
      fail!(buf);
    }
  }
  shader
}


fn gen_vertex(x_in: int, y_in: int, z_in: int, block_type: Block, chunk: &Chunk) -> ~[GLbyte] {

  let x: i8 = x_in as i8;
  let y: i8 = y_in as i8;
  let z: i8 = z_in as i8;
  let block_type: i8 = block_type as i8;

  let mut build_vec: ~[GLbyte] = ~[];

    // View from negative x
//  println!("{},{},{}",x_in,y_in,z_in);
  if chunk.get_block(x_in - 1, y_in , z_in).is_none() {
    build_vec = vec::append(build_vec,
                           [x,      y,      z,            block_type,
                            x,      y,      z + 1,        block_type,
                            x,      y + 1,  z,            block_type,
                            x,      y + 1,  z,            block_type,
                            x,      y,      z + 1,        block_type,
                            x,      y + 1,  z + 1,        block_type]);
  }

    // View from positive x
  if chunk.get_block(x_in + 1, y_in , z_in).is_none() {
    build_vec = vec::append(build_vec,
                           [x + 1,  y,      z,            block_type,
                            x + 1,  y + 1,  z,            block_type,
                            x + 1,  y,      z + 1,        block_type,
                            x + 1,  y + 1,  z,            block_type,
                            x + 1,  y + 1,  z + 1,        block_type,
                            x + 1,  y,      z + 1,        block_type]);
  }

    // View from negative y
  if chunk.get_block(x_in, y_in - 1, z_in).is_none() {
    build_vec = vec::append(build_vec,
                           [x,      y,      z,            block_type,
                            x + 1,  y,      z,            block_type,
                            x + 1,  y,      z + 1,        block_type,
                            x + 1,  y,      z + 1,        block_type,
                            x,      y,      z + 1,        block_type,
                            x,      y,      z,            block_type]);
  }

    // View from positive y
  if chunk.get_block(x_in, y_in + 1, z_in).is_none() {
    build_vec = vec::append(build_vec,
                           [x,      y + 1,  z,            block_type,
                            x + 1,  y + 1,  z,            block_type,
                            x + 1,  y + 1,  z + 1,        block_type,
                            x + 1,  y + 1,  z + 1,        block_type,
                            x,      y + 1,  z + 1,        block_type,
                            x,      y + 1,  z,            block_type]);
  }

    // View from negative z
  if chunk.get_block(x_in, y_in, z_in - 1).is_none() {
    build_vec = vec::append(build_vec,
                           [x,      y,      z + 1,        block_type,
                            x,      y + 1,  z + 1,        block_type,
                            x + 1,  y,      z + 1,        block_type,
                            x + 1,  y,      z + 1,        block_type,
                            x,      y + 1,  z + 1,        block_type,
                            x + 1,  y + 1,  z + 1,        block_type]);
  }
    // View from positive z
  if chunk.get_block(x_in, y_in, z_in + 1).is_none() {
    build_vec = vec::append(build_vec,
                           [x,      y,      z,            block_type,
                            x + 1,  y,      z,            block_type,
                            x,      y + 1,  z,            block_type,
                            x + 1,  y,      z,            block_type,
                            x + 1,  y + 1,  z,            block_type,
                            x,      y + 1,  z,            block_type]);
  }
    build_vec
}
