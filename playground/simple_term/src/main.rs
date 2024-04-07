// https://poor.dev/blog/terminal-anatomy/

use core::panic;
use nix::pty::forkpty;
use nix::unistd::{read, write};
use std::{
    os::fd::{self, IntoRawFd},
    process::Command,
};

fn main() {
    let default_shell = std::env::var("SHELL").expect("no shell found");
    let stdout_fd = spawn_pty_with_shell(&default_shell);
    let mut read_buffer = vec![];
    loop {
        match read_from_fd(stdout_fd) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0);
            }
        }
    }
}

fn spawn_pty_with_shell(shell: &str) -> fd::RawFd {
    match unsafe { forkpty(None, None) } {
        Ok(fork_pty_res) => {
            let stdout_fd = fork_pty_res.master;
            if fork_pty_res.fork_result.is_child() {
                // slave
                Command::new(shell).spawn().expect("failed to spawn shell");
                std::thread::sleep(std::time::Duration::from_secs(1));
                std::process::exit(0);
            }
            stdout_fd.into_raw_fd()
        }
        Err(e) => {
            panic!("failed to fork_pty: {:?}", e);
        }
    }
}

fn read_from_fd(fd: fd::RawFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd, &mut read_buffer);
    match read_result {
        Ok(read_bytes) => {
            if read_bytes == 0 {
                None
            } else {
                Some(read_buffer[..read_bytes].to_vec())
            }
        }
        Err(_) => None,
    }
}
