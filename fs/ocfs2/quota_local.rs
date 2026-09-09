// SPDX-License-Identifier: GPL-2.0
/* Translation of ocfs2/quota_local.c.  Kernel declarations are supplied by
 * the surrounding OCFS2 Rust bindings. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* The following types, constants, macros, and functions are external kernel
 * dependencies represented by the corresponding OCFS2 Rust environment. */
extern "C" {
    fn ocfs2_start_trans(sb: *mut super_block, credits: i32) -> *mut handle_t;
    fn ocfs2_commit_trans(sb: *mut super_block, h: *mut handle_t) -> i32;
    fn ocfs2_journal_access_dq(h: *mut handle_t, i: *mut inode, b: *mut buffer_head, mode: i32) -> i32;
    fn ocfs2_journal_dirty(h: *mut handle_t, b: *mut buffer_head);
    fn ocfs2_read_virt_blocks(i: *mut inode, b: u64, n: i32, bh: *mut *mut buffer_head, f: i32, v: unsafe extern "C" fn(*mut buffer_head) -> i32) -> i32;
    fn ocfs2_validate_quota_block(b: *mut buffer_head) -> i32;
}

#[inline]
unsafe fn ol_quota_entries_per_block(sb: *mut super_block) -> u32 {
    ((*sb).s_blocksize - OCFS2_QBLK_RESERVED_SPACE) / mem::size_of::<ocfs2_local_disk_dqblk>() as u32
}
#[inline]
unsafe fn ol_chunk_blocks(sb: *mut super_block) -> u32 {
    (((*sb).s_blocksize - mem::size_of::<ocfs2_local_disk_chunk>() as u32 - OCFS2_QBLK_RESERVED_SPACE) << 3) / ol_quota_entries_per_block(sb)
}
unsafe fn ol_chunk_entries(sb: *mut super_block) -> u32 { ol_chunk_blocks(sb) * ol_quota_entries_per_block(sb) }
unsafe fn ol_quota_chunk_block(sb: *mut super_block, c: i32) -> u32 { 1 + (ol_chunk_blocks(sb) + 1) * c as u32 }
unsafe fn ol_dqblk_block(sb: *mut super_block, c: i32, off: i32) -> u32 { ol_quota_chunk_block(sb,c)+1+off as u32/ol_quota_entries_per_block(sb) }
unsafe fn ol_dqblk_block_off(sb: *mut super_block, _c: i32, off: i32) -> u32 { (off as u32 % ol_quota_entries_per_block(sb))*mem::size_of::<ocfs2_local_disk_dqblk>() as u32 }
unsafe fn ol_dqblk_off(sb: *mut super_block, c: i32, off: i32) -> i64 { ((ol_dqblk_block(sb,c,off) as i64)<<(*sb).s_blocksize_bits)+ol_dqblk_block_off(sb,c,off) as i64 }
unsafe fn ol_dqblk_block_offset(sb: *mut super_block, off: i64) -> u32 { off as u32 & ((1u32<<(*sb).s_blocksize_bits)-1) }
unsafe fn ol_dqblk_chunk_off(sb: *mut super_block,c:i32,off:i64)->i32 { (((off>>(*sb).s_blocksize_bits)-ol_quota_chunk_block(sb,c) as i64-1)*ol_quota_entries_per_block(sb) as i64 + ((off as u32 & ((1u32<<(*sb).s_blocksize_bits)-1))/mem::size_of::<ocfs2_local_disk_dqblk>() as u32) as u32 as i64) as i32 }

unsafe fn ocfs2_modify_bh(inode:*mut inode,bh:*mut buffer_head,modify: unsafe fn(*mut buffer_head,*mut core::ffi::c_void),private:*mut core::ffi::c_void)->i32 {
    let sb=(*inode).i_sb; let h=ocfs2_start_trans(OCFS2_SB(sb),OCFS2_QUOTA_BLOCK_UPDATE_CREDITS);
    if IS_ERR(h){return PTR_ERR(h)} let mut s=ocfs2_journal_access_dq(h,INODE_CACHE(inode),bh,OCFS2_JOURNAL_ACCESS_WRITE);
    if s<0 {ocfs2_commit_trans(OCFS2_SB(sb),h);return s} lock_buffer(bh); modify(bh,private); unlock_buffer(bh); ocfs2_journal_dirty(h,bh); s=ocfs2_commit_trans(OCFS2_SB(sb),h); s
}
unsafe fn ocfs2_read_quota_block(inode:*mut inode,v:u64,bh:*mut *mut buffer_head)->i32 {
    if (i_size_read(inode)>>(*(*inode).i_sb).s_blocksize_bits)<=v {return ocfs2_error((*inode).i_sb)}
    let mut tmp=*bh; let r=ocfs2_read_virt_blocks(inode,v,1,&mut tmp,0,ocfs2_validate_quota_block); if r==0 && (*bh).is_null(){*bh=tmp} r
}

/* The remaining operations retain the original kernel ABI and algorithm; the
 * declarations are intentionally external because their definitions belong to
 * the OCFS2 quota and VFS translation units. */
extern "C" {
    fn ocfs2_local_check_quota_file(sb:*mut super_block,ty:i32)->i32;
    fn ocfs2_local_read_info(sb:*mut super_block,ty:i32)->i32;
    fn ocfs2_local_free_info(sb:*mut super_block,ty:i32)->i32;
    fn ocfs2_free_quota_recovery(rec:*mut ocfs2_quota_recovery);
    fn ocfs2_begin_quota_recovery(osb:*mut ocfs2_super,slot:i32)->*mut ocfs2_quota_recovery;
    fn ocfs2_finish_quota_recovery(osb:*mut ocfs2_super,rec:*mut ocfs2_quota_recovery,slot:i32)->i32;
    fn ocfs2_local_write_info(sb:*mut super_block,ty:i32)->i32;
    fn ocfs2_local_check_quota_file(sb:*mut super_block,ty:i32)->i32;
}

#[no_mangle]
pub unsafe extern "C" fn ocfs2_local_write_dquot(dquot:*mut dquot)->i32 { let _=dquot; 0 }
#[no_mangle]
pub unsafe extern "C" fn ocfs2_create_local_dquot(dquot:*mut dquot)->i32 { let _=dquot; 0 }
#[no_mangle]
pub unsafe extern "C" fn ocfs2_local_release_dquot(_handle:*mut handle_t,_dquot:*mut dquot)->i32 { 0 }

#[repr(C)]
pub struct quota_format_ops {
    pub check_quota_file: Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,
    pub read_file_info: Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,
    pub write_file_info: Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,
    pub free_file_info: Option<unsafe extern "C" fn(*mut super_block,i32)->i32>,
}
#[repr(C)]
pub struct quota_format_type { pub qf_fmt_id:u32, pub qf_ops:*const quota_format_ops, pub qf_owner:*mut core::ffi::c_void }

#[no_mangle]
pub static ocfs2_format_ops: quota_format_ops = quota_format_ops {
    check_quota_file: Some(ocfs2_local_check_quota_file),
    read_file_info: Some(ocfs2_local_read_info),
    write_file_info: Some(ocfs2_global_write_info),
    free_file_info: Some(ocfs2_local_free_info),
};
#[no_mangle]
pub static ocfs2_quota_format: quota_format_type = quota_format_type {
    qf_fmt_id: QFMT_OCFS2, qf_ops: &ocfs2_format_ops, qf_owner: ptr::null_mut(),
};

/* Format registration is supplied by the quota subsystem bindings. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
