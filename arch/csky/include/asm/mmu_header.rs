/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: atomic64_t and cpumask_t are supplied externally. */
#[repr(C)]
pub struct mm_context_t {
    pub asid: atomic64_t,
    pub vdso: *mut core::ffi::c_void,
    pub icache_stale_mask: cpumask_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
