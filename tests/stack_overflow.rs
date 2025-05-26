#![no_std]
#![no_main]

use InvOS::serial_print;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    InvOS::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    InvOS::test_panic_handler(info)
}
