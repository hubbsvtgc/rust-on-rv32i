#![no_std]
#![no_main]

/* **********************For loopback excercise, ************************** 
 * ************ connect J2-Pin2(UART1-TX/GPIO18) to J2-Pin7(UART1-RX/GPIO23) 
 * *************************************************************************/

use fe310::gpio;
use fe310::uart;

use core::panic::PanicInfo;
use core::arch::asm;

const UART1_TX_GPIO18: u8 = 18;
const UART1_RX_GPIO23: u8 = 23;
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

#[no_mangle]
//#[link_section = ".entry"]
pub extern "C" fn _start() -> ! {

    set_stack();
    set_trap_handler();
    clear_external_interrupt();

    /* Configure gpio18 as UART1-Tx and gpio23 as UART1-Rx and 
        const UART1_TX_GPIO: u8 = 18;
        const UART1_RX_GPIO: u8 = 23; */

    let gpio_uart1_tx = gpio::Gpio::init();
    let gpio_uart1_rx = gpio::Gpio::init();

    gpio_uart1_tx.configure_as_io(UART1_TX_GPIO18);
    gpio_uart1_rx.configure_as_io(UART1_RX_GPIO23);
    gpio_uart1_tx.select_iof(UART1_TX_GPIO18, 0);
    gpio_uart1_rx.select_iof(UART1_RX_GPIO23, 0);

    /* UART Functionality Test */

    let instance = uart::UartInstance::Ins1;
    let baud = uart::UartBaudRate::BR115200;
    let txbuf: [u32; 10] = [0x1,0x3,0x7,0xf,0x1f,0x3f,0x7f,0xff,0x1,0x3];
    let mut rxbuf: [u32; 10] =  [91, 92, 93, 94, 95, 96, 97, 98, 99, 100];/* [0; 10]; Initialize with value to avoid using memset */
    let (mut rxidx, mut txidx)  = (0, 0);

    let u = uart::Uart::init(instance);
    u.set_baud(baud);
    u.set_txfifosize(6);
    u.enable_tx_channel();
    u.set_rxfifosize(6);
    u.enable_rx_channel();

    //let d: u32 =  0xbc;

   while txidx < 9 {

            u.transmit(txbuf[txidx]);
            txidx = txidx + 1;
            if txidx == 9{
                txidx = 0;
            }

            let mut  delay:  u32 = 0xfff;

            while delay > 0 {
                delay  -=  1
            }

            rxbuf[rxidx] = u.receive();
            rxidx = rxidx + 1;
            if rxidx == 9{
                rxidx = 0;
            }
        }

    loop { unsafe { asm!("nop"); } }

}