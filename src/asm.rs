//! Miscellaneous assembly instructions

/// Puts the processor in Debug state. Debuggers can pick this up as a "breakpoint".
///
/// **NOTE** calling `bkpt` when the processor is not connected to a debugger will cause an
/// exception.
#[inline(always)]
pub fn bkpt() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("bkpt" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __bkpt();
            }

            __bkpt();
        },

        // no effect of breakpoint for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Additional breakpoint instructions
//
// this however does not currently compile
// .. crate attributes... stdsimd, const_fn, rustc_attrs
// #[cfg(all(cortex_m, feature = "inline-asm"))]
// #[inline(always)]
// #[rustc_args_required_const(0)]
// pub unsafe fn bkpt_nr(nr: i32) {
//     core::arch::arm::__breakpoint(nr);
//}
// Workaround solution

#[inline(always)]
pub fn bkpt_nr(_nr: u8) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => match _nr {
            0 => unsafe { asm!("bkpt #0" :::: "volatile") },
            1 => unsafe { asm!("bkpt #1" :::: "volatile") },
            2 => unsafe { asm!("bkpt #2" :::: "volatile") },
            3 => unsafe { asm!("bkpt #3" :::: "volatile") },
            4 => unsafe { asm!("bkpt #4" :::: "volatile") },
            5 => unsafe { asm!("bkpt #5" :::: "volatile") },
            6 => unsafe { asm!("bkpt #6" :::: "volatile") },
            7 => unsafe { asm!("bkpt #7" :::: "volatile") },
            8 => unsafe { asm!("bkpt #8" :::: "volatile") },
            9 => unsafe { asm!("bkpt #9" :::: "volatile") },
            10 => unsafe { asm!("bkpt #10" :::: "volatile") },
            11 => unsafe { asm!("bkpt #11" :::: "volatile") },
            12 => unsafe { asm!("bkpt #12" :::: "volatile") },
            13 => unsafe { asm!("bkpt #13" :::: "volatile") },
            14 => unsafe { asm!("bkpt #14" :::: "volatile") },
            15 => unsafe { asm!("bkpt #15" :::: "volatile") },
            16 => unsafe { asm!("bkpt #16" :::: "volatile") },
            17 => unsafe { asm!("bkpt #17" :::: "volatile") },
            18 => unsafe { asm!("bkpt #18" :::: "volatile") },
            19 => unsafe { asm!("bkpt #19" :::: "volatile") },
            20 => unsafe { asm!("bkpt #20" :::: "volatile") },
            21 => unsafe { asm!("bkpt #21" :::: "volatile") },
            22 => unsafe { asm!("bkpt #22" :::: "volatile") },
            23 => unsafe { asm!("bkpt #23" :::: "volatile") },
            24 => unsafe { asm!("bkpt #24" :::: "volatile") },
            25 => unsafe { asm!("bkpt #25" :::: "volatile") },
            26 => unsafe { asm!("bkpt #26" :::: "volatile") },
            27 => unsafe { asm!("bkpt #27" :::: "volatile") },
            28 => unsafe { asm!("bkpt #28" :::: "volatile") },
            29 => unsafe { asm!("bkpt #29" :::: "volatile") },
            30 => unsafe { asm!("bkpt #30" :::: "volatile") },
            31 => unsafe { asm!("bkpt #31" :::: "volatile") },
            32 => unsafe { asm!("bkpt #32" :::: "volatile") },
            _ => unimplemented!(),
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unimplemented!(),

        // no effect of breakpoint for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Blocks the program for *at least* `n` instruction cycles
///
/// This is implemented in assembly so its execution time is the same regardless of the optimization
/// level.
///
/// NOTE that the delay can take much longer if interrupts are serviced during its execution.
#[inline]
pub fn delay(_n: u32) {
    // NOTE(divide by 4) is easier to compute than `/ 3` is it's just a shift (`>> 2`).
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe {
            asm!("1:
                  nop
                  subs $0, $$1
                  bne.n 1b"
                 : "+r"(_n / 4 + 1)
                 :
                 :
                 : "volatile");
        },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __delay(n: u32);
            }

            __delay(_n / 4 + 1);
        },

        // no effect of delay for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// A no-operation. Useful to prevent delay loops from being optimized away.
#[inline]
pub fn nop() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("nop" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __nop();
            }

            __nop()
        },

        // no effect of `nop` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Wait For Event
#[inline]
pub fn wfe() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("wfe" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __wfe();
            }

            __wfe()
        },

        // no effect of `wfe` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Wait For Interrupt
#[inline]
pub fn wfi() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("wfi" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __wfi();
            }

            __wfi()
        },

        // no effect of `wfi` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Send Event
#[inline]
pub fn sev() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("sev" :::: "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __sev();
            }

            __sev()
        },

        // no effect of `sev` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
pub fn isb() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("isb 0xF" ::: "memory" : "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __isb();
            }

            __isb()
            // XXX do we need a explicit compiler barrier here?
        },

        // no effect of `isb` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline]
pub fn dsb() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("dsb 0xF" ::: "memory" : "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __dsb();
            }

            __dsb()
            // XXX do we need a explicit compiler barrier here?
        },

        // no effect of `bsb` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
pub fn dmb() {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => unsafe { asm!("dmb 0xF" ::: "memory" : "volatile") },

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __dmb();
            }

            __dmb()
            // XXX do we need a explicit compiler barrier here?
        },

        // no effect of `dmb` for `klee-analysis`
        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}
