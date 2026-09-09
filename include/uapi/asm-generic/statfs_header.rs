/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Most 64-bit platforms use 'long', while most 32-bit platforms use '__u32'.
 * Yes, they differ in signedness as well as size.
 * Special cases can override it for themselves -- except for S390x, which
 * is just a little too special for us. And MIPS, which I'm not touching
 * with a 10' pole.
 *
 * The original selection is based on __BITS_PER_LONG.
 */
#[cfg(target_pointer_width = "64")]
pub type __statfs_word = __kernel_long_t;
#[cfg(not(target_pointer_width = "64"))]
pub type __statfs_word = __u32;

#[repr(C)]
pub struct statfs {
    pub f_type: __statfs_word,
    pub f_bsize: __statfs_word,
    pub f_blocks: __statfs_word,
    pub f_bfree: __statfs_word,
    pub f_bavail: __statfs_word,
    pub f_files: __statfs_word,
    pub f_ffree: __statfs_word,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __statfs_word,
    pub f_frsize: __statfs_word,
    pub f_flags: __statfs_word,
    pub f_spare: [__statfs_word; 4],
}

/*
 * ARM needs to avoid the 32-bit padding at the end, for consistency
 * between EABI and OABI
 *
 * ARCH_PACK_STATFS64 is empty in this generic header; architectures may
 * override it when including the original C header.
 */
#[repr(C)]
pub struct statfs64 {
    pub f_type: __statfs_word,
    pub f_bsize: __statfs_word,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __statfs_word,
    pub f_frsize: __statfs_word,
    pub f_flags: __statfs_word,
    pub f_spare: [__statfs_word; 4],
}

/*
 * IA64 and x86_64 need to avoid the 32-bit padding at the end,
 * to be compatible with the i386 ABI.
 *
 * ARCH_PACK_COMPAT_STATFS64 is empty in this generic header; architectures
 * may override it when including the original C header.
 */
#[repr(C)]
pub struct compat_statfs64 {
    pub f_type: __u32,
    pub f_bsize: __u32,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __u32,
    pub f_frsize: __u32,
    pub f_flags: __u32,
    pub f_spare: [__u32; 4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
