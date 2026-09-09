/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/arm-cci.h
 *
 * Copyright (C) 2015 ARM Ltd.
 */

/* __ASM_ARM_CCI_H */

/* CONFIG_MCPM */
#[cfg(CONFIG_MCPM)]
unsafe extern "C" {
    fn mcpm_is_available() -> bool;
}

/*
 * We don't have a reliable way of detecting whether,
 * if we have access to secure-only registers, unless
 * mcpm is registered.
 */
#[cfg(CONFIG_MCPM)]
pub fn platform_has_secure_cci_access() -> bool {
    unsafe { mcpm_is_available() }
}

#[cfg(not(CONFIG_MCPM))]
pub fn platform_has_secure_cci_access() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
