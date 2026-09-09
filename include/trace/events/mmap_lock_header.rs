/* SPDX-License-Identifier: GPL-2.0 */

// Translation of TRACE_SYSTEM mmap_lock.
// C header dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_void;

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn cgroup_id_from_mm(mm: *mut mm_struct) -> u64;
}

#[repr(C)]
pub struct MmapLockEntry {
    pub mm: *mut mm_struct,
    pub memcg_id: u64,
    pub write: bool,
}

#[repr(C)]
pub struct MmapLockAcquireReturnedEntry {
    pub mm: *mut mm_struct,
    pub memcg_id: u64,
    pub write: bool,
    pub success: bool,
}

// TP_fast_assign / TP_printk for the mmap_lock event class.
#[inline]
pub unsafe fn mmap_lock_entry_assign(
    entry: *mut MmapLockEntry,
    mm: *mut mm_struct,
    write: bool,
) {
    (*entry).mm = mm;
    (*entry).memcg_id = cgroup_id_from_mm(mm);
    (*entry).write = write;
}

#[inline]
pub unsafe fn mmap_lock_entry_print_values(
    entry: *const MmapLockEntry,
) -> (*mut c_void, u64, &'static str) {
    (
        (*entry).mm.cast::<c_void>(),
        (*entry).memcg_id,
        if (*entry).write { "true" } else { "false" },
    )
}

// DEFINE_MMAP_LOCK_EVENT(mmap_lock_start_locking)
// DEFINE_MMAP_LOCK_EVENT(mmap_lock_released)
pub const MMAP_LOCK_START_LOCKING: &str = "mmap_lock_start_locking";
pub const MMAP_LOCK_RELEASED: &str = "mmap_lock_released";

// TP_fast_assign / TP_printk for mmap_lock_acquire_returned.
#[inline]
pub unsafe fn mmap_lock_acquire_returned_entry_assign(
    entry: *mut MmapLockAcquireReturnedEntry,
    mm: *mut mm_struct,
    write: bool,
    success: bool,
) {
    (*entry).mm = mm;
    (*entry).memcg_id = cgroup_id_from_mm(mm);
    (*entry).write = write;
    (*entry).success = success;
}

#[inline]
pub unsafe fn mmap_lock_acquire_returned_entry_print_values(
    entry: *const MmapLockAcquireReturnedEntry,
) -> (*mut c_void, u64, &'static str, &'static str) {
    (
        (*entry).mm.cast::<c_void>(),
        (*entry).memcg_id,
        if (*entry).write { "true" } else { "false" },
        if (*entry).success { "true" } else { "false" },
    )
}

pub const MMAP_LOCK_ACQUIRE_RETURNED: &str = "mmap_lock_acquire_returned";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
