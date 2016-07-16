#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate pl;

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let mut a : i32 = 0;
    let mut b : i32 = 0;

    scanf!("%d %d", &mut a, &mut b);
    printf!("%d\n", a+b);
}
