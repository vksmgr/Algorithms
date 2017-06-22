
extern crate libc;

extern {
    fn run() -> libc::c_int;
}


fn main() {
    let call = unsafe {run()};
    println!("return call : {} ",call);
}
