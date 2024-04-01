use shared_memory::{Shmem, ShmemConf};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Host: share memory with guest!");

    let mut shmem = ShmemConf::new().size(1024).create()?;
    println!("created shmem id: {}", shmem.get_os_id());

    shmem.set_owner(true);

    let s = unsafe { shmem.as_slice_mut() };
    for i in 0..s.len() {
        s[i] = i as u8;
    }

    println!("sleep 10 minutes to wait for guest to read memory");
    std::thread::sleep(std::time::Duration::from_secs(600));

    Ok(())
}
