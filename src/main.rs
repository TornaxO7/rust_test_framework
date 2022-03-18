#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]


use os_clone2 as _;
use os_clone2::test_runner;

#[no_mangle]
pub extern "C" fn os_clone2_entry() -> ! {

    #[cfg(test)]
    test_main();

    loop {}
}
