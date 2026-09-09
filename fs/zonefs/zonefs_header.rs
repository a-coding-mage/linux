/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Simple zone file system for zoned block devices.
 *
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 */

/* Kernel dependencies supplied by other translation units/modules:
 * linux/fs.h, linux/magic.h, linux/uuid.h, linux/mutex.h, linux/rwsem.h,
 * linux/kobject.h, and linux/blkzoned.h.
 */

pub const ZONEFS_NAME_MAX: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zonefs_ztype {
    ZONEFS_ZTYPE_CNV = 0,
    ZONEFS_ZTYPE_SEQ = 1,
    ZONEFS_ZTYPE_MAX = 2,
}

pub unsafe fn zonefs_zone_type(zone: *mut blk_zone) -> zonefs_ztype {
    if (*zone).type_ == BLK_ZONE_TYPE_CONVENTIONAL {
        zonefs_ztype::ZONEFS_ZTYPE_CNV
    } else {
        zonefs_ztype::ZONEFS_ZTYPE_SEQ
    }
}

pub const ZONEFS_ZONE_INIT_MODE: u32 = 1u32 << 0;
pub const ZONEFS_ZONE_OPEN: u32 = 1u32 << 1;
pub const ZONEFS_ZONE_ACTIVE: u32 = 1u32 << 2;
pub const ZONEFS_ZONE_OFFLINE: u32 = 1u32 << 3;
pub const ZONEFS_ZONE_READONLY: u32 = 1u32 << 4;
pub const ZONEFS_ZONE_CNV: u32 = 1u32 << 31;

#[repr(C)]
pub struct zonefs_zone {
    pub z_flags: u32,
    pub z_sector: sector_t,
    pub z_size: loff_t,
    pub z_capacity: loff_t,
    pub z_wpoffset: loff_t,
    pub z_mode: umode_t,
    pub z_uid: kuid_t,
    pub z_gid: kgid_t,
}

#[repr(C)]
pub struct zonefs_zone_group {
    pub g_inode: *mut inode,
    pub g_nr_zones: u32,
    pub g_zones: *mut zonefs_zone,
}

#[repr(C)]
pub struct zonefs_inode_info {
    pub i_vnode: inode,
    pub i_truncate_mutex: mutex,
    pub i_wr_refcnt: u32,
}

pub unsafe fn ZONEFS_I(inode: *mut inode) -> *mut zonefs_inode_info {
    container_of!(inode, zonefs_inode_info, i_vnode)
}

pub unsafe fn zonefs_zone_is_cnv(z: *mut zonefs_zone) -> bool {
    (*z).z_flags & ZONEFS_ZONE_CNV != 0
}

pub unsafe fn zonefs_zone_is_seq(z: *mut zonefs_zone) -> bool {
    !zonefs_zone_is_cnv(z)
}

pub unsafe fn zonefs_inode_zone(inode: *mut inode) -> *mut zonefs_zone {
    (*inode).i_private as *mut zonefs_zone
}

pub unsafe fn zonefs_inode_is_cnv(inode: *mut inode) -> bool {
    zonefs_zone_is_cnv(zonefs_inode_zone(inode))
}

pub unsafe fn zonefs_inode_is_seq(inode: *mut inode) -> bool {
    zonefs_zone_is_seq(zonefs_inode_zone(inode))
}

pub const ZONEFS_LABEL_LEN: usize = 64;
pub const ZONEFS_UUID_SIZE: usize = 16;
pub const ZONEFS_SUPER_SIZE: usize = 4096;

#[repr(C, packed)]
pub struct zonefs_super {
    pub s_magic: __le32,
    pub s_crc: __le32,
    pub s_label: [::core::ffi::c_char; ZONEFS_LABEL_LEN],
    pub s_uuid: [__u8; ZONEFS_UUID_SIZE],
    pub s_features: __le64,
    pub s_uid: __le32,
    pub s_gid: __le32,
    pub s_perm: __le32,
    pub s_reserved: [__u8; 3988],
}

#[repr(C)]
pub enum zonefs_features {
    ZONEFS_F_AGGRCNV = 1u64 << 0,
    ZONEFS_F_UID = 1u64 << 1,
    ZONEFS_F_GID = 1u64 << 2,
    ZONEFS_F_PERM = 1u64 << 3,
}

pub const ZONEFS_F_DEFINED_FEATURES: u64 =
    (1u64 << 0) | (1u64 << 1) | (1u64 << 2) | (1u64 << 3);

pub const ZONEFS_MNTOPT_ERRORS_RO: i32 = 1 << 0;
pub const ZONEFS_MNTOPT_ERRORS_ZRO: i32 = 1 << 1;
pub const ZONEFS_MNTOPT_ERRORS_ZOL: i32 = 1 << 2;
pub const ZONEFS_MNTOPT_ERRORS_REPAIR: i32 = 1 << 3;
pub const ZONEFS_MNTOPT_ERRORS_MASK: i32 =
    ZONEFS_MNTOPT_ERRORS_RO | ZONEFS_MNTOPT_ERRORS_ZRO |
    ZONEFS_MNTOPT_ERRORS_ZOL | ZONEFS_MNTOPT_ERRORS_REPAIR;
pub const ZONEFS_MNTOPT_EXPLICIT_OPEN: i32 = 1 << 4;

#[repr(C)]
pub struct zonefs_sb_info {
    pub s_mount_opts: ::core::ffi::c_ulong,
    pub s_lock: spinlock_t,
    pub s_features: ::core::ffi::c_ulonglong,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub s_perm: umode_t,
    pub s_uuid: uuid_t,
    pub s_zone_sectors_shift: u32,
    pub s_zgroup: [zonefs_zone_group; zonefs_ztype::ZONEFS_ZTYPE_MAX as usize],
    pub s_blocks: loff_t,
    pub s_used_blocks: loff_t,
    pub s_max_wro_seq_files: u32,
    pub s_wro_seq_files: atomic_t,
    pub s_max_active_seq_files: u32,
    pub s_active_seq_files: atomic_t,
    pub s_sysfs_registered: bool,
    pub s_kobj: kobject,
    pub s_kobj_unregister: completion,
}

pub unsafe fn ZONEFS_SB(sb: *mut super_block) -> *mut zonefs_sb_info {
    (*sb).s_fs_info as *mut zonefs_sb_info
}

/* zonefs_info(), zonefs_err(), and zonefs_warn() are variadic kernel logging
 * macros. Their source-level intent is retained here; call sites should use
 * the corresponding kernel logging facilities.
 */

pub fn zonefs_inode_account_active(inode: *mut inode);
pub fn zonefs_inode_zone_mgmt(inode: *mut inode, op: req_op) -> i32;
pub fn zonefs_i_size_write(inode: *mut inode, isize: loff_t);
pub fn zonefs_update_stats(inode: *mut inode, new_isize: loff_t);
pub fn __zonefs_io_error(inode: *mut inode, write: bool);

pub unsafe fn zonefs_io_error(inode: *mut inode, write: bool) {
    let zi = ZONEFS_I(inode);
    mutex_lock(&mut (*zi).i_truncate_mutex);
    __zonefs_io_error(inode, write);
    mutex_unlock(&mut (*zi).i_truncate_mutex);
}

pub static zonefs_dir_inode_operations: inode_operations;
pub static zonefs_dir_operations: file_operations;
pub static zonefs_file_aops: address_space_operations;
pub static zonefs_file_operations: file_operations;
pub fn zonefs_file_truncate(inode: *mut inode, isize: loff_t) -> i32;

pub fn zonefs_sysfs_register(sb: *mut super_block) -> i32;
pub fn zonefs_sysfs_unregister(sb: *mut super_block);
pub fn zonefs_sysfs_init() -> i32;
pub fn zonefs_sysfs_exit();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
