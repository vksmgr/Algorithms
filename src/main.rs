
extern crate libc;

mod rust_src;
use rust_src::searching;

///this will include c functions
extern {
    fn run() -> libc::c_int;
}

fn main() {
    let call = unsafe {run()};
    println!("return call : {} ",call);
    searching::print_me();
}
