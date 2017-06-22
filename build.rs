extern crate gcc;

fn main() {
    //gcc::Config::new().file("src/double.c ").compile("libdouble.a");
    gcc::compile_library("libfoo.a", &["src/c/run.c"]);
}
