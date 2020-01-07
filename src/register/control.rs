//! Control register

/// Control register
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug)]
pub struct Control {
    bits: u32,
}

impl Control {
    /// Creates a `Control` value from raw bits.
    #[inline]
    pub fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(self) -> u32 {
        self.bits
    }

    /// Thread mode privilege level
    #[inline]
    pub fn npriv(self) -> Npriv {
        if self.bits & (1 << 0) == (1 << 0) {
            Npriv::Unprivileged
        } else {
            Npriv::Privileged
        }
    }

    /// Sets the thread mode privilege level value (nPRIV).
    #[inline]
    pub fn set_npriv(&mut self, npriv: Npriv) {
        let mask = 1 << 0;
        match npriv {
            Npriv::Unprivileged => self.bits |= mask,
            Npriv::Privileged => self.bits &= !mask,
        }
    }

    /// Currently active stack pointer
    #[inline]
    pub fn spsel(self) -> Spsel {
        if self.bits & (1 << 1) == (1 << 1) {
            Spsel::Psp
        } else {
            Spsel::Msp
        }
    }

    /// Sets the SPSEL value.
    #[inline]
    pub fn set_spsel(&mut self, spsel: Spsel) {
        let mask = 1 << 1;
        match spsel {
            Spsel::Psp => self.bits |= mask,
            Spsel::Msp => self.bits &= !mask,
        }
    }

    /// Whether context floating-point is currently active
    #[inline]
    pub fn fpca(self) -> Fpca {
        if self.bits & (1 << 2) == (1 << 2) {
            Fpca::Active
        } else {
            Fpca::NotActive
        }
    }

    /// Sets the FPCA value.
    #[inline]
    pub fn set_fpca(&mut self, fpca: Fpca) {
        let mask = 1 << 2;
        match fpca {
            Fpca::Active => self.bits |= mask,
            Fpca::NotActive => self.bits &= !mask,
        }
    }
}

/// Thread mode privilege level
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Npriv {
    /// Privileged
    Privileged,
    /// Unprivileged
    Unprivileged,
}

impl Npriv {
    /// Is in privileged thread mode?
    #[inline]
    pub fn is_privileged(self) -> bool {
        self == Npriv::Privileged
    }

    /// Is in unprivileged thread mode?
    #[inline]
    pub fn is_unprivileged(self) -> bool {
        self == Npriv::Unprivileged
    }
}

/// Currently active stack pointer
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spsel {
    /// MSP is the current stack pointer
    Msp,
    /// PSP is the current stack pointer
    Psp,
}

impl Spsel {
    /// Is MSP the current stack pointer?
    #[inline]
    pub fn is_msp(self) -> bool {
        self == Spsel::Msp
    }

    /// Is PSP the current stack pointer?
    #[inline]
    pub fn is_psp(self) -> bool {
        self == Spsel::Psp
    }
}

/// Whether context floating-point is currently active
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Fpca {
    /// Floating-point context active.
    Active,
    /// No floating-point context active
    NotActive,
}

impl Fpca {
    /// Is a floating-point context active?
    #[inline]
    pub fn is_active(self) -> bool {
        self == Fpca::Active
    }

    /// Is a floating-point context not active?
    #[inline]
    pub fn is_not_active(self) -> bool {
        self == Fpca::NotActive
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> Control {
    match () {
        #[cfg(cortex_m)]
        () => {
            let r = match () {
                #[cfg(feature = "inline-asm")]
                () => {
                    let r: u32;
                    unsafe { asm!("mrs $0, CONTROL" : "=r"(r) ::: "volatile") }
                    r
                }

                #[cfg(not(feature = "inline-asm"))]
                () => unsafe {
                    extern "C" {
                        fn __control_r() -> u32;
                    }

                    __control_r()
                },
            };

            Control { bits: r }
        }

        #[cfg(all(not(cortex_m), feature = "klee-analysis"))]
        () => {
            let mut r: u32 = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            klee_make_symbolic!(&mut r, "CONTROL");
            Control { bits: r }
        }

        #[cfg(all(not(cortex_m), not(feature = "klee-analysis")))]
        () => unimplemented!(),
    }
}

/// Writes to the CPU register.
#[inline]
pub unsafe fn write(_control: Control) {
    match () {
        #[cfg(cortex_m)]
        () => match () {
            #[cfg(feature = "inline-asm")]
            () => {
                let control = _control.bits();
                asm!("msr CONTROL, $0" :: "r"(control) : "memory" : "volatile");
            }

            #[cfg(not(feature = "inline-asm"))]
            () => {
                extern "C" {
                    fn __control_w(bits: u32);
                }

                __control_w(_control.bits());
            }
        },

        #[cfg(not(cortex_m))]
        () => unimplemented!(),
    }
}
