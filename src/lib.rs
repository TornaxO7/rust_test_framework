#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![feature(restricted_std)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn test_runner(_test: &[&i32]) {
    loop {}
}
