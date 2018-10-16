#![warn(dangling_ptr)]

#[allow(unused_variables)]
fn main() {
    let x = 1 as *const usize;
    let y = 1 as *mut f64;

    let z = 1;
    let z = z as *const usize; // this is currently not caught
}
