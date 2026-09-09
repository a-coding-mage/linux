// SPDX-License-Identifier: GPL-2.0
/*
 * fs-verity module initialization and logging
 *
 * Copyright 2019 Google LLC
 */

// CREATE_TRACE_POINTS
// Dependencies supplied by fsverity_private.h and linux/ratelimit.h remain external.

#[cfg(CONFIG_SYSCTL)]
#[cfg(CONFIG_FS_VERITY_BUILTIN_SIGNATURES)]
static FSVERITY_SYSCTL_TABLE: [CtlTable; 1] = [CtlTable {
    procname: b"require_signatures\0".as_ptr() as *const core::ffi::c_char,
    data: unsafe { &raw mut fsverity_require_signatures },
    maxlen: core::mem::size_of::<core::ffi::c_int>(),
    mode: 0o644,
    proc_handler: Some(proc_dointvec_minmax),
    extra1: SYSCTL_ZERO,
    extra2: SYSCTL_ONE,
}];

#[cfg(all(CONFIG_SYSCTL, not(CONFIG_FS_VERITY_BUILTIN_SIGNATURES)))]
static FSVERITY_SYSCTL_TABLE: [CtlTable; 0] = [];

#[cfg(CONFIG_SYSCTL)]
unsafe fn fsverity_init_sysctl() {
    register_sysctl_init(
        b"fs/verity\0".as_ptr() as *const core::ffi::c_char,
        FSVERITY_SYSCTL_TABLE.as_ptr(),
    );
}

#[cfg(not(CONFIG_SYSCTL))]
#[inline]
unsafe fn fsverity_init_sysctl() {}

#[repr(C)]
pub struct Inode {
    pub i_sb: *const SuperBlock,
    pub i_ino: u64,
}

#[repr(C)]
pub struct SuperBlock {
    pub s_id: *const core::ffi::c_char,
}

#[repr(C)]
pub struct VaFormat {
    pub fmt: *const core::ffi::c_char,
    pub va: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct CtlTable {
    pub procname: *const core::ffi::c_char,
    pub data: *mut core::ffi::c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn()>,
    pub extra1: *const core::ffi::c_void,
    pub extra2: *const core::ffi::c_void,
}

unsafe extern "C" {
    static mut fsverity_require_signatures: core::ffi::c_int;

    fn register_sysctl_init(path: *const core::ffi::c_char, table: *const CtlTable);
    fn proc_dointvec_minmax();
    fn __ratelimit(rs: *mut RateLimitState) -> bool;
    fn printk(fmt: *const core::ffi::c_char, ...);

    fn fsverity_check_hash_algs();
    fn fsverity_init_info_cache();
    fn fsverity_init_workqueue();
    fn fsverity_init_signature();
    fn fsverity_init_bpf();
}

const SYSCTL_ZERO: *const core::ffi::c_void = core::ptr::null();
const SYSCTL_ONE: *const core::ffi::c_void = core::ptr::null();

#[repr(C)]
pub struct RateLimitState {
    _private: [u8; 0],
}

const DEFAULT_RATELIMIT_INTERVAL: u32 = 5 * 60 * 1000;
const DEFAULT_RATELIMIT_BURST: i32 = 10;

pub unsafe extern "C" fn fsverity_msg(
    inode: *const Inode,
    level: *const core::ffi::c_char,
    fmt: *const core::ffi::c_char,
    mut args: ...,
) {
    static mut RS: RateLimitState = RateLimitState { _private: [] };
    let mut vaf = VaFormat {
        fmt,
        va: &mut args as *mut _ as *mut core::ffi::c_void,
    };

    if !__ratelimit(&raw mut RS) {
        return;
    }

    if !inode.is_null() {
        printk(
            b"%sfs-verity (%s, inode %llu): %pV\n\0".as_ptr() as *const core::ffi::c_char,
            level,
            (*(*inode).i_sb).s_id,
            (*inode).i_ino,
            &mut vaf,
        );
    } else {
        printk(
            b"%sfs-verity: %pV\n\0".as_ptr() as *const core::ffi::c_char,
            level,
            &mut vaf,
        );
    }
}

unsafe extern "C" fn fsverity_init() -> core::ffi::c_int {
    fsverity_check_hash_algs();
    fsverity_init_info_cache();
    fsverity_init_workqueue();
    fsverity_init_sysctl();
    fsverity_init_signature();
    fsverity_init_bpf();
    0
}

// late_initcall(fsverity_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
