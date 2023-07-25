
#[path = "fe310/gpio.rs"] mod gpio;
#[path = "fe310/uart.rs"] mod uart;

pub type UartBaud = u32;
pub type UartInstance = u8;

pub enum SerialDeviceType {
    Uart,
    Spi,
    I2c,
}

pub enum UartBitCount {
    One,
    Two,
}

pub enum UartFifoDepth {
    Min,
    Max,
}

pub struct UartConfig {
    pub baud: UartBaud,
    pub start_bits: UartBitCount,
    pub stop_bits: UartBitCount,
    pub fifo: UartFifoDepth,
}

pub struct Uart {
    pub instance: UartInstance,
    pub config: UartConfig,
}

pub trait SerialTrait {
    fn configure(&self) {}
    fn enable_tx(&self) {}
    fn disable_tx (&self) {}
    fn do_send_byte(&self, b: u8) {}
    fn enable_tx_wmark_interrupt(&self) {}
    fn atomic_send_byte ( &self, b: u8) -> bool;
    fn poll_tx_busy(&self) -> bool;
    fn send_byte(&self, b: u8);
}

impl SerialTrait for Uart {

    fn configure(&self){
        uart::uart_set_baud_divisor((*self).instance, 138);
        uart::uart_set_stopbits ( (*self).instance, 1);
        uart::uart_set_tx_fifo_depth( (*self).instance, 7);
    }

    fn do_send_byte(&self, b: u8){
        uart::uart_do_send_byte ( (*self).instance, b);
    }

    fn send_byte(&self, b: u8){
        uart::uart_send_byte ( (*self).instance, b);
    }

    fn enable_tx (&self){
        uart::uart_enable_tx ( (*self).instance);
    }

    fn disable_tx (&self){
        uart::uart_disable_tx ( (*self).instance);
    }

    fn enable_tx_wmark_interrupt(&self){
        uart::uart_enable_tx_wmark_interrupt ( (*self).instance);
    }

    fn atomic_send_byte(&self, b:u8 ) -> bool {
        return uart::uart_atomic_send_byte ( (*self).instance, b);
    }

    fn poll_tx_busy(&self) -> bool {
        return uart::poll_tx_busy ( (*self).instance);
    }
}
