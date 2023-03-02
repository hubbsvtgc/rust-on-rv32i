
const FIFO_SEG_BMASK: u32 = 0x0007_0000;
const DATA_SEG_BMASK: u32 = 0x0000_00FF;

#[repr(C)]
struct UartMmapRegs{
   txdata: u32, /*  Transmit data */
   rxdata: u32, /* Receive data  */
   txctrl: u32, /* Tx control */
   rxctrl: u32, /* Rx Control */
   ie: u32, /* Interrupt enable */
   ip: u32, /* Interrupt pending */
   div: u32, /* Divisor */
}

pub struct Uart(*mut UartMmapRegs);

pub enum UartInstance{
    Ins0,
    Ins1
}

pub enum UartStopBits{
    One,
    Two
}

pub enum UartBaudRate{ /* NOTE: Current clock rate is 16 Mhz */
    BR115200,
    BR31250,
}

impl Uart {

    #[no_mangle]
    pub fn init (instance: UartInstance) -> Self {
        match instance {
            UartInstance::Ins0 => Self (0x10013000 as *mut UartMmapRegs),
            UartInstance::Ins1 => Self (0x10023000 as *mut UartMmapRegs),
        }
    }

    #[no_mangle]
    pub fn set_baud(&self, br: UartBaudRate) {
        match br {
            UartBaudRate::BR115200 => { unsafe { (*self.0).div = 139;} }, 
            UartBaudRate::BR31250 => { unsafe { (*self.0).div = 512;} },
        }
    }

    #[no_mangle]
    pub fn set_stopbits(&self, s: UartStopBits) {
        match s {
            UartStopBits::One => { unsafe { (*self.0).txctrl = (*self.0).txctrl & ( ! 0x2);} }, 
            UartStopBits::Two => { unsafe { (*self.0).txctrl = (*self.0).txctrl | 0x2 ;} },
        }
    }

    #[no_mangle]
    pub fn set_txfifosize(&self, l: u32) {
        /* To trigger interrupts */
        if l > 7 {
            panic!("Out of bound tx fifo size");
        }
        unsafe { (*self.0).txctrl = (*self.0).txctrl & (!FIFO_SEG_BMASK) | (l << 16) ;} 
    }

    #[no_mangle]
    pub fn set_rxfifosize(&self, l: u32) {
        /* To trigger interrupts */
        if l > 7 {
            panic!("Out of bound tx fifo size");
        }
        unsafe { (*self.0).rxctrl = ((*self.0).rxctrl & (!FIFO_SEG_BMASK)) | (l << 16) ;}
    }

    #[no_mangle]
    pub fn enable_tx_channel(&self){
        unsafe { (*self.0).txctrl = (*self.0).txctrl | 0x1 ;}
    }

    #[no_mangle]
    pub fn enable_rx_channel(&self){
        unsafe { (*self.0).rxctrl = (*self.0).rxctrl | 0x1 ;}
    }

    #[no_mangle]
    pub fn transmit (&self, data: u32) {
        unsafe { 
            let ptr  = &*self.0;

            let field_ptr = core::ptr::addr_of!(ptr.txdata);

            core::ptr::write_volatile(field_ptr as *mut u32, data);
            
            //*ptr_field = data;

            //.txdata =  data ;}
        }
    }

    #[no_mangle]
    pub fn txfifo_full (&self) -> bool {
        let fs = unsafe { (*self.0).txdata >> 31 };
        if fs > 0 { 
            return true
        } else {
            return false
        }
    }

    #[no_mangle]
    pub fn rxfifo_empty (&self) -> bool {
        let fs = unsafe { (*self.0).rxdata >> 31 };
        if fs > 0 { 
            return true
        } else {
            return false
        }
    }

    #[no_mangle]
    pub fn receive (&self) -> u32 {
        unsafe {
            let ptr = &*self.0;
            let field_ptr = core::ptr::addr_of!(ptr.rxdata);

            return core::ptr::read_volatile( field_ptr) & DATA_SEG_BMASK;
            //return ((*self.0).rxdata) & DATA_SEG_BMASK ;
        }
    }
}