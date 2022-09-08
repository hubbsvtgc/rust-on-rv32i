#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
#[link_section = ".mtvec_base"]
 fn trap_handler() -> ! {
    loop{}
}

#[no_mangle]
pub fn _start() -> ! {

    extern "C" {
        static  _stack_start: u32;
    }

    type FnPtr = fn() -> !;
    let th: FnPtr = trap_handler;

    unsafe{

        let sp = &_stack_start;

        asm!("csrw mtvec, {}" ,
            in(reg) th);

        asm!("mv sp, {}" ,
            in(reg) sp);
    }

    loop {}
}