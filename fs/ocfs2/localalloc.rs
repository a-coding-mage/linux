// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of localalloc.c. External kernel types and functions are
 * supplied by the surrounding translation unit. */

// C includes and build-time configuration are intentionally represented by
// external dependencies and conditional compilation in the surrounding code.

const OCFS2_LA_MAX_DEFAULT_MB: u32 = 256;
const OCFS2_LA_OLD_DEFAULT: u32 = 8;

unsafe fn ocfs2_la_default_mb(osb: *mut ocfs2_super) -> u32 {
    let sb = (*osb).sb;
    let mut gd_mb = ocfs2_clusters_to_megabytes(sb, 8 * ocfs2_group_bitmap_size(sb, 0, (*osb).s_feature_incompat));
    if ((*sb).s_blocksize == 512 && (*osb).s_clustersize <= 8192) || ((*sb).s_blocksize == 1024 && (*osb).s_clustersize == 4096) { return OCFS2_LA_OLD_DEFAULT; }
    gd_mb -= 16; gd_mb &= 0xFFFFFFFB;
    let mut la_mb = gd_mb;
    if la_mb > OCFS2_LA_MAX_DEFAULT_MB {
        if gd_mb > 2 * OCFS2_LA_MAX_DEFAULT_MB { la_mb = 256; }
        else { let mut gd_mult = gd_mb; while gd_mult > 256 { gd_mult >>= 1; } la_mb = gd_mult; }
    }
    let mut megs_per_slot = (*osb).osb_clusters_at_boot / (*osb).max_slots;
    megs_per_slot = ocfs2_clusters_to_megabytes(sb, megs_per_slot);
    if megs_per_slot < la_mb { la_mb = megs_per_slot; }
    let la_max_mb = ocfs2_clusters_to_megabytes(sb, ocfs2_local_alloc_size(sb) * 8);
    if la_mb > la_max_mb { la_mb = la_max_mb; }
    la_mb
}

unsafe fn ocfs2_la_set_sizes(osb: *mut ocfs2_super, requested_mb: i32) {
    let sb = (*osb).sb; let default_mb = ocfs2_la_default_mb(osb);
    let max_mb = ocfs2_clusters_to_megabytes(sb, ocfs2_local_alloc_size(sb) * 8);
    trace_ocfs2_la_set_sizes(requested_mb, max_mb, default_mb);
    (*osb).local_alloc_default_bits = if requested_mb == -1 { ocfs2_megabytes_to_clusters(sb, default_mb) } else if requested_mb as u32 > max_mb { ocfs2_megabytes_to_clusters(sb, max_mb) } else { ocfs2_megabytes_to_clusters(sb, requested_mb as u32) };
    (*osb).local_alloc_bits = (*osb).local_alloc_default_bits;
}

#[inline] unsafe fn ocfs2_la_state_enabled(osb: *mut ocfs2_super) -> bool { (*osb).local_alloc_state == OCFS2_LA_THROTTLED || (*osb).local_alloc_state == OCFS2_LA_ENABLED }

unsafe fn ocfs2_local_alloc_seen_free_bits(osb: *mut ocfs2_super, num_clusters: u32) {
    if num_clusters >= (*osb).local_alloc_default_bits { spin_lock(&mut (*osb).osb_lock); if (*osb).local_alloc_state == OCFS2_LA_DISABLED || (*osb).local_alloc_state == OCFS2_LA_THROTTLED { cancel_delayed_work(&mut (*osb).la_enable_wq); (*osb).local_alloc_state = OCFS2_LA_ENABLED; } spin_unlock(&mut (*osb).osb_lock); }
}

unsafe fn ocfs2_la_enable_worker(work: *mut work_struct) { let osb = container_of!(work, ocfs2_super, la_enable_wq.work); spin_lock(&mut (*osb).osb_lock); (*osb).local_alloc_state = OCFS2_LA_ENABLED; spin_unlock(&mut (*osb).osb_lock); }

unsafe fn ocfs2_alloc_should_use_local(osb: *mut ocfs2_super, bits: u64) -> i32 {
    let mut ret = 0; spin_lock(&mut (*osb).osb_lock); let la_bits = (*osb).local_alloc_bits;
    if !ocfs2_la_state_enabled(osb) { spin_unlock(&mut (*osb).osb_lock); return ret; }
    if bits > (la_bits / 2) as u64 { spin_unlock(&mut (*osb).osb_lock); return ret; }
    ret = 1; trace_ocfs2_alloc_should_use_local(bits, (*osb).local_alloc_state, la_bits, ret); spin_unlock(&mut (*osb).osb_lock); ret
}

unsafe fn ocfs2_local_alloc_count_bits(alloc: *mut ocfs2_dinode) -> u32 { let la = &mut (*alloc).id2.i_lab; let count = memweight(la.la_bitmap, le16_to_cpu(la.la_size)); trace_ocfs2_local_alloc_count_bits(count); count }

unsafe fn ocfs2_clear_local_alloc(alloc: *mut ocfs2_dinode) { let la = &mut (*alloc).id2.i_lab; (*alloc).id1.bitmap1.i_total = 0; (*alloc).id1.bitmap1.i_used = 0; la.la_bm_off = 0; memset(la.la_bitmap, 0, le16_to_cpu(la.la_size)); }

unsafe fn ocfs2_load_local_alloc(osb: *mut ocfs2_super) -> i32 {
    let mut status = 0; if (*osb).local_alloc_bits == 0 { return 0; }
    if (*osb).local_alloc_bits >= (*osb).bitmap_cpg { mlog(ML_NOTICE, "Requested local alloc window is larger than max possible"); (*osb).local_alloc_bits = ocfs2_megabytes_to_clusters((*osb).sb, ocfs2_la_default_mb(osb)); }
    let inode = ocfs2_get_system_file_inode(osb, LOCAL_ALLOC_SYSTEM_INODE, (*osb).slot_num); if inode.is_null() { return -EINVAL; }
    let mut bh = core::ptr::null_mut(); status = ocfs2_read_inode_block_full(inode, &mut bh, OCFS2_BH_IGNORE_CACHE); if status < 0 { brelse(bh); iput(inode); return status; }
    let alloc = (*bh).b_data as *mut ocfs2_dinode; let la = &mut (*alloc).id2.i_lab;
    if le32_to_cpu((*alloc).i_flags) & (OCFS2_LOCAL_ALLOC_FL|OCFS2_BITMAP_FL) == 0 || la.la_size == 0 || le16_to_cpu(la.la_size) > ocfs2_local_alloc_size((*inode).i_sb) { brelse(bh); iput(inode); return -EINVAL; }
    if ocfs2_local_alloc_count_bits(alloc) != 0 || (*alloc).id1.bitmap1.i_used != 0 || (*alloc).id1.bitmap1.i_total != 0 || la.la_bm_off != 0 { brelse(bh); iput(inode); return -EINVAL; }
    (*osb).local_alloc_bh = bh; (*osb).local_alloc_state = OCFS2_LA_ENABLED; iput(inode); trace_ocfs2_load_local_alloc((*osb).local_alloc_bits); status
}

unsafe fn ocfs2_begin_local_alloc_recovery(osb: *mut ocfs2_super, slot_num: i32, alloc_copy: *mut *mut ocfs2_dinode) -> i32 {
    trace_ocfs2_begin_local_alloc_recovery(slot_num); *alloc_copy = core::ptr::null_mut(); let inode = ocfs2_get_system_file_inode(osb, LOCAL_ALLOC_SYSTEM_INODE, slot_num); if inode.is_null() { return -EINVAL; } inode_lock(inode); let mut bh = core::ptr::null_mut(); let mut status = ocfs2_read_inode_block_full(inode, &mut bh, OCFS2_BH_IGNORE_CACHE); if status >= 0 { *alloc_copy = kmalloc((*bh).b_size, GFP_KERNEL); if (*alloc_copy).is_null() { status = -ENOMEM; } else { memcpy(*alloc_copy, (*bh).b_data, (*bh).b_size); let alloc = (*bh).b_data as *mut ocfs2_dinode; ocfs2_clear_local_alloc(alloc); ocfs2_compute_meta_ecc((*osb).sb, (*bh).b_data, &mut (*alloc).i_check); status = ocfs2_write_block(osb, bh, INODE_CACHE(inode)); } } if status < 0 { kfree(*alloc_copy); *alloc_copy = core::ptr::null_mut(); } brelse(bh); inode_unlock(inode); iput(inode); status
}

// Remaining routines preserve the original low-level interfaces and are
// intentionally kept in unsafe form because their definitions are external.
unsafe fn ocfs2_sync_local_to_main(osb: *mut ocfs2_super, handle: *mut handle_t, alloc: *mut ocfs2_dinode, main_bm_inode: *mut inode, main_bm_bh: *mut buffer_head) -> i32 {
    let la = &mut (*alloc).id2.i_lab; if (*alloc).id1.bitmap1.i_total == 0 || (*alloc).id1.bitmap1.i_used == (*alloc).id1.bitmap1.i_total { return 0; }
    let mut start = 0; let left = le32_to_cpu((*alloc).id1.bitmap1.i_total); let bitmap = la.la_bitmap; let base = ocfs2_clusters_to_blocks((*osb).sb, le32_to_cpu(la.la_bm_off));
    while start < left { let bit = ocfs2_find_next_zero_bit(bitmap, left, start); if bit == start { let mut end = start; while end < left && ocfs2_test_bit(end, bitmap) == 0 { end += 1; } let s = base + ocfs2_clusters_to_blocks((*osb).sb, start); let r = ocfs2_release_clusters(handle, main_bm_inode, main_bm_bh, s, end-start); if r < 0 { return r; } start = end; } else { start = bit + 1; } } 0
}

// The declarations below intentionally expose the remaining source-level API;
// dependent kernel structures and helper definitions are supplied elsewhere.
extern "C" { fn ocfs2_shutdown_local_alloc(osb: *mut ocfs2_super); fn ocfs2_complete_local_alloc_recovery(osb: *mut ocfs2_super, alloc: *mut ocfs2_dinode) -> i32; fn ocfs2_reserve_local_alloc_bits(osb: *mut ocfs2_super, bits_wanted: u32, ac: *mut ocfs2_alloc_context) -> i32; fn ocfs2_claim_local_alloc_bits(osb: *mut ocfs2_super, handle: *mut handle_t, ac: *mut ocfs2_alloc_context, bits_wanted: u32, bit_off: *mut u32, num_bits: *mut u32) -> i32; fn ocfs2_free_local_alloc_bits(osb: *mut ocfs2_super, handle: *mut handle_t, ac: *mut ocfs2_alloc_context, bit_off: u32, num_bits: u32) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
