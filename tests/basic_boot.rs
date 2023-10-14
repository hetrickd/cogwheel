#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cogwheel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cogwheel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

pub fn test_runner(_tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cogwheel::test_panic_handler(info)
}


#[test_case]
fn test_println() {
    println!("test_println output");
}