/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/fs/nfs/delegation.h
 *
 * Copyright (c) Trond Myklebust
 *
 * Definitions pertaining to NFS delegated files
 */

/* C: all declarations in this section are enabled when CONFIG_NFS_V4 is enabled. */
#[cfg(feature = "CONFIG_NFS_V4")]
#[repr(C)]
pub struct nfs_delegation {
    pub hash: hlist_node,
    pub super_list: list_head,
    pub cred: *const cred,
    pub inode: *mut inode,
    pub stateid: nfs4_stateid,
    pub type_: fmode_t,
    pub pagemod_limit: c_ulong,
    pub change_attr: __u64,
    pub test_gen: c_ulong,
    pub flags: c_ulong,
    pub refcount: refcount_t,
    pub lock: spinlock_t,
    pub entry: list_head,
    pub rcu: rcu_head,
}

#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_NEED_RECLAIM: i32 = 0;
#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_RETURN_IF_CLOSED: i32 = 1;
#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_REFERENCED: i32 = 2;
#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_RETURNING: i32 = 3;
#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_REVOKED: i32 = 4;
#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_TEST_EXPIRED: i32 = 5;
#[cfg(feature = "CONFIG_NFS_V4")]
pub const NFS_DELEGATION_DELEGTIME: i32 = 6;

pub const NFS_DELEGATION_FLAG_TIME: c_ulong = 1 << 1;

extern "C" {
    pub fn nfs_inode_set_delegation(inode: *mut inode, cred: *const cred, type_: fmode_t,
        stateid: *const nfs4_stateid, pagemod_limit: c_ulong, deleg_type: u32) -> i32;
    pub fn nfs_inode_reclaim_delegation(inode: *mut inode, cred: *const cred, type_: fmode_t,
        stateid: *const nfs4_stateid, pagemod_limit: c_ulong, deleg_type: u32);
    pub fn nfs4_inode_return_delegation(inode: *mut inode);
    pub fn nfs4_inode_return_delegation_on_close(inode: *mut inode);
    pub fn nfs4_inode_set_return_delegation_on_close(inode: *mut inode);
    pub fn nfs_async_inode_return_delegation(inode: *mut inode, stateid: *const nfs4_stateid) -> i32;
    pub fn nfs_inode_evict_delegation(inode: *mut inode);
    pub fn nfs_delegation_find_inode(clp: *mut nfs_client, fhandle: *const nfs_fh) -> *mut inode;
    pub fn nfs_server_return_all_delegations(server: *mut nfs_server);
    pub fn nfs_expire_all_delegations(clp: *mut nfs_client);
    pub fn nfs_expire_unused_delegation_types(clp: *mut nfs_client, flags: fmode_t);
    pub fn nfs_expire_unreferenced_delegations(clp: *mut nfs_client);
    pub fn nfs_client_return_marked_delegations(clp: *mut nfs_client) -> i32;
    pub fn nfs_delegations_present(clp: *mut nfs_client) -> i32;
    pub fn nfs_remove_bad_delegation(inode: *mut inode, stateid: *const nfs4_stateid);
    pub fn nfs_delegation_mark_returned(inode: *mut inode, stateid: *const nfs4_stateid);
    pub fn nfs_delegation_mark_reclaim(clp: *mut nfs_client);
    pub fn nfs_delegation_reap_unclaimed(clp: *mut nfs_client);
    pub fn nfs_mark_test_expired_all_delegations(clp: *mut nfs_client);
    pub fn nfs_test_expired_all_delegations(clp: *mut nfs_client);
    pub fn nfs_reap_expired_delegations(clp: *mut nfs_client);
    pub fn nfs4_proc_delegreturn(inode: *mut inode, cred: *const cred, stateid: *const nfs4_stateid,
        delegation: *mut nfs_delegation, issync: i32) -> i32;
    pub fn nfs4_open_delegation_recall(ctx: *mut nfs_open_context, state: *mut nfs4_state,
        stateid: *const nfs4_stateid) -> i32;
    pub fn nfs4_lock_delegation_recall(fl: *mut file_lock, state: *mut nfs4_state,
        stateid: *const nfs4_stateid) -> i32;
    pub fn nfs4_copy_delegation_stateid(inode: *mut inode, flags: fmode_t, dst: *mut nfs4_stateid,
        cred: *mut *const cred) -> bool;
    pub fn nfs4_refresh_delegation_stateid(dst: *mut nfs4_stateid, inode: *mut inode) -> bool;
    pub fn nfs4_get_valid_delegation(inode: *const inode) -> *mut nfs_delegation;
    pub fn nfs_put_delegation(delegation: *mut nfs_delegation);
    pub fn nfs_mark_delegation_referenced(delegation: *mut nfs_delegation);
    pub fn nfs4_have_delegation(inode: *mut inode, type_: fmode_t, flags: i32) -> i32;
    pub fn nfs4_check_delegation(inode: *mut inode, type_: fmode_t) -> i32;
    pub fn nfs4_delegation_flush_on_close(inode: *const inode) -> bool;
    pub fn nfs_inode_find_delegation_state_and_recover(inode: *mut inode, stateid: *const nfs4_stateid);
    pub fn nfs4_inode_make_writeable(inode: *mut inode);
    pub fn nfs_update_delegated_atime(inode: *mut inode);
    pub fn nfs_update_delegated_mtime(inode: *mut inode);
    pub fn nfs_update_delegated_mtime_locked(inode: *mut inode);
}

extern "C" {
    pub static mut directory_delegations: bool;
}

/* The following inline functions directly preserve the C NFS_PROTO dispatch. */
#[inline]
pub unsafe fn nfs_have_read_or_write_delegation(inode: *mut inode) -> i32 {
    nfs4_have_delegation(inode, FMODE_READ, 0)
}
#[inline]
pub unsafe fn nfs_have_write_delegation(inode: *mut inode) -> i32 {
    nfs4_have_delegation(inode, FMODE_WRITE, 0)
}
#[inline]
pub unsafe fn nfs_have_delegated_attributes(inode: *mut inode) -> i32 {
    nfs4_have_delegation(inode, FMODE_READ, 0)
}
#[inline]
pub unsafe fn nfs_have_delegated_atime(inode: *mut inode) -> i32 {
    nfs4_have_delegation(inode, FMODE_READ, NFS_DELEGATION_FLAG_TIME as i32)
}
#[inline]
pub unsafe fn nfs_have_delegated_mtime(inode: *mut inode) -> i32 {
    nfs4_have_delegation(inode, FMODE_WRITE, NFS_DELEGATION_FLAG_TIME as i32)
}
#[inline]
pub unsafe fn nfs_request_directory_delegation(inode: *mut inode) {
    if S_ISDIR((*inode).i_mode) {
        set_bit(NFS_INO_REQ_DIR_DELEG, &mut (*NFS_I(inode)).flags);
    }
}
#[inline]
pub unsafe fn nfs_have_directory_delegation(inode: *mut inode) -> bool {
    S_ISDIR((*inode).i_mode) && nfs_have_delegated_attributes(inode) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
