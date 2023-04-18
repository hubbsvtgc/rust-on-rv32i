#![no_std]
#![no_main]

use core::arch::asm;

pub type PmpRegionStartAddr = u32;
pub type PmpRegionSize = u32;

pub enum PmpRegionAddrMatchMode{
    PmpDisabled,
    TopOfRange,
    NatAlignedFourByte,
    NatAlignedPwrOfTwo,
}

pub enum PmpRegionState{
    Created,
    CreationFailed,
    Active,
    Disabled,
}

pub struct PmpRegion {
    pub start: PmpRegionStartAddr,
    pub sz: PmpRegionSize,
    pub amm: PmpRegionAddrMatchMode,
    pub r: bool, // Read
    pub w: bool, // Write
    pub x: bool, // Execute
    pub l: bool, // Lock
    pub status: PmpRegionState,
}

impl PmpRegion {
    pub fn pmp_configure_region(&self) {
        match &(*self).amm {

            PmpDisabled => {
                unsafe {
                    asm!("nop");
                }
            }
            TopOfRange => {
                unsafe {
                    asm!("nop");
                }
            }
            NatAlignedFourByte => {
                unsafe {
                    asm!("nop");
                }
            }
            NatAlignedPwrOfTwo => {
                unsafe {
                    asm!("nop");
                }
            }
        }
    }
}