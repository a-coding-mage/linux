/* SPDX-License-Identifier: GPL-2.0 */
/* Events for filesystem locks. Rust translation of trace/events/filelock.h. */

// C includes and tracepoint-generation macros are supplied by the surrounding
// kernel translation unit and are intentionally not implemented here.

#[allow(non_camel_case_types)]
pub type u64_t = u64;
pub type loff_t = i64;
pub type dev_t = u64;
pub type fl_owner_t = *mut core::ffi::c_void;

#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct file_lock;
#[repr(C)]
pub struct file_lock_context;
#[repr(C)]
pub struct file_lock_core;
#[repr(C)]
pub struct file_lease;

pub const SHOW_FL_FLAGS: &[(&str, u32)] = &[
    ("FL_POSIX", FL_POSIX), ("FL_FLOCK", FL_FLOCK),
    ("FL_DELEG", FL_DELEG), ("FL_ACCESS", FL_ACCESS),
    ("FL_EXISTS", FL_EXISTS), ("FL_LEASE", FL_LEASE),
    ("FL_CLOSE", FL_CLOSE), ("FL_SLEEP", FL_SLEEP),
    ("FL_DOWNGRADE_PENDING", FL_DOWNGRADE_PENDING),
    ("FL_UNLOCK_PENDING", FL_UNLOCK_PENDING), ("FL_OFDLCK", FL_OFDLCK),
    ("FL_RECLAIM", FL_RECLAIM), ("FL_IGN_DIR_CREATE", FL_IGN_DIR_CREATE),
    ("FL_IGN_DIR_DELETE", FL_IGN_DIR_DELETE),
    ("FL_IGN_DIR_RENAME", FL_IGN_DIR_RENAME),
];

pub const SHOW_FL_TYPE: &[(i32, &str)] = &[
    (F_RDLCK, "F_RDLCK"), (F_WRLCK, "F_WRLCK"), (F_UNLCK, "F_UNLCK"),
];

pub const SHOW_LEASE_BREAK_FLAGS: &[(&str, u32)] = &[
    ("LEASE", LEASE_BREAK_LEASE), ("DELEG", LEASE_BREAK_DELEG),
    ("LAYOUT", LEASE_BREAK_LAYOUT), ("NONBLOCK", LEASE_BREAK_NONBLOCK),
    ("OPEN_RDONLY", LEASE_BREAK_OPEN_RDONLY),
    ("DIR_CREATE", LEASE_BREAK_DIR_CREATE),
    ("DIR_DELETE", LEASE_BREAK_DIR_DELETE),
    ("DIR_RENAME", LEASE_BREAK_DIR_RENAME),
];

// The following trace event declarations preserve the C event interfaces and
// field layouts. Their actual registration/printing is generated externally.
#[repr(C)]
pub struct locks_get_lock_context_entry {
    pub i_ino: u64,
    pub ctx: *mut file_lock_context,
    pub s_dev: dev_t,
    pub type_: u8,
}

#[repr(C)]
pub struct filelock_lock_entry {
    pub i_ino: u64, pub fl_start: loff_t, pub fl_end: loff_t,
    pub fl: *mut file_lock, pub blocker: *mut file_lock_core,
    pub owner: fl_owner_t, pub s_dev: dev_t, pub pid: u32,
    pub flags: u32, pub type_: u8, pub ret: i32,
}

#[repr(C)]
pub struct break_lease_entry {
    pub i_ino: usize, pub s_dev: dev_t, pub flags: u32,
}

#[repr(C)]
pub struct filelock_lease_entry {
    pub i_ino: u64, pub fl: *mut file_lease,
    pub blocker: *mut file_lock_core, pub owner: fl_owner_t,
    pub break_time: usize, pub downgrade_time: usize,
    pub s_dev: dev_t, pub flags: u32, pub type_: u8,
}

#[repr(C)]
pub struct generic_add_lease_entry {
    pub i_ino: u64, pub owner: fl_owner_t, pub s_dev: dev_t,
    pub wcount: i32, pub rcount: i32, pub icount: i32,
    pub flags: u32, pub type_: u8,
}

#[repr(C)]
pub struct leases_conflict_entry {
    pub lease: *mut core::ffi::c_void, pub breaker: *mut core::ffi::c_void,
    pub l_fl_flags: u32, pub b_fl_flags: u32,
    pub l_fl_type: u8, pub b_fl_type: u8, pub conflict: bool,
}

// Event names: locks_get_lock_context, posix_lock_inode, fcntl_setlk,
// locks_remove_posix, flock_lock_inode, break_lease, break_lease_noblock,
// break_lease_block, break_lease_unblock, generic_delete_lease,
// time_out_leases, generic_add_lease, and leases_conflict.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
