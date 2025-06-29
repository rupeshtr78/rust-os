#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_v1::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_v1::println;

#[cfg(not(test))] // This is only for the normal build, not for tests.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os_v1::hlt_loop();
}

// This function is called on panic in tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_v1::test_panic_handler(info)
}

// static HELLO: &[u8] = b"Hello World!";

/// First, we cast the integer 0xb8000 into a raw pointer.
/// Then we iterate over the bytes of the static HELLO byte string.
/// We use the enumerate method to additionally get a running variable i.
/// In the body of the for loop, we use the offset method to write the string byte and the corresponding color byte (0xb is a light cyan).
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!(
        r#"
        ____  _   _ ____ _____   ___  ____  
        |  _ \| | | / ___|_   _| / _ \/ ___| 
        | |_) | | | \___ \ | |  | | | \___ \ 
        |  _ <| |_| |___) || |  | |_| |___) |
        |_| \_\\___/|____/ |_|   \___/|____/ 
                    V1
        "#
    );
    println!("Starting Kernal{}", "!");

    rust_os_v1::init();

    // invoke breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // // trigger a page fault
    // unsafe {
    //     let ptr = 0xdeadbeef as *mut u8; // Invalid address
    //     *ptr = 42; // This will cause a page fault
    // }

    println!("Hello Again {}", "!");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    // loop {}
    rust_os_v1::hlt_loop();
}
