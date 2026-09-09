/* SPDX-License-Identifier: GPL-2.0 */
/*
 * befs.h
 *
 * Copyright (C) 2001-2002 Will Dyson <will_dyson@pobox.com>
 * Copyright (C) 1999 Makoto Kato (m_kato@ga2.so-net.ne.jp)
 */

// C header guard: _LINUX_BEFS_H
// Dependency: "befs_fs_types.h"

/* used in debug.c */
pub const BEFS_VERSION: &str = "0.9.3";

pub type befs_blocknr_t = u64;

/*
 * BeFS in memory structures
 */

#[repr(C)]
pub struct befs_mount_options {
    pub gid: kgid_t,
    pub uid: kuid_t,
    pub use_gid: ::std::os::raw::c_int,
    pub use_uid: ::std::os::raw::c_int,
    pub debug: ::std::os::raw::c_int,
    pub iocharset: *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct befs_sb_info {
    pub magic1: u32,
    pub block_size: u32,
    pub block_shift: u32,
    pub byte_order: ::std::os::raw::c_int,
    pub num_blocks: befs_off_t,
    pub used_blocks: befs_off_t,
    pub inode_size: u32,
    pub magic2: u32,

    /* Allocation group information */
    pub blocks_per_ag: u32,
    pub ag_shift: u32,
    pub num_ags: u32,

    /* State of the superblock */
    pub flags: u32,

    /* Journal log entry */
    pub log_blocks: befs_block_run,
    pub log_start: befs_off_t,
    pub log_end: befs_off_t,

    pub root_dir: befs_inode_addr,
    pub indices: befs_inode_addr,
    pub magic3: u32,

    pub mount_opts: befs_mount_options,
    pub nls: *mut nls_table,
}

#[repr(C)]
pub union befs_inode_data {
    pub ds: befs_data_stream,
    pub symlink: [::std::os::raw::c_char; BEFS_SYMLINK_LEN],
}

#[repr(C)]
pub struct befs_inode_info {
    pub i_flags: u32,
    pub i_type: u32,

    pub i_inode_num: befs_inode_addr,
    pub i_parent: befs_inode_addr,
    pub i_attribute: befs_inode_addr,

    pub i_data: befs_inode_data,

    pub vfs_inode: inode,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum befs_err {
    BEFS_OK,
    BEFS_ERR,
    BEFS_BAD_INODE,
    BEFS_BT_END,
    BEFS_BT_EMPTY,
    BEFS_BT_MATCH,
    BEFS_BT_OVERFLOW,
    BEFS_BT_NOT_FOUND,
}

/****************************/
/* debug.c */
extern "C" {
    pub fn befs_error(sb: *const super_block, fmt: *const ::std::os::raw::c_char, ...);
    pub fn befs_warning(sb: *const super_block, fmt: *const ::std::os::raw::c_char, ...);
    pub fn befs_debug(sb: *const super_block, fmt: *const ::std::os::raw::c_char, ...);

    pub fn befs_dump_super_block(sb: *const super_block, _: *mut befs_super_block);
    pub fn befs_dump_inode(sb: *const super_block, _: *mut befs_inode);
    pub fn befs_dump_index_entry(sb: *const super_block, _: *mut befs_disk_btree_super);
    pub fn befs_dump_index_node(sb: *const super_block, _: *mut befs_btree_nodehead);
}
/****************************/

/* Gets a pointer to the private portion of the super_block
 * structure from the public part
 */
#[inline]
pub unsafe fn BEFS_SB(super_: *const super_block) -> *mut befs_sb_info {
    (*super_).s_fs_info as *mut befs_sb_info
}

#[inline]
pub unsafe fn BEFS_I(inode_: *const inode) -> *mut befs_inode_info {
    container_of!(inode_, befs_inode_info, vfs_inode)
}

#[inline]
pub unsafe fn iaddr2blockno(sb: *mut super_block, iaddr: *const befs_inode_addr) -> befs_blocknr_t {
    ((*iaddr).allocation_group << (*BEFS_SB(sb)).ag_shift) + (*iaddr).start
}

#[inline]
pub unsafe fn blockno2iaddr(sb: *mut super_block, blockno: befs_blocknr_t) -> befs_inode_addr {
    let mut iaddr: befs_inode_addr;

    iaddr.allocation_group = blockno >> (*BEFS_SB(sb)).ag_shift;
    iaddr.start = blockno - (iaddr.allocation_group << (*BEFS_SB(sb)).ag_shift);
    iaddr.len = 1;

    iaddr
}

#[inline]
pub unsafe fn befs_iaddrs_per_block(sb: *mut super_block) -> ::std::os::raw::c_uint {
    (*BEFS_SB(sb)).block_size / ::std::mem::size_of::<befs_disk_inode_addr>() as u32
}

// Dependency: "endian.h"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
