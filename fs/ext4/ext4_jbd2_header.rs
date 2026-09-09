// SPDX-License-Identifier: GPL-2.0+
/* Ext4-specific journaling extensions. */

// Dependencies supplied by the surrounding translation unit.

pub const EXT4_XATTR_TRANS_BLOCKS: u32 = 6;
pub const EXT4_MAX_TRANS_DATA: u32 = 64;
pub const EXT4_RESERVE_TRANS_BLOCKS: u32 = 12;
pub const EXT4_INDEX_EXTRA_TRANS_BLOCKS: u32 = 12;

pub const EXT4_HT_MISC: i32 = 0;
pub const EXT4_HT_INODE: i32 = 1;
pub const EXT4_HT_WRITE_PAGE: i32 = 2;
pub const EXT4_HT_MAP_BLOCKS: i32 = 3;
pub const EXT4_HT_DIR: i32 = 4;
pub const EXT4_HT_TRUNCATE: i32 = 5;
pub const EXT4_HT_QUOTA: i32 = 6;
pub const EXT4_HT_RESIZE: i32 = 7;
pub const EXT4_HT_MIGRATE: i32 = 8;
pub const EXT4_HT_MOVE_EXTENTS: i32 = 9;
pub const EXT4_HT_XATTR: i32 = 10;
pub const EXT4_HT_EXT_CONVERT: i32 = 11;
pub const EXT4_HT_MAX: i32 = 12;

pub const EXT4_INODE_JOURNAL_DATA_MODE: i32 = 0x01;
pub const EXT4_INODE_ORDERED_DATA_MODE: i32 = 0x02;
pub const EXT4_INODE_WRITEBACK_DATA_MODE: i32 = 0x04;

#[inline] pub unsafe fn ext4_singled_data_trans_blocks(sb: *mut super_block) -> u32 { if ext4_has_feature_extents(sb) != 0 { 20 } else { 8 } }
#[inline] pub unsafe fn ext4_data_trans_blocks(sb: *mut super_block) -> u32 { ext4_singled_data_trans_blocks(sb) + EXT4_XATTR_TRANS_BLOCKS - 2 + ext4_maxquotas_trans_blocks(sb) }
#[inline] pub unsafe fn ext4_meta_trans_blocks(sb: *mut super_block) -> u32 { EXT4_XATTR_TRANS_BLOCKS + ext4_maxquotas_trans_blocks(sb) }

#[cfg(feature = "CONFIG_QUOTA")]
#[inline] pub unsafe fn ext4_quota_trans_blocks(sb: *mut super_block) -> i32 { if ext4_quota_capable(sb) != 0 { 1 } else { 0 } }
#[cfg(not(feature = "CONFIG_QUOTA"))]
#[inline] pub unsafe fn ext4_quota_trans_blocks(_sb: *mut super_block) -> i32 { 0 }
#[inline] pub unsafe fn ext4_maxquotas_trans_blocks(sb: *mut super_block) -> u32 { (EXT4_MAXQUOTAS as u32) * ext4_quota_trans_blocks(sb) as u32 }

extern "C" {
    pub fn ext4_mark_iloc_dirty(handle: *mut handle_t, inode: *mut inode, iloc: *mut ext4_iloc) -> i32;
    pub fn ext4_reserve_inode_write(handle: *mut handle_t, inode: *mut inode, iloc: *mut ext4_iloc) -> i32;
    pub fn __ext4_mark_inode_dirty(handle: *mut handle_t, inode: *mut inode, func: *const c_char, line: u32) -> i32;
    pub fn ext4_expand_extra_isize(inode: *mut inode, new_extra_isize: u32, iloc: *mut ext4_iloc) -> i32;
    pub fn __ext4_journal_get_write_access(where_: *const c_char, line: u32, handle: *mut handle_t, sb: *mut super_block, bh: *mut buffer_head, trigger_type: ext4_journal_trigger_type) -> i32;
    pub fn __ext4_forget(where_: *const c_char, line: u32, handle: *mut handle_t, is_metadata: i32, inode: *mut inode, bh: *mut buffer_head, blocknr: ext4_fsblk_t) -> i32;
    pub fn __ext4_journal_get_create_access(where_: *const c_char, line: u32, handle: *mut handle_t, sb: *mut super_block, bh: *mut buffer_head, trigger_type: ext4_journal_trigger_type) -> i32;
    pub fn __ext4_handle_dirty_metadata(where_: *const c_char, line: u32, handle: *mut handle_t, inode: *mut inode, bh: *mut buffer_head) -> i32;
    pub fn __ext4_journal_start_sb(inode: *mut inode, sb: *mut super_block, line: u32, type_: i32, blocks: i32, rsv_blocks: i32, revoke_creds: i32) -> *mut handle_t;
    pub fn __ext4_journal_stop(where_: *const c_char, line: u32, handle: *mut handle_t) -> i32;
    pub fn __ext4_journal_start_reserved(handle: *mut handle_t, line: u32, type_: i32) -> *mut handle_t;
    pub fn __ext4_journal_ensure_credits(handle: *mut handle_t, check_cred: i32, extend_cred: i32, revoke_cred: i32) -> i32;
    pub fn ext4_force_commit(sb: *mut super_block) -> i32;
    pub fn ext4_inode_journal_mode(inode: *mut inode) -> i32;
}

#[inline] pub unsafe fn ext4_handle_valid(handle: *mut handle_t) -> bool { !handle.is_null() && (*handle).h_invalid == 0 }
#[inline] pub unsafe fn ext4_handle_sync(handle: *mut handle_t) { if ext4_handle_valid(handle) { (*handle).h_sync = 1; } }
#[inline] pub unsafe fn ext4_handle_is_aborted(handle: *mut handle_t) -> i32 { if ext4_handle_valid(handle) { is_handle_aborted(handle) } else { 0 } }
#[inline] pub unsafe fn ext4_free_metadata_revoke_credits(sb: *mut super_block, blocks: i32) -> i32 { blocks * (*EXT4_SB(sb)).s_cluster_ratio }
#[inline] pub unsafe fn ext4_trans_default_revoke_credits(sb: *mut super_block) -> i32 { ext4_free_metadata_revoke_credits(sb, 8) }
#[inline] pub unsafe fn ext4_journal_current_handle() -> *mut handle_t { journal_current_handle() }
#[inline] pub unsafe fn ext4_journal_extend(handle: *mut handle_t, nblocks: i32, revoke: i32) -> i32 { if ext4_handle_valid(handle) { jbd2_journal_extend(handle, nblocks, revoke) } else { 0 } }
#[inline] pub unsafe fn ext4_journal_restart(handle: *mut handle_t, nblocks: i32, revoke: i32) -> i32 { if ext4_handle_valid(handle) { jbd2__journal_restart(handle, nblocks, revoke, GFP_NOFS) } else { 0 } }
#[inline] pub unsafe fn ext4_journal_ensure_credits(handle: *mut handle_t, credits: i32, revoke_creds: i32) -> i32 { ext4_journal_ensure_credits_fn(handle, credits, credits, revoke_creds, 0) }
#[inline] pub unsafe fn ext4_journal_ensure_credits_fn(handle: *mut handle_t, check_cred: i32, extend_cred: i32, revoke_cred: i32, fn_result: i32) -> i32 { let mut err = __ext4_journal_ensure_credits(handle, check_cred, extend_cred, revoke_cred); if err <= 0 { return err; } err = fn_result; if err < 0 { return err; } err = ext4_journal_restart(handle, extend_cred, revoke_cred); if err == 0 { err = 1; } err }
#[inline] pub unsafe fn ext4_should_journal_data(inode: *mut inode) -> i32 { ext4_inode_journal_mode(inode) & EXT4_INODE_JOURNAL_DATA_MODE }
#[inline] pub unsafe fn ext4_should_order_data(inode: *mut inode) -> i32 { ext4_inode_journal_mode(inode) & EXT4_INODE_ORDERED_DATA_MODE }
#[inline] pub unsafe fn ext4_should_writeback_data(inode: *mut inode) -> i32 { ext4_inode_journal_mode(inode) & EXT4_INODE_WRITEBACK_DATA_MODE }

#[inline] pub unsafe fn ext4_journal_blocks_per_folio(inode: *mut inode) -> i32 { if ext4_journal(inode) != core::ptr::null_mut() { jbd2_journal_blocks_per_folio(inode) } else { 0 } }
#[inline] pub unsafe fn ext4_journal_force_commit(journal: *mut journal_t) -> i32 { if !journal.is_null() { jbd2_journal_force_commit(journal) } else { 0 } }
#[inline] pub unsafe fn ext4_jbd2_inode_add_write(handle: *mut handle_t, inode: *mut inode, start_byte: loff_t, length: loff_t) -> i32 { if ext4_handle_valid(handle) { jbd2_journal_inode_ranged_write(handle, (*EXT4_I(inode)).jinode, start_byte, length) } else { 0 } }
#[inline] pub unsafe fn ext4_jbd2_inode_add_wait(handle: *mut handle_t, inode: *mut inode, start_byte: loff_t, length: loff_t) -> i32 { if ext4_handle_valid(handle) { jbd2_journal_inode_ranged_wait(handle, (*EXT4_I(inode)).jinode, start_byte, length) } else { 0 } }
#[inline] pub unsafe fn ext4_should_dioread_nolock(inode: *mut inode) -> i32 { if test_opt((*inode).i_sb, DIOREAD_NOLOCK) == 0 || S_ISREG((*inode).i_mode) == 0 || ext4_test_inode_flag(inode, EXT4_INODE_EXTENTS) == 0 || ext4_should_journal_data(inode) != 0 || test_opt((*inode).i_sb, DELALLOC) == 0 { 0 } else { 1 } }

pub const EXT4_JOURNAL_DESTROY: u32 = 0; // supplied by ext4.h
#[inline] pub unsafe fn ext4_journal_destroy(sbi: *mut ext4_sb_info, journal: *mut journal_t) -> i32 { ext4_set_mount_flag((*sbi).s_sb, EXT4_MF_JOURNAL_DESTROY); ext4_force_commit((*sbi).s_sb); flush_work(&mut (*sbi).s_sb_upd_work); let err = jbd2_journal_destroy(journal); (*sbi).s_journal = core::ptr::null_mut(); err }

#[inline] pub unsafe fn ext4_free_data_revoke_credits(inode: *mut inode, blocks: i32) -> i32 { if test_opt((*inode).i_sb, DATA_FLAGS) == EXT4_MOUNT_JOURNAL_DATA || ext4_should_journal_data(inode) == 0 { 0 } else { blocks + 2 * ((*EXT4_SB((*inode).i_sb)).s_cluster_ratio - 1) } }
#[inline] pub unsafe fn ext4_handle_sync_update(handle: *mut handle_t, inode: *mut inode, datasync: i32) { let ei = EXT4_I(inode); if ext4_handle_valid(handle) && is_handle_aborted(handle) == 0 { (*ei).i_sync_tid = (*(*handle).h_transaction).t_tid; if datasync != 0 { (*ei).i_datasync_tid = (*(*handle).h_transaction).t_tid; } } }
#[inline] pub unsafe fn ext4_journal_start_sb(sb: *mut super_block, type_: i32, nblocks: i32) -> *mut handle_t { __ext4_journal_start_sb(core::ptr::null_mut(), sb, line!(), type_, nblocks, 0, ext4_trans_default_revoke_credits(sb)) }
#[inline] pub unsafe fn ext4_journal_stop(handle: *mut handle_t) -> i32 { __ext4_journal_stop(core::ptr::null(), line!(), handle) }
#[inline] pub unsafe fn ext4_journal_start_reserved(handle: *mut handle_t, type_: i32) -> *mut handle_t { __ext4_journal_start_reserved(handle, line!(), type_) }

// External declarations from the included kernel headers.
extern "C" {
    pub fn ext4_has_feature_extents(sb: *mut super_block) -> i32;
    pub fn ext4_quota_capable(sb: *mut super_block) -> i32;
    pub fn ext4_journal(inode: *mut inode) -> *mut journal_t;
    pub fn is_handle_aborted(handle: *mut handle_t) -> i32;
    pub fn journal_current_handle() -> *mut handle_t;
    pub fn jbd2_journal_extend(handle: *mut handle_t, nblocks: i32, revoke: i32) -> i32;
    pub fn jbd2__journal_restart(handle: *mut handle_t, nblocks: i32, revoke: i32, gfp: u32) -> i32;
    pub fn jbd2_journal_blocks_per_folio(inode: *mut inode) -> i32;
    pub fn jbd2_journal_force_commit(journal: *mut journal_t) -> i32;
    pub fn jbd2_journal_inode_ranged_write(handle: *mut handle_t, jinode: *mut jbd2_inode, start: loff_t, len: loff_t) -> i32;
    pub fn jbd2_journal_inode_ranged_wait(handle: *mut handle_t, jinode: *mut jbd2_inode, start: loff_t, len: loff_t) -> i32;
    pub fn ext4_test_inode_flag(inode: *mut inode, flag: i32) -> i32;
    pub fn test_opt(sb: *mut super_block, opt: i32) -> i32;
    pub fn ext4_set_mount_flag(sb: *mut super_block, flag: i32);
    pub fn flush_work(work: *mut work_struct);
    pub fn jbd2_journal_destroy(journal: *mut journal_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
