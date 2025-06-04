#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_v1::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_v1::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

// fn test_runner(tests: &[&dyn Fn()]) {
//     unimplemented!();
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_v1::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    // serial_print!("Trivial assertion test ...");
    assert_eq!(1, 1);
    // serial_println!("[ok]");
}

#[test_case]
fn test_serial_macro_fail() {
    // serial_println!("Test: {}", 42); // Should print "Test: 42"
    assert_eq!(1, 1);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
