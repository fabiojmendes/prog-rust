use std::{env, process};

#[inline(never)]
fn test_func(i: i32) {
    if i > 1 {
        process::exit(i);
    } else {
        process::exit(0);
    }
}

fn main() {
    let size = env::args().count();
    test_func(size as i32);
}
