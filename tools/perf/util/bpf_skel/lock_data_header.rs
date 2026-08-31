// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Data structures shared between BPF and tools. */

#[repr(C)]
pub struct owner_tracing_data {
    pub pid: u32,       // Who has the lock.
    pub count: u32,     // How many waiters for this lock.
    pub timestamp: u64, // The time while the owner acquires lock and contention is going on.
    pub stack_id: i32,  // Identifier for `owner_stat`, which stores as value in `owner_stacks`
}

#[repr(C)]
pub struct tstamp_data {
    pub timestamp: u64,
    pub lock: u64,
    pub cgroup_id: u64,
    pub flags: u32,
    pub stack_id: i32,
}

#[repr(C)]
pub struct contention_key {
    pub stack_id: i32,
    pub pid: u32,
    pub lock_addr_or_cgroup: u64,
}

pub const TASK_COMM_LEN: usize = 16;

#[repr(C)]
pub struct contention_task_data {
    pub comm: [core::ffi::c_char; TASK_COMM_LEN],
}

/* default buffer size */
pub const MAX_ENTRIES: u32 = 16384;

/*
 * Upper bits of the flags in the contention_data are used to identify
 * some well-known locks which do not have symbols (non-global locks).
 */
pub const LCD_F_MMAP_LOCK: u32 = 1u32 << 31;
pub const LCD_F_SIGHAND_LOCK: u32 = 1u32 << 30;

pub const LCB_F_SLAB_ID_SHIFT: u32 = 16;
pub const LCB_F_SLAB_ID_START: u32 = 1u32 << 16;
pub const LCB_F_SLAB_ID_END: u32 = 1u32 << 26;
pub const LCB_F_SLAB_ID_MASK: u32 = 0x03FF0000u32;

pub const LCB_F_TYPE_MAX: u32 = 1u32 << 7;
pub const LCB_F_TYPE_MASK: u32 = 0x0000007Fu32;

pub const SLAB_NAME_MAX: usize = 28;

#[repr(C)]
pub struct contention_data {
    pub total_time: u64,
    pub min_time: u64,
    pub max_time: u64,
    pub count: u32,
    pub flags: u32,
}

#[repr(C)]
pub enum lock_aggr_mode {
    LOCK_AGGR_ADDR = 0,
    LOCK_AGGR_TASK,
    LOCK_AGGR_CALLER,
    LOCK_AGGR_CGROUP,
}

#[repr(C)]
pub enum lock_class_sym {
    LOCK_CLASS_NONE,
    LOCK_CLASS_RQLOCK,
    LOCK_CLASS_ZONE_LOCK,
}

#[repr(C)]
pub struct slab_cache_data {
    pub id: u32,
    pub name: [core::ffi::c_char; SLAB_NAME_MAX],
}
