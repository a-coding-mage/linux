/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2010 Ben Dooks <ben-linux@fluff.org>
 *
 * Support for wakeup mask interrupts on newer SoCs
 */

/// If no IRQ has yet been defined, but masking is still desired.
pub const NO_WAKEUP_IRQ: u32 = 0x9000_0000;

/// Wakeup mask information.
///
/// `irq` is the interrupt associated with this wakeup. `bit` is the bit
/// (1 << bit number) controlling this source.
#[repr(C)]
pub struct samsung_wakeup_mask {
    pub irq: u32,
    pub bit: u32,
}

/// Synchronize wakeup mask information for power management.
///
/// `reg` is the register that is used, `masks` is the list of masks to use,
/// and `nr_masks` is the number of entries pointed to by `masks`.
///
/// Synchronize the wakeup mask information at suspend time from the list of
/// interrupts and control bits in `masks`. This is done at suspend time as
/// overriding the relevant IRQ chips is harder and the register is only
/// required to be correct before entering sleep.
extern "C" {
    pub fn samsung_sync_wakemask(
        reg: *mut core::ffi::c_void,
        masks: *const samsung_wakeup_mask,
        nr_masks: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
