//! Flash Patch and Breakpoint unit
//!
//! *NOTE* Available only on ARMv7-M (`thumbv7*m-none-eabi*`)

#[cfg(feature = "klee-debug")]
extern crate array_debug;

#[cfg(feature = "klee-debug")]
use self::array_debug::ArrayDebug;

use volatile_register::{RO, RW, WO};

/// Register block
#[repr(C)]
#[cfg_attr(feature = "klee-debug", derive(Debug))]
pub struct RegisterBlock {
    /// Control
    pub ctrl: RW<u32>,
    /// Remap
    pub remap: RW<u32>,
    /// Comparator
    #[cfg(feature = "klee-debug")]
    pub comp: ArrayDebug<[RW<u32>; 127], RW<u32>>,
    #[cfg(not(feature = "klee-debug"))]
    pub comp: [RW<u32>; 127],
    #[cfg(feature = "klee-debug")]
    reserved: ArrayDebug<[u32; 875], u32>,
    #[cfg(not(feature = "klee-debug"))]
    reserved: [u32; 875],
    /// Lock Access
    pub lar: WO<u32>,
    /// Lock Status
    pub lsr: RO<u32>,
}
