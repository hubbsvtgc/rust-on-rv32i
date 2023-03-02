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

const GPIO: *mut GpioMmapRegs = 0x1001_2000 as *mut GpioMmapRegs;

fn generate_mask (num: u8) -> u32{
    if num == 0 {
        return 1
    }else {
        return 1 << num
    }
}
    pub (crate) fn enable_inlet( p: u8) {
        unsafe {
            (*GPIO).iput_async_en = (*GPIO).iput_async_en | generate_mask(p);
        }
    }

    pub (crate) fn enable_outlet(p: u8) {
        unsafe {
            (*GPIO).oput_en  = (*GPIO).oput_en | generate_mask(p);
        }
    }
 
    pub fn set_as_iof(p: u8, ) {
        unsafe {
            (*GPIO).iof_en = (*GPIO).iof_en | generate_mask(p);
        }
    }

    pub fn set_as_dio(p: u8) {
        unsafe {
            (*GPIO).iof_en = (*GPIO).iof_en & (! generate_mask(p));
        }
    }

    pub fn set_high( p: u8) {
        unsafe {
            (*GPIO).oput_val =  (*GPIO).oput_val  | generate_mask(p);
        }
    }
  
    pub fn set_low(p: u8) { 
        unsafe {
            (*GPIO).oput_val =  (*GPIO).oput_val & (! generate_mask(p)) ;
        }
    }
 