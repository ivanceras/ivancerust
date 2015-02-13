struct Voxel{
    x:u64,
    y:u64,
    z:u64,
}

impl Voxel{

    fn new(x:u64,y:u64)->Voxel{
        let z = x+y;
        Voxel{x:x,y:y,z:z}
    }
    
}
fn main(){
    let mut v = Voxel::new(1,2);
    println!("z: {}",v.z);
}
