use pic8259::ChainedPics;
use x86_64::{
    instructions::port::{self, Port},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{gdt, print, println};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    keyboard,
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {   // lazy_static! is a macro that allows you to initialize a static variable lazily (i.e., the first time it’s accessed).
    static ref IDT: InterruptDescriptorTable = {    // values computed at runtime
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // The timer interrupt handler is set up to handle timer interrupts.
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

        // The keyboard interrupt handler is set up to handle keyboard interrupts.
        idt[InterruptIndex::keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);


        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(">");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    // print!("{}", scancode);

    // KeyEvent contains the key which caused the event and whether it was a press or release event.
    // To interpret this key event, we pass it to the process_keyevent method, which translates the key event to a character,
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::keyboard.as_u8());
    }
}

// The CPU tries to write to 0xdeadbeef, which causes a page fault.
// The CPU looks at the corresponding entry in the IDT and sees that no handler function is specified.
// Thus, it can’t call the page fault handler and a double fault occurs.
// The CPU looks at the IDT entry of the double fault handler, but this entry does not specify a handler function either.
// Thus, a triple fault occurs.
// A triple fault is fatal. QEMU reacts to it like most real hardware and issues a system reset.
//
// The CPU tries to write to 0xdeadbeef, which causes a page fault.
// Like before, the CPU looks at the corresponding entry in the IDT and sees that no handler function is defined.
// Thus, a double fault occurs.
// The CPU jumps to the – now present – below double fault handler.
// The triple fault (and the boot-loop) no longer occurs, since the CPU can now call the double fault handler.
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
