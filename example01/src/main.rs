use std::os::raw::c_int;

extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let k = unsafe { add(2, 4) };
    assert!(k == 6);
}
