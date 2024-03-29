
//!  Hifive1-RevB board Uart Interface
use core::ptr; // for read/write volatile 

#[repr(C)]
struct UartMmapRegs{

    /// Writing to txdata register, enqueues the character 
    /// to transmit FIFO if FIFO is able to accept new entries.
    /// Reading from txdata returns the currrent value of the full flag
    /// and zero in the data field.
    /// 
    /// ```
    /// --------------------------
    /// | FULL | RESERVED | DATA |
    /// --------------------------
    /// | 31   | 30 : 8 | [7:0]|
    /// --------------------------
    /// ```
    /// 
    /// `Full` flag indicates whether the FIFO is able to accept new entries.
    /// When `full` flag is set, writes are ignored. 
    /// RISC-V `amoor.w` instruction can be used to both read the status 
    /// and attempt to enqueue data with a non-zero return value 
    /// indicating the character was NOT accepted. 
   txdata: u32, /*  Transmit data */

   /// Reading the rxdata register dequeues the data from the receive FIFO
    /// and returns the value in the data field. The 'empty` flag indicates if
    /// fifo is empty and when set 'data` field dont have valid data. 
    /// Writes to `rxdata` register are ignored. 
    /// 
    /// ```
    /// --------------------------
    /// | EMPTY | RESERVED | DATA |
    /// --------------------------
    /// | 31   | [30: 8]  | [7:0]|
    /// --------------------------
    /// ```
    //
   rxdata: u32, /* Receive data  */

    /// `txctrl` register controls the operation of the transmit channel. 
    /// The `txen` bit controls weather Tx Channel is active. When
    /// cleared, transmission of Tx FIFO contents are suppressed, and the 
    /// `txd` pin is driven high. 
    /// 
    /// ```
    /// ------------------------------------------------------
    /// | RESERVED | WATERMARK | RESERVED | STOP BITS | ENABLE|
    /// ------------------------------------------------------
    /// | [31:19]  | [18:16]   |  [15:2]  |     1     |   0   |
    /// ------------------------------------------------------
    /// ```
    /// The stop bits specifies the number of stop bits, `0` for 1 stop bit
    /// and `1` for two stop bits. 
    /// Watermark/txcnt field specifies the watermark interrupt triggers.
    /// At reset, `txctrl` register is reset to 0. 
    ///
   txctrl: u32, /* Tx control */

    /// The RW `rxctrl` register controls the operation of the receive channel. 
    /// ```
    /// ------------------------------------------------------
    /// | RESERVED | WATERMARK | RESERVED | ENABLE|
    /// ------------------------------------------------------
    /// | [31:19]  | [18:16]   |  [15:1]  |   0   |
    /// ------------------------------------------------------
    /// ```
    /// The `rxen` bit controls wether Rx channel is active. 
    /// When cleared, the state of `rxd` pin is ignored and 
    /// no characters will be enqueued into the Rx FIFO. 
    /// 
    /// The watermark field specifies the threshold at which 
    /// the Rx FIFO watermark interrupt triggers. 
    /// 
    /// The `rxctrl` register is reset to 0. 
    /// Characters are enqueued when a zero (low) start bit 
    /// is seen. 
    ///
   rxctrl: u32, /* Rx Control */

   /// `ie` register controls which UART interrupts are enabled. 
    /// ```
    /// ------------------------------------------------------------------
    /// | RESERVED | Rx WATERMARK INTR ENABLE | Tx WATERMARK INTR ENABLE | 
    /// ------------------------------------------------------------------
    /// | [31:2]   |         1                |            0             |
    /// -----------------------------------------------------------------
    /// ```
   ie: u32, /* Interrupt enable */

    /// The `ip` is a read-only register indicating the pending
    /// interrupt conditions.
    /// ```
    /// ------------------------------------------------------------------
    /// | RESERVED | Rx WATERMARK INTR PEND | Tx WATERMARK INTR PEND | 
    /// ------------------------------------------------------------------
    /// | [31:2]   |         1                |            0             |
    /// -----------------------------------------------------------------
    /// ```
   ip: u32, /* Interrupt pending */

   /// The RW `div` register specifies the divisor used by baud rate
    /// generation for both Tx and Rx Channels. The relationship between
    /// input clock and baud rate is given by following formula
    /// 
    /// ``` baud = input clk / (div + 1) ```
    /// 
    /// Input clock is the bus clock `tlclk`. 
    /// 
    /// **Example** <br>
    /// To get baud rate 115200, with default `tlclk`  16Mhz, 
    /// the `div` register value should be 138. 
    /// 
   div: u32, /* Divisor */
}

const UART0: *mut UartMmapRegs = 0x10013000 as *mut UartMmapRegs; // private to this file 
const UART1: *mut UartMmapRegs = 0x10023000 as *mut UartMmapRegs; // private to this file

pub (crate) fn uart_set_baud_divisor ( instance: u8, div: u32) {
    if instance == 0 {
        unsafe { (*UART0).div = div; }
    } else if instance == 1 {
        unsafe { (*UART1).div = div; }
    }
}

pub (crate) fn uart_set_stopbits ( instance: u8, sbc: u8) {
    // ------------------------TXCTRL-------------------------
    // | RESERVED | WATERMARK | RESERVED | STOP BITS | ENABLE|
    // ------------------------------------------------------
    // | [31:19]  | [18:16]   |  [15:2]  |     1     |   0   |
    // ------------------------------------------------------
    match  instance {
        0 =>{ if sbc == 1 {
                unsafe { (*UART0).txctrl = (*UART0).txctrl & (0xFFFF_FFFD);}
            } else if sbc == 2 {
                unsafe { (*UART0).txctrl = (*UART0).txctrl | (0x2u32);}
            } else {
                panic!("Invalid stop bits count")
            }
        }

        1 =>{ if sbc == 1 {
                unsafe {(*UART1).txctrl = (*UART1).txctrl & (0xFFFF_FFFD);}
            } else if sbc == 2 {
                    unsafe {(*UART1).txctrl = (*UART1).txctrl | (0x2u32);}
            } else {
                panic!("Invalid stop bits count")
            }
        }
        2_u8..=u8::MAX => panic!("Invalid Uart Instance")
    }
}

pub (crate) fn uart_set_tx_fifo_depth ( instance: u8, depth: u8) {

    // ------------------------TXCTRL-------------------------
    // | RESERVED | WMARK/TXCNT| RESERVED | STOP BITS | ENABLE|
    // ------------------------------------------------------
    // | [31:19]  | [18:16]   |  [15:2]  |     1     |   0   |
    // -------------------------------------------------------
    match  instance {
        0 => unsafe { (*UART0).txctrl = ((*UART0).txctrl & 0xFFF8_FFFF) | ((depth as u32) << 15);}
        1 => unsafe { (*UART1).txctrl = ((*UART1).txctrl & 0xFFF8_FFFF) | ((depth as u32) << 15);}
        2_u8..=u8::MAX => panic!("Invalid Uart Instance")
    }
}

pub (crate) fn uart_do_send_byte ( instance: u8, b: u8) {

    // --------------------------
    // | FULL | RESERVED | DATA |
    // --------------------------
    // | 31   | [30: 8]  | [7:0]|
    // --------------------------

    match  instance {
        0 => {
            unsafe { 
                while ((ptr::read_volatile(&(*UART0).txdata as *const u32) >> 31) == 1) { }
                (*UART0).txdata = (ptr::read_volatile(&(*UART0).txdata as *const u32) & 0xFFFF_FFF0)| (b as u32);
            }
        }
        1 => {
            unsafe { 
                while ((ptr::read_volatile(&(*UART1).txdata as *const u32) >> 31) == 1) {}
                (*UART1).txdata =  (ptr::read_volatile(&(*UART1).txdata as *const u32) & 0xFFFF_FFF0) | (b as u32);
            }
        }
        2_u8..=u8::MAX => panic!("Invalid Uart Instance")
    }
}

pub (crate) fn uart_enable_tx ( instance: u8) {

    // ------------------------TXCTRL-------------------------
    // | RESERVED | WMARK/TXCNT| RESERVED | STOP BITS | ENABLE|
    // ------------------------------------------------------
    // | [31:19]  | [18:16]   |  [15:2]  |     1     |   0   |
    // -------------------------------------------------------

    match  instance {
        0 => unsafe { (*UART0).txctrl = ptr::read_volatile(&(*UART0).txctrl as *const u32) | 1u32;}
        1 => unsafe { (*UART1).txctrl = ptr::read_volatile(&(*UART1).txctrl as *const u32) | 1u32;}
        2_u8..=u8::MAX => panic!("Invalid Uart Instance")
    }
}

pub (crate) fn uart_disable_tx ( instance: u8) {

    // ------------------------TXCTRL-------------------------
    // | RESERVED | WMARK/TXCNT| RESERVED | STOP BITS | ENABLE|
    // ------------------------------------------------------
    // | [31:19]  | [18:16]   |  [15:2]  |     1     |   0   |
    // -------------------------------------------------------

    match  instance {
        0 => unsafe { (*UART0).txctrl =  ptr::read_volatile(&(*UART0).txctrl as *const u32) & 0xFFFF_FFFE;}
        1 => unsafe { (*UART1).txctrl =  ptr::read_volatile(&(*UART1).txctrl as *const u32) & 0xFFFF_FFFE;}
        2_u8..=u8::MAX => panic!("Invalid Uart Instance")
    }
}