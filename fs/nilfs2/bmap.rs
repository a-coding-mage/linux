// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS block mapping.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Kernel and NILFS declarations are supplied by the surrounding translation unit.

pub unsafe fn nilfs_bmap_get_dat(bmap: *const nilfs_bmap) -> *mut the_nilfs {
    (*(*bmap).b_inode).i_sb.as_ref().unwrap().s_fs_info
}

unsafe fn nilfs_bmap_convert_error(bmap: *mut nilfs_bmap, fname: *const core::ffi::c_char, mut err: i32) -> i32 {
    let inode = (*bmap).b_inode;
    if err == -EINVAL {
        __nilfs_error((*inode).i_sb, fname, c"broken bmap (inode number=%llu)".as_ptr(), (*inode).i_ino);
        err = -EIO;
    }
    err
}

pub unsafe fn nilfs_bmap_lookup_at_level(bmap: *mut nilfs_bmap, key: u64, level: i32, ptrp: *mut u64) -> i32 {
    let mut blocknr: sector_t = 0;
    down_read(&mut (*bmap).b_sem);
    let mut ret = ((*(*bmap).b_ops).bop_lookup)(bmap, key, level, ptrp);
    if ret >= 0 && NILFS_BMAP_USE_VBN(bmap) {
        ret = nilfs_dat_translate(nilfs_bmap_get_dat(bmap), *ptrp, &mut blocknr);
        if ret == 0 { *ptrp = blocknr; }
        else if ret == -ENOENT { ret = -EINVAL; }
    }
    up_read(&mut (*bmap).b_sem);
    nilfs_bmap_convert_error(bmap, c"nilfs_bmap_lookup_at_level".as_ptr(), ret)
}

pub unsafe fn nilfs_bmap_lookup_contig(bmap: *mut nilfs_bmap, key: u64, ptrp: *mut u64, maxblocks: u32) -> i32 {
    down_read(&mut (*bmap).b_sem);
    let ret = ((*(*bmap).b_ops).bop_lookup_contig)(bmap, key, ptrp, maxblocks);
    up_read(&mut (*bmap).b_sem);
    nilfs_bmap_convert_error(bmap, c"nilfs_bmap_lookup_contig".as_ptr(), ret)
}

unsafe fn nilfs_bmap_do_insert(bmap: *mut nilfs_bmap, key: u64, ptr: u64) -> i32 {
    let mut keys = [0u64; NILFS_BMAP_SMALL_HIGH as usize + 1];
    let mut ptrs = [0u64; NILFS_BMAP_SMALL_HIGH as usize + 1];
    if let Some(check) = (*(*bmap).b_ops).bop_check_insert {
        let ret = check(bmap, key);
        if ret > 0 {
            let n = ((*(*bmap).b_ops).bop_gather_data)(bmap, keys.as_mut_ptr(), ptrs.as_mut_ptr(), NILFS_BMAP_SMALL_HIGH + 1);
            if n < 0 { return n; }
            let ret = nilfs_btree_convert_and_insert(bmap, key, ptr, keys.as_mut_ptr(), ptrs.as_mut_ptr(), n);
            if ret == 0 { (*bmap).b_u.u_flags |= NILFS_BMAP_LARGE; }
            return ret;
        } else if ret < 0 { return ret; }
    }
    ((*(*bmap).b_ops).bop_insert)(bmap, key, ptr)
}

pub unsafe fn nilfs_bmap_insert(bmap: *mut nilfs_bmap, key: u64, rec: usize) -> i32 {
    down_write(&mut (*bmap).b_sem);
    let ret = nilfs_bmap_do_insert(bmap, key, rec as u64);
    up_write(&mut (*bmap).b_sem);
    nilfs_bmap_convert_error(bmap, c"nilfs_bmap_insert".as_ptr(), ret)
}

unsafe fn nilfs_bmap_do_delete(bmap: *mut nilfs_bmap, key: u64) -> i32 {
    let mut keys = [0u64; NILFS_BMAP_LARGE_LOW as usize + 1];
    let mut ptrs = [0u64; NILFS_BMAP_LARGE_LOW as usize + 1];
    if let Some(check) = (*(*bmap).b_ops).bop_check_delete {
        let ret = check(bmap, key);
        if ret > 0 {
            let n = ((*(*bmap).b_ops).bop_gather_data)(bmap, keys.as_mut_ptr(), ptrs.as_mut_ptr(), NILFS_BMAP_LARGE_LOW + 1);
            if n < 0 { return n; }
            let ret = nilfs_direct_delete_and_convert(bmap, key, keys.as_mut_ptr(), ptrs.as_mut_ptr(), n);
            if ret == 0 { (*bmap).b_u.u_flags &= !NILFS_BMAP_LARGE; }
            return ret;
        } else if ret < 0 { return ret; }
    }
    ((*(*bmap).b_ops).bop_delete)(bmap, key, false)
}

pub unsafe fn nilfs_bmap_seek_key(bmap: *mut nilfs_bmap, start: u64, keyp: *mut u64) -> i32 {
    down_read(&mut (*bmap).b_sem); let ret = ((*(*bmap).b_ops).bop_seek_key)(bmap, start, keyp); up_read(&mut (*bmap).b_sem);
    if ret < 0 { nilfs_bmap_convert_error(bmap, c"nilfs_bmap_seek_key".as_ptr(), ret) } else { ret }
}
pub unsafe fn nilfs_bmap_last_key(bmap: *mut nilfs_bmap, keyp: *mut u64) -> i32 {
    down_read(&mut (*bmap).b_sem); let ret = ((*(*bmap).b_ops).bop_last_key)(bmap, keyp); up_read(&mut (*bmap).b_sem);
    if ret < 0 { nilfs_bmap_convert_error(bmap, c"nilfs_bmap_last_key".as_ptr(), ret) } else { ret }
}

pub unsafe fn nilfs_bmap_delete(bmap: *mut nilfs_bmap, key: u64) -> i32 {
    down_write(&mut (*bmap).b_sem); let ret = nilfs_bmap_do_delete(bmap, key); up_write(&mut (*bmap).b_sem);
    nilfs_bmap_convert_error(bmap, c"nilfs_bmap_delete".as_ptr(), ret)
}

unsafe fn nilfs_bmap_do_truncate(bmap: *mut nilfs_bmap, key: u64) -> i32 {
    let mut lastkey = 0u64; let mut ret = ((*(*bmap).b_ops).bop_last_key)(bmap, &mut lastkey);
    if ret < 0 { return if ret == -ENOENT { 0 } else { ret }; }
    while key <= lastkey {
        ret = nilfs_bmap_do_delete(bmap, lastkey); if ret < 0 { return ret; }
        ret = ((*(*bmap).b_ops).bop_last_key)(bmap, &mut lastkey);
        if ret < 0 { return if ret == -ENOENT { 0 } else { ret }; }
    } 0
}
pub unsafe fn nilfs_bmap_truncate(bmap: *mut nilfs_bmap, key: u64) -> i32 {
    down_write(&mut (*bmap).b_sem); let ret = nilfs_bmap_do_truncate(bmap, key); up_write(&mut (*bmap).b_sem);
    nilfs_bmap_convert_error(bmap, c"nilfs_bmap_truncate".as_ptr(), ret)
}

pub unsafe fn nilfs_bmap_clear(bmap: *mut nilfs_bmap) { down_write(&mut (*bmap).b_sem); if let Some(f) = (*(*bmap).b_ops).bop_clear { f(bmap); } up_write(&mut (*bmap).b_sem); }
pub unsafe fn nilfs_bmap_propagate(bmap: *mut nilfs_bmap, bh: *mut buffer_head) -> i32 { down_write(&mut (*bmap).b_sem); let ret = ((*(*bmap).b_ops).bop_propagate)(bmap, bh); up_write(&mut (*bmap).b_sem); nilfs_bmap_convert_error(bmap, c"nilfs_bmap_propagate".as_ptr(), ret) }
pub unsafe fn nilfs_bmap_lookup_dirty_buffers(bmap: *mut nilfs_bmap, listp: *mut list_head) { if let Some(f) = (*(*bmap).b_ops).bop_lookup_dirty_buffers { f(bmap, listp); } }
pub unsafe fn nilfs_bmap_assign(bmap: *mut nilfs_bmap, bh: *mut *mut buffer_head, blocknr: usize, binfo: *mut nilfs_binfo) -> i32 { down_write(&mut (*bmap).b_sem); let ret = ((*(*bmap).b_ops).bop_assign)(bmap, bh, blocknr, binfo); up_write(&mut (*bmap).b_sem); nilfs_bmap_convert_error(bmap, c"nilfs_bmap_assign".as_ptr(), ret) }
pub unsafe fn nilfs_bmap_mark(bmap: *mut nilfs_bmap, key: u64, level: i32) -> i32 { if (*(*bmap).b_ops).bop_mark.is_none() { return 0; } down_write(&mut (*bmap).b_sem); let ret = ((*(*bmap).b_ops).bop_mark.unwrap())(bmap, key, level); up_write(&mut (*bmap).b_sem); nilfs_bmap_convert_error(bmap, c"nilfs_bmap_mark".as_ptr(), ret) }
pub unsafe fn nilfs_bmap_test_and_clear_dirty(bmap: *mut nilfs_bmap) -> i32 { down_write(&mut (*bmap).b_sem); let ret = nilfs_bmap_dirty(bmap); nilfs_bmap_clear_dirty(bmap); up_write(&mut (*bmap).b_sem); ret }

pub unsafe fn nilfs_bmap_data_get_key(bmap: *const nilfs_bmap, bh: *const buffer_head) -> u64 { let pos = folio_pos((*bh).b_folio) + bh_offset(bh); (pos >> (*(*bmap).b_inode).i_blkbits) as u64 }
pub unsafe fn nilfs_bmap_find_target_seq(bmap: *const nilfs_bmap, key: u64) -> u64 { let diff = key.wrapping_sub((*bmap).b_last_allocated_key) as i64; if nilfs_bmap_keydiff_abs(diff) < NILFS_INODE_BMAP_SIZE && (*bmap).b_last_allocated_ptr != NILFS_BMAP_INVALID_PTR && ((*bmap).b_last_allocated_ptr as i64 + diff) > 0 { ((*bmap).b_last_allocated_ptr as i64 + diff) as u64 } else { NILFS_BMAP_INVALID_PTR } }

pub unsafe fn nilfs_bmap_find_target_in_group(bmap: *const nilfs_bmap) -> u64 { let dat = nilfs_bmap_get_dat(bmap); let entries = nilfs_palloc_entries_per_group((*dat).ns_inode); let group = (*(*bmap).b_inode).i_ino / entries; let index = (*(*bmap).b_inode).i_ino & (8 - 1); group * entries + index * (entries / 8) }

static mut nilfs_bmap_dat_lock_key: lock_class_key = lock_class_key { _unused: 0 };
static mut nilfs_bmap_mdt_lock_key: lock_class_key = lock_class_key { _unused: 0 };

pub unsafe fn nilfs_bmap_read(bmap: *mut nilfs_bmap, raw_inode: *mut nilfs_inode) -> i32 {
    if raw_inode.is_null() { memset((*bmap).b_u.u_data.as_mut_ptr(), 0, NILFS_BMAP_SIZE); } else { memcpy((*bmap).b_u.u_data.as_mut_ptr(), (*raw_inode).i_bmap.as_ptr(), NILFS_BMAP_SIZE); }
    init_rwsem(&mut (*bmap).b_sem); (*bmap).b_state = 0; (*bmap).b_inode = &mut NILFS_BMAP_I(bmap).vfs_inode;
    match (*(*bmap).b_inode).i_ino {
        NILFS_DAT_INO => { (*bmap).b_ptr_type = NILFS_BMAP_PTR_P; (*bmap).b_last_allocated_key = 0; (*bmap).b_last_allocated_ptr = NILFS_BMAP_NEW_PTR_INIT; lockdep_set_class(&mut (*bmap).b_sem, &raw mut nilfs_bmap_dat_lock_key); }
        NILFS_CPFILE_INO | NILFS_SUFILE_INO => { (*bmap).b_ptr_type = NILFS_BMAP_PTR_VS; (*bmap).b_last_allocated_key = 0; (*bmap).b_last_allocated_ptr = NILFS_BMAP_INVALID_PTR; lockdep_set_class(&mut (*bmap).b_sem, &raw mut nilfs_bmap_mdt_lock_key); }
        NILFS_IFILE_INO => { lockdep_set_class(&mut (*bmap).b_sem, &raw mut nilfs_bmap_mdt_lock_key); (*bmap).b_ptr_type = NILFS_BMAP_PTR_VM; (*bmap).b_last_allocated_key = 0; (*bmap).b_last_allocated_ptr = NILFS_BMAP_INVALID_PTR; }
        _ => { (*bmap).b_ptr_type = NILFS_BMAP_PTR_VM; (*bmap).b_last_allocated_key = 0; (*bmap).b_last_allocated_ptr = NILFS_BMAP_INVALID_PTR; }
    }
    if ((*bmap).b_u.u_flags & NILFS_BMAP_LARGE) != 0 { nilfs_btree_init(bmap) } else { nilfs_direct_init(bmap) }
}

pub unsafe fn nilfs_bmap_write(bmap: *mut nilfs_bmap, raw_inode: *mut nilfs_inode) { memcpy((*raw_inode).i_bmap.as_mut_ptr(), (*bmap).b_u.u_data.as_ptr(), NILFS_INODE_BMAP_SIZE * core::mem::size_of::<u64>()); if (*(*bmap).b_inode).i_ino == NILFS_DAT_INO { (*bmap).b_last_allocated_ptr = NILFS_BMAP_NEW_PTR_INIT; } }
pub unsafe fn nilfs_bmap_init_gc(bmap: *mut nilfs_bmap) { memset(&mut (*bmap).b_u as *mut _, 0, NILFS_BMAP_SIZE); init_rwsem(&mut (*bmap).b_sem); (*bmap).b_inode = &mut NILFS_BMAP_I(bmap).vfs_inode; (*bmap).b_ptr_type = NILFS_BMAP_PTR_U; (*bmap).b_last_allocated_key = 0; (*bmap).b_last_allocated_ptr = NILFS_BMAP_INVALID_PTR; (*bmap).b_state = 0; nilfs_btree_init_gc(bmap); }
pub unsafe fn nilfs_bmap_save(bmap: *const nilfs_bmap, store: *mut nilfs_bmap_store) { memcpy((*store).data.as_mut_ptr(), (*bmap).b_u.u_data.as_ptr(), core::mem::size_of_val(&(*store).data)); (*store).last_allocated_key = (*bmap).b_last_allocated_key; (*store).last_allocated_ptr = (*bmap).b_last_allocated_ptr; (*store).state = (*bmap).b_state; }
pub unsafe fn nilfs_bmap_restore(bmap: *mut nilfs_bmap, store: *const nilfs_bmap_store) { memcpy((*bmap).b_u.u_data.as_mut_ptr(), (*store).data.as_ptr(), core::mem::size_of_val(&(*store).data)); (*bmap).b_last_allocated_key = (*store).last_allocated_key; (*bmap).b_last_allocated_ptr = (*store).last_allocated_ptr; (*bmap).b_state = (*store).state; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
