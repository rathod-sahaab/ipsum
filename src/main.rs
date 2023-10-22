#![no_std]
#![no_main]

use panic_halt as _;
use rp_pico::entry;

mod clipper;
mod content;

#[entry]
fn main() -> ! {
    loop {}
}
