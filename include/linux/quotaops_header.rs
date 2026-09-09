/* SPDX-License-Identifier: GPL-2.0 */
/* Definitions for diskquota operations. */

pub const DQUOT_SPACE_WARN: u32 = 0x1;
pub const DQUOT_SPACE_RESERVE: u32 = 0x2;
pub const DQUOT_SPACE_NOFAIL: u32 = 0x4;

#[inline]
pub unsafe fn sb_dqopt(sb: *mut super_block) -> *mut quota_info {
    &mut (*sb).s_dquot
}

/* i_rwsem must be held. */
#[inline]
pub unsafe fn is_quota_modification(
    idmap: *mut mnt_idmap,
    inode: *mut inode,
    ia: *mut iattr,
) -> bool {
    ((*ia).ia_valid & ATTR_SIZE) != 0
        || i_uid_needs_update(idmap, ia, inode)
        || i_gid_needs_update(idmap, ia, inode)
}

#[cfg(CONFIG_QUOTA)]
extern "C" {
    pub fn __quota_error(sb: *mut super_block, func: *const core::ffi::c_char,
                         fmt: *const core::ffi::c_char, ...);
    pub fn dquot_initialize(inode: *mut inode) -> i32;
    pub fn dquot_initialize_needed(inode: *mut inode) -> bool;
    pub fn dquot_drop(inode: *mut inode);
    pub fn dqget(sb: *mut super_block, qid: kqid) -> *mut dquot;
    pub fn dqgrab(dquot: *mut dquot) -> *mut dquot;
    pub fn dqput(dquot: *mut dquot);
    pub fn dquot_scan_active(sb: *mut super_block,
        func: Option<unsafe extern "C" fn(*mut dquot, usize) -> i32>, priv_: usize) -> i32;
    pub fn dquot_alloc(sb: *mut super_block, type_: i32) -> *mut dquot;
    pub fn dquot_destroy(dquot: *mut dquot);
    pub fn __dquot_alloc_space(inode: *mut inode, number: qsize_t, flags: i32) -> i32;
    pub fn __dquot_free_space(inode: *mut inode, number: qsize_t, flags: i32);
    pub fn dquot_alloc_inode(inode: *mut inode) -> i32;
    pub fn dquot_claim_space_nodirty(inode: *mut inode, number: qsize_t);
    pub fn dquot_free_inode(inode: *mut inode);
    pub fn dquot_reclaim_space_nodirty(inode: *mut inode, number: qsize_t);
    pub fn dquot_disable(sb: *mut super_block, type_: i32, flags: u32) -> i32;
    pub fn dquot_resume(sb: *mut super_block, type_: i32) -> i32;
    pub fn dquot_commit(dquot: *mut dquot) -> i32;
    pub fn dquot_acquire(dquot: *mut dquot) -> i32;
    pub fn dquot_release(dquot: *mut dquot) -> i32;
    pub fn dquot_commit_info(sb: *mut super_block, type_: i32) -> i32;
    pub fn dquot_get_next_id(sb: *mut super_block, qid: *mut kqid) -> i32;
    pub fn dquot_mark_dquot_dirty(dquot: *mut dquot) -> i32;
    pub fn dquot_file_open(inode: *mut inode, file: *mut file) -> i32;
    pub fn dquot_load_quota_sb(sb: *mut super_block, type_: i32, format_id: i32, flags: u32) -> i32;
    pub fn dquot_load_quota_inode(inode: *mut inode, type_: i32, format_id: i32, flags: u32) -> i32;
    pub fn dquot_quota_on(sb: *mut super_block, type_: i32, format_id: i32, path: *const path) -> i32;
    pub fn dquot_quota_on_mount(sb: *mut super_block, qf_name: *mut i8, format_id: i32, type_: i32) -> i32;
    pub fn dquot_quota_off(sb: *mut super_block, type_: i32) -> i32;
    pub fn dquot_writeback_dquots(sb: *mut super_block, type_: i32) -> i32;
    pub fn dquot_quota_sync(sb: *mut super_block, type_: i32) -> i32;
    pub fn dquot_get_state(sb: *mut super_block, state: *mut qc_state) -> i32;
    pub fn dquot_set_dqinfo(sb: *mut super_block, type_: i32, ii: *mut qc_info) -> i32;
    pub fn dquot_get_dqblk(sb: *mut super_block, id: kqid, di: *mut qc_dqblk) -> i32;
    pub fn dquot_get_next_dqblk(sb: *mut super_block, id: *mut kqid, di: *mut qc_dqblk) -> i32;
    pub fn dquot_set_dqblk(sb: *mut super_block, id: kqid, di: *mut qc_dqblk) -> i32;
    pub fn __dquot_transfer(inode: *mut inode, transfer_to: *mut *mut dquot) -> i32;
    pub fn dquot_transfer(idmap: *mut mnt_idmap, inode: *mut inode, iattr: *mut iattr) -> i32;
    pub static dquot_operations: dquot_operations;
    pub static dquot_quotactl_sysfile_ops: quotactl_ops;
}

#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn dquot_is_busy(d: *mut dquot) -> bool {
    if test_bit(DQ_MOD_B, &mut (*d).dq_flags) { return true; }
    if atomic_read(&(*d).dq_count) > 0 { return true; }
    false
}

#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_dqinfo(sb: *mut super_block, type_: i32) -> *mut mem_dqinfo {
    (*sb_dqopt(sb)).info.add(type_ as usize)
}
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_has_quota_usage_enabled(sb:*mut super_block,t:i32)->bool { ((*sb_dqopt(sb)).flags & dquot_state_flag(DQUOT_USAGE_ENABLED,t)) != 0 }
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_has_quota_limits_enabled(sb:*mut super_block,t:i32)->bool { ((*sb_dqopt(sb)).flags & dquot_state_flag(DQUOT_LIMITS_ENABLED,t)) != 0 }
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_has_quota_suspended(sb:*mut super_block,t:i32)->bool { ((*sb_dqopt(sb)).flags & dquot_state_flag(DQUOT_SUSPENDED,t)) != 0 }
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_any_quota_suspended(sb:*mut super_block)->u32 { dquot_state_types((*sb_dqopt(sb)).flags,DQUOT_SUSPENDED) }
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_has_quota_loaded(sb:*mut super_block,t:i32)->bool { sb_has_quota_usage_enabled(sb,t) }
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_any_quota_loaded(sb:*mut super_block)->u32 { dquot_state_types((*sb_dqopt(sb)).flags,DQUOT_USAGE_ENABLED) }
#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn sb_has_quota_active(sb:*mut super_block,t:i32)->bool { sb_has_quota_loaded(sb,t) && !sb_has_quota_suspended(sb,t) }

#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_has_quota_usage_enabled(_: *mut super_block, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_has_quota_limits_enabled(_: *mut super_block, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_has_quota_suspended(_: *mut super_block, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_any_quota_suspended(_: *mut super_block) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_has_quota_loaded(_: *mut super_block, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_any_quota_loaded(_: *mut super_block) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn sb_has_quota_active(_: *mut super_block, _: i32) -> i32 { 0 }

#[cfg(CONFIG_QUOTA)]
#[inline] pub unsafe fn dquot_suspend(sb: *mut super_block, type_: i32) -> i32 {
    dquot_disable(sb, type_, DQUOT_SUSPENDED)
}

#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn __dquot_alloc_space(inode: *mut inode, number: qsize_t, flags: i32) -> i32 {
    if (flags as u32 & DQUOT_SPACE_RESERVE) == 0 { inode_add_bytes(inode, number); }
    0
}
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn __dquot_free_space(inode: *mut inode, number: qsize_t, flags: i32) {
    if (flags as u32 & DQUOT_SPACE_RESERVE) == 0 { inode_sub_bytes(inode, number); }
}

#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_claim_space_nodirty(inode: *mut inode, number: qsize_t) { inode_add_bytes(inode, number); }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_reclaim_space_nodirty(inode: *mut inode, number: qsize_t) -> i32 { inode_sub_bytes(inode, number); 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_alloc_inode(_: *mut inode) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_free_inode(_: *mut inode) {}
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_initialize(_: *mut inode) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_initialize_needed(_: *mut inode) -> bool { false }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_drop(_: *mut inode) {}
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_transfer(_: *mut mnt_idmap, _: *mut inode, _: *mut iattr) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_disable(_: *mut super_block, _: i32, _: u32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_suspend(_: *mut super_block, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_resume(_: *mut super_block, _: i32) -> i32 { 0 }
#[cfg(not(CONFIG_QUOTA))]
#[inline] pub unsafe fn dquot_writeback_dquots(_: *mut super_block, _: i32) -> i32 { 0 }

#[inline] pub unsafe fn dquot_alloc_space_nodirty(i: *mut inode, n: qsize_t) -> i32 { __dquot_alloc_space(i, n, DQUOT_SPACE_WARN as i32) }
#[inline] pub unsafe fn dquot_alloc_space_nofail(i: *mut inode, n: qsize_t) { __dquot_alloc_space(i, n, (DQUOT_SPACE_WARN|DQUOT_SPACE_NOFAIL) as i32); mark_inode_dirty_sync(i); }
#[inline] pub unsafe fn dquot_alloc_space(i: *mut inode, n: qsize_t) -> i32 { let r= dquot_alloc_space_nodirty(i,n); if r==0 { mark_inode_dirty(i); } r }
#[inline] pub unsafe fn dquot_alloc_block_nodirty(i:*mut inode,n:qsize_t)->i32 { dquot_alloc_space_nodirty(i,n << (*i).i_blkbits) }
#[inline] pub unsafe fn dquot_alloc_block_nofail(i:*mut inode,n:qsize_t) { dquot_alloc_space_nofail(i,n << (*i).i_blkbits); }
#[inline] pub unsafe fn dquot_alloc_block(i:*mut inode,n:qsize_t)->i32 { dquot_alloc_space(i,n << (*i).i_blkbits) }
#[inline] pub unsafe fn dquot_prealloc_block_nodirty(i:*mut inode,n:qsize_t)->i32 { __dquot_alloc_space(i,n << (*i).i_blkbits,0) }
#[inline] pub unsafe fn dquot_prealloc_block(i:*mut inode,n:qsize_t)->i32 { let r=dquot_prealloc_block_nodirty(i,n); if r==0 { mark_inode_dirty_sync(i); } r }
#[inline] pub unsafe fn dquot_reserve_block(i:*mut inode,n:qsize_t)->i32 { __dquot_alloc_space(i,n << (*i).i_blkbits,(DQUOT_SPACE_WARN|DQUOT_SPACE_RESERVE) as i32) }
#[inline] pub unsafe fn dquot_claim_block(i:*mut inode,n:qsize_t) { dquot_claim_space_nodirty(i,n << (*i).i_blkbits); mark_inode_dirty_sync(i); }
#[inline] pub unsafe fn dquot_reclaim_block(i:*mut inode,n:qsize_t) { dquot_reclaim_space_nodirty(i,n << (*i).i_blkbits); mark_inode_dirty_sync(i); }
#[inline] pub unsafe fn dquot_free_space_nodirty(i:*mut inode,n:qsize_t) { __dquot_free_space(i,n,0); }
#[inline] pub unsafe fn dquot_free_space(i:*mut inode,n:qsize_t) { dquot_free_space_nodirty(i,n); mark_inode_dirty_sync(i); }
#[inline] pub unsafe fn dquot_free_block_nodirty(i:*mut inode,n:qsize_t) { dquot_free_space_nodirty(i,n << (*i).i_blkbits); }
#[inline] pub unsafe fn dquot_free_block(i:*mut inode,n:qsize_t) { dquot_free_space(i,n << (*i).i_blkbits); }
#[inline] pub unsafe fn dquot_release_reservation_block(i:*mut inode,n:qsize_t) { __dquot_free_space(i,n << (*i).i_blkbits,DQUOT_SPACE_RESERVE as i32); }

extern "C" { pub fn qtype_enforce_flag(type_: i32) -> u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
