/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/cpumask.h, linux/errno.h, and asm/asm-extable.h.

#[repr(C)]
pub struct mm_context_t {
    pub lock: spinlock_t,
    pub cpu_attach_mask: cpumask_t,
    pub flush_count: atomic_t,
    pub flush_mm: core::ffi::c_uint,
    pub gmap_list: list_head,
    pub gmap_asce: core::ffi::c_ulong,
    pub asce: core::ffi::c_ulong,
    pub asce_limit: core::ffi::c_ulong,
    pub vdso_base: core::ffi::c_ulong,
    /* The mmu context belongs to a secure guest. */
    pub protected_count: atomic_t,
    /*
     * The mmu context allows COW-sharing of memory pages (KSM, zeropage).
     * Note that COW-sharing during fork() is currently always allowed.
     */
    pub allow_cow_sharing: u32,
}

#[macro_export]
macro_rules! INIT_MM_CONTEXT {
    ($name:expr) => {
        .context.lock = __SPIN_LOCK_UNLOCKED!($name.context.lock),
        .context.gmap_list = LIST_HEAD_INIT!($name.context.gmap_list),
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
