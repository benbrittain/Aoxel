// world.rs
// manage loading chunks

extern mod noise;
extern mod gl;

use chunk::*;
use renderer::*;

use gl::*;
use gl::types::*;
use noise::Perlin;
use chunk::Block;

use std::hashmap::*;
use std::rand::*;
use std::rand;

pub struct World {
  chunks: HashMap<(int,int,int), Chunk>,
  renderer: Renderer
}

impl World {
  pub fn new() -> World {
    let mut x = World {
      chunks: HashMap::new(),
      renderer: Renderer::new()
    };
//    x.init_chunks();
    x.init_terrain();
    x
  }

  fn init_chunks(&mut self) -> () {
    // for now just init 3 x 3 x 1 chunks (x,y,z)

    for x in range(0, 5) {
      for y in range(0, 5) {
        self.chunks.find_or_insert((x,y,0), Chunk::new_with_random(x, y, 0));
      }
    }
  }

//  fn get_chunk(&self, x: int, y: int, z: int) -> Option<&Chunk> {
//    if (self.chunks.len() <= 0) {
//      return None;
//    }
//    Some(self.chunks.get(&(x, y, z)))
//  }

//  fn is_loaded(&self, x: int, y: int, z: int) -> bool {
//    match self.get_chunk(x, y, z) {
//      Some(_) => true,
//      None => false
//    }
//  }

  fn set(&mut self, x: int, y: int, z: int, t: int) -> () {
    let block_coord = (x % Chunk::size(), y % Chunk::size(), z % Chunk::size());
    let chunk_coord = (x / Chunk::size(), y / Chunk::size(), z / Chunk::size());
    let mut rng = rand::task_rng();

    match chunk_coord {
      (x, y, z) =>  {
        match self.chunks.find(&(x,y,z)) {
          None => { self.chunks.find_or_insert((x,y,z), Chunk::new(x, y, z)); }
          _ => ()
        }
        match block_coord {
          (i, j, k) =>  {
            let mut ch = self.chunks.get_mut(&(x,y,z));
            let block = match rng.gen_range(1,4) {
              0 => Space,
              1 => Generic,
              2 => Generic2,
              3 => Generic3,
              _ => Space
            };

            ch.set_block(block, i, j, k);
          }
        }
      }
    }
  }

  // 'cause I have no clue
  fn init_terrain(&mut self) -> () {
    let mut rng = rand::task_rng();
    let perlin = Perlin::from_seed_str(rng.gen_ascii_str(5));
    let mut build_vec: ~[Chunk] = ~[];
    let mut world: [[int, ..50], ..50] = [[0, ..50], ..50];
    for y in range(0, 50) {
      for x in range(0, 50) {
        let mut z = perlin.gen([ x as f32 * 0.1 ,
                             y as f32 * 0.1 ]);
        if z < 0.0 { z = 0.0;}

        world[y][x] = (z*100.0) as int;
      }
    }
          let mut rng = rand::task_rng();
    // make into 16x16x16 chunks
    for x in range(0, 50) {
      for y in range(0, 50) {
        for z in range(0, world[x][y]) {
          self.set(x, y, z, 1);
        }
      }
    }
  }

  pub fn render(&mut self) -> () {
    gl::ClearColor(0.1, 0.1, 0.1, 0.1);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    // TODO parallelize w/ tasks
    for x in range(0 as int, self.chunks.len() as int) {
      for y in range(0 as int, self.chunks.len() as int) {
        for z in range(0 as int, self.chunks.len() as int) {
          match self.chunks.find_mut(&(x,y,z)) {
            None => (),
            Some(chunk) => { self.renderer.update(chunk);
              chunk.reset_update();
            }
          }
        }
      }
    }
  }
}
