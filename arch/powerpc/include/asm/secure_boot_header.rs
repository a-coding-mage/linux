/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Secure boot definitions
 *
 * Copyright (C) 2019 IBM Corporation
 * Author: Nayna Jain
 */

// The C header guard is omitted; Rust modules provide equivalent inclusion
// protection.

// Build-time condition preserved from CONFIG_PPC_SECURE_BOOT.
#[cfg(CONFIG_PPC_SECURE_BOOT)]
extern "C" {
    pub fn is_ppc_secureboot_enabled() -> bool;
    pub fn is_ppc_trustedboot_enabled() -> bool;
}

// Corresponds to the #else branch when CONFIG_PPC_SECURE_BOOT is not set.
#[cfg(not(CONFIG_PPC_SECURE_BOOT))]
#[inline]
pub fn is_ppc_secureboot_enabled() -> bool {
    false
}

#[cfg(not(CONFIG_PPC_SECURE_BOOT))]
#[inline]
pub fn is_ppc_trustedboot_enabled() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
