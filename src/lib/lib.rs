
//! # Hifive1-RevB board Library
//!
//! The Hifive1-RevB board Library provides the essential
//! interface for using peripherals in board.

#![no_std]
#[cfg(feature = "dio")]
pub mod dio;

#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "fe310")]
pub mod fe310;