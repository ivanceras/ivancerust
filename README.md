#Ivancerust
Me(ivanceras) playing on rust code, so IVANCERUST!!! :D

Terrible, i know, we'll get a better name later.


Ivancerust is a 3D rendering voxel using raytracing.

TLDR; instead of polygons, I use voxels. 

Instead of rasterizing I am raytracing the voxels.

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

Sphere at LOD 6 = (2^6)^3 = 64x64x64 = 262144 voxel grid

![Sphere LOD 6](https://raw.githubusercontent.com/ivanceras/ivancerust/0.0.3/images/sphere6-trace6.png)



Sphere at LOD 5 = (2^5)^3 = 32x32x32 = 32768 voxel grid
![Sphere LOD 5](https://raw.githubusercontent.com/ivanceras/ivancerust/0.0.3/images/sphere6-trace5.png)



Sphere at LOD 4 = (2^4)^3 = 16x16x16 = 4096 voxel grid
![Sphere LOD4](https://raw.githubusercontent.com/ivanceras/ivancerust/0.0.3/images/sphere5-trace4.png)


Sphere at LOD 3 = (2^3)^3 = 8x8x8 = 512 voxel grid

![Sphere LOD3](https://raw.githubusercontent.com/ivanceras/ivancerust/0.0.3/images/sphere5-trace3.png)


##Render voxel format
Using binvox format as famous in minecraft
the binvox are stored in `./data` directory

```
cargo run --example render_lucy
```
Lucy at LOD 8 = (2^8)^3 = 16777216 voxel grid
![Lucy](https://raw.githubusercontent.com/ivanceras/ivancerust/0.0.3/images/lucy8-trace8.png)



##Render bunny

```
cargo run --example render_bunny
```
Bunny at LOD 7 = (2^7)^3 = 2097152 voxel grid

![Bunny](https://raw.githubusercontent.com/ivanceras/ivancerust/0.0.3/images/bunny7-trace7.png)

The images are in ppm format saved in `./renders/` directory


##Roadmap

1. Make it realtime
2. Add physics
3. Create a game using the engine
4. ????
5. Profit. 



