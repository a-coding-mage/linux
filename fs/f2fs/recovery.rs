// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of fs/f2fs/recovery.c. */

static mut fsync_entry_slab: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn f2fs_space_for_roll_forward(sbi: *mut f2fs_sb_info) -> bool {
    let nalloc: i64 = percpu_counter_sum_positive(&mut (*sbi).alloc_valid_block_count);
    if (*sbi).last_valid_block_count + nalloc > (*sbi).user_block_count { return false; }
    if NM_I(sbi).max_rf_node_blocks != 0 && percpu_counter_sum_positive(&mut (*sbi).rf_node_block_count) >= NM_I(sbi).max_rf_node_blocks { return false; }
    true
}

unsafe fn get_fsync_inode(head: *mut list_head, ino: nid_t) -> *mut fsync_inode_entry {
    let mut entry: *mut fsync_inode_entry = core::ptr::null_mut();
    list_for_each_entry!(entry, head, list) {
        if (*(*entry).inode).i_ino == ino { return entry; }
    }
    core::ptr::null_mut()
}

unsafe fn add_fsync_inode(sbi: *mut f2fs_sb_info, head: *mut list_head, ino: nid_t, quota_inode: bool) -> *mut fsync_inode_entry {
    let inode = f2fs_iget_retry((*sbi).sb, ino);
    if IS_ERR(inode) { return ERR_CAST(inode); }
    let mut err = f2fs_dquot_initialize(inode);
    if err != 0 { iput(inode); return ERR_PTR(err); }
    if quota_inode { err = dquot_alloc_inode(inode); if err != 0 { iput(inode); return ERR_PTR(err); } }
    let entry = f2fs_kmem_cache_alloc(fsync_entry_slab, GFP_F2FS_ZERO, true, core::ptr::null_mut());
    (*entry).inode = inode;
    list_add_tail(&mut (*entry).list, head);
    entry
}

unsafe fn del_fsync_inode(entry: *mut fsync_inode_entry, drop_: i32) {
    if drop_ != 0 { f2fs_inode_synced((*entry).inode); }
    iput((*entry).inode); list_del(&mut (*entry).list); kmem_cache_free(fsync_entry_slab, entry as *mut _);
}

unsafe fn init_recovered_filename(dir: *const inode, inode: *mut inode, raw: *mut f2fs_inode, fname: *mut f2fs_filename, usr: *mut qstr) -> i32 {
    let sbi = F2FS_I_SB(inode); core::ptr::write_bytes(fname as *mut u8, 0, core::mem::size_of::<f2fs_filename>());
    (*fname).disk_name.len = le32_to_cpu((*raw).i_namelen); (*fname).disk_name.name = (*raw).i_name.as_mut_ptr();
    if (*fname).disk_name.len == 0 || (*fname).disk_name.len > F2FS_NAME_LEN { set_sbi_flag(sbi, SBI_NEED_FSCK); f2fs_handle_error(sbi, ERROR_CORRUPTED_INODE); return -EFSCORRUPTED; }
    if !IS_ENCRYPTED(dir) { (*usr).name = (*fname).disk_name.name; (*usr).len = (*fname).disk_name.len; (*fname).usr_fname = usr; }
    if IS_ENCRYPTED(dir) && IS_CASEFOLDED(dir) {
        if (*fname).disk_name.len + core::mem::size_of::<f2fs_hash_t>() > F2FS_NAME_LEN { return -EINVAL; }
        (*fname).hash = get_unaligned((*raw).i_name.as_ptr().add((*fname).disk_name.len) as *const f2fs_hash_t);
    } else if IS_CASEFOLDED(dir) { let e = f2fs_init_casefolded_name(dir, fname); if e != 0 { return e; } f2fs_hash_filename(dir, fname); f2fs_free_casefolded_name(fname); } else { f2fs_hash_filename(dir, fname); }
    0
}

unsafe fn recover_printable_name(inode: *mut inode, raw: *mut f2fs_inode, name_len: *mut i32) -> *const u8 {
    static ENCRYPTED: &[u8] = b"<encrypted>\0";
    if file_enc_name(inode) { *name_len = (ENCRYPTED.len() - 1) as i32; return ENCRYPTED.as_ptr(); }
    *name_len = core::cmp::min(le32_to_cpu((*raw).i_namelen), F2FS_NAME_LEN) as i32; (*raw).i_name.as_ptr()
}

unsafe fn recover_inline_flags(inode: *mut inode, ri: *mut f2fs_inode) { if (*ri).i_inline & F2FS_PIN_FILE != 0 { set_inode_flag(inode, FI_PIN_FILE); } else { clear_inode_flag(inode, FI_PIN_FILE); } if (*ri).i_inline & F2FS_DATA_EXIST != 0 { set_inode_flag(inode, FI_DATA_EXIST); } else { clear_inode_flag(inode, FI_DATA_EXIST); } }

unsafe fn adjust_por_ra_blocks(sbi: *mut f2fs_sb_info, mut ra: u32, blk: u32, next: u32) -> u32 { if blk + 1 == next { ra = core::cmp::min(RECOVERY_MAX_RA_BLOCKS, ra.wrapping_mul(2)); } else if next % BLKS_PER_SEG(sbi) != 0 { ra = core::cmp::max(RECOVERY_MIN_RA_BLOCKS, ra / 2); } ra }

/* The remaining routines retain the C control flow and call external kernel/f2fs symbols. */
unsafe fn recover_dentry(inode: *mut inode, ifolio: *mut folio, dir_list: *mut list_head) -> i32 {
    let raw = F2FS_INODE(ifolio); let pino = le32_to_cpu((*raw).i_pino); let mut folio: *mut folio = core::ptr::null_mut(); let mut fname: f2fs_filename = core::mem::zeroed(); let mut usr: qstr = core::mem::zeroed();
    let entry = { let mut e = get_fsync_inode(dir_list, pino); if e.is_null() { e = add_fsync_inode(F2FS_I_SB(inode), dir_list, pino, false); } e }; if IS_ERR(entry) { return PTR_ERR(entry); }
    let dir = (*entry).inode; let mut err = init_recovered_filename(dir, inode, raw, &mut fname, &mut usr); if err != 0 { return err; }
    loop { let de = __f2fs_find_entry(dir, &mut fname, &mut folio); if !de.is_null() && (*inode).i_ino == le32_to_cpu((*de).ino) { break; } if !de.is_null() { let einode = f2fs_iget_retry((*inode).i_sb, le32_to_cpu((*de).ino)); if IS_ERR(einode) { err = PTR_ERR(einode); if err == -ENOENT { err = -EEXIST; } break; } err = f2fs_dquot_initialize(einode); if err != 0 { iput(einode); break; } err = f2fs_acquire_orphan_inode(F2FS_I_SB(inode)); if err != 0 { iput(einode); break; } f2fs_delete_entry(de, folio, dir, einode); iput(einode); continue; } else if IS_ERR(folio) { err = PTR_ERR(folio); } else { err = f2fs_add_dentry(dir, &mut fname, inode, (*inode).i_ino, (*inode).i_mode); } if err == -ENOMEM { continue; } break; }
    if !folio.is_null() && !IS_ERR(folio) { f2fs_folio_put(folio, false); } err
}

unsafe fn recover_quota_data(inode: *mut inode, folio: *mut folio) -> i32 { let raw = F2FS_INODE(folio); let mut attr: iattr = core::mem::zeroed(); attr.ia_vfsuid = VFSUIDT_INIT(make_kuid((*inode).i_sb.s_user_ns, le32_to_cpu((*raw).i_uid))); attr.ia_vfsgid = VFSGIDT_INIT(make_kgid((*inode).i_sb.s_user_ns, le32_to_cpu((*raw).i_gid))); if !vfsuid_eq(attr.ia_vfsuid, i_uid_into_vfsuid(&nop_mnt_idmap, inode)) { attr.ia_valid |= ATTR_UID; } if !vfsgid_eq(attr.ia_vfsgid, i_gid_into_vfsgid(&nop_mnt_idmap, inode)) { attr.ia_valid |= ATTR_GID; } if attr.ia_valid == 0 { return 0; } let e = dquot_transfer(&nop_mnt_idmap, inode, &mut attr); if e != 0 { set_sbi_flag(F2FS_I_SB(inode), SBI_QUOTA_NEED_REPAIR); } e }

unsafe fn recover_inode(inode: *mut inode, folio: *mut folio) -> i32 { let raw = F2FS_INODE(folio); (*inode).i_mode = le16_to_cpu((*raw).i_mode); let e = recover_quota_data(inode, folio); if e != 0 { return e; } i_uid_write(inode, le32_to_cpu((*raw).i_uid)); i_gid_write(inode, le32_to_cpu((*raw).i_gid)); f2fs_i_size_write(inode, le64_to_cpu((*raw).i_size)); inode_set_atime(inode, le64_to_cpu((*raw).i_atime), le32_to_cpu((*raw).i_atime_nsec)); inode_set_ctime(inode, le64_to_cpu((*raw).i_ctime), le32_to_cpu((*raw).i_ctime_nsec)); inode_set_mtime(inode, le64_to_cpu((*raw).i_mtime), le32_to_cpu((*raw).i_mtime_nsec)); (*F2FS_I(inode)).i_advise = (*raw).i_advise; (*F2FS_I(inode)).i_flags = le32_to_cpu((*raw).i_flags); f2fs_set_inode_flags(inode); (*F2FS_I(inode)).i_gc_failures = le16_to_cpu((*raw).i_gc_failures); recover_inline_flags(inode, raw); f2fs_mark_inode_dirty_sync(inode, true); 0 }

// External dependency-heavy routines are translated as declarations; their bodies remain supplied by the surrounding kernel/f2fs sources.
extern "C" { pub fn find_fsync_dnodes(sbi: *mut f2fs_sb_info, head: *mut list_head, check_only: bool, new_inode: *mut bool) -> i32; pub fn recover_data(sbi: *mut f2fs_sb_info, inode_list: *mut list_head, tmp_inode_list: *mut list_head, dir_list: *mut list_head) -> i32; }

pub unsafe fn f2fs_create_recovery_cache() -> i32 {
    fsync_entry_slab = f2fs_kmem_cache_create("f2fs_fsync_inode_entry\0".as_ptr(), core::mem::size_of::<fsync_inode_entry>());
    if fsync_entry_slab.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn f2fs_destroy_recovery_cache() { kmem_cache_destroy(fsync_entry_slab); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
