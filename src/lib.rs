
//! # Hifive1-RevB board Library
//!
//! The Hifive1-RevB board Library provides the essential
//! interface for using peripherals in board.
#![no_std]


#[repr(C)]
pub struct GpioMmapRegs{
    pub iput_val: u32, /* Pin Value */
    pub iput_async_en: u32, /* Pin input enable */
    pub oput_en: u32, /* pin output enable */
    pub oput_val: u32, /* Output Value */
    pub pue_async: u32, /* Internal pull-up enable*/
    pub ds: u32, /* Pin drive strength */
    pub rise_ie: u32, /* Rise interrupt enable */
    pub rise_ip: u32, /* Rise intr pending */
    pub fall_ie: u32, /* Fall intr enable */
    pub fall_ip: u32, /* Fall intr pending */
    pub low_ie: u32,
    pub low_ip: u32,
    pub iof_en: u32,
    pub iof_sel: u32,
    pub out_xor: u32,
    pub passthru_high_en: u32,
    pub pthru_low_en: u32,
}