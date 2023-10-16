
/* NOTE: 
    psd lf clksel is grounded in schematics enabling 
    psd alternate clock (32.768 external xtal) 
    for low frequency. Hence RTC by default is supplied 
    with  32.768 khz xtal. 
*/

// AON base mmap addr 0x1000_0000

/* lfosccfg (offset 0x70) */
const LFOSC_CFG_PTR: *mut u32 = 0x1000_0070 as *mut u32;

/* lfosccfg (offset 0x7C) */
const LFEXT_CLKSEL_PTR: *mut u32 = 0x1000_007C as *mut u32;

/* rtccmp (offset 0x60) */
const RTC_COMP_PTR: *mut u32 = 0x1000_0060 as *mut u32;

/* rtcs (offset 0x50) - low bits counter */
const RTCS_PTR: *const u32 = 0x1000_0050 as *const u32;

/* rtccounthi (offset 0x04C) - high bits counter */
const RTC_COUNT_HI_PTR: *mut u32 = 0x1000_004C as *mut u32;

/* rtccountlo (offset 0x048) - low bits counter */
const RTC_COUNT_LO_PTR: *mut u32 = 0x1000_0048 as *mut u32;

/* rtccfg (offset 0x040):
    [3:0] - rtcscale
    [11:4] - rsvd
    [12] - rtcenalways
    [27:13] - rsvd
    [28] - rtcip0
    [31-29] - rsvd */

const RTCCFG_PTR: *mut u32 = 0x1000_0040 as *mut u32;

pub (crate) fn clksel_32768k() {

    let rosccfg = LFOSC_CFG_PTR as *const u32;
    let rclksel = LFEXT_CLKSEL_PTR as *const u32;
    
    let wosccfg = rosccfg as *mut u32;
    let wclksel = rclksel as *mut u32;

    unsafe {

        // disabel lfosc
        wosccfg.write_volatile(rosccfg.read_volatile() | ((1 << 30) as u32) );

        // Set 32.768 khz for rtc clock source
        wclksel.write_volatile(rclksel.read_volatile() | 1 as u32);

    }
}

pub (crate) fn reset_rtc_counter() {
    let rhi = RTC_COUNT_HI_PTR as *const u32;
    let rlo = RTC_COUNT_LO_PTR as *const u32;
    let rcmp = RTC_COMP_PTR as *const u32;
    let whi = rhi as *mut u32;
    let wlo = rlo as *mut u32;
    let wcmp = rcmp as *mut u32;

    unsafe {
        whi.write_volatile(0);
        wlo.write_volatile(0);
        wcmp.write_volatile(0);
    }
}

pub (crate) fn restart() {
    clksel_32768k();
    reset_rtc_counter();
    let r = RTCCFG_PTR as *const u32;
    let w = r as *mut u32;

    unsafe {
        w.write_volatile(r.read_volatile() | ( 1 << 12));
    }
}

pub (crate) fn set_scale(scale: u8){

    assert!(scale <16);

    let r = RTCCFG_PTR as *const u32;
    let w = r as *mut u32;
    let s = scale as u32;

    unsafe {
        w.write_volatile(r.read_volatile() | s);
    }
}

pub fn get_rtc_counter() -> u64 {
    let hi = RTC_COUNT_HI_PTR as *const u32;
    let lo = RTC_COUNT_LO_PTR as *const u32;
    let mut c:  u64 = 0;

    unsafe {
        c = ((hi.read_volatile() as u64) << 32) | (lo.read_volatile() as u64);
    }
    return c;
}

pub fn get_rtcs() -> u32 {
    let rtcs = RTCS_PTR as *const u32;  

    unsafe {
        return rtcs.read_volatile();
    }
}

pub fn set_rtc_cmp(v: u32) {

    let rcmp = RTC_COMP_PTR as *const u32;
    let wcmp = rcmp as *mut u32;

    unsafe {
        wcmp.write_volatile(v);
    }
}

pub fn enable() {

    let r = RTCCFG_PTR as *const u32;
    let w = r as *mut u32;

    unsafe {
        w.write_volatile(r.read_volatile() | ( 1 << 12));
    }

}

pub fn disable(){

    let r = RTCCFG_PTR as *const u32;
    let w = r as *mut u32;
    let disable_mask = !(( 1 << 12) as u32);

    unsafe {
        w.write_volatile(r.read_volatile() & disable_mask);
    }
}

pub fn is_cmp_reached() -> bool {

    let r = RTCCFG_PTR as *const u32;
    let ip0_mask: u32 = 0x10000000;

    unsafe {
        if (r.read_volatile() & ip0_mask) != 0 {
            return true;
        } else {
            return false;
        }
    }
}




