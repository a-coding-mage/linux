// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Audit helpers
 *
 * Copyright © 2023-2025 Microsoft Corporation
 */

// C dependencies: <linux/types.h>, "access.h".

#[repr(C)]
pub struct landlock_hierarchy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_request {
    _private: [u8; 0],
}

// From "access.h".
pub type access_mask_t = core::ffi::c_uint;

// C conditional intent: when CONFIG_AUDIT is enabled, these are external
// declarations; otherwise the header provides empty static inline functions.
#[cfg(CONFIG_AUDIT)]
unsafe extern "C" {
    pub fn landlock_audit_denial(
        request: *const landlock_request,
        youngest_denied: *mut landlock_hierarchy,
        missing: access_mask_t,
        logged: bool,
    );

    pub fn landlock_audit_free_domain(hierarchy: *const landlock_hierarchy);
}

#[cfg(not(CONFIG_AUDIT))]
#[inline]
pub unsafe fn landlock_audit_denial(
    request: *const landlock_request,
    youngest_denied: *mut landlock_hierarchy,
    missing: access_mask_t,
    logged: bool,
) {
}

#[cfg(not(CONFIG_AUDIT))]
#[inline]
pub unsafe fn landlock_audit_free_domain(hierarchy: *const landlock_hierarchy) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
