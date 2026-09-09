/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2012 ARM Limited
 */

// Header guard: __ASM_ARM_PSCI_H

unsafe extern "C" {
    pub static psci_smp_ops: smp_operations;
}

// Corresponds to CONFIG_SMP && CONFIG_ARM_PSCI.
#[cfg(all(feature = "CONFIG_SMP", feature = "CONFIG_ARM_PSCI"))]
unsafe extern "C" {
    pub fn psci_smp_available() -> bool;
}

#[cfg(not(all(feature = "CONFIG_SMP", feature = "CONFIG_ARM_PSCI")))]
#[inline]
pub const fn psci_smp_available() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
