/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header. The original includes provide
// compiler annotations and ioctl encoding helpers.

pub const BLKPG: u32 = (0x12u32 << 8) | 105u32;

/* The argument structure */
#[repr(C)]
pub struct blkpg_ioctl_arg {
    pub op: core::ffi::c_int,
    pub flags: core::ffi::c_int,
    pub datalen: core::ffi::c_int,
    pub data: *mut core::ffi::c_void,
}

/* The subfunctions (for the op field) */
pub const BLKPG_ADD_PARTITION: core::ffi::c_int = 1;
pub const BLKPG_DEL_PARTITION: core::ffi::c_int = 2;
pub const BLKPG_RESIZE_PARTITION: core::ffi::c_int = 3;

/* Sizes of name fields. Unused at present. */
pub const BLKPG_DEVNAMELTH: usize = 64;
pub const BLKPG_VOLNAMELTH: usize = 64;

/* The data structure for ADD_PARTITION and DEL_PARTITION */
#[repr(C)]
pub struct blkpg_partition {
    pub start: i64, /* starting offset in bytes */
    pub length: i64, /* length in bytes */
    pub pno: core::ffi::c_int, /* partition number */
    pub devname: [core::ffi::c_char; BLKPG_DEVNAMELTH], /* unused / ignored */
    pub volname: [core::ffi::c_char; BLKPG_VOLNAMELTH], /* unused / ignore */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
