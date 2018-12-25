//! Main Stack Pointer

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => {
            let r;
            unsafe { asm!("mrs $0,MSP" : "=r"(r) ::: "volatile") }
            r
        }

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => unsafe {
            extern "C" {
                fn __msp_r() -> u32;
            }

            __msp_r()
        },

        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {
            let mut r: u32 = unsafe { core::mem::uninitialized() };
            ksymbol!(&mut r, "MSP");
            r
        }

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(_bits: u32) {
    match () {
        #[cfg(all(cortex_m, feature = "inline-asm"))]
        () => asm!("msr MSP,$0" :: "r"(_bits) :: "volatile"),

        #[cfg(all(cortex_m, not(feature = "inline-asm")))]
        () => {
            extern "C" {
                fn __msp_w(_: u32);
            }

            __msp_w(_bits);
        }

        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {}

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}
