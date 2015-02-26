//octree.rs

struct Atom{
	color: Color,
	normal: Normal,
	reflectivity: f32,
	refraction: f32,
	diffuse: f32
}

struct Octree{
    depth: u64,
    children: Vec<Octree>,
    leaf_data: Vec<Atom>,
}

struct VoxelOctree{
	root: Octree,
}

impl Octree{
	
	fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}
