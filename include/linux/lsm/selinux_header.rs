/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Linux Security Module interface to other subsystems.
 * SELinux presents a single u32 value which is known as a secid.
 */

// Dependency corresponding to <linux/types.h>.

#[repr(C)]
pub struct lsm_prop_selinux {
    // Preserved from CONFIG_SECURITY_SELINUX.
    #[cfg(feature = "CONFIG_SECURITY_SELINUX")]
    pub secid: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
