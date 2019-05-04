#[macro_use]
extern crate proc_upload;

#[derive(LS, Debug)]
struct x;

fn main() {
    let x = x;
    println!("Hello, world! {:?}", x);
}
