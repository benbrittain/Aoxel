// octree.rs
// An octree implementation
// for things like storing chunks
//

// TODO remove, unnecessary
#[feature(globs)]
use chunk::Chunk;

mod chunk;

enum Tree<T> {
  Nil,
  Node(~Octree<T>)
}

//pub enum Node<T> {
//  Node(T),
//  Nil
//}

pub struct Octree<T> {
  // Root Chunk Coordinates
  coord: (int, int, int),

  // length/width/height of chunk
  size: int,

  // length/width/height of chunk
  children: [Tree<Octree<Chunk>>, ..8]
}

pub impl Octree<Chunk> {
  pub fn new(data: Chunk) -> Octree<Chunk> {
    Octree {
      coord: (0,0,0),
      size: 16,
      children: [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
      data: data
    }
  }
  pub fn insert(&mut self) -> () {
    if self.is_leaf() {
      for child in self.children {
      //  child = self.new(origin, 
      }

//                                        for(int i=0; i<8; ++i) {
//                                                // Compute new bounding box for this child
//                                                Vec3 newOrigin = origin;
//                                                newOrigin.x += halfDimension.x * (i&4 ? .5f : -.5f);
//                                                newOrigin.y += halfDimension.y * (i&2 ? .5f : -.5f);
//                                                newOrigin.z += halfDimension.z * (i&1 ? .5f : -.5f);
//                                                children[i] = new Octree(newOrigin, halfDimension*.5f);
//                                        }
//
//

    }
  }

}

