/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated uapi/linux/falloc.h header.

/*
 * Space reservation ioctls and argument structure
 * are designed to be compatible with the legacy XFS ioctls.
 */
#[repr(C)]
pub struct space_resv {
    pub l_type: __s16,
    pub l_whence: __s16,
    pub l_start: __s64,
    pub l_len: __s64, // len == 0 means until end of file
    pub l_sysid: __s32,
    pub l_pid: __u32,
    pub l_pad: [__s32; 4], // reserved area
}

pub const FS_IOC_RESVSP: _ = _IOW(b'X', 40, space_resv);
pub const FS_IOC_UNRESVSP: _ = _IOW(b'X', 41, space_resv);
pub const FS_IOC_RESVSP64: _ = _IOW(b'X', 42, space_resv);
pub const FS_IOC_UNRESVSP64: _ = _IOW(b'X', 43, space_resv);
pub const FS_IOC_ZERO_RANGE: _ = _IOW(b'X', 57, space_resv);

/*
 * Mask of all supported fallocate modes.  Only one can be set at a time.
 *
 * In addition to the mode bit, the mode argument can also encode flags.
 * FALLOC_FL_KEEP_SIZE is the only supported flag so far.
 */
pub const FALLOC_FL_MODE_MASK: _ = FALLOC_FL_ALLOCATE_RANGE
    | FALLOC_FL_PUNCH_HOLE
    | FALLOC_FL_COLLAPSE_RANGE
    | FALLOC_FL_ZERO_RANGE
    | FALLOC_FL_INSERT_RANGE
    | FALLOC_FL_UNSHARE_RANGE
    | FALLOC_FL_WRITE_ZEROES;

/* on ia32 l_start is on a 32-bit boundary */
#[cfg(CONFIG_X86_64)]
#[repr(C, packed)]
pub struct space_resv_32 {
    pub l_type: __s16,
    pub l_whence: __s16,
    pub l_start: __s64,
    // len == 0 means until end of file
    pub l_len: __s64,
    pub l_sysid: __s32,
    pub l_pid: __u32,
    pub l_pad: [__s32; 4], // reserve area
}

#[cfg(CONFIG_X86_64)]
pub const FS_IOC_RESVSP_32: _ = _IOW(b'X', 40, space_resv_32);
#[cfg(CONFIG_X86_64)]
pub const FS_IOC_UNRESVSP_32: _ = _IOW(b'X', 41, space_resv_32);
#[cfg(CONFIG_X86_64)]
pub const FS_IOC_RESVSP64_32: _ = _IOW(b'X', 42, space_resv_32);
#[cfg(CONFIG_X86_64)]
pub const FS_IOC_UNRESVSP64_32: _ = _IOW(b'X', 43, space_resv_32);
#[cfg(CONFIG_X86_64)]
pub const FS_IOC_ZERO_RANGE_32: _ = _IOW(b'X', 57, space_resv_32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
