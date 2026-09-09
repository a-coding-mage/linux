// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS direct block pointer.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Dependencies supplied by nilfs headers and other translation units.

unsafe fn nilfs_direct_dptrs(direct: *const nilfs_bmap) -> *mut __le64 {
    ((*((*direct).b_u.u_data as *mut nilfs_direct_node)).add(1)) as *mut __le64
}

unsafe fn nilfs_direct_get_ptr(direct: *const nilfs_bmap, key: __u64) -> __u64 {
    le64_to_cpu(*nilfs_direct_dptrs(direct).add(key as usize))
}

unsafe fn nilfs_direct_set_ptr(direct: *mut nilfs_bmap, key: __u64, ptr: __u64) {
    *nilfs_direct_dptrs(direct).add(key as usize) = cpu_to_le64(ptr);
}

unsafe fn nilfs_direct_lookup(direct: *const nilfs_bmap, key: __u64, level: i32,
                              ptrp: *mut __u64) -> i32 {
    if key > NILFS_DIRECT_KEY_MAX || level != 1 { return -ENOENT; }
    let ptr = nilfs_direct_get_ptr(direct, key);
    if ptr == NILFS_BMAP_INVALID_PTR { return -ENOENT; }
    *ptrp = ptr;
    0
}

unsafe fn nilfs_direct_lookup_contig(direct: *const nilfs_bmap, key: __u64,
                                     ptrp: *mut __u64, mut maxblocks: u32) -> i32 {
    let mut dat: *mut inode = core::ptr::null_mut();
    let mut ptr: __u64;
    let mut ptr2: __u64;
    let mut blocknr: sector_t = 0;
    let mut ret: i32;
    let mut cnt: i32;
    if key > NILFS_DIRECT_KEY_MAX { return -ENOENT; }
    ptr = nilfs_direct_get_ptr(direct, key);
    if ptr == NILFS_BMAP_INVALID_PTR { return -ENOENT; }
    if NILFS_BMAP_USE_VBN(direct) {
        dat = nilfs_bmap_get_dat(direct);
        ret = nilfs_dat_translate(dat, ptr, &mut blocknr);
        if ret < 0 { return nilfs_direct_lookup_contig_dat_error(ret); }
        ptr = blocknr;
    }
    maxblocks = core::cmp::min(maxblocks, (NILFS_DIRECT_KEY_MAX - key + 1) as u32);
    cnt = 1;
    while (cnt as u32) < maxblocks {
        ptr2 = nilfs_direct_get_ptr(direct, key + cnt as u64);
        if ptr2 == NILFS_BMAP_INVALID_PTR { break; }
        if !dat.is_null() {
            ret = nilfs_dat_translate(dat, ptr2, &mut blocknr);
            if ret < 0 { return nilfs_direct_lookup_contig_dat_error(ret); }
            ptr2 = blocknr;
        }
        if ptr2 != ptr.wrapping_add(cnt as u64) { break; }
        cnt += 1;
    }
    *ptrp = ptr;
    cnt
}

unsafe fn nilfs_direct_lookup_contig_dat_error(mut ret: i32) -> i32 {
    if ret == -ENOENT { ret = -EINVAL; }
    ret
}

unsafe fn nilfs_direct_find_target_v(direct: *const nilfs_bmap, key: __u64) -> __u64 {
    let ptr = nilfs_bmap_find_target_seq(direct, key);
    if ptr != NILFS_BMAP_INVALID_PTR { return ptr; }
    nilfs_bmap_find_target_in_group(direct)
}

unsafe fn nilfs_direct_insert(bmap: *mut nilfs_bmap, key: __u64, ptr: __u64) -> i32 {
    let mut req: nilfs_bmap_ptr_req = core::mem::zeroed();
    let mut dat: *mut inode = core::ptr::null_mut();
    if key > NILFS_DIRECT_KEY_MAX { return -ENOENT; }
    if nilfs_direct_get_ptr(bmap, key) != NILFS_BMAP_INVALID_PTR { return -EEXIST; }
    if NILFS_BMAP_USE_VBN(bmap) { req.bpr_ptr = nilfs_direct_find_target_v(bmap, key); dat = nilfs_bmap_get_dat(bmap); }
    let ret = nilfs_bmap_prepare_alloc_ptr(bmap, &mut req, dat);
    if ret == 0 {
        let bh = ptr as *mut buffer_head;
        set_buffer_nilfs_volatile(bh);
        nilfs_bmap_commit_alloc_ptr(bmap, &mut req, dat);
        nilfs_direct_set_ptr(bmap, key, req.bpr_ptr);
        if !nilfs_bmap_dirty(bmap) { nilfs_bmap_set_dirty(bmap); }
        if NILFS_BMAP_USE_VBN(bmap) { nilfs_bmap_set_target_v(bmap, key, req.bpr_ptr); }
        nilfs_inode_add_blocks((*bmap).b_inode, 1);
    }
    ret
}

unsafe fn nilfs_direct_delete(bmap: *mut nilfs_bmap, key: __u64, _deform: bool) -> i32 {
    let mut req: nilfs_bmap_ptr_req = core::mem::zeroed();
    if key > NILFS_DIRECT_KEY_MAX || nilfs_direct_get_ptr(bmap, key) == NILFS_BMAP_INVALID_PTR { return -ENOENT; }
    let dat = if NILFS_BMAP_USE_VBN(bmap) { nilfs_bmap_get_dat(bmap) } else { core::ptr::null_mut() };
    req.bpr_ptr = nilfs_direct_get_ptr(bmap, key);
    let ret = nilfs_bmap_prepare_end_ptr(bmap, &mut req, dat);
    if ret == 0 { nilfs_bmap_commit_end_ptr(bmap, &mut req, dat); nilfs_direct_set_ptr(bmap, key, NILFS_BMAP_INVALID_PTR); nilfs_inode_sub_blocks((*bmap).b_inode, 1); }
    ret
}

unsafe fn nilfs_direct_seek_key(direct: *const nilfs_bmap, start: __u64, keyp: *mut __u64) -> i32 {
    let mut key = start;
    while key <= NILFS_DIRECT_KEY_MAX { if nilfs_direct_get_ptr(direct, key) != NILFS_BMAP_INVALID_PTR { *keyp = key; return 0; } key += 1; }
    -ENOENT
}

unsafe fn nilfs_direct_last_key(direct: *const nilfs_bmap, keyp: *mut __u64) -> i32 {
    let mut lastkey = NILFS_DIRECT_KEY_MAX + 1;
    for key in NILFS_DIRECT_KEY_MIN..=NILFS_DIRECT_KEY_MAX { if nilfs_direct_get_ptr(direct, key) != NILFS_BMAP_INVALID_PTR { lastkey = key; } }
    if lastkey == NILFS_DIRECT_KEY_MAX + 1 { return -ENOENT; }
    *keyp = lastkey; 0
}

unsafe fn nilfs_direct_check_insert(_bmap: *const nilfs_bmap, key: __u64) -> i32 { (key > NILFS_DIRECT_KEY_MAX) as i32 }

unsafe fn nilfs_direct_gather_data(direct: *mut nilfs_bmap, keys: *mut __u64, ptrs: *mut __u64, mut nitems: i32) -> i32 {
    if nitems > NILFS_DIRECT_NBLOCKS { nitems = NILFS_DIRECT_NBLOCKS; }
    let mut n = 0; for key in 0..nitems as u64 { let ptr = nilfs_direct_get_ptr(direct, key); if ptr != NILFS_BMAP_INVALID_PTR { *keys.add(n as usize) = key; *ptrs.add(n as usize) = ptr; n += 1; } } n
}

pub unsafe fn nilfs_direct_delete_and_convert(bmap: *mut nilfs_bmap, key: __u64, keys: *mut __u64, ptrs: *mut __u64, n: i32) -> i32 {
    let ret = ((*(*bmap).b_ops).bop_delete)(bmap, key, true); if ret < 0 { return ret; }
    if !((*(*bmap).b_ops).bop_clear).is_none() { ((*(*bmap).b_ops).bop_clear)(bmap); }
    let dptrs = nilfs_direct_dptrs(bmap); let mut j = 0;
    for i in 0..NILFS_DIRECT_NBLOCKS { if j < n && i == *keys.add(j as usize) { *dptrs.add(i as usize) = if i != key { cpu_to_le64(*ptrs.add(j as usize)) } else { NILFS_BMAP_INVALID_PTR }; j += 1; } else { *dptrs.add(i as usize) = NILFS_BMAP_INVALID_PTR; } }
    nilfs_direct_init(bmap); 0
}

unsafe fn nilfs_direct_propagate(bmap: *mut nilfs_bmap, bh: *mut buffer_head) -> i32 {
    if !NILFS_BMAP_USE_VBN(bmap) { return 0; }
    let dat = nilfs_bmap_get_dat(bmap); let key = nilfs_bmap_data_get_key(bmap, bh); let ptr = nilfs_direct_get_ptr(bmap, key); if ptr == NILFS_BMAP_INVALID_PTR { return -EINVAL; }
    if !buffer_nilfs_volatile(bh) { let mut oldreq: nilfs_palloc_req = core::mem::zeroed(); let mut newreq = oldreq; oldreq.pr_entry_nr = ptr; newreq.pr_entry_nr = ptr; let ret = nilfs_dat_prepare_update(dat, &mut oldreq, &mut newreq); if ret < 0 { return ret; } nilfs_dat_commit_update(dat, &mut oldreq, &mut newreq, (*bmap).b_ptr_type == NILFS_BMAP_PTR_VS); set_buffer_nilfs_volatile(bh); nilfs_direct_set_ptr(bmap, key, newreq.pr_entry_nr); ret } else { nilfs_dat_mark_dirty(dat, ptr) }
}

unsafe fn nilfs_direct_assign_v(direct: *mut nilfs_bmap, key: __u64, ptr: __u64, _bh: *mut *mut buffer_head, blocknr: sector_t, binfo: *mut nilfs_binfo) -> i32 {
    let dat = nilfs_bmap_get_dat(direct); let mut req: nilfs_bmap_ptr_req = core::mem::zeroed(); req.bpr_ptr = ptr; let ret = nilfs_dat_prepare_start(dat, &mut req.bpr_req); if ret == 0 { nilfs_dat_commit_start(dat, &mut req.bpr_req, blocknr); (*binfo).bi_v.bi_vblocknr = cpu_to_le64(ptr); (*binfo).bi_v.bi_blkoff = cpu_to_le64(key); } ret
}

unsafe fn nilfs_direct_assign_p(direct: *mut nilfs_bmap, key: __u64, _ptr: __u64, _bh: *mut *mut buffer_head, blocknr: sector_t, binfo: *mut nilfs_binfo) -> i32 {
    nilfs_direct_set_ptr(direct, key, blocknr); (*binfo).bi_dat.bi_blkoff = cpu_to_le64(key); (*binfo).bi_dat.bi_level = 0; core::ptr::write_bytes((*binfo).bi_dat.bi_pad.as_mut_ptr(), 0, core::mem::size_of_val(&(*binfo).bi_dat.bi_pad)); 0
}

unsafe fn nilfs_direct_assign(bmap: *mut nilfs_bmap, bh: *mut *mut buffer_head, blocknr: sector_t, binfo: *mut nilfs_binfo) -> i32 {
    let key = nilfs_bmap_data_get_key(bmap, *bh); if key > NILFS_DIRECT_KEY_MAX { return -EINVAL; } let ptr = nilfs_direct_get_ptr(bmap, key); if ptr == NILFS_BMAP_INVALID_PTR { return -EINVAL; }
    if NILFS_BMAP_USE_VBN(bmap) { nilfs_direct_assign_v(bmap, key, ptr, bh, blocknr, binfo) } else { nilfs_direct_assign_p(bmap, key, ptr, bh, blocknr, binfo) }
}

pub unsafe fn nilfs_direct_init(bmap: *mut nilfs_bmap) -> i32 { (*bmap).b_ops = &nilfs_direct_ops; 0 }

static nilfs_direct_ops: nilfs_bmap_operations = nilfs_bmap_operations {
    bop_lookup: Some(nilfs_direct_lookup), bop_lookup_contig: Some(nilfs_direct_lookup_contig), bop_insert: Some(nilfs_direct_insert), bop_delete: Some(nilfs_direct_delete), bop_clear: None,
    bop_propagate: Some(nilfs_direct_propagate), bop_lookup_dirty_buffers: None, bop_assign: Some(nilfs_direct_assign), bop_mark: None,
    bop_seek_key: Some(nilfs_direct_seek_key), bop_last_key: Some(nilfs_direct_last_key), bop_check_insert: Some(nilfs_direct_check_insert), bop_check_delete: None, bop_gather_data: Some(nilfs_direct_gather_data),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
