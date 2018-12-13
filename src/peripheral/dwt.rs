//! Data Watchpoint and Trace unit
#[cfg(feature = "klee-debug")]
use core::fmt::{self, Debug, UpperHex};
use peripheral::DWT;
use volatile_register::{RO, RW, WO};

/// Register block
#[repr(C)]
//#[cfg_attr(feature = "klee-debug", derive(Debug))]
pub struct RegisterBlock {
    /// Control
    pub ctrl: RW<u32>,
    /// Cycle Count
    pub cyccnt: RW<u32>,
    /// CPI Count
    pub cpicnt: RW<u32>,
    /// Exception Overhead Count
    pub exccnt: RW<u32>,
    /// Sleep Count
    pub sleepcnt: RW<u32>,
    /// LSU Count
    pub lsucnt: RW<u32>,
    /// Folded-instruction Count
    pub foldcnt: RW<u32>,
    /// Program Counter Sample
    pub pcsr: RO<u32>,
    /// Comparators
    pub c: [Comparator; 16],
    reserved: [u32; 932],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}

#[cfg(feature = "klee-debug")]
impl fmt::Debug for RegisterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RegisterBlock {{ ctrl: {:?}, ..}}", self.ctrl)
    }
    // /// Cycle Count
    // pub cyccnt: RW<u32>,
    // /// CPI Count
    // pub cpicnt: RW<u32>,
    // /// Exception Overhead Count
    // pub exccnt: RW<u32>,
    // /// Sleep Count
    // pub sleepcnt: RW<u32>,
    // /// LSU Count
    // pub lsucnt: RW<u32>,
    // /// Folded-instruction Count
    // pub foldcnt: RW<u32>,
    // /// Program Counter Sample
    // pub pcsr: RO<u32>,
    // /// Comparators
    // pub c: [Comparator; 16],
    // reserved: [u32; 932],
    // /// Lock Access
    // pub lar: WO<u32>,
    // /// Lock Status
    // pub lsr: RO<u32>,
    // }
}

/// Comparator
#[repr(C)]
#[cfg_attr(feature = "klee-debug", derive(Debug))]
pub struct Comparator {
    /// Comparator
    pub comp: RW<u32>,
    /// Comparator Mask
    pub mask: RW<u32>,
    /// Comparator Function
    pub function: RW<u32>,
    reserved: u32,
}

impl DWT {
    /// Enables the cycle counter
    pub fn enable_cycle_counter(&mut self) {
        unsafe { self.ctrl.modify(|r| r | 1) }
    }

    /// Returns the current clock cycle count
    pub fn get_cycle_count() -> u32 {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::ptr()).cyccnt.read() }
    }
}
