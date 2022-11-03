
//! # Hifive1-RevB board Library
//!
//! The Hifive1-RevB board Library provides the essential
//! interface for using peripherals in board.


#[repr(C)]
 struct GpioMmapRegs{
    iput_val: u32, /* Pin Value */
    iput_async_en: u32, /* Pin input enable */
    oput_en: u32, /* pin output enable */
    oput_val: u32, /* Output Value */
    pue_async: u32, /* Internal pull-up enable*/
    ds: u32, /* Pin drive strength */
    rise_ie: u32, /* Rise interrupt enable */
    rise_ip: u32, /* Rise intr pending */
    fall_ie: u32, /* Fall intr enable */
    fall_ip: u32, /* Fall intr pending */
    low_ie: u32,
    low_ip: u32,
    iof_en: u32,
    iof_sel: u32,
    out_xor: u32,
    passthru_high_en: u32,
    pthru_low_en: u32,
}

pub struct Gpio(*mut GpioMmapRegs);

fn generate_mask (num: u8) -> u32{
    if num == 0 {
        return 1
    }else {
        return 1 << num
    }
}

 impl Gpio {
    
    pub fn init() -> Self {
        Self(0x1001_2000 as *mut GpioMmapRegs)
    }

    pub fn configure_as_in(&self, p: u8) {
        unsafe {
            let  v: u32 = (*self.0).iput_async_en | generate_mask(p);
            (*self.0).iput_async_en = v;
        }
    }

    pub fn configure_as_out(&self, p: u8) {
        unsafe {
            let  v: u32 =  (*self.0).oput_en | generate_mask(p);
            (*self.0).oput_en = v;
        }
    }
 
    pub fn configure_as_io(&self, p: u8) {
        unsafe {
            let v: u32 = (*self.0).iof_en | generate_mask(p);
            (*self.0).iof_en  = v;
        }
    }

    pub fn write_high(&self, p: u8) {
        unsafe {
            let v: u32 =  (*self.0).oput_val | generate_mask(p);
            (*self.0).oput_val = v;
        }
    }
  
    pub fn write_low(&self, p: u8) { /* 3rd gpio pin, bit pos 2 => mask ...100 => ...11011 */
        unsafe {
            let  v: u32 = (*self.0).oput_val & (! generate_mask(p)) ;
            (*self.0).oput_val = v;
        }
    }
 }