// use nix::unistd::{fork, ForkResult, getpid, getppid, execvp, pipe, close};
// use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
// use std::{thread, time};
// use std::ffi::CString;
// use nix::unistd::dup2;
// use std::os::unix::io::AsRawFd;
// use std::io;
// use std::fs::File;
// use std::os::unix::io::FromRawFd;
// use std::io::{Read, Write};

// fn main(){
//     let (fd1, fd2) = pipe().unwrap(); //make another pipe
//     let mut file1 = unsafe{File::from_raw_fd(fd1)};
//     let mut file2 = unsafe{File::from_raw_fd(fd2)};
//     match unsafe{fork()} {
//         Ok(ForkResult::Parent { child }) => {
//             //close(fd1);
//             let mut buffer = String::new();
//             let stdin = io::stdin();
//             stdin.read_line(&mut buffer).unwrap();
//             file2.write(buffer.as_bytes());
//             close(fd2);
//         }
//         Ok(ForkResult::Child) => {
//             //close(fd2);
//             let mut buf = String::new();
//             file1.read_to_string(&mut buf).unwrap();
//             let mut buf_rev = buf.chars().rev().collect::<String>();
//             println!("buffer {}", buf);
//             file1.write(buf_rev.as_bytes());
//             close(fd1);
//             std::process::exit(1);
//         }
//         Err(_) => println!("Fork failed"),
//      }
// }
use nix::libc;
use nix::sys::signal::{self, SigHandler, Signal};
use std::thread;
use std::time::Duration;
use nix::unistd::getpid;

extern "C" fn handle_signal(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    if signal.as_str().to_string() == "SIGHUP"{
        println!("a");
    } else if signal.as_str().to_string() == "SIGINT"{
        println!("b");
    } else if signal.as_str().to_string() == "SIGQUIT"{
        println!("n");
    } else if signal.as_str().to_string() == "SIGTRAP"{
        println!("s");
    } else if signal.as_str().to_string() == "SIGFPE"{
        println!(" ");
    }
    print!("");
    //println!("received signal {}: {}", signal, signal.as_str());
}

fn main() {
    println!("my pid is: {}", getpid());
    let signals = [
        Signal::SIGHUP,
        Signal::SIGINT,
        Signal::SIGQUIT,
        Signal::SIGTRAP,
        Signal::SIGFPE,
    ];
    let handler = SigHandler::Handler(handle_signal);
    for signal in signals {
        unsafe { signal::signal(signal, handler) }.unwrap();
    }
    loop {
        thread::sleep(Duration::from_millis(10));
    }
}