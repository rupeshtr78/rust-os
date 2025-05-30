#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

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

    vga_buffer::write_something();

    loop {}
}
