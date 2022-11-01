#![no_std]
#![no_main]

extern crate hifive1_revb_board;
use core::panic::PanicInfo;
use core::arch::asm;

const BLUE_LED_GPIO: u8 = 21;
const PLIC_BASE: u32 = 0xC000000; // 0xC00 << 16 = 0xC00 0000,  
const PLIC_CLAIMCOMP_CTX1_OFFSET: u32 = 0x200004;
const MIE_SET: u32 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]

 fn trap_handler() {

    unsafe{
        asm!("mv t0, {}", in(reg) PLIC_BASE);
        asm!("mv t1, {}", in(reg) PLIC_CLAIMCOMP_CTX1_OFFSET);
        asm!("add t0, t1, t0");
        asm!("lw t1, 0(t0)");
        asm!("sw t1, 0(t0)");
        asm!("mret");
    }

/*
    li t0, PLIC_BASE;
li t1, PLIC_CLAIMCOMP_CTX1_OFFSET;
add t0, t1, t0;

lw t1, 0(t0);
sw t1, 0(t0);
slli t1, t1, 2;
*/

    //loop{}
}

fn set_stack(){
    extern "C" {
        static  _stack_start: u32;
    }
    unsafe{
        let sp1 = &_stack_start;
        asm!("mv sp, {}", in(reg) sp1);
    }
}

fn set_trap_handler()
{
    type FnPtr = fn();
    let th: FnPtr = trap_handler;
    unsafe{
        asm!("csrw mtvec, {}", in(reg) th);
    }
}

fn clear_external_interrupt()
{
    /* for now disable external interrupt to avoid mtvec */
    unsafe {
        asm!("csrr t0, mie");
        asm!("mv t1, {}", in(reg) MIE_SET);
        //asm!("or t0, t0, t1"); - this enables the external interrupt
        asm!("csrw mie, t1");
    }
}

#[no_mangle]

#[link_section = ".entry"]
pub extern "C" fn _start() -> ! {

    set_stack();
    set_trap_handler();
    clear_external_interrupt();

    hifive1_revb_board::Pin::set_as_out(BLUE_LED_GPIO);

    loop {

        // set high 1 - already pulleup so blue led on
        hifive1_revb_board::Pin::set_low(BLUE_LED_GPIO);

        let mut  delay:  u32 = 0xfffff;

        /* delay */
        while delay > 0 {
            delay -=  1
        } 
        
        // set high 1 - blue led off (since pulledup) 
        hifive1_revb_board::Pin::set_high(BLUE_LED_GPIO);

        delay = 0xfffff;

        while delay > 0 {
            delay  -=  1
        }
    }
}