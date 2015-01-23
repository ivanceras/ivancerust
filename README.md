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
    file:i64,
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

