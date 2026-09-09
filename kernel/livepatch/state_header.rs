/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/livepatch.h>

/// Opaque declaration corresponding to `struct klp_patch`.
#[repr(C)]
pub struct klp_patch {
    _private: [u8; 0],
}

extern "C" {
    pub fn klp_is_patch_compatible(patch: *mut klp_patch) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
