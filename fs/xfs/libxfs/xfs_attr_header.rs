/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from xfs_attr.h; external kernel types and functions are supplied elsewhere. */

pub const ATTR_MAX_VALUELEN: usize = 64 * 1024;

#[repr(C)]
pub struct xfs_attrlist_cursor_kern {
    pub hashval: u32,
    pub blkno: u32,
    pub offset: u32,
    pub pad1: u16,
    pub pad2: u8,
    pub initted: u8,
}

pub type put_listent_func_t = unsafe extern "C" fn(
    context: *mut xfs_attr_list_context,
    flags: i32,
    name: *mut u8,
    namelen: i32,
    value: *mut core::ffi::c_void,
    valuelen: i32,
);

#[repr(C)]
pub struct xfs_attr_list_context {
    pub tp: *mut xfs_trans,
    pub dp: *mut xfs_inode,
    pub cursor: xfs_attrlist_cursor_kern,
    pub buffer: *mut core::ffi::c_void,
    pub seen_enough: i32,
    pub allow_incomplete: bool,
    pub count: isize,
    pub dupcnt: i32,
    pub bufsize: i32,
    pub firstu: i32,
    pub attr_filter: u32,
    pub resynch: i32,
    pub put_listent: Option<put_listent_func_t>,
    pub index: i32,
}

#[repr(i32)]
pub enum xfs_delattr_state {
    XFS_DAS_UNINIT = 0,
    XFS_DAS_SF_ADD,
    XFS_DAS_SF_REMOVE,
    XFS_DAS_LEAF_ADD,
    XFS_DAS_LEAF_REMOVE,
    XFS_DAS_NODE_ADD,
    XFS_DAS_NODE_REMOVE,
    XFS_DAS_LEAF_SET_RMT,
    XFS_DAS_LEAF_ALLOC_RMT,
    XFS_DAS_LEAF_REPLACE,
    XFS_DAS_LEAF_REMOVE_OLD,
    XFS_DAS_LEAF_REMOVE_RMT,
    XFS_DAS_LEAF_REMOVE_ATTR,
    XFS_DAS_NODE_SET_RMT,
    XFS_DAS_NODE_ALLOC_RMT,
    XFS_DAS_NODE_REPLACE,
    XFS_DAS_NODE_REMOVE_OLD,
    XFS_DAS_NODE_REMOVE_RMT,
    XFS_DAS_NODE_REMOVE_ATTR,
    XFS_DAS_DONE,
}

#[repr(C)]
pub struct xfs_attr_intent {
    pub xattri_list: list_head,
    pub xattri_da_state: *mut xfs_da_state,
    pub xattri_da_args: *mut xfs_da_args,
    pub xattri_nameval: *mut xfs_attri_log_nameval,
    pub xattri_dela_state: xfs_delattr_state,
    pub xattri_op_flags: u32,
    pub xattri_lblkno: xfs_dablk_t,
    pub xattri_blkcnt: i32,
    pub xattri_map: xfs_bmbt_irec,
}

#[inline]
pub unsafe fn xfs_attr_intent_op(attr: *const xfs_attr_intent) -> u32 {
    (*attr).xattri_op_flags & XFS_ATTRI_OP_FLAGS_TYPE_MASK
}

pub const XFS_ATTRUPDATE_REMOVE: i32 = 0;
pub const XFS_ATTRUPDATE_UPSERT: i32 = 1;
pub const XFS_ATTRUPDATE_CREATE: i32 = 2;
pub const XFS_ATTRUPDATE_REPLACE: i32 = 3;

#[repr(i32)]
pub enum xfs_attr_update {
    XFS_ATTRUPDATE_REMOVE = 0,
    XFS_ATTRUPDATE_UPSERT,
    XFS_ATTRUPDATE_CREATE,
    XFS_ATTRUPDATE_REPLACE,
}

extern "C" {
    pub fn xfs_attr_inactive(dp: *mut xfs_inode) -> i32;
    pub fn xfs_attr_list_ilocked(context: *mut xfs_attr_list_context) -> i32;
    pub fn xfs_attr_list(context: *mut xfs_attr_list_context) -> i32;
    pub fn xfs_inode_hasattr(ip: *mut xfs_inode) -> i32;
    pub fn xfs_inode_has_attr_fork(ip: *mut xfs_inode) -> bool;
    pub fn xfs_attr_is_leaf(ip: *mut xfs_inode) -> bool;
    pub fn xfs_attr_get_ilocked(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_get(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_set(args: *mut xfs_da_args, op: xfs_attr_update, rsvd: bool) -> i32;
    pub fn xfs_attr_set_iter(attr: *mut xfs_attr_intent) -> i32;
    pub fn xfs_attr_remove_iter(attr: *mut xfs_attr_intent) -> i32;
    pub fn xfs_attr_check_namespace(attr_flags: u32) -> bool;
    pub fn xfs_attr_namecheck(attr_flags: u32, name: *const core::ffi::c_void, length: usize) -> bool;
    pub fn xfs_attr_calc_size(args: *mut xfs_da_args, local: *mut i32) -> i32;
    pub fn xfs_attr_hashname(name: *const u8, namelen: i32) -> xfs_dahash_t;
    pub fn xfs_attr_hashval(mp: *mut xfs_mount, attr_flags: u32, name: *const u8, namelen: i32,
        value: *const core::ffi::c_void, valuelen: i32) -> xfs_dahash_t;
    pub fn xfs_attr_intent_init_cache() -> i32;
    pub fn xfs_attr_intent_destroy_cache();
    pub fn xfs_attr_sf_totsize(dp: *mut xfs_inode) -> i32;
    pub fn xfs_attr_add_fork(ip: *mut xfs_inode, size: i32, rsvd: i32) -> i32;
    pub fn xfs_attr_setname(args: *mut xfs_da_args, rmt_blks: i32) -> i32;
    pub fn xfs_attr_removename(args: *mut xfs_da_args) -> i32;
    pub fn xfs_attr_replacename(args: *mut xfs_da_args, rmt_blks: i32) -> i32;
}

/* The following inline routines preserve the header's state-selection logic. */
#[inline]
pub unsafe fn xfs_attr_is_shortform(ip: *const xfs_inode) -> bool {
    (*ip).i_af.if_format == XFS_DINODE_FMT_LOCAL ||
        ((*ip).i_af.if_format == XFS_DINODE_FMT_EXTENTS && (*ip).i_af.if_nextents == 0)
}

#[inline]
pub unsafe fn xfs_attr_init_add_state(args: *mut xfs_da_args) -> xfs_delattr_state {
    if !xfs_inode_has_attr_fork((*args).dp) { return xfs_delattr_state::XFS_DAS_DONE; }
    (*args).op_flags |= XFS_DA_OP_ADDNAME;
    if xfs_attr_is_shortform((*args).dp) { xfs_delattr_state::XFS_DAS_SF_ADD }
    else if xfs_attr_is_leaf((*args).dp) { xfs_delattr_state::XFS_DAS_LEAF_ADD }
    else { xfs_delattr_state::XFS_DAS_NODE_ADD }
}

#[inline]
pub unsafe fn xfs_attr_init_remove_state(args: *mut xfs_da_args) -> xfs_delattr_state {
    if xfs_attr_is_shortform((*args).dp) { xfs_delattr_state::XFS_DAS_SF_REMOVE }
    else if xfs_attr_is_leaf((*args).dp) { xfs_delattr_state::XFS_DAS_LEAF_REMOVE }
    else { xfs_delattr_state::XFS_DAS_NODE_REMOVE }
}

#[inline]
pub unsafe fn xfs_attr_init_replace_state(args: *mut xfs_da_args) -> xfs_delattr_state {
    (*args).op_flags |= XFS_DA_OP_ADDNAME | XFS_DA_OP_REPLACE;
    if (*args).op_flags & XFS_DA_OP_LOGGED != 0 { xfs_attr_init_remove_state(args) }
    else { xfs_attr_init_add_state(args) }
}

#[inline]
pub unsafe fn xfs_attr_sethash(args: *mut xfs_da_args) {
    (*args).hashval = xfs_attr_hashval((*(*args).dp).i_mount,
        (*args).attr_filter, (*args).name, (*args).namelen, (*args).value, (*args).valuelen);
}

extern "C" {
    pub static mut xfs_attr_intent_cache: *mut kmem_cache;
    pub fn xfs_attr_set_resv(args: *const xfs_da_args) -> xfs_trans_res;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
