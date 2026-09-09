// SPDX-License-Identifier: GPL-2.0
/*
 * f2fs shrinker support
 *   the basic infra was copied from fs/ubifs/shrinker.c
 *
 * Copyright (c) 2015 Motorola Mobility
 * Copyright (c) 2015 Jaegeuk Kim <jaegeuk@kernel.org>
 */

// Dependencies supplied by the surrounding kernel/f2fs translation.

static mut f2fs_list: list_head = unsafe { core::mem::zeroed() };
static mut f2fs_list_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut shrinker_run_no: c_uint = 0;

unsafe fn __count_nat_entries(sbi: *mut f2fs_sb_info) -> c_ulong {
    (*NM_I(sbi)).nat_cnt[RECLAIMABLE_NAT]
}

unsafe fn __count_free_nids(sbi: *mut f2fs_sb_info) -> c_ulong {
    let count: c_long = (*NM_I(sbi)).nid_cnt[FREE_NID] - MAX_FREE_NIDS;
    if count > 0 { count as c_ulong } else { 0 }
}

unsafe fn __count_extent_cache(sbi: *mut f2fs_sb_info, type_: extent_type) -> c_ulong {
    let eti = &mut (*sbi).extent_tree[type_ as usize];
    (atomic_read(&eti.total_zombie_tree) + atomic_read(&eti.total_ext_node)) as c_ulong
}

pub unsafe fn f2fs_shrink_count(
    _shrink: *mut shrinker,
    _sc: *mut shrink_control,
) -> c_ulong {
    let mut sbi: *mut f2fs_sb_info;
    let mut p: *mut list_head;
    let mut count: c_ulong = 0;

    spin_lock(&mut f2fs_list_lock);
    p = (*(&raw mut f2fs_list)).next;
    while p != &raw mut f2fs_list {
        sbi = list_entry(p, f2fs_sb_info, s_list);

        /* stop f2fs_put_super */
        if !mutex_trylock(&mut (*sbi).umount_mutex) {
            p = (*p).next;
            continue;
        }
        spin_unlock(&mut f2fs_list_lock);

        /* count read extent cache entries */
        count += __count_extent_cache(sbi, EX_READ);
        /* count block age extent cache entries */
        count += __count_extent_cache(sbi, EX_BLOCK_AGE);
        /* count clean nat cache entries */
        count += __count_nat_entries(sbi);
        /* count free nids cache entries */
        count += __count_free_nids(sbi);

        spin_lock(&mut f2fs_list_lock);
        p = (*p).next;
        mutex_unlock(&mut (*sbi).umount_mutex);
    }
    spin_unlock(&mut f2fs_list_lock);
    if count != 0 { count } else { SHRINK_EMPTY }
}

pub unsafe fn f2fs_shrink_scan(
    _shrink: *mut shrinker,
    sc: *mut shrink_control,
) -> c_ulong {
    let nr = (*sc).nr_to_scan;
    let mut sbi: *mut f2fs_sb_info;
    let mut p: *mut list_head;
    let mut run_no: c_uint;
    let mut freed: c_ulong = 0;

    spin_lock(&mut f2fs_list_lock);
    loop {
        shrinker_run_no = shrinker_run_no.wrapping_add(1);
        run_no = shrinker_run_no;
        if run_no != 0 { break; }
    }
    p = (*(&raw mut f2fs_list)).next;
    while p != &raw mut f2fs_list {
        sbi = list_entry(p, f2fs_sb_info, s_list);
        if (*sbi).shrinker_run_no == run_no { break; }
        /* stop f2fs_put_super */
        if !mutex_trylock(&mut (*sbi).umount_mutex) {
            p = (*p).next;
            continue;
        }
        spin_unlock(&mut f2fs_list_lock);
        (*sbi).shrinker_run_no = run_no;
        /* shrink extent cache entries */
        freed += f2fs_shrink_age_extent_tree(sbi, nr >> 2);
        /* shrink read extent cache entries */
        if freed < nr { freed += f2fs_shrink_read_extent_tree(sbi, nr >> 2); }
        /* shrink clean nat cache entries */
        if freed < nr { freed += f2fs_try_to_free_nats(sbi, nr - freed); }
        /* shrink free nids cache entries */
        if freed < nr { freed += f2fs_try_to_free_nids(sbi, nr - freed); }
        spin_lock(&mut f2fs_list_lock);
        p = (*p).next;
        list_move_tail(&mut (*sbi).s_list, &mut f2fs_list);
        mutex_unlock(&mut (*sbi).umount_mutex);
        if freed >= nr { break; }
    }
    spin_unlock(&mut f2fs_list_lock);
    freed
}

pub unsafe fn f2fs_donate_files() -> c_uint {
    let mut p: *mut list_head;
    let mut donate_files: c_uint = 0;
    spin_lock(&mut f2fs_list_lock);
    p = (*(&raw mut f2fs_list)).next;
    while p != &raw mut f2fs_list {
        let sbi = list_entry(p, f2fs_sb_info, s_list);
        /* stop f2fs_put_super */
        if !mutex_trylock(&mut (*sbi).umount_mutex) { p = (*p).next; continue; }
        spin_unlock(&mut f2fs_list_lock);
        donate_files += (*sbi).donate_files;
        spin_lock(&mut f2fs_list_lock);
        p = (*p).next;
        mutex_unlock(&mut (*sbi).umount_mutex);
    }
    spin_unlock(&mut f2fs_list_lock);
    donate_files
}

unsafe fn do_reclaim_caches(sbi: *mut f2fs_sb_info, reclaim_caches_kb: c_uint) -> c_uint {
    let mut npages: pgoff_t = reclaim_caches_kb >> (PAGE_SHIFT - 10);
    let mut nfiles = (*sbi).donate_files;
    while npages != 0 && { nfiles -= 1; nfiles != c_uint::MAX } {
        let fi: *mut f2fs_inode_info;
        spin_lock(&mut (*sbi).inode_lock[DONATE_INODE]);
        if list_empty(&(*sbi).inode_list[DONATE_INODE]) {
            spin_unlock(&mut (*sbi).inode_lock[DONATE_INODE]); break;
        }
        fi = list_first_entry(&mut (*sbi).inode_list[DONATE_INODE], f2fs_inode_info, gdonate_list);
        list_move_tail(&mut (*fi).gdonate_list, &mut (*sbi).inode_list[DONATE_INODE]);
        let inode = igrab(&mut (*fi).vfs_inode);
        spin_unlock(&mut (*sbi).inode_lock[DONATE_INODE]);
        if inode.is_null() { continue; }
        inode_lock(inode);
        if !is_inode_flag_set(inode, FI_DONATE_FINISHED) {
            let len = (*fi).donate_end - (*fi).donate_start + 1;
            npages = if npages < len { 0 } else { npages - len };
            invalidate_inode_pages2_range((*inode).i_mapping, (*fi).donate_start, (*fi).donate_end);
            set_inode_flag(inode, FI_DONATE_FINISHED);
        }
        inode_unlock(inode);
        iput(inode);
        cond_resched();
    }
    npages << (PAGE_SHIFT - 10)
}

pub unsafe fn f2fs_reclaim_caches(mut reclaim_caches_kb: c_uint) {
    spin_lock(&mut f2fs_list_lock);
    let mut p = (*(&raw mut f2fs_list)).next;
    while p != &raw mut f2fs_list && reclaim_caches_kb != 0 {
        let sbi = list_entry(p, f2fs_sb_info, s_list);
        /* stop f2fs_put_super */
        if !mutex_trylock(&mut (*sbi).umount_mutex) { p = (*p).next; continue; }
        spin_unlock(&mut f2fs_list_lock);
        reclaim_caches_kb = do_reclaim_caches(sbi, reclaim_caches_kb);
        spin_lock(&mut f2fs_list_lock);
        p = (*p).next;
        mutex_unlock(&mut (*sbi).umount_mutex);
    }
    spin_unlock(&mut f2fs_list_lock);
}

pub unsafe fn f2fs_join_shrinker(sbi: *mut f2fs_sb_info) {
    spin_lock(&mut f2fs_list_lock);
    list_add_tail(&mut (*sbi).s_list, &mut f2fs_list);
    spin_unlock(&mut f2fs_list_lock);
}

pub unsafe fn f2fs_leave_shrinker(sbi: *mut f2fs_sb_info) {
    f2fs_shrink_read_extent_tree(sbi, __count_extent_cache(sbi, EX_READ));
    f2fs_shrink_age_extent_tree(sbi, __count_extent_cache(sbi, EX_BLOCK_AGE));
    spin_lock(&mut f2fs_list_lock);
    list_del_init(&mut (*sbi).s_list);
    spin_unlock(&mut f2fs_list_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
