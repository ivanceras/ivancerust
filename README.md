# ivancerust
Unlimited detail voxel engine in rust


Voxel renderer




##LOD

LOD is relative to Resolution of the screen or image
if image required to be produce is 1920x1080, then the LOD directly in front of projection plane needs to have 1920x1920x1920 voxel maximum capacity(1920 is used since it is max of (1920,1080) to obtain the best possible detail).
This is equivalent to 7_077_888_000 which 1 < 33 = 8_589_934_592 maximum voxels. That is the parent voxel at level 1, 8 octree is divided to half 11 times.



This is the LOD of octree when focal distance (eye distance from projection plane) to 2 times the projection plane.

```
level  0 1<<0                1 voxel       1 voxel  wide
level  1 1<<3                8 voxels      2 voxels wide
level  2 1<<6               64 voxels      4 voxels wide
level  3 1<<9              512 voxels      8 voxels wide
level  4 1<<12           4_096 voxels     16 voxels wide
level  5 1<<15          32_768 voxels     32 voxels wide
level  6 1<<18         262_144 voxels     64 voxels wide
level  7 1<<21       2_097_152 voxels    128 voxels wide
level  8 1<<24      16_777_216 voxels    256 voxels wide
level  9 1<<27     134_217_728 voxels    512 voxels wide
level 10 1<<30   1_073_741_824 voxels   1024 voxels wide
level 11 1<<33   8_589_934_592 voxels   2048 voxels wide
```





voxels wide at the side

fd = focal distance from the voxel to the eye ( camera)

** 1fd is the voxels that are within the eye to the length where 2fd starts

```
1fd = 1920^3 = 7_077_888_000 <  level 11 1<<33   8_589_934_592 voxels   2048 voxels wide
2fd =  960^3 =   884_736_000 <  level 10 1<<30   1_073_741_824 voxels   1024 voxels wide
3fd =  480^3 =   110_592_000 <  level  9 1<<27     134_217_728 voxels    512 voxels wide
4fd =  240^3 =    138_24_000 <  level  8 1<<24      16_777_216 voxels    256 voxels wide
5fb =  120^3 =     1_728_000 <  level  7 1<<21       2_097_152 voxels    128 voxels wide
6fd =   60^3 =       216_000 <  level  6 1<<18         262_144 voxels     64 voxels wide
7fd =   30^3 =        27_000 <  level  5 1<<15          32_768 voxels     32 voxels wide
8fd =   15^3 =         2_197 <  level  4 1<<12           4_096 voxels     16 voxels wide
9fd =  7.5^3 =       421.875 <  level  3 1<<9              512 voxels      8 voxels wide
10fd= 3.75^3 =     52.734375 <  level  2 1<<6               64 voxels      4 voxels wide
11fd=1.875^3 =   6.591796875 <  level  1 1<<3                8 voxels      2 voxels wide
12fd=0.9375^3=   0.823974609 <  level  0 1<<0                1 voxel       1 voxel  wide
```

in 90 degree FOV
1fb = 960 pixels away, if 1 pixel = 1 voxel size then that is 960 voxels at level 11 aligned to the back of the plane

```
level 11 voxels:

 1fb =    960
 2fb =  1_920
 3fb =  2_880
 4fb =  3_840
 5fb =  4_800
 6fb =  5_760
 7fd =  6_720
 8fb =  7_680
 9fd =  8_640
10fd =  9_600
11fd = 10_560
```


primary rays = 1920x1080 = 2_073_600 rays, 1 per pixel --using simple raytracing with 1 ray per pixel

```
if depth_of_field and blurring_included {
    2_073_600 * 64 = 132_710_400 rays to be traced
}
```

Could have used the diagonal of the screen (1980^2+1080^2).sqrt().
This would lead to 
10_691_619_427 max voxels on the near plane


##Storing the data efficiently.

Storing voxel data can be disk consuming, but if we do it right, we can get an impressive way of storing it.

Files

* .vox  - contains the voxel bit data
* .leaf - determines whether the voxel is a leaf or not
* .file - the voxel data is not a leaf and it resides in a new file

```

struct Voxel{
    voxel:Vector[u8],
    leaf:bool,
    file:i32,
}

```

Mostly CPU's a re 64 bit.

Computation of operands should only be using 32 bits in order to get away with overflows in integer computation.

```
2^32 bits = 1 << 33 = 8_589_934_592 = 2048 * 2048 * 2048
```


the largest file will be 8 gig 8_589_934_592 voxels

* .vox file contains a series of `0`'s and `1`'s, which just tell if the voxel is empty or not.
1 voxel is 1 bit and can be inspected more in detail by looking at the octree. Octree is 8 bits, and each of these bits is also a voxel or octant. The relative distance of each octant to the center of the voxel is determine by its position on the bits.


1 octree has 8 elements, each can be accessed with index 0-7
```
at bit[0]:  {-1.0, -1.0, -1.0}

at bit[1]:  { 1.0, -1.0, -1.0}

at bit[2]:  {-1.0,  1.0, -1.0}

at bit[3]:  { 1.0,  1.0, -1.0}

at bit[4]:  {-1.0, -1.0,  1.0}

at bit[5]:  { 1.0, -1.0,  1.0}

at bit[6]:  {-1.0,  1.0,  1.0}

at bit[7]:  { 1.0,  1.0,  1.0}

```

##Root voxel
The root voxel is most engine is the world.
Let's make our root voxel the universe. This is the largest possible world your game could be.
Let's assume that the universe is divided into 1 << 33 parts, so you will have 0's in most of the areas. 1 only in the place where you are concerned. say at 2048, 2048, 2048 LOD, the planet earth resides. This will be the basis of your computation.

.vox

```
byte[0] : 0000 0001
byte[1] : 0000 0001
byte[2] : 0000 0001
byte[3] : 0000 0001

byte[4] : 0000 0001
byte[5] : 0000 0001
byte[6] : 0000 0001
byte[7] : 0000 0001

byte[8] : 0000 0001
byte[9] : 0000 0001
byte[10] : 0000 0001


```
.leaf
```
0000 0000
0000 0000
0000 0000
0000 0000

0000 0000
0000 0000
0000 0000
0000 0000

0000 0000
0000 0000
0000 0000
0000 0001

```


.file
```

```


Seeking of more details of voxels will be terminated by:
1. If a leaf is encountered - meaning no more details at this level
2. It is not a leaf, but the LOD is high enough that seeking more details will no longer affect the rendering.

This means that on the root first byte that the only area of the voxel resides on the first octant.

Voxel file will be stored on disk as:

Ideal:
```
[8 bits][2 bits][32 bits]
```
first 8 bits is the octree
next 2 bits is determines whether a leaf node or branch, and if more detail is in a file numbered in the 32 bits.
1 voxel(8 octants) is defined by 10 bits on average. Some will be 42 bits if there is more detail on that voxel stored in another file. The filename is the number, this allows us to partition data into another file, while not having to read the necessary sequence of data to determine the voxel data at a certain location.


Efficient Calculations:
The data is structured in an octree, traversal of data will be as efficient as octree data structure.
In order for the alogrithm to be efficient as possible, there should be a minimal use of complex calculation.
Finding for the voxels at a certain camera possition is no more than comparing voxels.
The ray, at each certain LOD(level of detail) falls on a certain voxel. Determining the right voxel at the right LOD is comparing the ray in voxel representation at certain direction with LOD will be compared to the array of voxels to be viewed.



Extraction of voxels from the camera position is by extracting the all the voxels at certain sphere level where in the the LOD is fading when it reaches far away from the camera location.

When using a slimmer FOV (i.e lenses, telescope), the LOD will fade very slowly at the direction where the camera is facing (it's frustum view).

The extraction of voxel is spherical outward, more detail near the center, while loosing detail as it expands farther.

At certain point in the sphere where a voxel is found at a satisfied LOD, the search on that point is terminated, and will be used for the calculation of the final image.


Spherical outward search is terminated at 2 conditions:
* A voxel is found at a satisfied/required LOD.
* No voxel is found until it reached the whole voxel tree, including the root tree, the universe.



##LOD Calculation
LOD depends on your image size (e.g 1920x1080 ) , FOV angle and the zoom.
LOD is directly proportional to ZOOM, the higher the zoom, the higher the LOD.
At distance 0 from the projection plane, the LOD should be no more than 1 pixel wide, unless it is the leaf voxel.


##Location
Where is the camera location from the voxel tree.
As opposed to common 3D engines, the camera space will be relative to the root voxel.
So, the camera location will also be expressed in terms of voxel location.
There is only 1 (1)'s in every node of the octree, since the camera has only 1 location
as oppose to voxels where is occupies multiple areas of the voxel.

```
struct Camera{
   location:Vec[u8];
}
```
```
camera = vec![
 000000001,
 000000001,
 000000001,
 000000001,
 
 000000001,
 000000001,
 000000001,
 000000001,

...
]
```

Once the camera location is specified, the voxel octree will be traversed in accordance with the camera. If the traversal encounters 0, it means there is nothing there.
The extraction of points starts at the octree where the value is 1, outwards. The LOD will be determine by LOD difference of the camera and the encountered octree, and is affected by the distance and zoom. 

new structure of the Voxel

```
struct Voxel{
    voxel:Vector[u8],
    leaf:bool,
    detail: Voxel,
}
```

![Figure traversal](http://ivanceras.github.io/ivancerust/traversal.svg)
