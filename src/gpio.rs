
//! # Gpio HAL 
//!
//! The Gpio HAL module provides the essential
//! interface for using gpio.

pub struct Pin;

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

fn generate_mask (num: u8) -> u32{
    if num == 0 {
        return 1
    }else {
        return 1 << num
    }
}

 impl Pin {
    #[no_mangle]
    pub fn set_as_in(p: u8) {
        unsafe {
            let v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).iput_async_en | generate_mask(p) ;
            (*t).iput_async_en = v;
        }
    }
    #[no_mangle]
    pub fn set_as_out(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).oput_en;
        
            v = v | generate_mask(p);
            (*t).oput_en = v;
        }
    }
    #[no_mangle]
    pub fn set_as_gpio(p: u8) {
        unsafe {
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            let v: u32 = (*t).iof_en & (!generate_mask(p));
            (*t).iof_en = v;
        }
    }
    #[no_mangle]
    pub fn set_as_io(p: u8) {
        unsafe {
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            let v: u32 = (*t).iof_en  | generate_mask(p);
            (*t).iof_en = v;
        }
    }
    #[no_mangle]
    pub fn select_iof(p: u8) {
        unsafe {
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            let v: u32 = (*t).iof_sel | generate_mask(p);
            (*t).iof_sel = v;
        }
    }
    #[no_mangle]
    pub fn set_high(p: u8) {
        unsafe {
            let v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).oput_val | generate_mask(p);
            (*t).oput_val = v;
        }
    }
    #[no_mangle]
    pub fn set_low(p: u8) {
        /* reg value = reg value & ( !mask) */
        unsafe {
            let v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).oput_val & (!generate_mask(p));
            (*t).oput_val = v;
        }
    }
 }