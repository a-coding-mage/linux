/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * nilfs2_api.h - NILFS2 user space API
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 */

/* Translated from the C header. The linux types and ioctl encoding helpers are
 * supplied by the surrounding low-level bindings. */

#[repr(C)]
pub struct nilfs_cpinfo {
    pub ci_flags: u32,
    pub ci_pad: u32,
    pub ci_cno: u64,
    pub ci_create: u64,
    pub ci_nblk_inc: u64,
    pub ci_inodes_count: u64,
    pub ci_blocks_count: u64,
    pub ci_next: u64,
}

pub const NILFS_CPINFO_SNAPSHOT: i32 = 0;
pub const NILFS_CPINFO_INVALID: i32 = 1;
pub const NILFS_CPINFO_SKETCH: i32 = 2;
pub const NILFS_CPINFO_MINOR: i32 = 3;

#[inline]
pub unsafe fn nilfs_cpinfo_snapshot(cpinfo: *const nilfs_cpinfo) -> i32 {
    (((*cpinfo).ci_flags & (1u32 << NILFS_CPINFO_SNAPSHOT)) != 0) as i32
}

#[inline]
pub unsafe fn nilfs_cpinfo_invalid(cpinfo: *const nilfs_cpinfo) -> i32 {
    (((*cpinfo).ci_flags & (1u32 << NILFS_CPINFO_INVALID)) != 0) as i32
}

#[inline]
pub unsafe fn nilfs_cpinfo_minor(cpinfo: *const nilfs_cpinfo) -> i32 {
    (((*cpinfo).ci_flags & (1u32 << NILFS_CPINFO_MINOR)) != 0) as i32
}

#[repr(C)]
pub struct nilfs_suinfo {
    pub sui_lastmod: u64,
    pub sui_nblocks: u32,
    pub sui_flags: u32,
}

pub const NILFS_SUINFO_ACTIVE: i32 = 0;
pub const NILFS_SUINFO_DIRTY: i32 = 1;
pub const NILFS_SUINFO_ERROR: i32 = 2;

#[inline]
pub unsafe fn nilfs_suinfo_active(si: *const nilfs_suinfo) -> i32 {
    ((*si).sui_flags & (1u32 << NILFS_SUINFO_ACTIVE)) as i32
}

#[inline]
pub unsafe fn nilfs_suinfo_dirty(si: *const nilfs_suinfo) -> i32 {
    ((*si).sui_flags & (1u32 << NILFS_SUINFO_DIRTY)) as i32
}

#[inline]
pub unsafe fn nilfs_suinfo_error(si: *const nilfs_suinfo) -> i32 {
    ((*si).sui_flags & (1u32 << NILFS_SUINFO_ERROR)) as i32
}

#[inline]
pub unsafe fn nilfs_suinfo_clean(si: *const nilfs_suinfo) -> i32 {
    ((*si).sui_flags == 0) as i32
}

#[repr(C)]
pub struct nilfs_suinfo_update {
    pub sup_segnum: u64,
    pub sup_flags: u32,
    pub sup_reserved: u32,
    pub sup_sui: nilfs_suinfo,
}

pub const NILFS_SUINFO_UPDATE_LASTMOD: i32 = 0;
pub const NILFS_SUINFO_UPDATE_NBLOCKS: i32 = 1;
pub const NILFS_SUINFO_UPDATE_FLAGS: i32 = 2;
pub const __NR_NILFS_SUINFO_UPDATE_FIELDS: i32 = 3;

#[inline]
pub unsafe fn nilfs_suinfo_update_set_lastmod(sup: *mut nilfs_suinfo_update) {
    (*sup).sup_flags |= 1u32 << NILFS_SUINFO_UPDATE_LASTMOD;
}
#[inline]
pub unsafe fn nilfs_suinfo_update_clear_lastmod(sup: *mut nilfs_suinfo_update) {
    (*sup).sup_flags &= !(1u32 << NILFS_SUINFO_UPDATE_LASTMOD);
}
#[inline]
pub unsafe fn nilfs_suinfo_update_lastmod(sup: *const nilfs_suinfo_update) -> i32 {
    (((*sup).sup_flags & (1u32 << NILFS_SUINFO_UPDATE_LASTMOD)) != 0) as i32
}
#[inline]
pub unsafe fn nilfs_suinfo_update_set_nblocks(sup: *mut nilfs_suinfo_update) {
    (*sup).sup_flags |= 1u32 << NILFS_SUINFO_UPDATE_NBLOCKS;
}
#[inline]
pub unsafe fn nilfs_suinfo_update_clear_nblocks(sup: *mut nilfs_suinfo_update) {
    (*sup).sup_flags &= !(1u32 << NILFS_SUINFO_UPDATE_NBLOCKS);
}
#[inline]
pub unsafe fn nilfs_suinfo_update_nblocks(sup: *const nilfs_suinfo_update) -> i32 {
    (((*sup).sup_flags & (1u32 << NILFS_SUINFO_UPDATE_NBLOCKS)) != 0) as i32
}
#[inline]
pub unsafe fn nilfs_suinfo_update_set_flags(sup: *mut nilfs_suinfo_update) {
    (*sup).sup_flags |= 1u32 << NILFS_SUINFO_UPDATE_FLAGS;
}
#[inline]
pub unsafe fn nilfs_suinfo_update_clear_flags(sup: *mut nilfs_suinfo_update) {
    (*sup).sup_flags &= !(1u32 << NILFS_SUINFO_UPDATE_FLAGS);
}
#[inline]
pub unsafe fn nilfs_suinfo_update_flags(sup: *const nilfs_suinfo_update) -> i32 {
    (((*sup).sup_flags & (1u32 << NILFS_SUINFO_UPDATE_FLAGS)) != 0) as i32
}

pub const NILFS_CHECKPOINT: i32 = 0;
pub const NILFS_SNAPSHOT: i32 = 1;

#[repr(C)]
pub struct nilfs_cpmode { pub cm_cno: u64, pub cm_mode: u32, pub cm_pad: u32 }
#[repr(C)]
pub struct nilfs_argv { pub v_base: u64, pub v_nmembs: u32, pub v_size: u16, pub v_flags: u16, pub v_index: u64 }
#[repr(C)]
pub struct nilfs_period { pub p_start: u64, pub p_end: u64 }
#[repr(C)]
pub struct nilfs_cpstat { pub cs_cno: u64, pub cs_ncps: u64, pub cs_nsss: u64 }
#[repr(C)]
pub struct nilfs_sustat { pub ss_nsegs: u64, pub ss_ncleansegs: u64, pub ss_ndirtysegs: u64, pub ss_ctime: u64, pub ss_nongc_ctime: u64, pub ss_prot_seq: u64 }
#[repr(C)]
pub struct nilfs_vinfo { pub vi_vblocknr: u64, pub vi_start: u64, pub vi_end: u64, pub vi_blocknr: u64 }
#[repr(C)]
pub struct nilfs_vdesc { pub vd_ino: u64, pub vd_cno: u64, pub vd_vblocknr: u64, pub vd_period: nilfs_period, pub vd_blocknr: u64, pub vd_offset: u64, pub vd_flags: u32, pub vd_pad: u32 }
#[repr(C)]
pub struct nilfs_bdesc { pub bd_ino: u64, pub bd_oblocknr: u64, pub bd_blocknr: u64, pub bd_offset: u64, pub bd_level: u32, pub bd_pad: u32 }

pub const NILFS_IOCTL_IDENT: u8 = b'n';
/* _IOW, _IOR, and _IOWR are provided by the dependent ioctl bindings. */
pub const NILFS_IOCTL_CHANGE_CPMODE: usize = _IOW(NILFS_IOCTL_IDENT, 0x80, nilfs_cpmode);
pub const NILFS_IOCTL_DELETE_CHECKPOINT: usize = _IOW(NILFS_IOCTL_IDENT, 0x81, u64);
pub const NILFS_IOCTL_GET_CPINFO: usize = _IOR(NILFS_IOCTL_IDENT, 0x82, nilfs_argv);
pub const NILFS_IOCTL_GET_CPSTAT: usize = _IOR(NILFS_IOCTL_IDENT, 0x83, nilfs_cpstat);
pub const NILFS_IOCTL_GET_SUINFO: usize = _IOR(NILFS_IOCTL_IDENT, 0x84, nilfs_argv);
pub const NILFS_IOCTL_GET_SUSTAT: usize = _IOR(NILFS_IOCTL_IDENT, 0x85, nilfs_sustat);
pub const NILFS_IOCTL_GET_VINFO: usize = _IOWR(NILFS_IOCTL_IDENT, 0x86, nilfs_argv);
pub const NILFS_IOCTL_GET_BDESCS: usize = _IOWR(NILFS_IOCTL_IDENT, 0x87, nilfs_argv);
pub const NILFS_IOCTL_CLEAN_SEGMENTS: usize = _IOW(NILFS_IOCTL_IDENT, 0x88, [nilfs_argv; 5]);
pub const NILFS_IOCTL_SYNC: usize = _IOR(NILFS_IOCTL_IDENT, 0x8A, u64);
pub const NILFS_IOCTL_RESIZE: usize = _IOW(NILFS_IOCTL_IDENT, 0x8B, u64);
pub const NILFS_IOCTL_SET_ALLOC_RANGE: usize = _IOW(NILFS_IOCTL_IDENT, 0x8C, [u64; 2]);
pub const NILFS_IOCTL_SET_SUINFO: usize = _IOW(NILFS_IOCTL_IDENT, 0x8D, nilfs_argv);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
