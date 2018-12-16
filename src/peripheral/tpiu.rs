//! Trace Port Interface Unit;
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
    /// Supported Parallel Port Sizes
    pub sspsr: RO<u32>,

    /// Current Parallel Port Size
    pub cspsr: RW<u32>,
    reserved0: [u32; 2],

    /// Asynchronous Clock Prescaler
    pub acpr: RW<u32>,

    #[cfg(feature = "klee-debug")]
    reserved1: ArrayDebug<[u32; 55], u32>,
    #[cfg(not(feature = "klee-debug"))]
    reserved1: [u32; 55],

    /// Selected Pin Control
    pub sppr: RW<u32>,

    #[cfg(feature = "klee-debug")]
    reserved2: ArrayDebug<[u32; 132], u32>,
    #[cfg(not(feature = "klee-debug"))]
    reserved2: [u32; 132],

    /// Formatter and Flush Control
    pub ffcr: RW<u32>,

    #[cfg(feature = "klee-debug")]
    reserved3: ArrayDebug<[u32; 810], u32>,
    #[cfg(not(feature = "klee-debug"))]
    reserved3: [u32; 810],

    /// Lock Access
    pub lar: WO<u32>,

    /// Lock Status
    pub lsr: RO<u32>,

    reserved4: [u32; 4],

    /// TPIU Type
    pub _type: RO<u32>,
}
