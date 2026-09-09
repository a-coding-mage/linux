/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The Kernel Concurrency Sanitizer (KCSAN) infrastructure. Public interface and
 * data structures to set up runtime. See kcsan-checks.h for explicit checks and
 * modifiers. For more info please see Documentation/dev-tools/kcsan.rst.
 *
 * Copyright (C) 2019, Google LLC.
 */

/* Translated from the C header. CONFIG_KCSAN is a build-time configuration. */
#[cfg(feature = "CONFIG_KCSAN")]
#[repr(C)]
pub struct kcsan_ctx {
    pub disable_count: i32, /* disable counter */
    pub disable_scoped: i32, /* disable scoped access counter */
    pub atomic_next: i32, /* number of following atomic ops */

    /*
     * We distinguish between nestable atomic regions and flat atomic regions.
     * Both are tracked independently.
     */
    pub atomic_nest_count: i32,
    pub in_flat_atomic: bool,

    /* Access mask for all accesses if non-zero. */
    pub access_mask: core::ffi::c_ulong,

    /* List of scoped accesses; likely to be empty. */
    pub scoped_accesses: list_head,

    #[cfg(feature = "CONFIG_KCSAN_WEAK_MEMORY")]
    /* Scoped access for modeling access reordering to detect missing memory barriers. */
    pub reorder_access: kcsan_scoped_access,
}

/**
 * kcsan_init - initialize KCSAN runtime
 */
#[cfg(feature = "CONFIG_KCSAN")]
unsafe extern "C" {
    pub fn kcsan_init();
}

#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline]
pub const fn kcsan_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
