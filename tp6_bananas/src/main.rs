use nix::libc;
//use nix::sys::signal::{self, SigHandler, Signal};
use std::thread;
use std::time::Duration;
use nix::unistd::getpid;
use nix::sys::signal::kill;
use std::env;
use nix::unistd::Pid;
use nix::sys::signal::{self, SigHandler, Signal, SIGINT, SIGHUP, SIGQUIT};

fn main() {
    let args: Vec<String> = env::args().collect();
    let pid:i32 = str::parse(&args[1]).unwrap();
    //sighup = a
    //sigint = b
    //sigquit = n
    //sigtrap = s
    //sigfpe = space
    thread::sleep(Duration::from_millis(100));
    kill(Pid::from_raw(pid), SIGINT).unwrap(); //b
    thread::sleep(Duration::from_millis(100));
    kill(Pid::from_raw(pid), SIGHUP).unwrap(); //a
    thread::sleep(Duration::from_millis(100));
    kill(Pid::from_raw(pid), SIGQUIT).unwrap(); //n
    thread::sleep(Duration::from_millis(100));
    kill(Pid::from_raw(pid), SIGHUP).unwrap(); //a
    thread::sleep(Duration::from_millis(100));
    kill(Pid::from_raw(pid), SIGQUIT).unwrap(); //n
    thread::sleep(Duration::from_millis(100));
    kill(Pid::from_raw(pid), SIGHUP).unwrap(); //a
    thread::sleep(Duration::from_millis(100));
    
}
