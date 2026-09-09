// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS dat/inode allocator
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Originally written by Koji Sato.
 * Two allocators were unified by Ryusuke Konishi and Amagai Yoshiji.
 */

// C dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn nilfs_palloc_groups_per_desc_block(inode: *const inode) -> c_ulong {
    i_blocksize(inode) / core::mem::size_of::<nilfs_palloc_group_desc>() as c_ulong
}

#[inline]
unsafe fn nilfs_palloc_groups_count(inode: *const inode) -> c_ulong {
    1 as c_ulong << (BITS_PER_LONG - ((*inode).i_blkbits + 3))
}

unsafe fn nilfs_palloc_init_blockgroup(inode: *mut inode, entry_size: c_uint) -> c_int {
    let mi = NILFS_MDT(inode);
    (*mi).mi_bgl = kmalloc_obj((*mi).mi_bgl, GFP_NOFS);
    if (*mi).mi_bgl.is_null() { return -ENOMEM; }
    bgl_lock_init((*mi).mi_bgl);
    nilfs_mdt_set_entry_size(inode, entry_size, 0);
    (*mi).mi_blocks_per_group = DIV_ROUND_UP(nilfs_palloc_entries_per_group(inode), (*mi).mi_entries_per_block) + 1;
    (*mi).mi_blocks_per_desc_block = nilfs_palloc_groups_per_desc_block(inode) * (*mi).mi_blocks_per_group + 1;
    0
}

unsafe fn nilfs_palloc_group(inode: *const inode, nr: u64, offset: *mut c_ulong) -> c_ulong {
    let mut group = nr;
    *offset = do_div(&mut group, nilfs_palloc_entries_per_group(inode));
    group as c_ulong
}

unsafe fn nilfs_palloc_desc_blkoff(inode: *const inode, group: c_ulong) -> c_ulong {
    (group / nilfs_palloc_groups_per_desc_block(inode)) * (*NILFS_MDT(inode)).mi_blocks_per_desc_block
}

unsafe fn nilfs_palloc_bitmap_blkoff(inode: *const inode, group: c_ulong) -> c_ulong {
    let desc_offset = group % nilfs_palloc_groups_per_desc_block(inode);
    nilfs_palloc_desc_blkoff(inode, group) + 1 + desc_offset * (*NILFS_MDT(inode)).mi_blocks_per_group
}

unsafe fn nilfs_palloc_group_desc_nfrees(desc: *const nilfs_palloc_group_desc, lock: *mut spinlock_t) -> c_ulong {
    spin_lock(lock);
    let nfree = le32_to_cpu((*desc).pg_nfrees) as c_ulong;
    spin_unlock(lock);
    nfree
}

unsafe fn nilfs_palloc_group_desc_add_entries(desc: *mut nilfs_palloc_group_desc, lock: *mut spinlock_t, n: u32) -> u32 {
    spin_lock(lock);
    le32_add_cpu(&mut (*desc).pg_nfrees, n);
    let nfree = le32_to_cpu((*desc).pg_nfrees);
    spin_unlock(lock);
    nfree
}

unsafe fn nilfs_palloc_entry_blkoff(inode: *const inode, nr: u64) -> c_ulong {
    let mut group_offset = 0;
    let group = nilfs_palloc_group(inode, nr, &mut group_offset);
    nilfs_palloc_bitmap_blkoff(inode, group) + 1 + group_offset / (*NILFS_MDT(inode)).mi_entries_per_block
}

unsafe fn nilfs_palloc_desc_block_init(inode: *mut inode, _bh: *mut buffer_head, from: *mut c_void) {
    let mut desc = from as *mut nilfs_palloc_group_desc;
    let mut n = nilfs_palloc_groups_per_desc_block(inode);
    let nfrees = cpu_to_le32(nilfs_palloc_entries_per_group(inode));
    while n > 0 { (*desc).pg_nfrees = nfrees; desc = desc.add(1); n -= 1; }
}

unsafe fn nilfs_palloc_get_block(inode: *mut inode, blkoff: c_ulong, create: c_int,
    init_block: Option<unsafe extern "C" fn(*mut inode, *mut buffer_head, *mut c_void)>,
    bhp: *mut *mut buffer_head, prev: *mut nilfs_bh_assoc, lock: *mut spinlock_t) -> c_int {
    spin_lock(lock);
    if !(*prev).bh.is_null() && blkoff == (*prev).blkoff && likely(buffer_uptodate((*prev).bh)) {
        get_bh((*prev).bh); *bhp = (*prev).bh; spin_unlock(lock); return 0;
    }
    spin_unlock(lock);
    let ret = nilfs_mdt_get_block(inode, blkoff, create, init_block, bhp);
    if ret == 0 {
        spin_lock(lock);
        brelse((*prev).bh); get_bh(*bhp); (*prev).bh = *bhp; (*prev).blkoff = blkoff;
        spin_unlock(lock);
    }
    ret
}

unsafe fn nilfs_palloc_delete_block(inode: *mut inode, blkoff: c_ulong, prev: *mut nilfs_bh_assoc, lock: *mut spinlock_t) -> c_int {
    spin_lock(lock);
    if !(*prev).bh.is_null() && blkoff == (*prev).blkoff { brelse((*prev).bh); (*prev).bh = core::ptr::null_mut(); }
    spin_unlock(lock);
    nilfs_mdt_delete_block(inode, blkoff)
}

unsafe fn nilfs_palloc_get_desc_block(inode: *mut inode, group: c_ulong, create: c_int, bhp: *mut *mut buffer_head) -> c_int {
    let cache = (*NILFS_MDT(inode)).mi_palloc_cache;
    nilfs_palloc_get_block(inode, nilfs_palloc_desc_blkoff(inode, group), create, Some(nilfs_palloc_desc_block_init), bhp, &mut (*cache).prev_desc, &mut (*cache).lock)
}

unsafe fn nilfs_palloc_get_bitmap_block(inode: *mut inode, group: c_ulong, create: c_int, bhp: *mut *mut buffer_head) -> c_int {
    let cache = (*NILFS_MDT(inode)).mi_palloc_cache;
    nilfs_palloc_get_block(inode, nilfs_palloc_bitmap_blkoff(inode, group), create, None, bhp, &mut (*cache).prev_bitmap, &mut (*cache).lock)
}

unsafe fn nilfs_palloc_delete_bitmap_block(inode: *mut inode, group: c_ulong) -> c_int {
    let cache = (*NILFS_MDT(inode)).mi_palloc_cache;
    nilfs_palloc_delete_block(inode, nilfs_palloc_bitmap_blkoff(inode, group), &mut (*cache).prev_bitmap, &mut (*cache).lock)
}

unsafe fn nilfs_palloc_get_entry_block(inode: *mut inode, nr: u64, create: c_int, bhp: *mut *mut buffer_head) -> c_int {
    let cache = (*NILFS_MDT(inode)).mi_palloc_cache;
    nilfs_palloc_get_block(inode, nilfs_palloc_entry_blkoff(inode, nr), create, None, bhp, &mut (*cache).prev_entry, &mut (*cache).lock)
}

unsafe fn nilfs_palloc_delete_entry_block(inode: *mut inode, nr: u64) -> c_int {
    let cache = (*NILFS_MDT(inode)).mi_palloc_cache;
    nilfs_palloc_delete_block(inode, nilfs_palloc_entry_blkoff(inode, nr), &mut (*cache).prev_entry, &mut (*cache).lock)
}

unsafe fn nilfs_palloc_group_desc_offset(inode: *const inode, group: c_ulong, bh: *const buffer_head) -> usize {
    offset_in_folio((*bh).b_folio, (*bh).b_data) + core::mem::size_of::<nilfs_palloc_group_desc>() * (group % nilfs_palloc_groups_per_desc_block(inode)) as usize
}

unsafe fn nilfs_palloc_bitmap_offset(bh: *const buffer_head) -> usize { offset_in_folio((*bh).b_folio, (*bh).b_data) }

unsafe fn nilfs_palloc_entry_offset(inode: *const inode, nr: u64, bh: *const buffer_head) -> usize {
    let mut entry_index_in_group = 0;
    nilfs_palloc_group(inode, nr, &mut entry_index_in_group);
    let entry_index_in_block = entry_index_in_group % (*NILFS_MDT(inode)).mi_entries_per_block;
    offset_in_folio((*bh).b_folio, (*bh).b_data) + entry_index_in_block as usize * (*NILFS_MDT(inode)).mi_entry_size as usize
}

unsafe fn nilfs_palloc_find_available_slot(bitmap: *mut u8, target: c_ulong, bsize: c_uint, lock: *mut spinlock_t, wrap: bool) -> c_int {
    let mut end = bsize as c_int;
    if likely(target < bsize as c_ulong) {
        let mut pos = target as c_int;
        loop {
            pos = nilfs_find_next_zero_bit(bitmap, end as c_ulong, pos as c_ulong) as c_int;
            if pos >= end { break; }
            if !nilfs_set_bit_atomic(lock, pos as c_ulong, bitmap) { return pos; }
            pos += 1;
            if pos >= end { break; }
        }
        end = target as c_int;
    }
    if !wrap { return -ENOSPC; }
    let mut pos = 0;
    while pos < end {
        pos = nilfs_find_next_zero_bit(bitmap, end as c_ulong, pos as c_ulong) as c_int;
        if pos >= end { break; }
        if !nilfs_set_bit_atomic(lock, pos as c_ulong, bitmap) { return pos; }
        pos += 1;
    }
    -ENOSPC
}

unsafe fn nilfs_palloc_rest_groups_in_desc_block(inode: *const inode, curr: c_ulong, max: c_ulong) -> c_ulong {
    min_t(nilfs_palloc_groups_per_desc_block(inode) - curr % nilfs_palloc_groups_per_desc_block(inode), max - curr + 1)
}

unsafe fn nilfs_palloc_count_desc_blocks(inode: *mut inode, desc_blocks: *mut c_ulong) -> c_int {
    let mut blknum = 0u64;
    let ret = nilfs_bmap_last_key((*NILFS_I(inode)).i_bmap, &mut blknum);
    if likely(ret == 0) { *desc_blocks = DIV_ROUND_UP(blknum as c_ulong, (*NILFS_MDT(inode)).mi_blocks_per_desc_block); }
    ret
}

#[inline]
unsafe fn nilfs_palloc_mdt_file_can_grow(inode: *mut inode, desc_blocks: c_ulong) -> bool {
    nilfs_palloc_groups_per_desc_block(inode) * desc_blocks < nilfs_palloc_groups_count(inode)
}

unsafe fn nilfs_palloc_count_max_entries(inode: *mut inode, nused: u64, nmaxp: *mut u64) -> c_int {
    let mut desc_blocks = 0;
    let err = nilfs_palloc_count_desc_blocks(inode, &mut desc_blocks);
    if unlikely(err != 0) { return err; }
    let entries_per_desc_block = nilfs_palloc_entries_per_group(inode) as u64 * nilfs_palloc_groups_per_desc_block(inode);
    let mut nmax = entries_per_desc_block * desc_blocks as u64;
    if nused == nmax && nilfs_palloc_mdt_file_can_grow(inode, desc_blocks) { nmax += entries_per_desc_block; }
    if nused > nmax { return -ERANGE; }
    *nmaxp = nmax; 0
}

unsafe fn nilfs_palloc_prepare_alloc_entry(inode: *mut inode, req: *mut nilfs_palloc_req, wrap: bool) -> c_int {
    let (mut desc_bh, mut bitmap_bh) = (core::ptr::null_mut(), core::ptr::null_mut());
    let mut group_offset = 0; let mut maxgroup_offset = 0;
    let ngroups = nilfs_palloc_groups_count(inode); let mut maxgroup = ngroups - 1;
    let mut group = nilfs_palloc_group(inode, (*req).pr_entry_nr, &mut group_offset);
    let entries_per_group = nilfs_palloc_entries_per_group(inode);
    let mut i = 0;
    while i < ngroups {
        if group >= ngroups && wrap { group = 0; maxgroup = nilfs_palloc_group(inode, (*req).pr_entry_nr, &mut maxgroup_offset) - 1; }
        let ret = nilfs_palloc_get_desc_block(inode, group, 1, &mut desc_bh); if ret < 0 { return ret; }
        let doff = nilfs_palloc_group_desc_offset(inode, group, desc_bh);
        let mut desc = kmap_local_folio((*desc_bh).b_folio, doff) as *mut nilfs_palloc_group_desc;
        let n = nilfs_palloc_rest_groups_in_desc_block(inode, group, maxgroup);
        let mut j = 0;
        while j < n {
            let lock = nilfs_mdt_bgl_lock(inode, group);
            if nilfs_palloc_group_desc_nfrees(desc.add(j as usize), lock) != 0 {
                kunmap_local(desc as *mut c_void);
                let ret = nilfs_palloc_get_bitmap_block(inode, group, 1, &mut bitmap_bh);
                if ret < 0 { brelse(desc_bh); return ret; }
                desc = kmap_local_folio((*desc_bh).b_folio, doff) as *mut nilfs_palloc_group_desc;
                let bitmap = kmap_local_folio((*bitmap_bh).b_folio, nilfs_palloc_bitmap_offset(bitmap_bh)) as *mut u8;
                let pos = nilfs_palloc_find_available_slot(bitmap, group_offset, entries_per_group as c_uint, lock, wrap);
                kunmap_local(bitmap as *mut c_void);
                if pos >= 0 { nilfs_palloc_group_desc_add_entries(desc.add(j as usize), lock, u32::MAX); (*req).pr_entry_nr = entries_per_group as u64 * group as u64 + pos as u64; kunmap_local(desc as *mut c_void); (*req).pr_desc_bh = desc_bh; (*req).pr_bitmap_bh = bitmap_bh; return 0; }
                brelse(bitmap_bh); desc = kmap_local_folio((*desc_bh).b_folio, doff) as *mut nilfs_palloc_group_desc;
            }
            j += 1; group += 1; group_offset = 0;
        }
        kunmap_local(desc as *mut c_void); brelse(desc_bh); i += n;
    }
    -ENOSPC
}

unsafe fn nilfs_palloc_commit_alloc_entry(_inode: *mut inode, req: *mut nilfs_palloc_req) { mark_buffer_dirty((*req).pr_bitmap_bh); mark_buffer_dirty((*req).pr_desc_bh); nilfs_mdt_mark_dirty(_inode); brelse((*req).pr_bitmap_bh); brelse((*req).pr_desc_bh); }

unsafe fn nilfs_palloc_commit_free_entry(inode: *mut inode, req: *mut nilfs_palloc_req) {
    let mut group_offset = 0; let group = nilfs_palloc_group(inode, (*req).pr_entry_nr, &mut group_offset);
    let desc = kmap_local_folio((*(*req).pr_desc_bh).b_folio, nilfs_palloc_group_desc_offset(inode, group, (*req).pr_desc_bh)) as *mut nilfs_palloc_group_desc;
    let bitmap = kmap_local_folio((*(*req).pr_bitmap_bh).b_folio, nilfs_palloc_bitmap_offset((*req).pr_bitmap_bh)) as *mut u8;
    let lock = nilfs_mdt_bgl_lock(inode, group);
    if nilfs_clear_bit_atomic(lock, group_offset, bitmap) == 0 { nilfs_warn((*inode).i_sb, "%s (ino=%llu): entry number %llu already freed", __func__, (*inode).i_ino, (*req).pr_entry_nr); } else { nilfs_palloc_group_desc_add_entries(desc, lock, 1); }
    kunmap_local(bitmap as *mut c_void); kunmap_local(desc as *mut c_void); mark_buffer_dirty((*req).pr_desc_bh); mark_buffer_dirty((*req).pr_bitmap_bh); nilfs_mdt_mark_dirty(inode); brelse((*req).pr_bitmap_bh); brelse((*req).pr_desc_bh);
}

unsafe fn nilfs_palloc_abort_alloc_entry(_inode: *mut inode, req: *mut nilfs_palloc_req) { brelse((*req).pr_bitmap_bh); brelse((*req).pr_desc_bh); (*req).pr_entry_nr = 0; (*req).pr_bitmap_bh = core::ptr::null_mut(); (*req).pr_desc_bh = core::ptr::null_mut(); }

unsafe fn nilfs_palloc_prepare_free_entry(inode: *mut inode, req: *mut nilfs_palloc_req) -> c_int {
    let mut group_offset = 0; let group = nilfs_palloc_group(inode, (*req).pr_entry_nr, &mut group_offset);
    let mut desc_bh = core::ptr::null_mut(); let mut bitmap_bh = core::ptr::null_mut();
    let mut ret = nilfs_palloc_get_desc_block(inode, group, 1, &mut desc_bh); if ret < 0 { return ret; }
    ret = nilfs_palloc_get_bitmap_block(inode, group, 1, &mut bitmap_bh); if ret < 0 { brelse(desc_bh); return ret; }
    (*req).pr_desc_bh = desc_bh; (*req).pr_bitmap_bh = bitmap_bh; 0
}

unsafe fn nilfs_palloc_abort_free_entry(_inode: *mut inode, req: *mut nilfs_palloc_req) { brelse((*req).pr_bitmap_bh); brelse((*req).pr_desc_bh); (*req).pr_entry_nr = 0; (*req).pr_bitmap_bh = core::ptr::null_mut(); (*req).pr_desc_bh = core::ptr::null_mut(); }

// The bulk-free path follows the source algorithm; bitmap and descriptor updates
// are performed under the allocator group lock and empty blocks are removed.
unsafe fn nilfs_palloc_freev(inode: *mut inode, entry_nrs: *mut u64, nitems: usize) -> c_int {
    let epg = nilfs_palloc_entries_per_group(inode); let epb = (*NILFS_MDT(inode)).mi_entries_per_block;
    let mut i = 0;
    while i < nitems {
        let mut group_offset = 0; let group = nilfs_palloc_group(inode, *entry_nrs.add(i), &mut group_offset);
        let mut desc_bh = core::ptr::null_mut(); let mut bitmap_bh = core::ptr::null_mut();
        let mut ret = nilfs_palloc_get_desc_block(inode, group, 0, &mut desc_bh); if ret < 0 { return ret; }
        ret = nilfs_palloc_get_bitmap_block(inode, group, 0, &mut bitmap_bh); if ret < 0 { brelse(desc_bh); return ret; }
        let bitmap = kmap_local_folio((*bitmap_bh).b_folio, nilfs_palloc_bitmap_offset(bitmap_bh)) as *mut u8; let lock = nilfs_mdt_bgl_lock(inode, group);
        let group_min_nr = group as u64 * epg as u64; let mut j = i; let mut n = 0u32;
        while j < nitems { if *entry_nrs.add(j) < group_min_nr || *entry_nrs.add(j) >= group_min_nr + epg as u64 { break; } if nilfs_clear_bit_atomic(lock, *entry_nrs.add(j) - group_min_nr, bitmap) != 0 { n += 1; } j += 1; }
        kunmap_local(bitmap as *mut c_void); mark_buffer_dirty(bitmap_bh); brelse(bitmap_bh);
        let desc = kmap_local_folio((*desc_bh).b_folio, nilfs_palloc_group_desc_offset(inode, group, desc_bh)) as *mut nilfs_palloc_group_desc; let nfree = nilfs_palloc_group_desc_add_entries(desc, lock, n); kunmap_local(desc as *mut c_void); mark_buffer_dirty(desc_bh); nilfs_mdt_mark_dirty(inode); brelse(desc_bh);
        if nfree == epg { ret = nilfs_palloc_delete_bitmap_block(inode, group); if ret != 0 && ret != -ENOENT { nilfs_warn((*inode).i_sb, "error %d deleting bitmap block of group=%lu, ino=%llu", ret, group, (*inode).i_ino); } }
        i = j;
    }
    0
}

unsafe fn nilfs_palloc_setup_cache(inode: *mut inode, cache: *mut nilfs_palloc_cache) { (*NILFS_MDT(inode)).mi_palloc_cache = cache; spin_lock_init(&mut (*cache).lock); }
unsafe fn nilfs_palloc_clear_cache(inode: *mut inode) { let cache = (*NILFS_MDT(inode)).mi_palloc_cache; spin_lock(&mut (*cache).lock); brelse((*cache).prev_desc.bh); brelse((*cache).prev_bitmap.bh); brelse((*cache).prev_entry.bh); (*cache).prev_desc.bh = core::ptr::null_mut(); (*cache).prev_bitmap.bh = core::ptr::null_mut(); (*cache).prev_entry.bh = core::ptr::null_mut(); spin_unlock(&mut (*cache).lock); }
unsafe fn nilfs_palloc_destroy_cache(inode: *mut inode) { nilfs_palloc_clear_cache(inode); (*NILFS_MDT(inode)).mi_palloc_cache = core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
