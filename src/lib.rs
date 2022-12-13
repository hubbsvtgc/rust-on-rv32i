
//! # Hifive1-RevB board Library
//!
//! The Hifive1-RevB board Library provides the essential
//! interface for using peripherals in board.

#![no_std]
#[cfg(feature = "gpio")]
pub mod gpio;

#[cfg(feature = "uart")]
pub mod uart;
