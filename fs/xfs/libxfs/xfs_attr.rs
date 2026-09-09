// SPDX-License-Identifier: GPL-2.0
/* Translated from xfs_attr.c.  Included C headers and external definitions
 * are supplied by the surrounding translation unit. */

use core::ffi::c_void;

extern "C" {
    static mut xfs_attr_intent_cache: *mut kmem_cache;
    fn xfs_inode_has_attr_fork(ip: *mut xfs_inode) -> bool;
    fn xfs_need_iread_extents(ifp: *mut xfs_ifork) -> bool;
    fn xfs_iext_first(ifp: *mut xfs_ifork, icur: *mut xfs_iext_cursor);
    fn xfs_iext_get_extent(ifp: *mut xfs_ifork, icur: *mut xfs_iext_cursor, imap: *mut xfs_bmbt_irec);
    fn xfs_iread_extents(tp: *mut xfs_trans, ip: *mut xfs_inode, fork: i32) -> i32;
    fn xfs_attr_shortform_getvalue(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_leaf_get(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_node_get(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_sethash(args: *mut xfs_da_args);
    fn xfs_attr_leaf_newentsize(args: *mut xfs_da_args, local: *mut i32) -> i32;
    fn xfs_attr_shortform_create(args: *mut xfs_da_args);
    fn xfs_attr_shortform_addname(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_shortform_to_leaf(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_sf_findname(args: *mut xfs_da_args) -> bool;
    fn xfs_attr_sf_removename(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_shortform_replace(args: *mut xfs_da_args) -> i32;
    fn xfs_attr_defer_add(args: *mut xfs_da_args, kind: i32);
    fn xfs_da_hashname(name: *const u8, len: i32) -> u32;
    fn xfs_parent_hashattr(mp: *mut xfs_mount, n: *const u8, nl: i32, v: *const c_void, vl: i32) -> u32;
    fn xfs_attr_check_namespace(flags: u32) -> bool;
    fn xfs_parent_namecheck(flags: u32, name: *const c_void, len: usize) -> bool;
    fn hweight32(v: u32) -> u32;
    fn memchr(p: *const c_void, c: i32, n: usize) -> *const c_void;
    fn kmem_cache_create(n: *const u8, size: usize, a: usize, f: usize, ctor: *const c_void) -> *mut kmem_cache;
    fn kmem_cache_destroy(c: *mut kmem_cache);
}

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct xfs_inode { pub i_af: xfs_ifork, pub i_mount: *mut xfs_mount }
#[repr(C)] pub struct xfs_ifork { pub if_format: i32, pub if_nextents: i32, pub if_data: *mut c_void }
#[repr(C)] pub struct xfs_mount { pub m_attr_geo: *mut xfs_da_geometry }
#[repr(C)] pub struct xfs_da_geometry { pub blksize: i32 }
#[repr(C)] pub struct xfs_iext_cursor { _private: [u8; 0] }
#[repr(C)] pub struct xfs_bmbt_irec { pub br_startoff: i64, pub br_blockcount: i64 }
#[repr(C)] pub struct xfs_trans { _private: [u8; 0] }
#[repr(C)] pub struct xfs_da_args { pub dp: *mut xfs_inode, pub trans: *mut xfs_trans, pub owner: u64, pub geo: *mut xfs_da_geometry, pub whichfork: i32, pub op_flags: u32, pub attr_filter: u32, pub name: *const u8, pub namelen: i32, pub value: *const c_void, pub valuelen: i32, pub new_name: *const u8, pub new_namelen: i32, pub new_value: *const c_void, pub new_valuelen: i32, pub blkno: i64, pub index: i32, pub rmtblkno: i64, pub rmtblkcnt: i32, pub rmtvaluelen: i32, pub blkno2: i64, pub index2: i32, pub rmtblkno2: i64, pub rmtblkcnt2: i32, pub rmtvaluelen2: i32, pub total: u32 }
#[repr(C)] pub struct xfs_attr_intent { pub xattri_da_args: *mut xfs_da_args, pub xattri_dela_state: i32, pub xattri_blkcnt: i32, pub xattri_da_state: *mut xfs_da_state }
#[repr(C)] pub struct xfs_da_state { pub args: *mut xfs_da_args, pub path: xfs_da_state_path }
#[repr(C)] pub struct xfs_da_state_path { pub active: i32, pub blk: *mut xfs_da_state_blk }
#[repr(C)] pub struct xfs_da_state_blk { pub bp: *mut xfs_buf, pub magic: u32 }
#[repr(C)] pub struct xfs_buf { _private: [u8; 0] }
#[repr(C)] pub struct xfs_trans_res { pub tr_logres: i32, pub tr_logcount: i32, pub tr_logflags: u32 }

pub const XFS_DINODE_FMT_EXTENTS: i32 = 2;
pub const XFS_DINODE_FMT_LOCAL: i32 = 1;
pub const XFS_ATTR_FORK: i32 = 1;
pub const XFS_ATTR_PARENT: u32 = 0x10;
pub const XFS_ATTR_INCOMPLETE: u32 = 0x80;
pub const XFS_DA_OP_REPLACE: u32 = 1 << 1;
pub const XFS_DA_OP_RECOVERY: u32 = 1 << 2;
pub const XFS_DA_OP_ADDNAME: u32 = 1 << 0;
pub const MAXNAMELEN: usize = 256;

#[inline] pub unsafe fn xfs_inode_hasattr(ip: *mut xfs_inode) -> i32 { if !xfs_inode_has_attr_fork(ip) { return 0 } if (*ip).i_af.if_format == XFS_DINODE_FMT_EXTENTS && (*ip).i_af.if_nextents == 0 { 0 } else { 1 } }

pub unsafe fn xfs_attr_is_leaf(ip: *mut xfs_inode) -> bool {
    let ifp = &mut (*ip).i_af as *mut xfs_ifork;
    if (*ifp).if_nextents != 1 || (*ifp).if_format != XFS_DINODE_FMT_EXTENTS { return false; }
    let mut icur = core::mem::zeroed::<xfs_iext_cursor>();
    let mut imap = core::mem::zeroed::<xfs_bmbt_irec>();
    xfs_iext_first(ifp, &mut icur); xfs_iext_get_extent(ifp, &mut icur, &mut imap);
    imap.br_startoff == 0 && imap.br_blockcount == 1
}

pub unsafe fn xfs_attr_hashname(name: *const u8, namelen: i32) -> u32 { xfs_da_hashname(name, namelen) }
pub unsafe fn xfs_attr_hashval(mp: *mut xfs_mount, flags: u32, name: *const u8, nl: i32, value: *const c_void, vl: i32) -> u32 { if flags & XFS_ATTR_PARENT != 0 { xfs_parent_hashattr(mp,name,nl,value,vl) } else { xfs_attr_hashname(name,nl) } }
pub unsafe fn xfs_attr_check_namespace_local(flags: u32) -> bool { hweight32(flags) < 2 }
pub unsafe fn xfs_attr_namecheck(flags: u32, name: *const c_void, len: usize) -> bool { if !xfs_attr_check_namespace(flags) || len >= MAXNAMELEN { return false; } if flags & XFS_ATTR_PARENT != 0 { return xfs_parent_namecheck(flags,name,len) } memchr(name,0,len).is_null() }

pub unsafe fn xfs_attr_intent_init_cache() -> i32 { xfs_attr_intent_cache = kmem_cache_create(b"xfs_attr_intent\0".as_ptr(), core::mem::size_of::<xfs_attr_intent>(),0,0,core::ptr::null()); if !xfs_attr_intent_cache.is_null() { 0 } else { -12 } }
pub unsafe fn xfs_attr_intent_destroy_cache() { kmem_cache_destroy(xfs_attr_intent_cache); xfs_attr_intent_cache = core::ptr::null_mut(); }

// The following interfaces retain the source-level entry points; their
// filesystem helpers and state-machine definitions are provided by the other
// translated XFS units.
extern "C" {
    pub fn xfs_attr_get_ilocked(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_get(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_calc_size(args: *mut xfs_da_args, local: *mut i32) -> i32;
    pub fn xfs_attr_set_resv(args: *const xfs_da_args) -> xfs_trans_res;
    pub fn xfs_attr_set_iter(attr: *mut xfs_attr_intent) -> i32;
    pub fn xfs_attr_lookup(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_add_fork(ip: *mut xfs_inode, size: i32, rsvd: i32) -> i32;
    pub fn xfs_attr_setname(args: *mut xfs_da_args, rmt_blks: i32) -> i32;
    pub fn xfs_attr_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_replacename(args: *mut xfs_da_args, rmt_blks: i32) -> i32;
    pub fn xfs_attr_set(args: *mut xfs_da_args, op: i32, rsvd: bool) -> i32;
    pub fn xfs_attr_sf_totsize(dp: *mut xfs_inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
