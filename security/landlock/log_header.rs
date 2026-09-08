/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Log helpers
 *
 * Copyright © 2023-2025 Microsoft Corporation
 * Copyright © 2026 Cloudflare, Inc.
 */

/* C header dependencies:
 * #include <linux/lsm_audit.h>
 * #include "access.h"
 */

#[repr(C)]
pub struct landlock_cred_security {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct landlock_hierarchy {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum landlock_request_type {
    LANDLOCK_REQUEST_PTRACE = 1,
    LANDLOCK_REQUEST_FS_CHANGE_TOPOLOGY = 2,
    LANDLOCK_REQUEST_FS_ACCESS = 3,
    LANDLOCK_REQUEST_NET_ACCESS = 4,
    LANDLOCK_REQUEST_SCOPE_ABSTRACT_UNIX_SOCKET = 5,
    LANDLOCK_REQUEST_SCOPE_SIGNAL = 6,
}

/*
 * We should be careful to only use a variable of this type for
 * landlock_log_denial().  This way, the compiler can remove it entirely if
 * CONFIG_SECURITY_LANDLOCK_LOG is not set.
 */
#[repr(C)]
pub struct landlock_request {
    /* Mandatory fields. */
    pub type_: landlock_request_type,
    pub audit: common_audit_data,

    /**
     * layer_plus_one: First layer level that denies the request + 1.  The
     * extra one is useful to detect uninitialized field.
     */
    pub layer_plus_one: usize,

    /* Required field for configurable access control. */
    pub access: access_mask_t,

    /* Required fields for requests with layer masks. */
    pub layer_masks: *const layer_masks,

    /* Required fields for requests with deny masks. */
    pub all_existing_optional_access: access_mask_t,
    pub deny_masks: deny_masks_t,
    pub quiet_optional_accesses: optional_access_t,

    /*
     * Other-party domain ID for a relational (scope/ptrace) denial, or 0 if
     * that party is unsandboxed.  An ID, not a pointer: the other task can
     * replace its credential and free the domain it referenced.  Trace path
     * only; audit ignores it.
     */
    pub other_domain_id: u64,
}

/* CONFIG_SECURITY_LANDLOCK_LOG */
#[cfg(CONFIG_SECURITY_LANDLOCK_LOG)]
unsafe extern "C" {
    pub fn landlock_log_free_domain(hierarchy: *const landlock_hierarchy);

    pub fn landlock_log_denial(
        subject: *const landlock_cred_security,
        request: *const landlock_request,
    );
}

/* !CONFIG_SECURITY_LANDLOCK_LOG */
#[cfg(not(CONFIG_SECURITY_LANDLOCK_LOG))]
#[inline]
pub unsafe fn landlock_log_free_domain(hierarchy: *const landlock_hierarchy) {
    let _ = hierarchy;
}

#[cfg(not(CONFIG_SECURITY_LANDLOCK_LOG))]
#[inline]
pub unsafe fn landlock_log_denial(
    subject: *const landlock_cred_security,
    request: *const landlock_request,
) {
    let _ = subject;
    let _ = request;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
