/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Architecture specific compatibility types
 *
 * The original header includes Linux and asm-generic compatibility
 * definitions; those supplied types are referenced here as external
 * dependencies.
 */

pub type compat_mode_t = u16;
pub type __compat_uid_t = u16;
pub type __compat_gid_t = u16;
pub type compat_dev_t = u16;
pub type compat_ipc_pid_t = u16;

// `compat_ipc64_perm` is supplied by the surrounding compatibility API.
pub const COMPAT_RLIM_INFINITY: u32 = 0x7fffffff;
pub const COMPAT_UTS_MACHINE: &[u8] = b"sparc\0\0";

pub type compat_nlink_t = i16;

#[repr(C)]
pub struct compat_stat {
    pub st_dev: compat_dev_t,
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_nlink_t,
    pub st_uid: __compat_uid_t,
    pub st_gid: __compat_gid_t,
    pub st_rdev: compat_dev_t,
    pub st_size: compat_off_t,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: compat_ulong_t,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: compat_ulong_t,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: compat_ulong_t,
    pub st_blksize: compat_off_t,
    pub st_blocks: compat_off_t,
    pub __unused4: [u32; 2],
}

#[repr(C)]
pub struct compat_stat64 {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __pad3: [u8; 8],
    pub st_size: i64,
    pub st_blksize: u32,
    pub __pad4: [u8; 8],
    pub st_blocks: u32,
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub __unused4: u32,
    pub __unused5: u32,
}

// #define __ARCH_COMPAT_FLOCK_PAD short __unused;

#[repr(C)]
pub struct compat_ipc64_perm {
    pub key: compat_key_t,
    pub uid: __compat_uid32_t,
    pub gid: __compat_gid32_t,
    pub cuid: __compat_uid32_t,
    pub cgid: __compat_gid32_t,
    pub __pad1: u16,
    pub mode: compat_mode_t,
    pub __pad2: u16,
    pub seq: u16,
    pub __unused1: core::ffi::c_ulong,
    pub __unused2: core::ffi::c_ulong,
}

#[repr(C)]
pub struct compat_semid64_ds {
    pub sem_perm: compat_ipc64_perm,
    pub sem_otime_high: u32,
    pub sem_otime: u32,
    pub sem_ctime_high: u32,
    pub sem_ctime: u32,
    pub sem_nsems: u32,
    pub __unused1: u32,
    pub __unused2: u32,
}

#[repr(C)]
pub struct compat_msqid64_ds {
    pub msg_perm: compat_ipc64_perm,
    pub msg_stime_high: u32,
    pub msg_stime: u32,
    pub msg_rtime_high: u32,
    pub msg_rtime: u32,
    pub msg_ctime_high: u32,
    pub msg_ctime: u32,
    pub msg_cbytes: u32,
    pub msg_qnum: u32,
    pub msg_qbytes: u32,
    pub msg_lspid: compat_pid_t,
    pub msg_lrpid: compat_pid_t,
    pub __unused1: u32,
    pub __unused2: u32,
}

#[repr(C)]
pub struct compat_shmid64_ds {
    pub shm_perm: compat_ipc64_perm,
    pub shm_atime_high: u32,
    pub shm_atime: u32,
    pub shm_dtime_high: u32,
    pub shm_dtime: u32,
    pub shm_ctime_high: u32,
    pub shm_ctime: u32,
    pub shm_segsz: compat_size_t,
    pub shm_cpid: compat_pid_t,
    pub shm_lpid: compat_pid_t,
    pub shm_nattch: u32,
    pub __unused1: u32,
    pub __unused2: u32,
}

// Preserved from CONFIG_COMPAT; these names are provided by other headers.
#[cfg(CONFIG_COMPAT)]
pub unsafe fn is_compat_task() -> i32 {
    test_thread_flag(TIF_32BIT)
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn in_compat_syscall() -> bool {
    /* Vector 0x110 is LINUX_32BIT_SYSCALL_TRAP */
    pt_regs_trap_type(current_pt_regs()) == 0x110
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
