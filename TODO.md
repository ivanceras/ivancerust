//TODO:

~~implement a screen ray vectors aligned with the voxel in subject orientation~~

cargofy the project

Save voxelized voxel octree into a file to save computation time of on the spot objects.

Add colors and normals to the voxel array


Use quarternions on camera rotations

1. Indexes are not soo efficient
    We can replace it make a sorted indexes and lookups.
    
2. Find the relationship between indexes and the bits at each LOD, these can tell if the voxel is occupied or not


Arrays of 8bits or arrays of 64bits.
Each voxel can be contained either by 64bits or 8bits. A long list of 8bits or 1/8th less of 64bits.
