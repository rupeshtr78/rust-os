#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod serial;
mod vga_buffer;

#[cfg(not(test))] // This is only for the normal build, not for tests.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// This function is called on panic in tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

/// First, we cast the integer 0xb8000 into a raw pointer.
/// Then we iterate over the bytes of the static HELLO byte string.
/// We use the enumerate method to additionally get a running variable i.
/// In the body of the for loop, we use the offset method to write the string byte and the corresponding color byte (0xb is a light cyan).
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         // Each character is represented by two bytes in VGA text mode:
    //         // one for the character and one for the attribute (color).
    //         let offset = i as isize; // 1-based index
    //         *vga_buffer.offset(offset * 2) = byte; // Character byte
    //         *vga_buffer.offset(offset * 2 + 1) = 0xb; // Attribute byte (Light cyan)
    //     }
    // }

    // vga_buffer::write_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER
    //     .lock()
    //     .write_str("Hello again")
    //     .unwrap_err();

    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap_err();

    // panic!("Some panic message");
    println!("Hello World{}", "!");
    println!("Hello Again {}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x010,
    Failed = 0x11,
}

/// The function creates a new Port at 0xf4, which is the iobase of the isa-debug-exit device.
/// Then it writes the passed exit code to the port.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion test ...");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

#[test_case]
fn test_serial_macro_fail() {
    serial_println!("Test: {}", 42); // Should print "Test: 42"
    assert_eq!(1, 1);
}
