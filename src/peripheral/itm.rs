//! Instrumentation Trace Macrocell
//!
//! *NOTE* Available only on ARMv7-M (`thumbv7*m-none-eabi*`)
#[cfg(feature = "klee-debug")]
extern crate array_debug;

#[cfg(feature = "klee-debug")]
use self::array_debug::ArrayDebug;

use core::cell::UnsafeCell;
use core::ptr;

use volatile_register::{RO, RW, WO};

/// Register block
#[repr(C)]
#[cfg_attr(feature = "klee-debug", derive(Debug))]
pub struct RegisterBlock {
    /// Stimulus Port
    #[cfg(feature = "klee-debug")]
    pub stum: ArrayDebug<[Stim; 640], Stim>,
    #[cfg(not(feature = "klee-debug"))]
    pub stim: [Stim; 256],
    #[cfg(feature = "klee-debug")]
    reserved0: ArrayDebug<[u32; 640], u32>,
    #[cfg(not(feature = "klee-debug"))]
    reserved0: [u32; 640],
    /// Trace Enable
    pub ter: [RW<u32>; 8],
    reserved1: [u32; 8],
    /// Trace Privilege
    pub tpr: RW<u32>,
    reserved2: [u32; 15],
    /// Trace Control
    pub tcr: RW<u32>,
    #[cfg(feature = "klee-debug")]
    reserved3: ArrayDebug<[u32; 75], u32>,
    #[cfg(not(feature = "klee-debug"))]
    reserved3: [u32; 75],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}

/// Stimulus Port
#[cfg_attr(feature = "klee-debug", derive(Debug))]
pub struct Stim {
    register: UnsafeCell<u32>,
}

impl Stim {
    /// Writes an `u8` payload into the stimulus port
    pub fn write_u8(&mut self, value: u8) {
        unsafe { ptr::write_volatile(self.register.get() as *mut u8, value) }
    }

    /// Writes an `u16` payload into the stimulus port
    pub fn write_u16(&mut self, value: u16) {
        unsafe { ptr::write_volatile(self.register.get() as *mut u16, value) }
    }

    /// Writes an `u32` payload into the stimulus port
    pub fn write_u32(&mut self, value: u32) {
        unsafe { ptr::write_volatile(self.register.get(), value) }
    }

    /// Returns `true` if the stimulus port is ready to accept more data
    pub fn is_fifo_ready(&self) -> bool {
        unsafe { ptr::read_volatile(self.register.get()) == 1 }
    }
}
