/* SPDX-License-Identifier: GPL-2.0 */
/* Architecture specific compatibility types (originally guarded by __KERNEL__). */

pub type compat_ipc_pid_t = u16;

#[cfg(target_endian = "big")]
pub const COMPAT_UTS_MACHINE: &[u8] = b"ppc\0\0";
#[cfg(target_endian = "little")]
pub const COMPAT_UTS_MACHINE: &[u8] = b"ppcle\0\0";

pub type compat_nlink_t = i16;

#[repr(C)]
pub struct compat_stat {
    pub st_dev: compat_dev_t,
    pub st_ino: compat_ino_t,
    pub st_mode: compat_mode_t,
    pub st_nlink: compat_nlink_t,
    pub st_uid: __compat_uid32_t,
    pub st_gid: __compat_gid32_t,
    pub st_rdev: compat_dev_t,
    pub st_size: compat_off_t,
    pub st_blksize: compat_off_t,
    pub st_blocks: compat_off_t,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: u32,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: u32,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: u32,
    pub __unused4: [u32; 2],
}

/* ipc64_perm is 32/64-bit clean, but the compat layer refers to it. */
#[repr(C)]
pub struct compat_ipc64_perm {
    pub key: compat_key_t,
    pub uid: __compat_uid_t,
    pub gid: __compat_gid_t,
    pub cuid: __compat_uid_t,
    pub cgid: __compat_gid_t,
    pub mode: compat_mode_t,
    pub seq: u32,
    pub __pad2: u32,
    /* yes they really are 64-bit pads */
    pub __unused1: usize,
    pub __unused2: usize,
}

#[repr(C)]
pub struct compat_semid64_ds {
    pub sem_perm: compat_ipc64_perm,
    pub sem_otime_high: u32,
    pub sem_otime: u32,
    pub sem_ctime_high: u32,
    pub sem_ctime: u32,
    pub sem_nsems: compat_ulong_t,
    pub __unused3: compat_ulong_t,
    pub __unused4: compat_ulong_t,
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
    pub shm_atime_high: u32,
    pub shm_atime: u32,
    pub shm_dtime_high: u32,
    pub shm_dtime: u32,
    pub shm_ctime_high: u32,
    pub shm_ctime: u32,
    pub __unused4: u32,
    pub shm_segsz: compat_size_t,
    pub shm_cpid: compat_pid_t,
    pub shm_lpid: compat_pid_t,
    pub shm_nattch: compat_ulong_t,
    pub __unused5: compat_ulong_t,
    pub __unused6: compat_ulong_t,
}

extern "C" {
    pub fn is_32bit_task() -> bool;
}

pub unsafe fn is_compat_task() -> bool {
    is_32bit_task()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
