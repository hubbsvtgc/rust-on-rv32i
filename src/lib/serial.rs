
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

pub trait Configure {
    fn configure(&self) {}
}

pub trait EnableTx {
    fn enable_tx(&self) {}
}

pub trait DisableTx {
    fn disable_tx (&self) {}
}

pub trait DoSendByte {
    fn do_send_byte(&self, b: u8) {}
}

impl Configure for Uart {
    fn configure(&self){
        uart::uart_set_baud_divisor((*self).instance, 138);
        uart::uart_set_stopbits ( (*self).instance, 1);
        uart::uart_set_tx_fifo_depth( (*self).instance, 7);
    }
}


impl DoSendByte for Uart {
    fn do_send_byte(&self, b: u8){
        uart::uart_do_send_byte ( (*self).instance, b);
    }
}

impl EnableTx for Uart {
    fn enable_tx (&self){
        uart::uart_enable_tx ( (*self).instance);
    }
}

impl DisableTx for Uart {
    fn disable_tx (&self){
        uart::uart_disable_tx ( (*self).instance);
    }
}