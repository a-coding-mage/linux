// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2004-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// 32-bit compatibility ioctl structures and associated ioctl numbers.

pub const XFS_IOC_GETVERSION_32: _ = FS_IOC32_GETVERSION;

#[cfg(target_arch = "x86_64")]
pub const BROKEN_X86_ALIGNMENT: bool = true;

#[repr(C)]
pub struct compat_xfs_bstime {
    pub tv_sec: old_time32_t,
    pub tv_nsec: __s32,
}
pub type compat_xfs_bstime_t = compat_xfs_bstime;

#[repr(C, packed)]
pub struct compat_xfs_bstat {
    pub bs_ino: __u64,
    pub bs_mode: __u16,
    pub bs_nlink: __u16,
    pub bs_uid: __u32,
    pub bs_gid: __u32,
    pub bs_rdev: __u32,
    pub bs_blksize: __s32,
    pub bs_size: __s64,
    pub bs_atime: compat_xfs_bstime_t,
    pub bs_mtime: compat_xfs_bstime_t,
    pub bs_ctime: compat_xfs_bstime_t,
    pub bs_blocks: i64,
    pub bs_xflags: __u32,
    pub bs_extsize: __s32,
    pub bs_extents: __s32,
    pub bs_gen: __u32,
    pub bs_projid_lo: __u16,
    // bs_projid is the historical alias for bs_projid_lo.
    pub bs_forkoff: __u16,
    pub bs_projid_hi: __u16,
    pub bs_pad: [u8; 10],
    pub bs_dmevmask: __u32,
    pub bs_dmstate: __u16,
    pub bs_aextents: __u16,
}

#[repr(C)]
pub struct compat_xfs_fsop_bulkreq {
    pub lastip: compat_uptr_t,
    pub icount: __s32,
    pub ubuffer: compat_uptr_t,
    pub ocount: compat_uptr_t,
}

pub const XFS_IOC_FSBULKSTAT_32: _ = _IOWR('X' as u32, 101, compat_xfs_fsop_bulkreq);
pub const XFS_IOC_FSBULKSTAT_SINGLE_32: _ = _IOWR('X' as u32, 102, compat_xfs_fsop_bulkreq);
pub const XFS_IOC_FSINUMBERS_32: _ = _IOWR('X' as u32, 103, compat_xfs_fsop_bulkreq);

#[repr(C)]
pub struct compat_xfs_fsop_handlereq {
    pub fd: __u32,
    pub path: compat_uptr_t,
    pub oflags: __u32,
    pub ihandle: compat_uptr_t,
    pub ihandlen: __u32,
    pub ohandle: compat_uptr_t,
    pub ohandlen: compat_uptr_t,
}
pub type compat_xfs_fsop_handlereq_t = compat_xfs_fsop_handlereq;

pub const XFS_IOC_PATH_TO_FSHANDLE_32: _ = _IOWR('X' as u32, 104, compat_xfs_fsop_handlereq);
pub const XFS_IOC_PATH_TO_HANDLE_32: _ = _IOWR('X' as u32, 105, compat_xfs_fsop_handlereq);
pub const XFS_IOC_FD_TO_HANDLE_32: _ = _IOWR('X' as u32, 106, compat_xfs_fsop_handlereq);
pub const XFS_IOC_OPEN_BY_HANDLE_32: _ = _IOWR('X' as u32, 107, compat_xfs_fsop_handlereq);
pub const XFS_IOC_READLINK_BY_HANDLE_32: _ = _IOWR('X' as u32, 108, compat_xfs_fsop_handlereq);

#[repr(C, packed)]
pub struct compat_xfs_swapext {
    pub sx_version: i64,
    pub sx_fdtarget: i64,
    pub sx_fdtmp: i64,
    pub sx_offset: xfs_off_t,
    pub sx_length: xfs_off_t,
    pub sx_pad: [core::ffi::c_char; 16],
    pub sx_stat: compat_xfs_bstat,
}

pub const XFS_IOC_SWAPEXT_32: _ = _IOWR('X' as u32, 109, compat_xfs_swapext);

#[repr(C, packed)]
pub struct compat_xfs_fsop_attrlist_handlereq {
    pub hreq: compat_xfs_fsop_handlereq,
    pub pos: xfs_attrlist_cursor,
    pub flags: __u32,
    pub buflen: __u32,
    pub buffer: compat_uptr_t,
}
pub type compat_xfs_fsop_attrlist_handlereq_t = compat_xfs_fsop_attrlist_handlereq;

pub const XFS_IOC_ATTRLIST_BY_HANDLE_32: _ = _IOW('X' as u32, 122, compat_xfs_fsop_attrlist_handlereq);

#[repr(C)]
pub struct compat_xfs_attr_multiop {
    pub am_opcode: __u32,
    pub am_error: __s32,
    pub am_attrname: compat_uptr_t,
    pub am_attrvalue: compat_uptr_t,
    pub am_length: __u32,
    pub am_flags: __u32,
}
pub type compat_xfs_attr_multiop_t = compat_xfs_attr_multiop;

#[repr(C)]
pub struct compat_xfs_fsop_attrmulti_handlereq {
    pub hreq: compat_xfs_fsop_handlereq,
    pub opcount: __u32,
    pub ops: compat_uptr_t,
}
pub type compat_xfs_fsop_attrmulti_handlereq_t = compat_xfs_fsop_attrmulti_handlereq;

pub const XFS_IOC_ATTRMULTI_BY_HANDLE_32: _ = _IOW('X' as u32, 123, compat_xfs_fsop_attrmulti_handlereq);

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
pub struct compat_xfs_fsop_geom_v1 {
    pub blocksize: __u32, pub rtextsize: __u32, pub agblocks: __u32, pub agcount: __u32,
    pub logblocks: __u32, pub sectsize: __u32, pub inodesize: __u32, pub imaxpct: __u32,
    pub datablocks: __u64, pub rtblocks: __u64, pub rtextents: __u64, pub logstart: __u64,
    pub uuid: [u8; 16], pub sunit: __u32, pub swidth: __u32, pub version: __s32,
    pub flags: __u32, pub logsectsize: __u32, pub rtsectsize: __u32, pub dirblocksize: __u32,
}
#[cfg(target_arch = "x86_64")]
pub type compat_xfs_fsop_geom_v1_t = compat_xfs_fsop_geom_v1;

#[cfg(target_arch = "x86_64")]
pub const XFS_IOC_FSGEOMETRY_V1_32: _ = _IOR('X' as u32, 100, compat_xfs_fsop_geom_v1);

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
pub struct compat_xfs_inogrp {
    pub xi_startino: __u64,
    pub xi_alloccount: __s32,
    pub xi_allocmask: __u64,
}

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
pub struct compat_xfs_growfs_data {
    pub newblocks: __u64,
    pub imaxpct: __u32,
}
#[cfg(target_arch = "x86_64")]
pub type compat_xfs_growfs_data_t = compat_xfs_growfs_data;

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
pub struct compat_xfs_growfs_rt {
    pub newblocks: __u64,
    pub extsize: __u32,
}
#[cfg(target_arch = "x86_64")]
pub type compat_xfs_growfs_rt_t = compat_xfs_growfs_rt;

#[cfg(target_arch = "x86_64")]
pub const XFS_IOC_FSGROWFSDATA_32: _ = _IOW('X' as u32, 110, compat_xfs_growfs_data);
#[cfg(target_arch = "x86_64")]
pub const XFS_IOC_FSGROWFSRT_32: _ = _IOW('X' as u32, 112, compat_xfs_growfs_rt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
