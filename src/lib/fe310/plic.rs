
use core::ptr; // for read/write volatile 

const COUNT_PLIC_INTR_SRCS: usize = 52;

#[repr(C)]
struct PlicMemMap {
    rsvd1: u32, // start addr 0x0C00_0000
    src_priority: [u32; COUNT_PLIC_INTR_SRCS], // start addr 0x0C00_0004
    rsvd2_blk: [u32; 971], // 0x0C00_00D4
    pend1: u32, // 0x0C00_1000
    pend2: u32, // 0x0C00_1004
    rsvd3_blk: [u32; 1022], // 0x0C00_1008
    Hart0MmodeIe1: u32, // 0x0C00_2000
    Hart0MmodeIe2: u32, // 0x0C00_2004
    rsvd4_blk: [u32; 522238], // 0x0C00_2008
    Hart0MmodePriThreshold: u32, // 0x0C20_0000
    Hart0MmodeClaimComplete: u32, // 0x0C20_0004
}

const FE310_PLIC_MMAP: *mut PlicMemMap = 0x0C00_0000 as *mut PlicMemMap;

/* This same is used for setting priority levels for interrupts
 * and to set the threshold */
 
pub enum PlicIntrPriorityLevels {
    level0_lowest = 0, // Enable all interrupts with non zero priority
    level1,
    level2,
    level3,
    level4,
    level5,
    level6,
    level7_highest, // Masks all 
}

pub enum PlicIntrSources {
    aon_wdog = 1,
    aon_rtc,
    uart0,
    uart1,
    qspi0, // 5
    spi1,
    spi2,
    gpio0,
    gpio1,
    gpio2, // 10
    gpio3,
    gpio4,
    gpio5,
    gpio6,
    gpio7, // 15
    gpio8,
    gpio9,
    gpio10,
    gpio11,
    gpio12, // 20
    gpio13,
    gpio14,
    gpio15,
    gpio16,
    gpio17, // 25
    gpio18,
    gpio19,
    gpio20,
    gpio21,
    gpio22, // 30
    gpio23,
    gpio24,
    gpio25,
    gpio26, 
    gpio27, // 35
    gpio28,
    gpio29,
    gpio30,
    gpio31, // 39 
    pwm0a,  // 40
    pwm0b,
    pwm0c,
    pwm0d,
    pwm1a,
    pwm1b, // 45
    pwm1c,
    pwm1d,
    pwm2a,
    pwm2b,
    pwm2c, // 50 
    pwm2d,
    i2c, // 52
    all, //53
}

pub fn plic_set_priority_threshold (pthreshold: PlicIntrPriorityLevels /* hart: u8 */){

    unsafe {
        let x = &(*FE310_PLIC_MMAP).Hart0MmodePriThreshold as *const u32;
        let y = x as *mut u32;
        y.write_volatile(pthreshold as u32);
    }
}

pub fn plic_enable_src_to_interrupt (src: PlicIntrSources) {

    match src {
        uart0 => {
            unsafe {
                let x = &(*FE310_PLIC_MMAP).Hart0MmodeIe1 as *const u32;
                let y = x as *mut u32;
                let mut v: u32 = PlicIntrSources::uart0 as u32;
                let v = 1 << v;
                y.write_volatile(x.read_volatile() | v);
            }
        }
        other => { panic!("PANIC ###"); }
    }
}

pub fn plic_set_intr_priority_for_src (src: PlicIntrSources, p: PlicIntrPriorityLevels) {

    match src {
        uart0 => {
            unsafe {
                let ptr = &(*FE310_PLIC_MMAP).src_priority[PlicIntrSources::uart0 as usize] as *const u32;
                let y = ptr as *mut u32;
                // *y.offset(PlicIntrSources::uart0 as isize) = p as u32;
                //ptr::write_volatile(y.offset(PlicIntrSources::uart0 as isize), p as u32);
                y.write_volatile(p as u32);
            }
        }
        other => { panic!("PANIC ###"); }
    }
}

pub fn plic_disable_src_to_interrupt (src: PlicIntrSources)
{
    match src {
        all => unsafe {

            let x = &(*FE310_PLIC_MMAP).Hart0MmodeIe1 as *const u32;
            let y = x as *mut u32;
            y.write_volatile(0);

            let x = &(*FE310_PLIC_MMAP).Hart0MmodeIe2 as *const u32;
            let y = x as *mut u32;
            y.write_volatile(0);
        }
    }

}   