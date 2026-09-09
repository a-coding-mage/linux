/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from fs/f2fs/gc.h. */

pub const GC_THREAD_MIN_WB_PAGES: u32 = 1;
pub const DEF_GC_THREAD_URGENT_SLEEP_TIME: u32 = 500;
pub const DEF_GC_THREAD_MIN_SLEEP_TIME: u32 = 30000;
pub const DEF_GC_THREAD_MAX_SLEEP_TIME: u32 = 60000;
pub const DEF_GC_THREAD_NOGC_SLEEP_TIME: u32 = 300000;

/* GC sleep parameters for zoned devices. */
pub const DEF_GC_THREAD_MIN_SLEEP_TIME_ZONED: u32 = 10;
pub const DEF_GC_THREAD_MAX_SLEEP_TIME_ZONED: u32 = 20;
pub const DEF_GC_THREAD_NOGC_SLEEP_TIME_ZONED: u32 = 60000;

pub const DEF_GC_THREAD_AGE_THRESHOLD: u32 = 60 * 60 * 24 * 7;
pub const DEF_GC_THREAD_CANDIDATE_RATIO: u32 = 20;
pub const DEF_GC_THREAD_MAX_CANDIDATE_COUNT: u32 = 10;
pub const DEF_GC_THREAD_AGE_WEIGHT: u32 = 60;
pub const DEF_GC_THREAD_VALID_THRESH_RATIO: u32 = 80;
pub const DEFAULT_ACCURACY_CLASS: u32 = 10000;

pub const LIMIT_INVALID_BLOCK: u32 = 40;
pub const LIMIT_FREE_BLOCK: u32 = 40;
pub const LIMIT_NO_ZONED_GC: u32 = 60;
pub const LIMIT_BOOST_ZONED_GC: u32 = 25;
pub const DEF_MIGRATION_WINDOW_GRANULARITY_ZONED: u32 = 3;
pub const BOOST_GC_MULTIPLE: u32 = 5;
pub const ZONED_PIN_SEC_REQUIRED_COUNT: u32 = 1;

pub const DEF_GC_FAILED_PINNED_FILES: u32 = 2048;
pub const MAX_GC_FAILED_PINNED_FILES: u32 = u16::MAX as u32;
pub const DEF_MAX_VICTIM_SEARCH: u32 = 4096;
pub const NR_GC_CHECKPOINT_SECS: u32 = 3;

#[repr(C)]
pub struct gc_inode_list {
    pub ilist: list_head,
    pub iroot: radix_tree_root,
}

#[repr(C)]
pub struct victim_entry {
    pub rb_node: rb_node,
    pub mtime: u64,
    pub segno: u32,
    pub list: list_head,
}

/* On a Zoned device, calculate usable blocks in currently free segments. */
#[inline]
pub unsafe fn free_segs_blk_count_zoned(sbi: *mut f2fs_sb_info) -> block_t {
    let mut free_seg_blks: block_t = 0;
    let free_i: *mut free_segmap_info = FREE_I(sbi);
    let mut j: i32 = 0;

    spin_lock(&mut (*free_i).segmap_lock);
    while j < MAIN_SEGS(sbi) {
        if !test_bit(j as usize, (*free_i).free_segmap) {
            free_seg_blks += f2fs_usable_blks_in_seg(sbi, j);
        }
        j += 1;
    }
    spin_unlock(&mut (*free_i).segmap_lock);

    free_seg_blks
}

#[inline]
pub unsafe fn free_segs_blk_count(sbi: *mut f2fs_sb_info) -> block_t {
    if f2fs_sb_has_blkzoned(sbi) {
        return free_segs_blk_count_zoned(sbi);
    }
    SEGS_TO_BLKS(sbi, free_segments(sbi))
}

#[inline]
pub unsafe fn free_user_blocks(sbi: *mut f2fs_sb_info) -> block_t {
    let free_blks = free_segs_blk_count(sbi);
    let ovp_blks = SEGS_TO_BLKS(sbi, overprovision_segments(sbi));

    if free_blks < ovp_blks { 0 } else { free_blks - ovp_blks }
}

#[inline]
pub fn limit_invalid_user_blocks(user_block_count: block_t) -> block_t {
    ((user_block_count * LIMIT_INVALID_BLOCK as block_t) as i64 / 100) as block_t
}

#[inline]
pub fn limit_free_user_blocks(reclaimable_user_blocks: block_t) -> block_t {
    ((reclaimable_user_blocks * LIMIT_FREE_BLOCK as block_t) as i64 / 100) as block_t
}

#[inline]
pub unsafe fn increase_sleep_time(gc_th: *mut f2fs_gc_kthread, wait: *mut u32) {
    let min_time = (*gc_th).min_sleep_time;
    let max_time = (*gc_th).max_sleep_time;
    if *wait == (*gc_th).no_gc_sleep_time { return; }
    if (*wait as i64) + (min_time as i64) > max_time as i64 {
        *wait = max_time;
    } else {
        *wait += min_time;
    }
}

#[inline]
pub unsafe fn decrease_sleep_time(gc_th: *mut f2fs_gc_kthread, wait: *mut u32) {
    let min_time = (*gc_th).min_sleep_time;
    if *wait == (*gc_th).no_gc_sleep_time { *wait = (*gc_th).max_sleep_time; }
    if (*wait as i64) - (min_time as i64) < min_time as i64 {
        *wait = min_time;
    } else {
        *wait -= min_time;
    }
}

#[inline]
pub unsafe fn has_enough_free_blocks(sbi: *mut f2fs_sb_info, limit_perc: u32) -> bool {
    free_sections(sbi) > ((*sbi).total_sections * limit_perc / 100)
}

#[inline]
pub unsafe fn has_enough_invalid_blocks(sbi: *mut f2fs_sb_info) -> bool {
    let user_block_count = (*sbi).user_block_count;
    let invalid_user_blocks = user_block_count - written_block_count(sbi);
    invalid_user_blocks > limit_invalid_user_blocks(user_block_count)
        && free_user_blocks(sbi) < limit_free_user_blocks(invalid_user_blocks)
}

#[inline]
pub unsafe fn need_to_boost_gc(sbi: *mut f2fs_sb_info) -> bool {
    if f2fs_sb_has_blkzoned(sbi) {
        return !has_enough_free_blocks(sbi, (*sbi).gc_thread.boost_zoned_gc_percent);
    }
    has_enough_invalid_blocks(sbi)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
