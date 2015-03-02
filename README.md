#Ivancerust

Me(ivanceras) playing on rust code, so IVANCERUST!!! :D
Terrible, i know, we'll get a better name later.

Ivancerust is a 3D rendering voxel using raytracing.

TLDR; instead of polygons, I use voxels. Instead of rasterizing I am raytracing the voxels.
And it is written in rust.


##Features:
	LOD - level of detail, voxels can be rendered at different Level of resolution.


##How fast? 
	~1 frame per minute (it's something).
	Why? More or less, this is a proof of concept. I've yet to study OpenCL yet to transcode that parts where needs heavy use of parallelization. Current code is utilizing only the CPU's though it uses threads to maximize the performance, it is not enough.


##Decipline and Principles:
	* Use simple algorithms that doesn't require too much clock cycles (bitwise operations)


##Render a shapes

```
cargo run --example render_shapes

```
This example build a voxelized sphere at a certain LOD, then rendered at lesser LOD




##Render voxel format
Using binvox format as famous in minecraft
the binvox are stored in `./data` directory

```
cargo run --example render_lucy
```



##Render bunny

```
cargo run --example render_bunny
```


The images are in ppm format saved in `./renders/` directory


##Roadmap

1. Make it realtime
2. Add physics
3. Create a game using the engine
4. ????
5. Profit. 



