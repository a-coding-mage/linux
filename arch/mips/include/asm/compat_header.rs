/* SPDX-License-Identifier: GPL-2.0 */
/* Architecture specific compatibility types. */
/* Dependencies: linux/thread_info.h, linux/types.h, asm/page.h, asm/ptrace.h,
 * and asm-generic/compat.h. */

pub type __compat_uid_t = i32;
pub type __compat_gid_t = i32;
pub type __compat_uid32_t = __compat_uid_t;
pub type __compat_gid32_t = __compat_gid_t;

pub const _COMPAT_NSIG: u32 = 128; /* Don't ask !$@#% ... */
pub const _COMPAT_NSIG_BPW: u32 = 32;
pub type compat_sigset_word = u32;

pub const COMPAT_RLIM_INFINITY: u32 = 0x7fffffff;
pub const COMPAT_UTS_MACHINE: &[u8] = b"mips\0\0\0";

pub type compat_nlink_t = u32;

#[repr(C)]
pub struct compat_stat {
    pub st_dev: compat_dev_t,
    pub st_pad1: [i32; 3],
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_nlink_t,
    pub st_uid: __compat_uid_t,
    pub st_gid: __compat_gid_t,
    pub st_rdev: compat_dev_t,
    pub st_pad2: [i32; 2],
    pub st_size: compat_off_t,
    pub st_pad3: i32,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: i32,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: i32,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: i32,
    pub st_blksize: i32,
    pub st_blocks: i32,
    pub st_pad4: [i32; 14],
}

/* __ARCH_COMPAT_FLOCK_EXTRA_SYSID: s32 l_sysid; */
/* __ARCH_COMPAT_FLOCK_PAD: s32 pad[4]; */

#[repr(C)]
pub struct compat_statfs {
    pub f_type: i32,
    pub f_bsize: i32,
    pub f_frsize: i32,
    pub f_blocks: i32,
    pub f_bfree: i32,
    pub f_files: i32,
    pub f_ffree: i32,
    pub f_bavail: i32,
    pub f_fsid: compat_fsid_t,
    pub f_namelen: i32,
    pub f_flags: i32,
    pub f_spare: [i32; 5],
}

#[repr(C)]
pub struct compat_ipc64_perm {
    pub key: compat_key_t,
    pub uid: __compat_uid32_t,
    pub gid: __compat_gid32_t,
    pub cuid: __compat_uid32_t,
    pub cgid: __compat_gid32_t,
    pub mode: compat_mode_t,
    pub seq: u16,
    pub __pad2: u16,
    pub __unused1: compat_ulong_t,
    pub __unused2: compat_ulong_t,
}

#[repr(C)]
pub struct compat_semid64_ds {
    pub sem_perm: compat_ipc64_perm,
    pub sem_otime: compat_ulong_t,
    pub sem_ctime: compat_ulong_t,
    pub sem_nsems: compat_ulong_t,
    pub sem_otime_high: compat_ulong_t,
    pub sem_ctime_high: compat_ulong_t,
}

#[repr(C)]
pub struct compat_msqid64_ds {
    pub msg_perm: compat_ipc64_perm,
    #[cfg(target_endian = "big")]
    pub msg_stime_high: compat_ulong_t,
    pub msg_stime: compat_ulong_t,
    #[cfg(target_endian = "little")]
    pub msg_stime_high: compat_ulong_t,
    #[cfg(target_endian = "big")]
    pub msg_rtime_high: compat_ulong_t,
    pub msg_rtime: compat_ulong_t,
    #[cfg(target_endian = "little")]
    pub msg_rtime_high: compat_ulong_t,
    #[cfg(target_endian = "big")]
    pub msg_ctime_high: compat_ulong_t,
    pub msg_ctime: compat_ulong_t,
    #[cfg(target_endian = "little")]
    pub msg_ctime_high: compat_ulong_t,
    pub msg_cbytes: compat_ulong_t,
    pub msg_qnum: compat_ulong_t,
    pub msg_qbytes: compat_ulong_t,
    pub msg_lspid: compat_pid_t,
    pub msg_lrpid: compat_pid_t,
    pub __unused4: compat_ulong_t,
    pub __unused5: compat_ulong_t,
}

#[repr(C)]
pub struct compat_shmid64_ds {
    pub shm_perm: compat_ipc64_perm,
    pub shm_segsz: compat_size_t,
    pub shm_atime: compat_ulong_t,
    pub shm_dtime: compat_ulong_t,
    pub shm_ctime: compat_ulong_t,
    pub shm_cpid: compat_pid_t,
    pub shm_lpid: compat_pid_t,
    pub shm_nattch: compat_ulong_t,
    pub shm_atime_high: compat_ushort_t,
    pub shm_dtime_high: compat_ushort_t,
    pub shm_ctime_high: compat_ushort_t,
    pub __unused2: compat_ushort_t,
}

/* MIPS has unusual order of fields in stack_t. */
#[repr(C)]
pub struct compat_sigaltstack {
    pub ss_sp: compat_uptr_t,
    pub ss_size: compat_size_t,
    pub ss_flags: i32,
}

pub type compat_stack_t = compat_sigaltstack;

pub unsafe fn is_compat_task() -> i32 {
    test_thread_flag(TIF_32BIT_ADDR)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
