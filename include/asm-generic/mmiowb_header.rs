/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Generic implementation of mmiowb() tracking for spinlocks.
 *
 * If the architecture does not ensure ordering of I/O writes across
 * spinlocked sections, the architecture-specific mmiowb implementation and
 * state accessor must be supplied by the surrounding translation unit.
 */

/* Corresponds to CONFIG_MMIOWB.  Enable this module when that build-time
 * configuration option is selected. */
#[cfg(feature = "CONFIG_MMIOWB")]
mod mmio_wb {
    use super::mmiowb_state;

    extern "C" {
        static mut __mmiowb_state: mmiowb_state;
        fn mmiowb();
    }

    #[inline(always)]
    unsafe fn state() -> *mut mmiowb_state {
        // Corresponds to raw_cpu_ptr(&__mmiowb_state); the per-CPU accessor is
        // supplied by the surrounding architecture-specific translation.
        &raw mut __mmiowb_state
    }

    #[inline(always)]
    pub unsafe fn mmiowb_set_pending() {
        let ms = &mut *state();

        if ms.nesting_count != 0 {
            ms.mmiowb_pending = ms.nesting_count;
        }
    }

    #[inline(always)]
    pub unsafe fn mmiowb_spin_lock() {
        let ms = &mut *state();
        ms.nesting_count = ms.nesting_count.wrapping_add(1);
    }

    #[inline(always)]
    pub unsafe fn mmiowb_spin_unlock() {
        let ms = &mut *state();

        if ms.mmiowb_pending != 0 {
            ms.mmiowb_pending = 0;
            mmiowb();
        }

        ms.nesting_count = ms.nesting_count.wrapping_sub(1);
    }
}

#[cfg(not(feature = "CONFIG_MMIOWB"))]
#[inline(always)]
pub fn mmiowb_set_pending() {}

#[cfg(not(feature = "CONFIG_MMIOWB"))]
#[inline(always)]
pub fn mmiowb_spin_lock() {}

#[cfg(not(feature = "CONFIG_MMIOWB"))]
#[inline(always)]
pub fn mmiowb_spin_unlock() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
