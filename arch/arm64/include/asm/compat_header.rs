/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

pub type compat_mode_t = u16;
pub type __compat_uid_t = u16;
pub type __compat_gid_t = u16;
pub type compat_ipc_pid_t = u16;

/* Declarations supplied by asm-generic/compat.h are external dependencies. */

#[cfg(feature = "CONFIG_COMPAT")]
pub const COMPAT_UTS_MACHINE: &[u8] = if cfg!(feature = "__AARCH64EB__") {
    b"armv8b\0\0"
} else {
    b"armv8l\0\0"
};

#[cfg(feature = "CONFIG_COMPAT")]
pub type __compat_uid16_t = u16;
#[cfg(feature = "CONFIG_COMPAT")]
pub type __compat_gid16_t = u16;
#[cfg(feature = "CONFIG_COMPAT")]
pub type compat_nlink_t = i32;

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct compat_stat {
    #[cfg(feature = "__AARCH64EB__")]
    pub st_dev: i16,
    #[cfg(feature = "__AARCH64EB__")]
    pub __pad1: i16,
    #[cfg(not(feature = "__AARCH64EB__"))]
    pub st_dev: compat_dev_t,
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_ushort_t,
    pub st_uid: __compat_uid16_t,
    pub st_gid: __compat_gid16_t,
    #[cfg(feature = "__AARCH64EB__")]
    pub st_rdev: i16,
    #[cfg(feature = "__AARCH64EB__")]
    pub __pad2: i16,
    #[cfg(not(feature = "__AARCH64EB__"))]
    pub st_rdev: compat_dev_t,
    pub st_size: compat_off_t,
    pub st_blksize: compat_off_t,
    pub st_blocks: compat_off_t,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: compat_ulong_t,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: compat_ulong_t,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: compat_ulong_t,
    pub __unused4: [compat_ulong_t; 2],
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct compat_statfs {
    pub f_type: i32,
    pub f_bsize: i32,
    pub f_blocks: i32,
    pub f_bfree: i32,
    pub f_bavail: i32,
    pub f_files: i32,
    pub f_ffree: i32,
    pub f_fsid: compat_fsid_t,
    pub f_namelen: i32, /* SunOS ignores this field. */
    pub f_frsize: i32,
    pub f_flags: i32,
    pub f_spare: [i32; 4],
}

#[cfg(feature = "CONFIG_COMPAT")]
#[inline]
pub unsafe fn compat_user_stack_pointer() -> _ {
    user_stack_pointer(task_pt_regs(current))
}

#[cfg(feature = "CONFIG_COMPAT")]
pub const COMPAT_MINSIGSTKSZ: i32 = 2048;

#[cfg(feature = "CONFIG_COMPAT")]
#[inline]
pub unsafe fn is_compat_task() -> i32 {
    test_thread_flag(TIF_32BIT)
}

#[cfg(feature = "CONFIG_COMPAT")]
#[inline]
pub unsafe fn is_compat_thread(thread: *mut thread_info) -> i32 {
    test_ti_thread_flag(thread, TIF_32BIT)
}

#[cfg(feature = "CONFIG_COMPAT")]
extern "C" {
    pub fn compat_arm_syscall(regs: *mut pt_regs, scno: i32) -> i64;
}

#[cfg(not(feature = "CONFIG_COMPAT"))]
#[inline]
pub unsafe fn is_compat_thread(thread: *mut thread_info) -> i32 {
    let _ = thread;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
