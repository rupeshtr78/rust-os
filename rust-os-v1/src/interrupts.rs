use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, println};
use lazy_static::lazy_static;

lazy_static! {   // lazy_static! is a macro that allows you to initialize a static variable lazily (i.e., the first time it’s accessed).
    static ref IDT: InterruptDescriptorTable = {    // values computed at runtime
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
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
