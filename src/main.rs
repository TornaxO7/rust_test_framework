#![no_std]
#![no_main]
#![feature(const_ptr_offset)]
#![reexport_test_harness_main = "test_main"]
#![feature(restricted_std)]

use os_clone2 as _;

#[no_mangle]
pub extern "C" fn os_clone2_entry() -> ! {

    #[cfg(test)]
    test_main();

    loop {}
}
