/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// This structure is used to pass the values needed for the identification
// of the resume swap area from a user space to the kernel via the
// SNAPSHOT_SET_SWAP_AREA ioctl.
#[repr(C, packed)]
pub struct resume_swap_area {
    pub offset: i64,
    pub dev: u32,
}

pub const SNAPSHOT_IOC_MAGIC: u8 = b'3';

// `_IO`, `_IOW`, and `_IOR` are supplied by the Linux ioctl definitions.
pub const SNAPSHOT_FREEZE: _ = _IO!(SNAPSHOT_IOC_MAGIC, 1);
pub const SNAPSHOT_UNFREEZE: _ = _IO!(SNAPSHOT_IOC_MAGIC, 2);
pub const SNAPSHOT_ATOMIC_RESTORE: _ = _IO!(SNAPSHOT_IOC_MAGIC, 4);
pub const SNAPSHOT_FREE: _ = _IO!(SNAPSHOT_IOC_MAGIC, 5);
pub const SNAPSHOT_FREE_SWAP_PAGES: _ = _IO!(SNAPSHOT_IOC_MAGIC, 9);
pub const SNAPSHOT_S2RAM: _ = _IO!(SNAPSHOT_IOC_MAGIC, 11);
pub const SNAPSHOT_SET_SWAP_AREA: _ =
    _IOW!(SNAPSHOT_IOC_MAGIC, 13, resume_swap_area);
pub const SNAPSHOT_GET_IMAGE_SIZE: _ =
    _IOR!(SNAPSHOT_IOC_MAGIC, 14, i64);
pub const SNAPSHOT_PLATFORM_SUPPORT: _ = _IO!(SNAPSHOT_IOC_MAGIC, 15);
pub const SNAPSHOT_POWER_OFF: _ = _IO!(SNAPSHOT_IOC_MAGIC, 16);
pub const SNAPSHOT_CREATE_IMAGE: _ = _IOW!(SNAPSHOT_IOC_MAGIC, 17, i32);
pub const SNAPSHOT_PREF_IMAGE_SIZE: _ = _IO!(SNAPSHOT_IOC_MAGIC, 18);
pub const SNAPSHOT_AVAIL_SWAP_SIZE: _ =
    _IOR!(SNAPSHOT_IOC_MAGIC, 19, i64);
pub const SNAPSHOT_ALLOC_SWAP_PAGE: _ =
    _IOR!(SNAPSHOT_IOC_MAGIC, 20, i64);
pub const SNAPSHOT_IOC_MAXNR: u32 = 20;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
