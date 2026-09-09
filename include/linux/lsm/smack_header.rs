/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Linux Security Module interface to other subsystems.
 * Smack presents a pointer into the global Smack label list.
 */

// Opaque declaration corresponding to the C forward declaration.
#[repr(C)]
pub struct smack_known {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_prop_smack {
    // Present only when CONFIG_SECURITY_SMACK is enabled in the C build.
    #[cfg(CONFIG_SECURITY_SMACK)]
    pub skp: *mut smack_known,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
