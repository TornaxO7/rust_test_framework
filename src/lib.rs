#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn test_runner(_test: &[&dyn Fn()]) {
    loop {}
}
