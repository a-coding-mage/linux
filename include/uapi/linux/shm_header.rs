/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux IPC and huge-page headers
// are referenced here but intentionally not implemented in this translation.

/*
 * SHMMNI, SHMMAX and SHMALL are default upper limits which can be
 * modified by sysctl. The SHMMAX and SHMALL values have been chosen to
 * be as large possible without facilitating scenarios where userspace
 * causes overflows when adjusting the limits via operations of the form
 * "retrieve current limit; add X; update limit". It is therefore not
 * advised to make SHMMAX and SHMALL any larger. These limits are
 * suitable for both 32 and 64-bit systems.
 */
pub const SHMMIN: usize = 1; // min shared seg size (bytes)
pub const SHMMNI: usize = 4096; // max num of segs system wide
pub const SHMMAX: usize = usize::MAX - (1usize << 24); // max shared seg size (bytes)
pub const SHMALL: usize = usize::MAX - (1usize << 24); // max shm system wide (pages)
pub const SHMSEG: usize = SHMMNI; // max shared segs per process

/* Obsolete, used only for backwards compatibility and libc5 compiles */
#[repr(C)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm, // operation perms
    pub shm_segsz: i32, // size of segment (bytes)
    pub shm_atime: __kernel_old_time_t, // last attach time
    pub shm_dtime: __kernel_old_time_t, // last detach time
    pub shm_ctime: __kernel_old_time_t, // last change time
    pub shm_cpid: __kernel_ipc_pid_t, // pid of creator
    pub shm_lpid: __kernel_ipc_pid_t, // pid of last operator
    pub shm_nattch: u16, // no. of current attaches
    pub shm_unused: u16, // compatibility
    pub shm_unused2: *mut core::ffi::c_void, // ditto - used by DIPC
    pub shm_unused3: *mut core::ffi::c_void, // unused
}

// Include the definition of shmid64_ds and shminfo64 from asm/shmbuf.h.

/*
 * shmget() shmflg values.
 */
/* The bottom nine bits are the same as open(2) mode flags */
pub const SHM_R: u32 = 0o400; // or S_IRUGO from <linux/stat.h>
pub const SHM_W: u32 = 0o200; // or S_IWUGO from <linux/stat.h>
/* Bits 9 & 10 are IPC_CREAT and IPC_EXCL */
pub const SHM_HUGETLB: u32 = 0o4000; // segment will use huge TLB pages
pub const SHM_NORESERVE: u32 = 0o10000; // don't check for reservations

/*
 * Huge page size encoding when SHM_HUGETLB is specified, and a huge page
 * size other than the default is desired.  See hugetlb_encode.h
 */
pub const SHM_HUGE_SHIFT: u32 = HUGETLB_FLAG_ENCODE_SHIFT;
pub const SHM_HUGE_MASK: u32 = HUGETLB_FLAG_ENCODE_MASK;

pub const SHM_HUGE_64KB: u32 = HUGETLB_FLAG_ENCODE_64KB;
pub const SHM_HUGE_512KB: u32 = HUGETLB_FLAG_ENCODE_512KB;
pub const SHM_HUGE_1MB: u32 = HUGETLB_FLAG_ENCODE_1MB;
pub const SHM_HUGE_2MB: u32 = HUGETLB_FLAG_ENCODE_2MB;
pub const SHM_HUGE_8MB: u32 = HUGETLB_FLAG_ENCODE_8MB;
pub const SHM_HUGE_16MB: u32 = HUGETLB_FLAG_ENCODE_16MB;
pub const SHM_HUGE_32MB: u32 = HUGETLB_FLAG_ENCODE_32MB;
pub const SHM_HUGE_256MB: u32 = HUGETLB_FLAG_ENCODE_256MB;
pub const SHM_HUGE_512MB: u32 = HUGETLB_FLAG_ENCODE_512MB;
pub const SHM_HUGE_1GB: u32 = HUGETLB_FLAG_ENCODE_1GB;
pub const SHM_HUGE_2GB: u32 = HUGETLB_FLAG_ENCODE_2GB;
pub const SHM_HUGE_16GB: u32 = HUGETLB_FLAG_ENCODE_16GB;

/*
 * shmat() shmflg values
 */
pub const SHM_RDONLY: u32 = 0o10000; // read-only access
pub const SHM_RND: u32 = 0o20000; // round attach address to SHMLBA boundary
pub const SHM_REMAP: u32 = 0o40000; // take-over region on attach
pub const SHM_EXEC: u32 = 0o100000; // execution access

/* super user shmctl commands */
pub const SHM_LOCK: i32 = 11;
pub const SHM_UNLOCK: i32 = 12;

/* ipcs ctl commands */
pub const SHM_STAT: i32 = 13;
pub const SHM_INFO: i32 = 14;
pub const SHM_STAT_ANY: i32 = 15;

/* Obsolete, used only for backwards compatibility */
#[repr(C)]
pub struct shminfo {
    pub shmmax: i32,
    pub shmmin: i32,
    pub shmmni: i32,
    pub shmseg: i32,
    pub shmall: i32,
}

#[repr(C)]
pub struct shm_info {
    pub used_ids: i32,
    pub shm_tot: __kernel_ulong_t, // total allocated shm
    pub shm_rss: __kernel_ulong_t, // total resident shm
    pub shm_swp: __kernel_ulong_t, // total swapped shm
    pub swap_attempts: __kernel_ulong_t,
    pub swap_successes: __kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
