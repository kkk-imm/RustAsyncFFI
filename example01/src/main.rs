use std::os::raw::c_int;

#[link(name="callee")]
extern "C" {
    fn sum(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let k = unsafe { sum(2, 4) };
    println!("sum result is {}",k);
}
