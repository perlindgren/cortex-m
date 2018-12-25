//! Link register

/// Reads the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r: u32;
            unsafe { asm!("mov $0,R14" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {
            let mut r: u32 = unsafe { core::mem::uninitialized() };
            ksymbol!(&mut r, "LR");
            r
        }

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
///
/// **NOTE** This function is available if `cortex-m` is built with the `"inline-asm"` feature.
#[inline]
pub unsafe fn write(_bits: u32) {
    match () {
        #[cfg(cortex_m)]
        () => asm!("mov R14,$0" :: "r"(_bits) :: "volatile"),

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
