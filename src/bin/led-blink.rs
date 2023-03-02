#![no_std]
#![no_main]

#[path = "../lib/dio.rs"] mod dio;

use core::panic::PanicInfo;
use core::arch::asm;

//const BLUE_LED_GPIO: dio::DioPinNum = 21;
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

fn delay(v: u32)
{
    let mut  d = v;
    /* delay */
    while d > 0 {
        d -=  1
    } 
}

#[no_mangle]
//#[link_section = ".entry"]
pub extern "C" fn _start() -> ! {

    set_stack();
    set_trap_handler();
    clear_external_interrupt();

    let p = dio::DioPin {instance: 0, port: 0, pin_num: 21};
    let mode = dio::DioFuncMode::Gpio;

    p.setup_pin();
    p.enable_pin_outlet();
    p.set_pin_func_mode(&mode);

    loop{

        p.set_pin_outlet_high();

        delay(0xfffff);

        p.set_pin_outlet_low();

        delay(0xfffff);

    }
}
