/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Tracepoint helpers
 *
 * Copyright © 2025 Microsoft Corporation
 * Copyright © 2026 Cloudflare, Inc.
 */

/* C dependency: #include "access.h" supplies access_mask_t. */

#[repr(C)]
pub struct landlock_hierarchy {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct landlock_request {
    _unused: [u8; 0],
}

/* Original C condition: #ifdef CONFIG_TRACEPOINTS */
#[cfg(CONFIG_TRACEPOINTS)]
unsafe extern "C" {
    pub fn landlock_trace_free_domain(hierarchy: *const landlock_hierarchy);

    pub fn landlock_trace_denial(
        request: *const landlock_request,
        youngest_denied: *const landlock_hierarchy,
        missing: access_mask_t,
        same_exec: bool,
        logged: bool,
    );
}

/* Original C fallback: #else CONFIG_TRACEPOINTS */
#[cfg(not(CONFIG_TRACEPOINTS))]
#[inline]
pub unsafe fn landlock_trace_free_domain(hierarchy: *const landlock_hierarchy) {
    let _ = hierarchy;
}

#[cfg(not(CONFIG_TRACEPOINTS))]
#[inline]
pub unsafe fn landlock_trace_denial(
    request: *const landlock_request,
    youngest_denied: *const landlock_hierarchy,
    missing: access_mask_t,
    same_exec: bool,
    logged: bool,
) {
    let _ = request;
    let _ = youngest_denied;
    let _ = missing;
    let _ = same_exec;
    let _ = logged;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
