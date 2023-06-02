#![no_std]
#![no_main]

#[path = "../lib/dio.rs"] mod dio;
#[path = "../lib/serial.rs"] mod serial;

use core::panic::PanicInfo;
use core::arch::asm;
use crate::serial::{Configure, DoSendByte, EnableTx, DisableTx};

const UART0_TX_GPIO17: u8 = 17; /* .equiv GPIO17, 17   --UART0 Tx */
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
        
//W e   l   c  o   m   e     t   o    L  e   a  r   n    R  I  S  C  V    LF CR  NULL
//87,101,108,99,111,109,101, 116,111, 76,101,97,114,110, 82,73,83,67,86,  10,13, 00;

/*NOTE; char in rust is NOT a byte, */
let note: [u8; 9] = [ b'W', b'e', b'l', b'c', b'o', b'm', b'e', 10, 13];

for i in 1..10 {
    for c in note.iter() {
        uart.do_send_byte(*c);
    }
}
    delay(0xfff); // Delay to flush fifo before its disabled
    uart.disable_tx();
    loop {}
}

