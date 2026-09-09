/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Kernel Electric-Fence (KFENCE). For more info please see
 * Documentation/dev-tools/kfence.rst.
 *
 * Copyright (C) 2020, Google LLC.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub extern "C" {
    pub static mut kfence_enabled: bool;
    pub static mut kfence_freelist_lock: raw_spinlock_t;
    pub static mut kfence_metadata: *mut kfence_metadata;
}

/*
 * Get the canary byte pattern for @addr. Use a pattern that varies based on the
 * lower 3 bits of the address, to detect memory corruptions with higher
 * probability, where similar constants are used.
 */
#[inline]
pub const fn KFENCE_CANARY_PATTERN_U8(addr: usize) -> u8 {
    0xaa_u8 ^ ((addr as u8) & 0x7)
}

/*
 * Define a continuous 8-byte canary starting from a multiple of 8. The canary
 * of each byte is only related to the lowest three bits of its address, so the
 * canary of every 8 bytes is the same. 64-bit memory can be filled and checked
 * at a time instead of byte by byte to improve performance.
 */
pub const KFENCE_CANARY_PATTERN_U64: u64 =
    0xaaaaaaaaaaaaaaaa_u64 ^ (le64_to_cpu(0x0706050403020100_u64));

/* Maximum stack depth for reports. */
pub const KFENCE_STACK_DEPTH: usize = 64;

/* KFENCE object states. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfence_object_state {
    KFENCE_OBJECT_UNUSED,
    KFENCE_OBJECT_ALLOCATED,
    KFENCE_OBJECT_RCU_FREEING,
    KFENCE_OBJECT_FREED,
}

/* Alloc/free tracking information. */
#[repr(C)]
pub struct kfence_track {
    pub pid: pid_t,
    pub cpu: ::core::ffi::c_int,
    pub ts_nsec: u64,
    pub num_stack_entries: ::core::ffi::c_int,
    pub stack_entries: [usize; KFENCE_STACK_DEPTH],
}

/* KFENCE metadata per guarded allocation. */
#[repr(C)]
pub struct kfence_metadata {
    pub list: list_head, // __guarded_by(&kfence_freelist_lock): Freelist node.
    pub rcu_head: rcu_head, // For delayed freeing.

    /* Lock protecting the data below. */
    pub lock: raw_spinlock_t,

    /* The current state of the object; see above. */
    pub state: kfence_object_state,

    /* Allocated object address; cannot be calculated from size. */
    pub addr: usize,

    /* The size of the original allocation. */
    pub size: usize,

    /* The kmem_cache cache of the last allocation. */
    pub cache: *mut kmem_cache,

    /* In case of an invalid access, the page that was unprotected. */
    pub unprotected_page: usize, // __guarded_by(&lock)

    /* Allocation and free stack information. */
    pub alloc_track: kfence_track, // __guarded_by(&lock)
    pub free_track: kfence_track, // __guarded_by(&lock)
    /* For updating alloc_covered on frees. */
    pub alloc_stack_hash: u32, // __guarded_by(&lock)
}

pub const KFENCE_METADATA_SIZE: usize =
    PAGE_ALIGN(core::mem::size_of::<kfence_metadata>() * CONFIG_KFENCE_NUM_OBJECTS);

#[inline]
pub unsafe fn addr_to_metadata(addr: usize) -> *mut kfence_metadata {
    let index: isize;

    /* The checks do not affect performance; only called from slow-paths. */
    if !is_kfence_address(addr as *mut core::ffi::c_void) {
        return core::ptr::null_mut();
    }

    /*
     * May be an invalid index if called with an address at the edge of
     * __kfence_pool, in which case we would report an "invalid access"
     * error.
     */
    index = ((addr - (__kfence_pool as usize)) / (PAGE_SIZE * 2)) as isize - 1;
    if index < 0 || index >= CONFIG_KFENCE_NUM_OBJECTS as isize {
        return core::ptr::null_mut();
    }

    kfence_metadata.add(index as usize)
}

/* KFENCE error types for report generation. */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfence_error_type {
    KFENCE_ERROR_OOB,
    KFENCE_ERROR_UAF,
    KFENCE_ERROR_CORRUPTION,
    KFENCE_ERROR_INVALID,
    KFENCE_ERROR_INVALID_FREE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfence_fault {
    KFENCE_FAULT_NONE,
    KFENCE_FAULT_REPORT,
    KFENCE_FAULT_OOPS,
    KFENCE_FAULT_PANIC,
}

pub extern "C" {
    pub fn kfence_report_error(
        address: usize,
        is_write: bool,
        regs: *mut pt_regs,
        meta: *const kfence_metadata,
        type_: kfence_error_type,
    ) -> kfence_fault;

    pub fn kfence_handle_fault(fault: kfence_fault);

    pub fn kfence_print_object(seq: *mut seq_file, meta: *const kfence_metadata);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
