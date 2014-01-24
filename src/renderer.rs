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
      outColor = vec4(Color, 1.0); \n\
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
    gl::VertexAttribPointer(pos_attr as u32, 3, gl::FLOAT, gl::FALSE,
                            (6 * mem::size_of::<GLfloat>()) as i32, ptr::null());

    let col_attr = "color".with_c_str(|ptr| gl::GetAttribLocation(renderer.program, ptr));
    gl::EnableVertexAttribArray(col_attr as GLuint);
    gl::VertexAttribPointer(col_attr as u32, 3, gl::FLOAT, gl::FALSE,
                            (6*mem::size_of::<GLfloat>()) as GLsizei,
                            cast::transmute(3*mem::size_of::<GLfloat>() as uint));

    //glVertexAttribPointer(colAttrib, 3, GL_FLOAT, GL_FALSE, 8 * sizeof(GLfloat), (void*)(3 * sizeof(GLfloat)));

    }
    renderer
  }

  pub fn update(&self, chunk: &mut Chunk) {
    println!("{}",chunk.update);
    if chunk.update {
      let mut block_vertexes: ~[GLfloat] = ~[];
      chunk.reset_update();

    // loop over blocks in the chunk
      for z in range(0, chunk.len()) {
        for x in range(0, chunk.len()) {
          for y in range(0, chunk.len()) {
            match chunk.get_block(x, y, z) {
              Some(block) =>  {
                //TODO append instead vec::append(block_vertexes, gen_vertex(x,y,z));
                let mut rng = rand::task_rng();
                let r: f32 = rng.gen_range(0.0 as f32, 1.0 as f32);
                let g: f32 = rng.gen_range(0.0 as f32, 1.0 as f32);
                let b: f32 = rng.gen_range(0.0 as f32, 1.0 as f32);

                for i in gen_vertex(x, y, z, r, g, b).iter(){
                block_vertexes.push(*i);
                }
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                unsafe{
                  gl::BufferData(gl::ARRAY_BUFFER,
                  (block_vertexes.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                  cast::transmute(&block_vertexes[0]), gl::STATIC_DRAW);
                }
              }
              None => println!("no block to render at ({},{},{})", x, y, z)
            }

          }
        }
      }


      unsafe {
        let view:Mat4<f32> = Mat4::look_at(&Point3::new(15.0 as f32, 15.0, 15.0),
        &Point3::new(0.0 as f32, 0.0, 0.0),
        &Vec3::new(0.0 as f32, 0.0, 1.0));
        let uni_view = "view".with_c_str(|ptr| gl::GetUniformLocation(self.program, ptr));
        gl::UniformMatrix4fv(uni_view , 1, gl::FALSE, view.ptr());

        let proj = projection::perspective(deg(45.0 as f32), 800.0/600.0, 1.0, 50.0);
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


fn gen_vertex(x: int, y: int, z: int, r: f32, g: f32, b: f32) -> [GLfloat, ..216] {
  let x = x as f32;
  let y = y as f32;
  let z = z as f32;
  let x: [GLfloat, ..216] = [
    x + -0.5, y +-0.5, z + -0.5, r, g, b,
    x + 0.5, y + -0.5, z + -0.5, r, g, b,
    x + 0.5, y +  0.5, z + -0.5, r, g, b,
    x + 0.5, y +  0.5, z + -0.5, r, g, b,
    x + -0.5, y + 0.5, z + -0.5, r, g, b,
    x + -0.5, y +-0.5, z + -0.5, r, g, b,

    x + -0.5, y +-0.5, z +  0.5, r, g, b,
    x +  0.5, y +-0.5, z +  0.5, r, g, b,
    x +  0.5, y + 0.5, z +  0.5, r, g, b,
    x +  0.5, y + 0.5, z +  0.5, r, g, b,
    x + -0.5, y + 0.5, z +  0.5, r, g, b,
    x + -0.5, y +-0.5, z +  0.5, r, g, b,

    x + -0.5, y + 0.5, z +  0.5, r, g, b,
    x + -0.5, y + 0.5, z + -0.5, r, g, b,
    x + -0.5, y +-0.5, z + -0.5, r, g, b,
    x + -0.5, y +-0.5, z + -0.5, r, g, b,
    x + -0.5, y +-0.5, z +  0.5, r, g, b,
    x + -0.5, y + 0.5, z +  0.5, r, g, b,

    x +  0.5, y + 0.5, z +  0.5, r, g, b,
    x +  0.5, y + 0.5, z + -0.5, r, g, b,
    x +  0.5, y +-0.5, z + -0.5, r, g, b,
    x +  0.5, y +-0.5, z + -0.5, r, g, b,
    x +  0.5, y +-0.5, z +  0.5, r, g, b,
    x +  0.5, y + 0.5, z +  0.5, r, g, b,

    x + -0.5, y +-0.5, z + -0.5, r, g, b,
    x +  0.5, y +-0.5, z + -0.5, r, g, b,
    x +  0.5, y +-0.5, z +  0.5, r, g, b,
    x +  0.5, y +-0.5, z +  0.5, r, g, b,
    x + -0.5, y +-0.5, z +  0.5, r, g, b,
    x + -0.5, y +-0.5, z + -0.5, r, g, b,

    x + -0.5, y + 0.5, z + -0.5, r, g, b,
    x +  0.5, y + 0.5, z + -0.5, r, g, b,
    x +  0.5, y + 0.5, z +  0.5, r, g, b,
    x +  0.5, y + 0.5, z +  0.5, r, g, b,
    x + -0.5, y + 0.5, z +  0.5, r, g, b,
    x + -0.5, y + 0.5, z + -0.5, r, g, b];
  x
}
