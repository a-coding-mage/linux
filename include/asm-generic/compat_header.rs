/* SPDX-License-Identifier: GPL-2.0 */

/* Build-time defaults from the C header. */
pub const COMPAT_USER_HZ: u32 = 100;
pub const COMPAT_RLIM_INFINITY: u32 = 0xffff_ffff;
pub const COMPAT_OFF_T_MAX: u32 = 0x7fff_ffff;

/* The C header selects argument order according to CONFIG_CPU_BIG_ENDIAN. */
#[cfg(not(CONFIG_CPU_BIG_ENDIAN))]
macro_rules! compat_arg_u64 {
    ($name:ident) => { u32 $name##_lo, u32 $name##_hi };
}
#[cfg(CONFIG_CPU_BIG_ENDIAN)]
macro_rules! compat_arg_u64 {
    ($name:ident) => { u32 $name##_hi, u32 $name##_lo };
}

#[cfg(not(CONFIG_CPU_BIG_ENDIAN))]
macro_rules! compat_arg_u64_dual {
    ($name:ident) => { u32, $name##_lo, u32, $name##_hi };
}
#[cfg(CONFIG_CPU_BIG_ENDIAN)]
macro_rules! compat_arg_u64_dual {
    ($name:ident) => { u32, $name##_hi, u32, $name##_lo };
}

macro_rules! compat_arg_u64_glue {
    ($lo:expr, $hi:expr) => {
        (($lo as u64 & 0xffff_ffffu64) | (($hi as u64) << 32))
    };
}

/* These types are common across all compat ABIs. */
pub type compat_size_t = u32;
pub type compat_ssize_t = i32;
pub type compat_clock_t = i32;
pub type compat_pid_t = i32;
pub type compat_ino_t = u32;
pub type compat_off_t = i32;
pub type compat_loff_t = i64;
pub type compat_daddr_t = i32;
pub type compat_timer_t = i32;
pub type compat_key_t = i32;
pub type compat_short_t = i16;
pub type compat_int_t = i32;
pub type compat_long_t = i32;
pub type compat_ushort_t = u16;
pub type compat_uint_t = u32;
pub type compat_ulong_t = u32;
pub type compat_uptr_t = u32;
pub type compat_caddr_t = u32;
pub type compat_aio_context_t = u32;
pub type compat_old_sigset_t = u32;

pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type __compat_uid32_t = u32;
pub type __compat_gid32_t = u32;
pub type compat_mode_t = u32;

/* CONFIG_COMPAT_FOR_U64_ALIGNMENT requests C alignment of four bytes. */
pub type compat_s64 = i64;
pub type compat_u64 = u64;

pub type compat_sigset_word = u32;
pub const _COMPAT_NSIG_BPW: u32 = 32;
/* _COMPAT_NSIG is supplied by the surrounding ABI definitions. */
pub const _COMPAT_NSIG: u32 = _NSIG;

pub type compat_dev_t = u32;
pub type compat_ipc_pid_t = i32;
pub type compat_fsid_t = __kernel_fsid_t;

#[repr(C)]
pub struct compat_statfs {
    pub f_type: compat_int_t,
    pub f_bsize: compat_int_t,
    pub f_blocks: compat_int_t,
    pub f_bfree: compat_int_t,
    pub f_bavail: compat_int_t,
    pub f_files: compat_int_t,
    pub f_ffree: compat_int_t,
    pub f_fsid: compat_fsid_t,
    pub f_namelen: compat_int_t,
    pub f_frsize: compat_int_t,
    pub f_flags: compat_int_t,
    pub f_spare: [compat_int_t; 4],
}

#[repr(C)]
pub struct compat_ipc64_perm {
    pub key: compat_key_t,
    pub uid: __compat_uid32_t,
    pub gid: __compat_gid32_t,
    pub cuid: __compat_uid32_t,
    pub cgid: __compat_gid32_t,
    pub mode: compat_mode_t,
    pub __pad1: [u8; 4 - core::mem::size_of::<compat_mode_t>()],
    pub seq: compat_ushort_t,
    pub __pad2: compat_ushort_t,
    pub unused1: compat_ulong_t,
    pub unused2: compat_ulong_t,
}

#[repr(C)]
pub struct compat_semid64_ds {
    pub sem_perm: compat_ipc64_perm,
    pub sem_otime: compat_ulong_t,
    pub sem_otime_high: compat_ulong_t,
    pub sem_ctime: compat_ulong_t,
    pub sem_ctime_high: compat_ulong_t,
    pub sem_nsems: compat_ulong_t,
    pub __unused3: compat_ulong_t,
    pub __unused4: compat_ulong_t,
}

#[repr(C)]
pub struct compat_msqid64_ds {
    pub msg_perm: compat_ipc64_perm,
    pub msg_stime: compat_ulong_t,
    pub msg_stime_high: compat_ulong_t,
    pub msg_rtime: compat_ulong_t,
    pub msg_rtime_high: compat_ulong_t,
    pub msg_ctime: compat_ulong_t,
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
    pub shm_atime_high: compat_ulong_t,
    pub shm_dtime: compat_ulong_t,
    pub shm_dtime_high: compat_ulong_t,
    pub shm_ctime: compat_ulong_t,
    pub shm_ctime_high: compat_ulong_t,
    pub shm_cpid: compat_pid_t,
    pub shm_lpid: compat_pid_t,
    pub shm_nattch: compat_ulong_t,
    pub __unused4: compat_ulong_t,
    pub __unused5: compat_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
