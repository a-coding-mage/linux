// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/affs/inode.c
 *
 *  (c) 1996  Hans-Joachim Widmaier - Rewritten
 *
 *  (C) 1993  Ray Burr - Modified for Amiga FFS filesystem.
 *
 *  (C) 1992  Eric Youngdale Modified for ISO9660 filesystem.
 *
 *  (C) 1991  Linus Torvalds - minix filesystem
 */

// Kernel dependencies supplied by the surrounding translation unit.

pub unsafe fn affs_iget(sb: *mut super_block, ino: ::core::ffi::c_ulong) -> *mut inode {
    let sbi = AFFS_SB(sb);
    let mut bh: *mut buffer_head;
    let tail: *mut affs_tail;
    let inode: *mut inode;
    let mut block: u32;
    let mut size: u32;
    let mut prot: u32;
    let mut id: u16;

    inode = iget_locked(sb, ino);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if (inode_state_read_once(inode) & I_NEW) == 0 { return inode; }
    pr_debug!("affs_iget({})\n", (*inode).i_ino);

    block = (*inode).i_ino as u32;
    bh = affs_bread(sb, block);
    if bh.is_null() { affs_warning(sb, "read_inode", "Cannot read block %d", block); goto_bad_inode!(inode, bh, -EIO); }
    if affs_checksum_block(sb, bh) != 0 || be32_to_cpu((*AFFS_HEAD(bh)).ptype) != T_SHORT {
        affs_warning(sb, "read_inode", "Checksum or type (ptype=%d) error on inode %d", (*AFFS_HEAD(bh)).ptype, block);
        goto_bad_inode!(inode, bh, -EIO);
    }
    tail = AFFS_TAIL(sb, bh);
    prot = be32_to_cpu((*tail).protect);
    (*inode).i_size = 0; set_nlink(inode, 1); (*inode).i_mode = 0;
    (*AFFS_I(inode)).i_extcnt = 1; (*AFFS_I(inode)).i_ext_last = !1;
    (*AFFS_I(inode)).i_protect = prot; atomic_set(&mut (*AFFS_I(inode)).i_opencnt, 0);
    (*AFFS_I(inode)).i_blkcnt = 0; (*AFFS_I(inode)).i_lc = core::ptr::null_mut();
    (*AFFS_I(inode)).i_lc_size = 0; (*AFFS_I(inode)).i_lc_shift = 0; (*AFFS_I(inode)).i_lc_mask = 0;
    (*AFFS_I(inode)).i_ac = core::ptr::null_mut(); (*AFFS_I(inode)).i_ext_bh = core::ptr::null_mut();
    (*AFFS_I(inode)).mmu_private = 0; (*AFFS_I(inode)).i_lastalloc = 0; (*AFFS_I(inode)).i_pa_cnt = 0;
    (*inode).i_mode = if affs_test_opt((*sbi).s_flags, SF_SETMODE) != 0 { (*sbi).s_mode } else { affs_prot_to_mode(prot) };
    id = be16_to_cpu((*tail).uid);
    if id == 0 || affs_test_opt((*sbi).s_flags, SF_SETUID) != 0 { (*inode).i_uid = (*sbi).s_uid; }
    else if id == 0xffff && affs_test_opt((*sbi).s_flags, SF_MUFS) != 0 { i_uid_write(inode, 0); } else { i_uid_write(inode, id); }
    id = be16_to_cpu((*tail).gid);
    if id == 0 || affs_test_opt((*sbi).s_flags, SF_SETGID) != 0 { (*inode).i_gid = (*sbi).s_gid; }
    else if id == 0xffff && affs_test_opt((*sbi).s_flags, SF_MUFS) != 0 { i_gid_write(inode, 0); } else { i_gid_write(inode, id); }

    match be32_to_cpu((*tail).stype) {
        ST_ROOT => { (*inode).i_uid = (*sbi).s_uid; (*inode).i_gid = (*sbi).s_gid; }
        ST_USERDIR => {
            if be32_to_cpu((*tail).stype) == ST_USERDIR || affs_test_opt((*sbi).s_flags, SF_SETMODE) != 0 {
                if (*inode).i_mode & S_IRUSR != 0 { (*inode).i_mode |= S_IXUSR; }
                if (*inode).i_mode & S_IRGRP != 0 { (*inode).i_mode |= S_IXGRP; }
                if (*inode).i_mode & S_IROTH != 0 { (*inode).i_mode |= S_IXOTH; }
                (*inode).i_mode |= S_IFDIR;
            } else { (*inode).i_mode = S_IRUGO | S_IXUGO | S_IWUSR | S_IFDIR; }
            (*inode).i_op = &affs_dir_inode_operations; (*inode).i_fop = &affs_dir_operations;
        }
        ST_LINKDIR => { (*inode).i_mode |= S_IFDIR; }
        ST_LINKFILE => { affs_warning(sb, "read_inode", "inode is LINKFILE"); goto_bad_inode!(inode, bh, -EIO); }
        ST_FILE => {
            size = be32_to_cpu((*tail).size); (*inode).i_mode |= S_IFREG; (*AFFS_I(inode)).mmu_private = size as _; (*inode).i_size = size as _;
            if (*inode).i_size != 0 { (*AFFS_I(inode)).i_blkcnt = (size - 1) / (*sbi).s_data_blksize + 1; (*AFFS_I(inode)).i_extcnt = ((*AFFS_I(inode)).i_blkcnt - 1) / (*sbi).s_hashsize + 1; }
            if (*tail).link_chain != 0 { set_nlink(inode, 2); }
            (*inode).i_mapping.a_ops = if affs_test_opt((*sbi).s_flags, SF_OFS) != 0 { &affs_aops_ofs } else { &affs_aops };
            (*inode).i_op = &affs_file_inode_operations; (*inode).i_fop = &affs_file_operations;
        }
        ST_SOFTLINK => { (*inode).i_size = strlen((*AFFS_HEAD(bh)).table as *const i8) as _; (*inode).i_mode |= S_IFLNK; inode_nohighmem(inode); (*inode).i_op = &affs_symlink_inode_operations; (*inode).i_data.a_ops = &affs_symlink_aops; }
        _ => {}
    }
    inode_set_mtime(inode, inode_set_atime(inode, inode_set_ctime(inode, (be32_to_cpu((*tail).change.days) as i64 * 86400 + be32_to_cpu((*tail).change.mins) as i64 * 60 + be32_to_cpu((*tail).change.ticks) as i64 / 50 + AFFS_EPOCH_DELTA as i64) + sys_tz.tz_minuteswest as i64 * 60, 0).tv_sec, 0).tv_sec, 0);
    affs_brelse(bh); unlock_new_inode(inode); inode
}

pub unsafe fn affs_write_inode(inode: *mut inode, _wbc: *mut writeback_control) -> i32 {
    let sb = (*inode).i_sb; let mut bh = affs_bread(sb, (*inode).i_ino); if (*inode).i_nlink == 0 { return 0; }
    if bh.is_null() { affs_error(sb, "write_inode", "Cannot read block %llu", (*inode).i_ino); return -EIO; }
    let tail = AFFS_TAIL(sb, bh);
    if (*tail).stype == cpu_to_be32(ST_ROOT) { affs_secs_to_datestamp(inode_get_mtime_sec(inode), &mut (*AFFS_ROOT_TAIL(sb, bh)).root_change); }
    else { (*tail).protect = cpu_to_be32((*AFFS_I(inode)).i_protect); (*tail).size = cpu_to_be32((*inode).i_size as _); affs_secs_to_datestamp(inode_get_mtime_sec(inode), &mut (*tail).change); let mut uid = i_uid_read(inode); let mut gid = i_gid_read(inode); if (*AFFS_SB(sb)).s_flags & SF_MUFS != 0 { if uid == 0 || uid == 0xffff { uid ^= !0; } if gid == 0 || gid == 0xffff { gid ^= !0; } } if (*inode).i_ino != (*AFFS_SB(sb)).s_root_block { if (*AFFS_SB(sb)).s_flags & SF_SETUID == 0 { (*tail).uid = cpu_to_be16(uid); } if (*AFFS_SB(sb)).s_flags & SF_SETGID == 0 { (*tail).gid = cpu_to_be16(gid); } } }
    affs_fix_checksum(sb, bh); mark_buffer_dirty(bh); affs_brelse(bh); affs_free_prealloc(inode); 0
}

pub unsafe fn affs_setattr(_idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 { let inode = d_inode(dentry); let mut error = setattr_prepare(&nop_mnt_idmap, dentry, attr); if error != 0 { return error; } if ((*attr).ia_valid & ATTR_UID != 0 && (*AFFS_SB((*inode).i_sb)).s_flags & SF_SETUID != 0) || ((*attr).ia_valid & ATTR_GID != 0 && (*AFFS_SB((*inode).i_sb)).s_flags & SF_SETGID != 0) || ((*attr).ia_valid & ATTR_MODE != 0 && (*AFFS_SB((*inode).i_sb)).s_flags & (AFFS_MOUNT_SF_SETMODE | AFFS_MOUNT_SF_IMMUTABLE) != 0) { if (*AFFS_SB((*inode).i_sb)).s_flags & SF_QUIET == 0 { error = -EPERM; } return error; } if (*attr).ia_valid & ATTR_SIZE != 0 && (*attr).ia_size != i_size_read(inode) { error = inode_newsize_ok(inode, (*attr).ia_size); if error != 0 { return error; } truncate_setsize(inode, (*attr).ia_size); affs_truncate(inode); } setattr_copy(&nop_mnt_idmap, inode, attr); mark_inode_dirty(inode); if (*attr).ia_valid & ATTR_MODE != 0 { affs_mode_to_prot(inode); } error }

pub unsafe fn affs_evict_inode(inode: *mut inode) { truncate_inode_pages_final(&mut (*inode).i_data); if (*inode).i_nlink == 0 { (*inode).i_size = 0; affs_truncate(inode); } clear_inode(inode); affs_free_prealloc(inode); let cache_page = (*AFFS_I(inode)).i_lc as usize; if cache_page != 0 { (*AFFS_I(inode)).i_lc = core::ptr::null_mut(); (*AFFS_I(inode)).i_ac = core::ptr::null_mut(); free_page(cache_page as _); } affs_brelse((*AFFS_I(inode)).i_ext_bh); (*AFFS_I(inode)).i_ext_last = !1; (*AFFS_I(inode)).i_ext_bh = core::ptr::null_mut(); if (*inode).i_nlink == 0 { affs_free_block((*inode).i_sb, (*inode).i_ino); } }

pub unsafe fn affs_new_inode(dir: *mut inode) -> *mut inode { let sb = (*dir).i_sb; let inode = new_inode(sb); if inode.is_null() { return core::ptr::null_mut(); } let block = affs_alloc_block(dir, (*dir).i_ino); if block == 0 { iput(inode); return core::ptr::null_mut(); } let bh = affs_getzeroblk(sb, block); if bh.is_null() { affs_free_block(sb, block); iput(inode); return core::ptr::null_mut(); } mark_buffer_dirty(bh); affs_brelse(bh); (*inode).i_uid = current_fsuid(); (*inode).i_gid = current_fsgid(); (*inode).i_ino = block as _; set_nlink(inode, 1); simple_inode_init_ts(inode); atomic_set(&mut (*AFFS_I(inode)).i_opencnt, 0); (*AFFS_I(inode)).i_blkcnt = 0; (*AFFS_I(inode)).i_lc = core::ptr::null_mut(); (*AFFS_I(inode)).i_lc_size = 0; (*AFFS_I(inode)).i_lc_shift = 0; (*AFFS_I(inode)).i_lc_mask = 0; (*AFFS_I(inode)).i_ac = core::ptr::null_mut(); (*AFFS_I(inode)).i_ext_bh = core::ptr::null_mut(); (*AFFS_I(inode)).mmu_private = 0; (*AFFS_I(inode)).i_protect = 0; (*AFFS_I(inode)).i_lastalloc = 0; (*AFFS_I(inode)).i_pa_cnt = 0; (*AFFS_I(inode)).i_extcnt = 1; (*AFFS_I(inode)).i_ext_last = !1; insert_inode_hash(inode); inode }

pub unsafe fn affs_add_entry(dir: *mut inode, inode: *mut inode, dentry: *mut dentry, type_: i32) -> i32 { let sb = (*dir).i_sb; let mut inode_bh: *mut buffer_head = core::ptr::null_mut(); let mut bh = affs_bread(sb, (*inode).i_ino); let mut block = 0u32; if bh.is_null() { return -EIO; } affs_lock_link(inode); if type_ == ST_LINKFILE || type_ == ST_LINKDIR { block = affs_alloc_block(dir, (*dir).i_ino); if block == 0 { affs_unlock_link(inode); affs_brelse(bh); return -ENOSPC; } inode_bh = bh; bh = affs_getzeroblk(sb, block); if bh.is_null() { affs_free_block(sb, block); affs_unlock_link(inode); affs_brelse(inode_bh); return -EIO; } } (*AFFS_HEAD(bh)).ptype = cpu_to_be32(T_SHORT); (*AFFS_HEAD(bh)).key = cpu_to_be32((*bh).b_blocknr); affs_copy_name((*AFFS_TAIL(sb, bh)).name.as_mut_ptr(), dentry); (*AFFS_TAIL(sb, bh)).stype = cpu_to_be32(type_ as _); (*AFFS_TAIL(sb, bh)).parent = cpu_to_be32((*dir).i_ino as _); if !inode_bh.is_null() { let chain = (*AFFS_TAIL(sb, inode_bh)).link_chain; (*AFFS_TAIL(sb, bh)).original = cpu_to_be32((*inode).i_ino as _); (*AFFS_TAIL(sb, bh)).link_chain = chain; (*AFFS_TAIL(sb, inode_bh)).link_chain = cpu_to_be32(block); affs_adjust_checksum(inode_bh, block - be32_to_cpu(chain)); mark_buffer_dirty(inode_bh); set_nlink(inode, 2); ihold(inode); } affs_fix_checksum(sb, bh); mark_buffer_dirty(bh); (*dentry).d_fsdata = (*bh).b_blocknr as usize as *mut core::ffi::c_void; affs_lock_dir(dir); let retval = affs_insert_hash(dir, bh); mark_buffer_dirty(bh); affs_unlock_dir(dir); affs_unlock_link(inode); d_instantiate(dentry, inode); affs_brelse(inode_bh); affs_brelse(bh); retval }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
