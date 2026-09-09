/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Linux Security Module interface to other subsystems.
 * AppArmor presents single pointer to an aa_label structure.
 */

// Forward declaration of the externally defined AppArmor label structure.
#[repr(C)]
pub struct aa_label {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_prop_apparmor {
    // Preserved from CONFIG_SECURITY_APPARMOR. This field is present only
    // when AppArmor security support is enabled by the build configuration.
    #[cfg(CONFIG_SECURITY_APPARMOR)]
    pub label: *mut aa_label,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
