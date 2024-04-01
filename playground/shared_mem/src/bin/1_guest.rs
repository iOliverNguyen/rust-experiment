use shared_memory::ShmemConf;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Guest: read memory from host!");

    let shmem_id = env::args().nth(1).ok_or("missing shmem id")?;

    let shmem = ShmemConf::new().os_id(&shmem_id).open()?;
    println!("opened shmem id: {}", &shmem_id);

    let s = unsafe { shmem.as_slice() };
    for i in 0..100 {
        println!("read {}", s[i]);
    }

    Ok(())
}
