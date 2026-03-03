use critical_section::{set_impl, Impl, RacyCell, RawRestoreState};

use crate::interrupt;
use crate::register::primask;

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    // #[inline(always)]
    // unsafe fn acquire() -> RawRestoreState {
    //     // Backup previous state of PRIMASK register. We access the entire register directly as a
    //     // u32 instead of using the primask::read() function to minimize the number of processor
    //     // cycles during which interrupts are disabled.
    //     let restore_state = primask::read_raw();
    //     // NOTE: Fence guarantees are provided by interrupt::disable(), which performs a `compiler_fence(SeqCst)`.
    //     interrupt::disable();
    //     restore_state
    // }

    // #[inline(always)]
    // unsafe fn release(restore_state: RawRestoreState) {
    //     // NOTE: Fence guarantees are provided by primask::write_raw(), which performs a `compiler_fence(SeqCst)`.
    //     primask::write_raw(restore_state);
    // }
    /// Should return the "enable" and "disable" restore states.
    ///
    /// By itself free of side effects
    unsafe fn get_states() -> (u32, u32) {
        (0x00, 0x01)
    }
    // /// Should return the current state, later to be restored by `set_state`.
    // unsafe fn get_state(store: &RacyCell<RawRestoreState>) -> bool {
    //     // We access the entire register directly as a u32 instead of using the primask::read() function to minimize the number of processor cycles during which interrupts are disabled.
    //     unsafe { *store.get() }
    //     // primask::read_raw() == 0
    // }
    // /// Should set the current state to the raw_restore_state.
    // unsafe fn set_state(raw_restore_state: RawRestoreState, store: &RacyCell<RawRestoreState>) {
    //     // NOTE: Fence guarantees are provided by primask::write_raw(), which performs a `compiler_fence(SeqCst)`.
    //     if unsafe { *store.get() } != raw_restore_state {
    //         // We are restoring the "enable" state, so we can just write 0 to PRIMASK to enable interrupts.
    //         unsafe { *store.get_mut() = raw_restore_state };
    //         primask::write_raw(if raw_restore_state { 0 } else { 1 });
    //     }
    // }

    /// Should return the current state, later to be restored by `set_state`.
    unsafe fn get_state() -> u32 {
        // We access the entire register directly as a u32 instead of using the primask::read() function to minimize the number of processor cycles during which interrupts are disabled.

        primask::read_raw()
    }
    /// Should set the current state to the raw_restore_state.
    unsafe fn set_state(raw_restore_state: RawRestoreState) {
        // NOTE: Fence guarantees are provided by primask::write_raw(), which performs a `compiler_fence(SeqCst)`.
        primask::write_raw(raw_restore_state);
    }
}
