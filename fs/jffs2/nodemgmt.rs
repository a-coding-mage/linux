/* JFFS2 -- Journalling Flash File System, Version 2. */
/* Direct low-level translation of nodemgmt.c; external kernel/JFFS2 symbols are supplied elsewhere. */

unsafe fn jffs2_rp_can_write(c: *mut jffs2_sb_info) -> i32 {
    let opts = &mut (*c).mount_opts;
    let avail = (*c).dirty_size + (*c).free_size + (*c).unchecked_size
        + (*c).erasing_size - (*c).resv_blocks_write * (*c).sector_size
        - (*c).nospc_dirty_size;
    if avail < 2 * opts.rp_size { jffs2_dbg!(1, "rpsize %u, dirty_size %u, free_size %u, erasing_size %u, unchecked_size %u, nr_erasing_blocks %u, avail %u, resrv %u\n", opts.rp_size, (*c).dirty_size, (*c).free_size, (*c).erasing_size, (*c).unchecked_size, (*c).nr_erasing_blocks, avail, (*c).nospc_dirty_size); }
    if avail > opts.rp_size { return 1; }
    if capable(CAP_SYS_RESOURCE) != 0 { return 1; }
    jffs2_dbg!(1, "forbid writing\n"); 0
}

unsafe fn jffs2_reserve_space(c: *mut jffs2_sb_info, mut minsize: u32, len: *mut u32, prio: i32, sumsize: u32) -> i32 {
    let mut ret = -EAGAIN;
    let blocksneeded = (*c).resv_blocks_write;
    minsize = PAD!(minsize);
    jffs2_dbg!(1, "%s(): Requested 0x%x bytes\n", __func__(), minsize);
    mutex_lock(&mut (*c).alloc_sem); jffs2_dbg!(1, "%s(): alloc sem got\n", __func__());
    spin_lock(&mut (*c).erase_completion_lock);
    if prio != ALLOC_DELETION && jffs2_rp_can_write(c) == 0 { ret = -ENOSPC; goto!(out); }
    while ret == -EAGAIN {
        while (*c).nr_free_blocks + (*c).nr_erasing_blocks < blocksneeded {
            let dirty = (*c).dirty_size + (*c).erasing_size - (*c).nr_erasing_blocks * (*c).sector_size + (*c).unchecked_size;
            if dirty < (*c).nospc_dirty_size {
                if !(prio == ALLOC_DELETION && (*c).nr_free_blocks + (*c).nr_erasing_blocks >= (*c).resv_blocks_deletion) { spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).alloc_sem); return -ENOSPC; }
                break;
            }
            let avail = (*c).free_size + (*c).dirty_size + (*c).erasing_size + (*c).unchecked_size;
            if avail / (*c).sector_size <= blocksneeded {
                if !(prio == ALLOC_DELETION && (*c).nr_free_blocks + (*c).nr_erasing_blocks >= (*c).resv_blocks_deletion) { spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).alloc_sem); return -ENOSPC; }
                break;
            }
            mutex_unlock(&mut (*c).alloc_sem); spin_unlock(&mut (*c).erase_completion_lock);
            ret = jffs2_garbage_collect_pass(c);
            if ret == -EAGAIN { cond_resched(); } else if ret != 0 { return ret; }
            cond_resched(); if signal_pending(current()) != 0 { return -EINTR; }
            mutex_lock(&mut (*c).alloc_sem); spin_lock(&mut (*c).erase_completion_lock);
        }
        ret = jffs2_do_reserve_space(c, minsize, len, sumsize);
    }
out:
    spin_unlock(&mut (*c).erase_completion_lock);
    if ret == 0 { ret = jffs2_prealloc_raw_node_refs(c, (*c).nextblock, 1); }
    if ret != 0 { mutex_unlock(&mut (*c).alloc_sem); } ret
}

unsafe fn jffs2_reserve_space_gc(c: *mut jffs2_sb_info, mut minsize: u32, len: *mut u32, sumsize: u32) -> i32 {
    minsize = PAD!(minsize);
    loop { spin_lock(&mut (*c).erase_completion_lock); let ret = jffs2_do_reserve_space(c, minsize, len, sumsize); spin_unlock(&mut (*c).erase_completion_lock); if ret != -EAGAIN { if ret == 0 { return jffs2_prealloc_raw_node_refs(c, (*c).nextblock, 1); } return ret; } cond_resched(); }
}

unsafe fn jffs2_close_nextblock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) {
    if (*c).nextblock.is_null() { return; }
    if ISDIRTY!((*jeb).wasted_size + (*jeb).dirty_size) { (*c).dirty_size += (*jeb).wasted_size; (*c).wasted_size -= (*jeb).wasted_size; (*jeb).dirty_size += (*jeb).wasted_size; (*jeb).wasted_size = 0; if VERYDIRTY!(c, (*jeb).dirty_size) { list_add_tail(&mut (*jeb).list, &mut (*c).very_dirty_list); } else { list_add_tail(&mut (*jeb).list, &mut (*c).dirty_list); } } else { list_add_tail(&mut (*jeb).list, &mut (*c).clean_list); }
    (*c).nextblock = core::ptr::null_mut();
}

unsafe fn jffs2_find_nextblock(c: *mut jffs2_sb_info) -> i32 {
    if list_empty(&(*c).free_list) != 0 {
        if (*c).nr_erasing_blocks == 0 && list_empty(&(*c).erasable_list) == 0 { let e = list_entry((*c).erasable_list.next, jffs2_eraseblock, list); list_move_tail(&mut (*e).list, &mut (*c).erase_pending_list); (*c).nr_erasing_blocks += 1; jffs2_garbage_collect_trigger(c); }
        if (*c).nr_erasing_blocks == 0 && list_empty(&(*c).erasable_pending_wbuf_list) == 0 { spin_unlock(&mut (*c).erase_completion_lock); jffs2_flush_wbuf_pad(c); spin_lock(&mut (*c).erase_completion_lock); return -EAGAIN; }
        if (*c).nr_erasing_blocks == 0 { return -ENOSPC; }
        spin_unlock(&mut (*c).erase_completion_lock); jffs2_erase_pending_blocks(c, 1); spin_lock(&mut (*c).erase_completion_lock); return -EAGAIN;
    }
    let next = (*c).free_list.next; list_del(next); (*c).nextblock = list_entry(next, jffs2_eraseblock, list); (*c).nr_free_blocks -= 1; jffs2_sum_reset_collected((*c).summary); 0
}

unsafe fn jffs2_do_reserve_space(c: *mut jffs2_sb_info, minsize: u32, len: *mut u32, _sumsize: u32) -> i32 {
    let mut jeb = (*c).nextblock;
    loop {
        if jeb.is_null() || minsize > (*jeb).free_size { if !jeb.is_null() { let waste = (*jeb).free_size; jffs2_link_node_ref(c, jeb, ((*jeb).offset + (*c).sector_size - waste) | REF_OBSOLETE, waste, core::ptr::null_mut()); (*jeb).dirty_size -= waste; (*c).dirty_size -= waste; (*jeb).wasted_size += waste; (*c).wasted_size += waste; jffs2_close_nextblock(c, jeb); } let r = jffs2_find_nextblock(c); if r != 0 { return r; } jeb = (*c).nextblock; }
        if (*jeb).free_size != (*c).sector_size - (*c).cleanmarker_size { jeb = (*c).nextblock; continue; }
        *len = (*jeb).free_size; return 0;
    }
}

pub unsafe fn jffs2_add_physical_node_ref(c: *mut jffs2_sb_info, ofs: u32, len: u32, ic: *mut jffs2_inode_cache) -> *mut jffs2_raw_node_ref { let jeb = &mut (*c).blocks[(ofs / (*c).sector_size) as usize]; spin_lock(&mut (*c).erase_completion_lock); let n = jffs2_link_node_ref(c, jeb, ofs, len, ic); spin_unlock(&mut (*c).erase_completion_lock); n }
pub unsafe fn jffs2_complete_reservation(c: *mut jffs2_sb_info) { spin_lock(&mut (*c).erase_completion_lock); jffs2_garbage_collect_trigger(c); spin_unlock(&mut (*c).erase_completion_lock); mutex_unlock(&mut (*c).alloc_sem); }
unsafe fn on_list(obj: *mut list_head, head: *mut list_head) -> i32 { let mut p = (*head).next; while p != head { if p == obj { return 1; } p = (*p).next; } 0 }

pub unsafe fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, ref_: *mut jffs2_raw_node_ref) { if ref_.is_null() || ref_obsolete(ref_) != 0 { return; } let blocknr = (*ref_).flash_offset / (*c).sector_size; if blocknr >= (*c).nr_blocks { BUG!(); } let jeb = &mut (*c).blocks[blocknr as usize]; spin_lock(&mut (*c).erase_completion_lock); let freed_len = ref_totlen(c, jeb, ref_); if ref_flags(ref_) == REF_UNCHECKED { (*jeb).unchecked_size -= freed_len; (*c).unchecked_size -= freed_len; } else { (*jeb).used_size -= freed_len; (*c).used_size -= freed_len; } if (*jeb).dirty_size != 0 || ISDIRTY!((*jeb).wasted_size + freed_len) && jeb != (*c).nextblock { (*jeb).dirty_size += freed_len; (*c).dirty_size += freed_len; } else { (*jeb).wasted_size += freed_len; (*c).wasted_size += freed_len; } (*ref_).flash_offset = ref_offset(ref_) | REF_OBSOLETE; spin_unlock(&mut (*c).erase_completion_lock); }

pub unsafe fn jffs2_thread_should_wake(c: *mut jffs2_sb_info) -> i32 { if list_empty(&(*c).erase_complete_list) == 0 || list_empty(&(*c).erase_pending_list) == 0 || (*c).unchecked_size != 0 { return 1; } let dirty = (*c).dirty_size + (*c).erasing_size - (*c).nr_erasing_blocks * (*c).sector_size; if (*c).nr_free_blocks + (*c).nr_erasing_blocks < (*c).resv_blocks_gctrigger && dirty > (*c).nospc_dirty_size { return 1; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
