
use core::arch::asm;
use core::ptr; // for read/write volatile 
#[path = "uart.rs"] mod u;

pub type mTrapHandlerFnPtr = fn();
const TRAP_CAUSE_INTR_BIT_MASk: u32 = 0x8000_0000;
const INTRPT_EXCEP_CODE_MASK: u32 = 0x0000_000F;
const HART0_MMODE_CLAIM: u32 =  0x0C20_0004;

const MSTATUS_MIE: u32 = 0x8; /* Bit 3 - Machine Interrupt Enable */
const MIE_MTIE: u32 = 0x80; /* Bit 7 - Machint Timer Interrupt */
const MIE_MEIE: u32 = 0x800; /* Bit 11 - Machine External Interrupt */

fn process_mexternal_interrupt()
{
    unsafe{
        let claim_mmap_addr = HART0_MMODE_CLAIM as *const u32;
        let intr_id = claim_mmap_addr.read_volatile();

        match intr_id {
            3=> { // Uart0 interrupt 
                let u0pend_mmap_addr = 0x1001_3000 as *const u32;
                let u0pend = u0pend_mmap_addr.read_volatile();

                if u0pend as u32 & 0x1 == 1 {
                    // Tx watermark interrupt
                    static mut COUNT: usize = 0;

                    /*NOTE; char in rust is NOT a byte, */
                    let note: [u8; 21] = [ b'W', b'e', b'l', b'c', b'o', b'm', b'e',  b't', b'o', b'L', b'e',
                    b'a', b'r', b'n', b'R', b'I', b'S', b'C', b'V', 10, 13];

                    u::uart_do_send_byte(0, note[COUNT]);
                    u::uart_do_send_byte(0, note[COUNT + 1]);
                    u::uart_do_send_byte(0, note[COUNT + 2]);
                    u::uart_do_send_byte(0, note[COUNT + 3]);
                    u::uart_do_send_byte(0, note[COUNT + 4]);

                    COUNT = COUNT + 5;

                    if COUNT == 21 {
                       COUNT = 0;
                    }

                } else if u0pend as u32 & 0x2 == 2 {
                    // Rx watermark interrupt
                    let rcvd_count = 0x8001_0000 as *mut u32; // TDB: Based on ELF size
                    *rcvd_count = *rcvd_count + 1; // initialized at start

                    let rcvd_curaddr = 0x8001_0004 as *mut u32;
                    *rcvd_curaddr = *rcvd_curaddr + 4;
                }
            }
            _ => {
                panic!("invalide");
            }
        }
    }
}

#[no_mangle]
pub fn m_trap_handler()
{
    let mtrap_cause: u32;
    let mut is_interrupt: bool = false;

    unsafe{
        asm!("csrr {}, mcause", out(reg) mtrap_cause);
    }

    if  mtrap_cause & TRAP_CAUSE_INTR_BIT_MASk != 0 {
        is_interrupt = true;
    }

    if (is_interrupt){ 

    let async_interrupt = mtrap_cause & INTRPT_EXCEP_CODE_MASK;

        match  async_interrupt {
            0..=2u32=> { 
                panic!("Rsvd trap cause")
            }
            3=> {
                // Machine Software interrupt
            }
            4..=6u32=> { 
                panic!("Rsvd trap cause")
            }
            7=> {
                // Machine Timer interrupt
            }
            8..=10u32=> { 
                panic!("Rsvd trap cause")
            }
            11=> {
                // Machine External interrupt
                process_mexternal_interrupt();
            }
            _ => {
                panic!("Rsvd trap cause")
            }
        }
    }
    else {
        let sync_exception = mtrap_cause & INTRPT_EXCEP_CODE_MASK;

        match sync_exception { // Exceptions
            0=> {
                // Instruction addr mis-aligned 
            }
            1=> {
                // Instruction access fault
            }
            2=> {
                // Illegal Instruction
            }
            3=> {
                // Break point
            }
            4=> {
                // Load addr mis-aligned 
            }
            5=>  { 
                // Load access fault 
            }
            6=>  { 
                // Store/AMO addr misaligned 
            }
            7=>  { 
                // Store/AMO access fault 
            }
            8=>  { 
                // Environment call from U mode  
            }
            9..=10u32=> { 
                panic!("Rsvd trap cause")
            }
            11=>  { 
                // Environment call from M mode  
            }
            _ => { panic!("Rsvd trap cause") }
        }
    }
}

pub fn enable_m_interrupt()
{

  /* Read MSTATUS csr and set bit 3
  to enable machine interrupt */

    unsafe {
        asm!("csrr t0, mstatus");
        asm!("li t1, 0x8");  /* MSTATUS_MIE: u32 = 0x8 */ 
        asm!("or t0, t0, t1");
        asm!("csrw mstatus, t0");
    }
}

pub fn enable_m_external_interrupt()
{

        /* Read MIE csr and set bit 11 to enable external
    * interrupt */

    unsafe {
        asm!("csrr t0, mie");
        asm!("li t1, 0x800");  /* MIE_MEIE: u32 = 0x800;  */
        asm!("or t0, t0, t1");
        asm!("csrw mie, t0");
    }
}

pub fn set_trap_handler()
{

    let th: mTrapHandlerFnPtr = m_trap_handler;
    unsafe{
        asm!("csrw mtvec, {}", in(reg) th);
    }
}