#![no_std]
#![no_main]

#[path = "../lib/dio.rs"] mod dio;
#[path = "../lib/serial.rs"] mod serial;
#[path = "../lib/fe310/interrupt.rs"] mod intr;
#[path = "../lib/fe310/plic.rs"] mod plic;

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

    /* Set trap handler to mtvec */

    intr::set_trap_handler();

    /* Enable machine & external interrupt */

    intr::enable_m_interrupt();

    intr::enable_m_external_interrupt();

    /* Disable all srcs to interrupt first */
    plic::plic_disable_src_to_interrupt(plic::PlicIntrSources::all);

    /* set priority 4 for uart0 */
    plic::plic_set_intr_priority_for_src(plic::PlicIntrSources::uart0, plic::PlicIntrPriorityLevels::level4);

    /* Set threshold and enable uart0 interrupt source */
    plic::plic_set_priority_threshold(plic::PlicIntrPriorityLevels::level3);

    /* Enable UART0 alone */
    plic::plic_enable_src_to_interrupt(plic::PlicIntrSources::uart0);

    /* Configure pin */

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
    uart.enable_tx_wmark_interrupt();

    delay(0xf);
    uart.enable_tx();

    //W e   l   c  o   m   e     t   o    L  e   a  r   n    R  I  S  C  V    LF CR  NULL
    //87,101,108,99,111,109,101, 116,111, 76,101,97,114,110, 82,73,83,67,86,  10,13, 00;

    /*NOTE; char in rust is NOT a byte, 
    let note: [u8; 21] = [ b'W', b'e', b'l', b'c', b'o', b'm', b'e',  b't', b'o', b'L', b'e',
                        b'a', b'r', b'n', b'R', b'I', b'S', b'C', b'V', 10, 13];

    for i in 1..10 {
        for c in note.iter() {
            uart.do_send_byte(*c);
        }
    }

    delay(0xfffff); // Delay to flush fifo before its disabled
    uart.disable_tx(); */

    loop {}
}
