#![cfg_attr(not(debug_assertions), no_main)]

#[macro_use]
extern crate pl;

use pl::iter::interleave;
use pl::collections::{Array, Stack, vectors};

#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let mut n : usize = 0;

    scanf!("%u", &mut n);

    let v = &mut vectors::Bounded::<usize>::new(n);

    for _ in 0..n {
        let mut a : usize = 0;
        scanf!("%u", &mut a);
        v.push(a);
    }

    for e in interleave(v.iter().rev().by_ref(), &(||->(){ printf!(" ");})) {
        printf!("%u", *e);
    }

    printf!("\n");
}
