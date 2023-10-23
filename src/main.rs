#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]

#[cfg(test)]
mod tests {

    use core::ops::Fn;

    pub fn test_runner(tests: &[&dyn Fn()]) {
        std::println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }
}

use panic_halt as _;
use rp_pico::entry;

mod clipper;
mod content;

#[entry]
fn main() -> ! {
    loop {}
}
