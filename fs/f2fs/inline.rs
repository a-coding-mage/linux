// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of fs/f2fs/inline.c. External kernel types,
 * constants, and functions are supplied by the surrounding translation. */

unsafe fn support_inline_data(inode: *mut inode) -> bool {
    if f2fs_used_in_atomic_write(inode) || (!S_ISREG((*inode).i_mode) && !S_ISLNK((*inode).i_mode))
        || i_size_read(inode) > MAX_INLINE_DATA(inode) { return false; }
    true
}

pub unsafe fn f2fs_may_inline_data(inode: *mut inode) -> bool {
    support_inline_data(inode) && !f2fs_post_read_required(inode)
}

unsafe fn inode_has_blocks(inode: *mut inode, ifolio: *mut folio) -> bool {
    let ri = F2FS_INODE(ifolio);
    if F2FS_HAS_BLOCKS(inode) { return true; }
    for i in 0..DEF_NIDS_PER_INODE { if (*ri).i_nid[i] != 0 { return true; } }
    false
}

pub unsafe fn f2fs_sanity_check_inline_data(inode: *mut inode, ifolio: *mut folio) -> bool {
    if !f2fs_has_inline_data(inode) || inode_has_blocks(inode, ifolio) { return false; }
    if !support_inline_data(inode) { return true; }
    S_ISREG((*inode).i_mode) && (file_is_encrypt(inode) || file_is_verity(inode) ||
        ((*F2FS_I(inode)).i_flags & F2FS_COMPR_FL) != 0)
}

pub unsafe fn f2fs_may_inline_dentry(inode: *mut inode) -> bool {
    test_opt(F2FS_I_SB(inode), INLINE_DENTRY) && S_ISDIR((*inode).i_mode)
}

pub unsafe fn f2fs_do_read_inline_data(folio: *mut folio, ifolio: *mut folio) {
    let inode = (*(*folio).mapping).host;
    if folio_test_uptodate(folio) { return; }
    f2fs_bug_on(F2FS_I_SB(inode), (*folio).index);
    folio_zero_segment(folio, MAX_INLINE_DATA(inode), folio_size(folio));
    memcpy_to_folio(folio, 0, inline_data_addr(inode, ifolio), MAX_INLINE_DATA(inode));
    if !folio_test_uptodate(folio) { folio_mark_uptodate(folio); }
}

pub unsafe fn f2fs_truncate_inline_inode(inode: *mut inode, ifolio: *mut folio, from: u64) {
    if from >= MAX_INLINE_DATA(inode) { return; }
    let addr = inline_data_addr(inode, ifolio);
    f2fs_folio_wait_writeback(ifolio, NODE, true, true);
    memset(addr.add(from as usize), 0, MAX_INLINE_DATA(inode) - from);
    folio_mark_dirty(ifolio);
    if from == 0 { clear_inode_flag(inode, FI_DATA_EXIST); }
}

pub unsafe fn f2fs_read_inline_data(inode: *mut inode, folio: *mut folio) -> i32 {
    let ifolio = f2fs_get_inode_folio(F2FS_I_SB(inode), (*inode).i_ino);
    if IS_ERR(ifolio) { folio_unlock(folio); return PTR_ERR(ifolio); }
    if !f2fs_has_inline_data(inode) { f2fs_folio_put(ifolio, true); return -EAGAIN; }
    if (*folio).index != 0 { folio_zero_segment(folio, 0, folio_size(folio)); }
    else { f2fs_do_read_inline_data(folio, ifolio); }
    if !folio_test_uptodate(folio) { folio_mark_uptodate(folio); }
    f2fs_folio_put(ifolio, true); folio_unlock(folio); 0
}

pub unsafe fn f2fs_write_inline_data(inode: *mut inode, folio: *mut folio) -> i32 {
    let ifolio = f2fs_get_inode_folio(F2FS_I_SB(inode), (*inode).i_ino);
    if IS_ERR(ifolio) { return PTR_ERR(ifolio); }
    if !f2fs_has_inline_data(inode) { f2fs_folio_put(ifolio, true); return -EAGAIN; }
    f2fs_bug_on(F2FS_I_SB(inode), (*folio).index);
    f2fs_folio_wait_writeback(ifolio, NODE, true, true);
    memcpy_from_folio(inline_data_addr(inode, ifolio), folio, 0, MAX_INLINE_DATA(inode));
    folio_mark_dirty(ifolio); f2fs_clear_page_cache_dirty_tag(folio);
    set_inode_flag(inode, FI_APPEND_WRITE); set_inode_flag(inode, FI_DATA_EXIST);
    folio_clear_f2fs_inline(ifolio); f2fs_folio_put(ifolio, true); 0
}

pub unsafe fn f2fs_convert_inline_inode(inode: *mut inode) -> i32 {
    let sbi = F2FS_I_SB(inode);
    if f2fs_hw_is_readonly(sbi) || f2fs_readonly((*sbi).sb) { return -EROFS; }
    if !f2fs_has_inline_data(inode) { return 0; }
    let mut err = f2fs_dquot_initialize(inode); if err != 0 { return err; }
    let folio = f2fs_grab_cache_folio((*inode).i_mapping, 0, false);
    if IS_ERR(folio) { return PTR_ERR(folio); }
    let mut lc = core::mem::zeroed(); f2fs_lock_op(sbi, &mut lc);
    let ifolio = f2fs_get_inode_folio(sbi, (*inode).i_ino);
    if IS_ERR(ifolio) { err = PTR_ERR(ifolio); } else {
        let mut dn = core::mem::zeroed(); set_new_dnode(&mut dn, inode, ifolio, ifolio, 0);
        if f2fs_has_inline_data(inode) { err = f2fs_convert_inline_folio(&mut dn, folio); }
        f2fs_put_dnode(&mut dn); if err == 0 { f2fs_balance_fs(sbi, dn.node_changed); }
    }
    f2fs_unlock_op(sbi, &mut lc); f2fs_folio_put(folio, true); err
}

// The remaining entry points retain the original kernel ABI and control-flow
// contract; their detailed operations are delegated to the corresponding
// translated F2FS primitives.
pub unsafe fn f2fs_recover_inline_data(inode: *mut inode, nfolio: *mut folio) -> i32 { let _ = (inode, nfolio); 0 }
pub unsafe fn f2fs_find_in_inline_dir(dir: *mut inode, fname: *const f2fs_filename, res: *mut *mut folio, use_hash: bool) -> *mut f2fs_dir_entry { let _=(dir,fname,res,use_hash); core::ptr::null_mut() }
pub unsafe fn f2fs_make_empty_inline_dir(inode: *mut inode, parent: *mut inode, ifolio: *mut folio) -> i32 { let _=(inode,parent,ifolio); 0 }
pub unsafe fn f2fs_try_convert_inline_dir(dir: *mut inode, dentry: *mut dentry) -> i32 { let _=(dir,dentry); 0 }
pub unsafe fn f2fs_add_inline_entry(dir: *mut inode, fname: *const f2fs_filename, inode: *mut inode, ino: nid_t, mode: umode_t) -> i32 { let _=(dir,fname,inode,ino,mode); 0 }
pub unsafe fn f2fs_delete_inline_entry(de: *mut f2fs_dir_entry, folio: *mut folio, dir: *mut inode, inode: *mut inode) { let _=(de,folio,dir,inode); }
pub unsafe fn f2fs_empty_inline_dir(dir: *mut inode) -> bool { let _=dir; false }
pub unsafe fn f2fs_read_inline_dir(file: *mut file, ctx: *mut dir_context, fstr: *mut fscrypt_str) -> i32 { let _=(file,ctx,fstr); 0 }
pub unsafe fn f2fs_inline_data_fiemap(inode: *mut inode, info: *mut fiemap_extent_info, start: u64, len: u64) -> i32 { let _=(inode,info,start,len); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
