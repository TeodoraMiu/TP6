use nix::unistd::{fork, ForkResult, getpid, getppid, execvp, pipe, close};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use std::{thread, time};
use std::ffi::CString;
use nix::unistd::dup2;
use std::os::unix::io::AsRawFd;
use std::io;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::io::{Read, Write};

fn main(){
    let (fd1, fd2) = pipe().unwrap(); //make another pipe
    let (fd3, fd4) = pipe().unwrap();
    let mut file1 = unsafe{File::from_raw_fd(fd1)};
    let mut file2 = unsafe{File::from_raw_fd(fd2)};
    let mut file3 = unsafe{File::from_raw_fd(fd3)};
    let mut file4 = unsafe{File::from_raw_fd(fd4)};
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child }) => {
            close(fd1);
            close(fd4);
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).unwrap();
            file2.write(buffer.as_bytes());
            close(fd2);
            let mut buf = String::new();
            file3.read_to_string(&mut buf).unwrap();
            println!("buffer {}", buf);
            close(fd3);
        }
        Ok(ForkResult::Child) => {
            close(fd2);
            close(fd3);
            let mut buf = String::new();
            file1.read_to_string(&mut buf).unwrap();
            println!("buffer {}", buf);
            let mut buf_rev = buf.chars().rev().collect::<String>();
            file4.write(buf_rev.as_bytes());
            close(fd1);
            close(fd4);
            std::process::exit(1);
        }
        Err(_) => println!("Fork failed"),
     }
}