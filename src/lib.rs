
//! # Hifive1-RevB board Library
//!
//! The Hifive1-RevB board Library provides the essential
//! interface for using peripherals in board.
#![no_std]

pub enum Dir {
    In,
    Out,
}

pub enum Func {
    Gpio,
    Iof,
}

pub struct GpioPin;

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

fn num2hexbit (num: u8) -> u32{
    if num == 0 {
        return 1
    }else {
        return 1 << num
    }
}

 impl GpioPin {
    #[no_mangle]
    pub fn configure_as_in(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).iput_async_en;
        
            v = v | num2hexbit(p);
            (*t).iput_async_en = v;
        }
    }
    #[no_mangle]
    pub fn configure_as_out(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).oput_en;
        
            v = v | num2hexbit(p);
            (*t).oput_en = v;
        }
    }
    #[no_mangle]
    pub fn configure_as_io(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).iof_en;
        
            v = v | num2hexbit(p);
            (*t).iof_en = v;
        }
    }
    #[no_mangle]
    pub fn select_iof(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).iof_sel;
        
            v = v | num2hexbit(p);
            (*t).iof_sel = v;
        }
    }
    #[no_mangle]
    pub fn write_high(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).oput_val;
        
            v = v | num2hexbit(p);
            (*t).oput_val = v;
        }
    }
    #[no_mangle]
    pub fn write_low(p: u8) {
        unsafe {
            let mut v: u32;
            let t = 0x1001_2000 as *mut GpioMmapRegs;
            v = (*t).oput_val;
            (*t).oput_val = 0;
        
            /* v = (*t).oput_val;
            v = v | num2hexbit(p);
            (*t).oput_val = v & !num2hexbit(p);
            
            80000080 <write_low>:
80000080:	100125b7          	lui	a1,0x10012
80000084:	45d0                	lw	a2,12(a1)
80000086:	0ff57713          	zext.b	a4,a0
8000008a:	56f9                	li	a3,-2
8000008c:	c711                	beqz	a4,80000098 <write_low+0x18>
8000008e:	4685                	li	a3,1
80000090:	00a69533          	sll	a0,a3,a0
80000094:	fff54693          	not	a3,a0
80000098:	00d67533          	and	a0,a2,a3
8000009c:	c5c8                	sw	a0,12(a1)
8000009e:	8082                	ret

*/
        }
    }

 }