/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2000,2002,2005 Silicon Graphics, Inc.
 * Copyright (c) 2013 Red Hat, Inc.
 * All Rights Reserved.
 */

// C header dependencies and build-time type definitions are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct xfs_da_geometry {
    pub blksize: core::ffi::c_uint,
    pub fsbcount: core::ffi::c_uint,
    pub fsblog: u8,
    pub blklog: u8,
    pub node_hdr_size: core::ffi::c_uint,
    pub node_ents: core::ffi::c_uint,
    pub magicpct: core::ffi::c_uint,
    pub datablk: xfs_dablk_t,
    pub leaf_hdr_size: core::ffi::c_uint,
    pub leaf_max_ents: core::ffi::c_uint,
    pub leafblk: xfs_dablk_t,
    pub free_hdr_size: core::ffi::c_uint,
    pub free_max_bests: core::ffi::c_uint,
    pub freeblk: xfs_dablk_t,
    pub max_extents: xfs_extnum_t,
    pub data_first_offset: xfs_dir2_data_aoff_t,
    pub data_entry_offset: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xfs_dacmp {
    XFS_CMP_DIFFERENT,
    XFS_CMP_EXACT,
    XFS_CMP_CASE,
}

#[repr(C)]
pub struct xfs_da_args {
    pub geo: *mut xfs_da_geometry,
    pub name: *const u8,
    pub new_name: *const u8,
    pub value: *mut core::ffi::c_void,
    pub new_value: *mut core::ffi::c_void,
    pub dp: *mut xfs_inode,
    pub trans: *mut xfs_trans,
    pub inumber: xfs_ino_t,
    pub owner: xfs_ino_t,
    pub valuelen: core::ffi::c_int,
    pub new_valuelen: core::ffi::c_int,
    pub filetype: u8,
    pub op_flags: u8,
    pub attr_filter: u8,
    pub namelen: core::ffi::c_short,
    pub new_namelen: core::ffi::c_short,
    pub hashval: xfs_dahash_t,
    pub total: xfs_extlen_t,
    pub whichfork: core::ffi::c_int,
    pub blkno: xfs_dablk_t,
    pub index: core::ffi::c_int,
    pub rmtblkno: xfs_dablk_t,
    pub rmtblkcnt: core::ffi::c_int,
    pub rmtvaluelen: core::ffi::c_int,
    pub blkno2: xfs_dablk_t,
    pub index2: core::ffi::c_int,
    pub rmtblkno2: xfs_dablk_t,
    pub rmtblkcnt2: core::ffi::c_int,
    pub rmtvaluelen2: core::ffi::c_int,
    pub cmpresult: xfs_dacmp,
}
pub type xfs_da_args_t = xfs_da_args;

pub const XFS_DA_OP_JUSTCHECK: u32 = 1u32 << 0;
pub const XFS_DA_OP_REPLACE: u32 = 1u32 << 1;
pub const XFS_DA_OP_ADDNAME: u32 = 1u32 << 2;
pub const XFS_DA_OP_OKNOENT: u32 = 1u32 << 3;
pub const XFS_DA_OP_CILOOKUP: u32 = 1u32 << 4;
pub const XFS_DA_OP_RECOVERY: u32 = 1u32 << 5;
pub const XFS_DA_OP_LOGGED: u32 = 1u32 << 6;
pub const XFS_DA_OP_FLAGS: &[(u32, &str)] = &[
    (XFS_DA_OP_JUSTCHECK, "JUSTCHECK"), (XFS_DA_OP_REPLACE, "REPLACE"),
    (XFS_DA_OP_ADDNAME, "ADDNAME"), (XFS_DA_OP_OKNOENT, "OKNOENT"),
    (XFS_DA_OP_CILOOKUP, "CILOOKUP"), (XFS_DA_OP_RECOVERY, "RECOVERY"),
    (XFS_DA_OP_LOGGED, "LOGGED"),
];

#[repr(C)]
pub struct xfs_da_state_blk {
    pub bp: *mut xfs_buf,
    pub blkno: xfs_dablk_t,
    pub disk_blkno: xfs_daddr_t,
    pub index: core::ffi::c_int,
    pub hashval: xfs_dahash_t,
    pub magic: core::ffi::c_int,
}
pub type xfs_da_state_blk_t = xfs_da_state_blk;

#[repr(C)]
pub struct xfs_da_state_path {
    pub active: core::ffi::c_int,
    pub blk: [xfs_da_state_blk_t; XFS_DA_NODE_MAXDEPTH as usize],
}
pub type xfs_da_state_path_t = xfs_da_state_path;

#[repr(C)]
pub struct xfs_da_state {
    pub args: *mut xfs_da_args_t,
    pub mp: *mut xfs_mount,
    pub path: xfs_da_state_path_t,
    pub altpath: xfs_da_state_path_t,
    pub inleaf: u8,
    pub extravalid: u8,
    pub extraafter: u8,
    pub extrablk: xfs_da_state_blk_t,
}
pub type xfs_da_state_t = xfs_da_state;

#[repr(C)]
pub struct xfs_da3_icnode_hdr {
    pub forw: u32,
    pub back: u32,
    pub magic: u16,
    pub count: u16,
    pub level: u16,
    pub btree: *mut xfs_da_node_entry,
}

pub const XFS_DABUF_MAP_HOLE_OK: u32 = 1u32 << 0;

extern "C" {
    pub fn xfs_da3_node_create(args: *mut xfs_da_args, blkno: xfs_dablk_t, level: core::ffi::c_int, bpp: *mut *mut xfs_buf, whichfork: core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da3_split(state: *mut xfs_da_state_t) -> core::ffi::c_int;
    pub fn xfs_da3_join(state: *mut xfs_da_state_t) -> core::ffi::c_int;
    pub fn xfs_da3_fixhashpath(state: *mut xfs_da_state, path_to_to_fix: *mut xfs_da_state_path);
    pub fn xfs_attr3_node_entry_remove(tp: *mut xfs_trans, dp: *mut xfs_inode, bp: *mut xfs_buf, index: core::ffi::c_int);
    pub fn xfs_da3_node_lookup_int(state: *mut xfs_da_state_t, result: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da3_path_shift(state: *mut xfs_da_state_t, path: *mut xfs_da_state_path_t, forward: core::ffi::c_int, release: core::ffi::c_int, result: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da3_blk_link(state: *mut xfs_da_state_t, old_blk: *mut xfs_da_state_blk_t, new_blk: *mut xfs_da_state_blk_t) -> core::ffi::c_int;
    pub fn xfs_da3_node_read(tp: *mut xfs_trans, dp: *mut xfs_inode, bno: xfs_dablk_t, bpp: *mut *mut xfs_buf, whichfork: core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da3_node_read_mapped(tp: *mut xfs_trans, dp: *mut xfs_inode, mappedbno: xfs_daddr_t, bpp: *mut *mut xfs_buf, whichfork: core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da_grow_inode(args: *mut xfs_da_args_t, new_blkno: *mut xfs_dablk_t) -> core::ffi::c_int;
    pub fn xfs_da_grow_inode_int(args: *mut xfs_da_args, bno: *mut xfs_fileoff_t, count: core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da_get_buf(trans: *mut xfs_trans, dp: *mut xfs_inode, bno: xfs_dablk_t, bp: *mut *mut xfs_buf, whichfork: core::ffi::c_int) -> core::ffi::c_int;
    pub fn xfs_da_read_buf(trans: *mut xfs_trans, dp: *mut xfs_inode, bno: xfs_dablk_t, flags: core::ffi::c_uint, bpp: *mut *mut xfs_buf, whichfork: core::ffi::c_int, ops: *const xfs_buf_ops) -> core::ffi::c_int;
    pub fn xfs_da_reada_buf(dp: *mut xfs_inode, bno: xfs_dablk_t, flags: core::ffi::c_uint, whichfork: core::ffi::c_int, ops: *const xfs_buf_ops) -> core::ffi::c_int;
    pub fn xfs_da_shrink_inode(args: *mut xfs_da_args_t, dead_blkno: xfs_dablk_t, dead_buf: *mut xfs_buf) -> core::ffi::c_int;
    pub fn xfs_da_buf_copy(dst: *mut xfs_buf, src: *mut xfs_buf, size: usize);
    pub fn xfs_da_hashname(name_string: *const u8, name_length: core::ffi::c_int) -> core::ffi::c_uint;
    pub fn xfs_da_compname(args: *mut xfs_da_args, name: *const u8, len: core::ffi::c_int) -> xfs_dacmp;
    pub fn xfs_da_state_alloc(args: *mut xfs_da_args) -> *mut xfs_da_state;
    pub fn xfs_da_state_free(state: *mut xfs_da_state_t);
    pub fn xfs_da_state_reset(state: *mut xfs_da_state, args: *mut xfs_da_args);
    pub fn xfs_da3_node_hdr_from_disk(mp: *mut xfs_mount, to: *mut xfs_da3_icnode_hdr, from: *mut xfs_da_intnode);
    pub fn xfs_da3_node_hdr_to_disk(mp: *mut xfs_mount, to: *mut xfs_da_intnode, from: *mut xfs_da3_icnode_hdr);
    pub fn xfs_da3_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
    pub fn xfs_da3_node_header_check(bp: *mut xfs_buf, owner: xfs_ino_t) -> xfs_failaddr_t;
    pub static mut xfs_da_state_cache: *mut kmem_cache;
}

// XFS_DA_LOGOFF/ XFS_DA_LOGRANGE preserve the original byte-offset arithmetic.
#[inline]
pub unsafe fn xfs_da_logoff(base: *const core::ffi::c_void, addr: *const core::ffi::c_void) -> isize {
    (addr as *const u8).offset_from(base as *const u8)
}
#[inline]
pub unsafe fn xfs_da_logrange(base: *const core::ffi::c_void, addr: *const core::ffi::c_void, size: usize) -> (u32, u32) {
    let off = xfs_da_logoff(base, addr);
    (off as u32, (off + size as isize - 1) as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
