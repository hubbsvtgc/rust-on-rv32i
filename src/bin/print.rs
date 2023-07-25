#![no_std]
#![no_main]

#[path = "../lib/dio.rs"] mod dio;
#[path = "../lib/serial.rs"] mod serial;

use core::panic::PanicInfo;
use core::arch::asm;
use crate::serial::SerialTrait;

const UART0_TX_GPIO17: u8 = 17; /* .equiv GPIO17, 17   --UART0 Tx */
const PLIC_BASE: u32 = 0xC000000; // 0xC00 << 16 = 0xC00 0000,  
const PLIC_CLAIMCOMP_CTX1_OFFSET: u32 = 0x200004;
const MIE_SET: u32 = 0;

const STACK4K_RAMADDR: usize = 0x80001000;
const FIXED_JUMPADDR: usize =  0x20000028;

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
#[link_section = ".prestart"]
fn prestart (){
    unsafe {
        asm!("nop");
        asm!("mv t0, {}", in(reg) STACK4K_RAMADDR);
        asm!("mv sp, t0");

        asm!("mv t1, {}", in(reg) FIXED_JUMPADDR);
        asm!("jr t1");
        asm!("nop");
    }
}

#[no_mangle]
#[link_section = ".entry"]
pub extern "C" fn _start() -> ! {

    set_stack();
    set_trap_handler();
    clear_external_interrupt();

    let p = dio::DioPin {instance: 0, port: 0, pin_num: 17};
    let mode = dio::DioFuncMode::Mux;

    p.setup_pin();
    p.set_pin_func_mode(&mode);
    p.select_pin_iof_func(false);

    let uart_cfg = serial::UartConfig {
        baud: 115200,
        start_bits: serial::UartBitCount::Two,
        stop_bits: serial::UartBitCount::Two,
        fifo: serial::UartFifoDepth::Max
    };

    let uart = serial::Uart {
        instance: 0,
        config: uart_cfg,
    };

    uart.configure();

    delay(0xffff);
    uart.enable_tx();

/********************************
*** send byte if NOT busy *******  
*********************************/

    let note = b"Passed!Prestart#Money$is\n";

    for i in 1..10 {
        for c in note.iter() {
            
           while (uart.poll_tx_busy()){
                delay(0xfff);
            }
            uart.do_send_byte(*c);
        }
    }

    let endnote = b"*******************\n";

    for c in endnote.iter() {
        while (uart.poll_tx_busy()){
            delay(0xfff);
        }
        uart.do_send_byte(*c);
    }

    delay(0xfff);
/*************************************
*** send atomic and then check *******  
*************************************

for i in 1..10 {
    for c in note.iter() {
        while uart.atomic_send_byte(*c) != true
        {
            delay(0xfff);
        }
    }
}


// print end line

for c in endnote.iter() {
    while (uart.poll_tx_busy()){
        delay(0xfff);
    }
    uart.do_send_byte(*c);
}
*/
delay(0xfff);

/*************************************
*** send and forget ******************  
*************************************

for i in 1..10 {
    for c in note.iter() {
        uart.send_byte(*c);
    }
}
*/

    delay(0xffff); // Delay to flush fifo before its disabled
    uart.disable_tx();
    loop {}
}

