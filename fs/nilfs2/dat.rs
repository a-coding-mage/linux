// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS disk address translation.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Dependencies are supplied by the surrounding kernel translation.

const NILFS_CNO_MIN: u64 = 1;
const NILFS_CNO_MAX: u64 = !0;

static mut dat_lock_key: lock_class_key = lock_class_key {};

#[repr(C)]
pub struct nilfs_dat_info {
    pub mi: nilfs_mdt_info,
    pub palloc_cache: nilfs_palloc_cache,
    pub shadow: nilfs_shadow_map,
}

#[inline]
unsafe fn NILFS_DAT_I(dat: *mut inode) -> *mut nilfs_dat_info {
    NILFS_MDT(dat) as *mut nilfs_dat_info
}

unsafe fn nilfs_dat_prepare_entry(dat: *mut inode, req: *mut nilfs_palloc_req, create: i32) -> i32 {
    let mut ret = nilfs_palloc_get_entry_block(dat, (*req).pr_entry_nr, create, &mut (*req).pr_entry_bh);
    if ret == -ENOENT {
        nilfs_err((*dat).i_sb, "DAT doesn't have a block to manage vblocknr = %llu", (*req).pr_entry_nr as u64);
        ret = -EINVAL;
    }
    ret
}

unsafe fn nilfs_dat_commit_entry(dat: *mut inode, req: *mut nilfs_palloc_req) {
    mark_buffer_dirty((*req).pr_entry_bh);
    nilfs_mdt_mark_dirty(dat);
    brelse((*req).pr_entry_bh);
}

unsafe fn nilfs_dat_abort_entry(_dat: *mut inode, req: *mut nilfs_palloc_req) {
    brelse((*req).pr_entry_bh);
}

pub unsafe fn nilfs_dat_prepare_alloc(dat: *mut inode, req: *mut nilfs_palloc_req) -> i32 {
    let mut ret = nilfs_palloc_prepare_alloc_entry(dat, req, true);
    if ret < 0 { return ret; }
    ret = nilfs_dat_prepare_entry(dat, req, 1);
    if ret < 0 { nilfs_palloc_abort_alloc_entry(dat, req); }
    ret
}

pub unsafe fn nilfs_dat_commit_alloc(dat: *mut inode, req: *mut nilfs_palloc_req) {
    let offset = nilfs_palloc_entry_offset(dat, (*req).pr_entry_nr, (*req).pr_entry_bh);
    let entry = kmap_local_folio((*(*req).pr_entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    (*entry).de_start = cpu_to_le64(NILFS_CNO_MIN);
    (*entry).de_end = cpu_to_le64(NILFS_CNO_MAX);
    (*entry).de_blocknr = cpu_to_le64(0);
    kunmap_local(entry as *mut core::ffi::c_void);
    nilfs_palloc_commit_alloc_entry(dat, req);
    nilfs_dat_commit_entry(dat, req);
}

pub unsafe fn nilfs_dat_abort_alloc(dat: *mut inode, req: *mut nilfs_palloc_req) {
    nilfs_dat_abort_entry(dat, req);
    nilfs_palloc_abort_alloc_entry(dat, req);
}

unsafe fn nilfs_dat_commit_free(dat: *mut inode, req: *mut nilfs_palloc_req) {
    let offset = nilfs_palloc_entry_offset(dat, (*req).pr_entry_nr, (*req).pr_entry_bh);
    let entry = kmap_local_folio((*(*req).pr_entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    (*entry).de_start = cpu_to_le64(NILFS_CNO_MIN);
    (*entry).de_end = cpu_to_le64(NILFS_CNO_MIN);
    (*entry).de_blocknr = cpu_to_le64(0);
    kunmap_local(entry as *mut core::ffi::c_void);
    nilfs_dat_commit_entry(dat, req);
    if (*req).pr_desc_bh.is_null() || (*req).pr_bitmap_bh.is_null() {
        nilfs_error((*dat).i_sb, "state inconsistency probably due to duplicate use of vblocknr = %llu", (*req).pr_entry_nr as u64);
        return;
    }
    nilfs_palloc_commit_free_entry(dat, req);
}

pub unsafe fn nilfs_dat_prepare_start(dat: *mut inode, req: *mut nilfs_palloc_req) -> i32 { nilfs_dat_prepare_entry(dat, req, 0) }

pub unsafe fn nilfs_dat_commit_start(dat: *mut inode, req: *mut nilfs_palloc_req, blocknr: sector_t) {
    let offset = nilfs_palloc_entry_offset(dat, (*req).pr_entry_nr, (*req).pr_entry_bh);
    let entry = kmap_local_folio((*(*req).pr_entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    (*entry).de_start = cpu_to_le64(nilfs_mdt_cno(dat));
    (*entry).de_blocknr = cpu_to_le64(blocknr);
    kunmap_local(entry as *mut core::ffi::c_void);
    nilfs_dat_commit_entry(dat, req);
}

pub unsafe fn nilfs_dat_prepare_end(dat: *mut inode, req: *mut nilfs_palloc_req) -> i32 {
    let mut ret = nilfs_dat_prepare_entry(dat, req, 0);
    if ret < 0 { return ret; }
    let offset = nilfs_palloc_entry_offset(dat, (*req).pr_entry_nr, (*req).pr_entry_bh);
    let entry = kmap_local_folio((*(*req).pr_entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    let start = le64_to_cpu((*entry).de_start);
    let blocknr = le64_to_cpu((*entry).de_blocknr);
    kunmap_local(entry as *mut core::ffi::c_void);
    if blocknr == 0 {
        ret = nilfs_palloc_prepare_free_entry(dat, req);
        if ret < 0 { nilfs_dat_abort_entry(dat, req); return ret; }
    }
    if start > nilfs_mdt_cno(dat) {
        nilfs_err((*dat).i_sb, "vblocknr = %llu has abnormal lifetime: start cno (= %llu) > current cno (= %llu)", (*req).pr_entry_nr as u64, start, nilfs_mdt_cno(dat));
        nilfs_dat_abort_entry(dat, req);
        return -EINVAL;
    }
    0
}

pub unsafe fn nilfs_dat_commit_end(dat: *mut inode, req: *mut nilfs_palloc_req, dead: i32) {
    let offset = nilfs_palloc_entry_offset(dat, (*req).pr_entry_nr, (*req).pr_entry_bh);
    let entry = kmap_local_folio((*(*req).pr_entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    let start = le64_to_cpu((*entry).de_start);
    let mut end = start;
    if dead == 0 { end = nilfs_mdt_cno(dat); }
    (*entry).de_end = cpu_to_le64(end);
    let blocknr = le64_to_cpu((*entry).de_blocknr);
    kunmap_local(entry as *mut core::ffi::c_void);
    if blocknr == 0 { nilfs_dat_commit_free(dat, req); } else { nilfs_dat_commit_entry(dat, req); }
}

pub unsafe fn nilfs_dat_abort_end(dat: *mut inode, req: *mut nilfs_palloc_req) {
    let offset = nilfs_palloc_entry_offset(dat, (*req).pr_entry_nr, (*req).pr_entry_bh);
    let entry = kmap_local_folio((*(*req).pr_entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    let start = le64_to_cpu((*entry).de_start);
    let blocknr = le64_to_cpu((*entry).de_blocknr);
    kunmap_local(entry as *mut core::ffi::c_void);
    if start == nilfs_mdt_cno(dat) && blocknr == 0 { nilfs_palloc_abort_free_entry(dat, req); }
    nilfs_dat_abort_entry(dat, req);
}

pub unsafe fn nilfs_dat_prepare_update(dat: *mut inode, oldreq: *mut nilfs_palloc_req, newreq: *mut nilfs_palloc_req) -> i32 {
    let mut ret = nilfs_dat_prepare_end(dat, oldreq);
    if ret == 0 { ret = nilfs_dat_prepare_alloc(dat, newreq); if ret < 0 { nilfs_dat_abort_end(dat, oldreq); } }
    ret
}

pub unsafe fn nilfs_dat_commit_update(dat: *mut inode, oldreq: *mut nilfs_palloc_req, newreq: *mut nilfs_palloc_req, dead: i32) {
    nilfs_dat_commit_end(dat, oldreq, dead); nilfs_dat_commit_alloc(dat, newreq);
}

pub unsafe fn nilfs_dat_abort_update(dat: *mut inode, oldreq: *mut nilfs_palloc_req, newreq: *mut nilfs_palloc_req) {
    nilfs_dat_abort_end(dat, oldreq); nilfs_dat_abort_alloc(dat, newreq);
}

pub unsafe fn nilfs_dat_mark_dirty(dat: *mut inode, vblocknr: u64) -> i32 {
    let mut req: nilfs_palloc_req = core::mem::zeroed(); req.pr_entry_nr = vblocknr;
    let ret = nilfs_dat_prepare_entry(dat, &mut req, 0);
    if ret == 0 { nilfs_dat_commit_entry(dat, &mut req); } ret
}

pub unsafe fn nilfs_dat_freev(dat: *mut inode, vblocknrs: *mut u64, nitems: usize) -> i32 { nilfs_palloc_freev(dat, vblocknrs, nitems) }

pub unsafe fn nilfs_dat_move(dat: *mut inode, vblocknr: u64, blocknr: sector_t) -> i32 {
    let mut entry_bh: *mut buffer_head = core::ptr::null_mut();
    let mut ret = nilfs_palloc_get_entry_block(dat, vblocknr, 0, &mut entry_bh);
    if ret < 0 { return ret; }
    if !buffer_nilfs_redirected(entry_bh) { ret = nilfs_mdt_freeze_buffer(dat, entry_bh); if ret != 0 { brelse(entry_bh); return ret; } }
    let offset = nilfs_palloc_entry_offset(dat, vblocknr, entry_bh);
    let entry = kmap_local_folio((*entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    if (*entry).de_blocknr == cpu_to_le64(0) { kunmap_local(entry as *mut core::ffi::c_void); brelse(entry_bh); return -EINVAL; }
    (*entry).de_blocknr = cpu_to_le64(blocknr); kunmap_local(entry as *mut core::ffi::c_void);
    mark_buffer_dirty(entry_bh); nilfs_mdt_mark_dirty(dat); brelse(entry_bh); 0
}

pub unsafe fn nilfs_dat_translate(dat: *mut inode, vblocknr: u64, blocknrp: *mut sector_t) -> i32 {
    let mut entry_bh: *mut buffer_head = core::ptr::null_mut();
    let mut ret = nilfs_palloc_get_entry_block(dat, vblocknr, 0, &mut entry_bh);
    if ret < 0 { return ret; }
    if !nilfs_doing_gc() && buffer_nilfs_redirected(entry_bh) { let bh = nilfs_mdt_get_frozen_buffer(dat, entry_bh); if !bh.is_null() { brelse(entry_bh); entry_bh = bh; } }
    let offset = nilfs_palloc_entry_offset(dat, vblocknr, entry_bh);
    let entry = kmap_local_folio((*entry_bh).b_folio, offset) as *mut nilfs_dat_entry;
    let blocknr = le64_to_cpu((*entry).de_blocknr);
    if blocknr == 0 { ret = -ENOENT; } else { *blocknrp = blocknr; }
    kunmap_local(entry as *mut core::ffi::c_void); brelse(entry_bh); ret
}

pub unsafe fn nilfs_dat_get_vinfo(dat: *mut inode, buf: *mut core::ffi::c_void, visz: u32, nvi: usize) -> isize {
    let entries_per_block = (*NILFS_MDT(dat)).mi_entries_per_block as u64;
    let entry_size = (*NILFS_MDT(dat)).mi_entry_size as usize;
    let mut vinfo = buf as *mut nilfs_vinfo; let mut i = 0usize;
    while i < nvi {
        let mut entry_bh: *mut buffer_head = core::ptr::null_mut();
        let ret = nilfs_palloc_get_entry_block(dat, (*vinfo).vi_vblocknr, 0, &mut entry_bh); if ret < 0 { return ret as isize; }
        let first = ((*vinfo).vi_vblocknr / entries_per_block) * entries_per_block;
        let last = first + entries_per_block - 1;
        let offset = nilfs_palloc_entry_offset(dat, first, entry_bh);
        let first_entry = kmap_local_folio((*entry_bh).b_folio, offset) as *mut u8;
        let mut j = i;
        while j < nvi && (*vinfo).vi_vblocknr >= first && (*vinfo).vi_vblocknr <= last {
            let entry = first_entry.add(((*vinfo).vi_vblocknr - first) as usize * entry_size) as *mut nilfs_dat_entry;
            (*vinfo).vi_start = le64_to_cpu((*entry).de_start); (*vinfo).vi_end = le64_to_cpu((*entry).de_end); (*vinfo).vi_blocknr = le64_to_cpu((*entry).de_blocknr);
            j += 1; i += 1; vinfo = (vinfo as *mut u8).add(visz as usize) as *mut nilfs_vinfo;
        }
        kunmap_local(first_entry as *mut core::ffi::c_void); brelse(entry_bh);
    }
    nvi as isize
}

pub unsafe fn nilfs_dat_read(sb: *mut super_block, entry_size: usize, raw_inode: *mut nilfs_inode, inodep: *mut *mut inode) -> i32 {
    if entry_size > (*sb).s_blocksize as usize { nilfs_err(sb, "too large DAT entry size: %zu bytes", entry_size); return -EINVAL; }
    if entry_size < NILFS_MIN_DAT_ENTRY_SIZE { nilfs_err(sb, "too small DAT entry size: %zu bytes", entry_size); return -EINVAL; }
    let dat = nilfs_iget_locked(sb, core::ptr::null_mut(), NILFS_DAT_INO); if dat.is_null() { return -ENOMEM; }
    if (inode_state_read_once(dat) & I_NEW) == 0 { *inodep = dat; return 0; }
    let mut err = nilfs_mdt_init(dat, NILFS_MDT_GFP, core::mem::size_of::<nilfs_dat_info>()); if err != 0 { iget_failed(dat); return err; }
    err = nilfs_palloc_init_blockgroup(dat, entry_size); if err != 0 { iget_failed(dat); return err; }
    let di = NILFS_DAT_I(dat); lockdep_set_class(&mut (*di).mi.mi_sem, &mut dat_lock_key); nilfs_palloc_setup_cache(dat, &mut (*di).palloc_cache);
    err = nilfs_mdt_setup_shadow_map(dat, &mut (*di).shadow); if err != 0 { iget_failed(dat); return err; }
    err = nilfs_attach_btree_node_cache(dat); if err != 0 { iget_failed(dat); return err; }
    err = nilfs_read_inode_common(dat, raw_inode); if err != 0 { iget_failed(dat); return err; }
    unlock_new_inode(dat); *inodep = dat; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
