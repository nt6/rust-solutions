#![cfg_attr(not(debug_assertions), no_main)]

extern crate pl;
use pl::io::{scanf, printf};

#[cfg(not(debug_assertions))]
#[no_mangle]
pub extern fn main() -> i32 {
    solution_main();
    0
}

#[cfg(debug_assertions)]
pub fn main() {
    solution_main();
}

fn solution_main() {
    let mut a : i32 = 0;
    let mut b : i32 = 0;

    unsafe {
        scanf(b"%d %d\0".as_ptr(), &a, &b);
        printf(b"%d\n\0".as_ptr(), a+b);
    }
}
