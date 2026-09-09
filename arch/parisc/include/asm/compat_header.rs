/* SPDX-License-Identifier: GPL-2.0 */
/* Architecture specific compatibility types. */
/* Types and symbols from linux/types.h, linux/sched.h, linux/thread_info.h,
 * and asm-generic/compat.h are supplied by other translated dependencies. */

pub type compat_mode_t = u16;
pub type compat_ipc_pid_t = u16;

pub const COMPAT_UTS_MACHINE: &[u8] = b"parisc\0\0";

pub type compat_nlink_t = u16;

#[repr(C)]
pub struct compat_stat {
    pub st_dev: compat_dev_t,       /* dev_t is 32 bits on parisc */
    pub st_ino: compat_ino_t,       /* 32 bits */
    pub st_mode: compat_mode_t,     /* 16 bits */
    pub st_nlink: compat_nlink_t,   /* 16 bits */
    pub st_reserved1: u16,          /* old st_uid */
    pub st_reserved2: u16,          /* old st_gid */
    pub st_rdev: compat_dev_t,
    pub st_size: compat_off_t,
    pub st_atime: old_time32_t,
    pub st_atime_nsec: u32,
    pub st_mtime: old_time32_t,
    pub st_mtime_nsec: u32,
    pub st_ctime: old_time32_t,
    pub st_ctime_nsec: u32,
    pub st_blksize: i32,
    pub st_blocks: i32,
    pub __unused1: u32,              /* ACL stuff */
    pub __unused2: compat_dev_t,     /* network */
    pub __unused3: compat_ino_t,     /* network */
    pub __unused4: u32,              /* cnodes */
    pub __unused5: u16,              /* netsite */
    pub st_fstype: i16,
    pub st_realdev: compat_dev_t,
    pub st_basemode: u16,
    pub st_spareshort: u16,
    pub st_uid: __compat_uid32_t,
    pub st_gid: __compat_gid32_t,
    pub st_spare4: [u32; 3],
}

#[repr(C)]
pub struct compat_sigcontext {
    pub sc_flags: compat_int_t,
    pub sc_gr: [compat_int_t; 32], /* PSW in sc_gr[0] */
    pub sc_fr: [u64; 32],
    pub sc_iasq: [compat_int_t; 2],
    pub sc_iaoq: [compat_int_t; 2],
    pub sc_sar: compat_int_t,        /* cr11 */
}

#[repr(C)]
pub struct compat_ipc64_perm {
    pub key: compat_key_t,
    pub uid: __compat_uid_t,
    pub gid: __compat_gid_t,
    pub cuid: __compat_uid_t,
    pub cgid: __compat_gid_t,
    pub __pad1: u16,
    pub mode: compat_mode_t,
    pub __pad2: u16,
    pub seq: u16,
    pub __pad3: u32,
    pub __unused1: usize,            /* yes they really are 64bit pads */
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

/* The type of struct elf_prstatus.pr_reg in compatible core dumps. */
pub const COMPAT_ELF_NGREG: usize = 80;
pub type compat_elf_gregset_t = [compat_ulong_t; COMPAT_ELF_NGREG];

pub unsafe fn __is_compat_task(t: *mut task_struct) -> i32 {
    if IS_ENABLED(CONFIG_COMPAT) && test_tsk_thread_flag(t, TIF_32BIT) != 0 {
        1
    } else {
        0
    }
}

pub unsafe fn is_compat_task() -> i32 {
    __is_compat_task(current)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
