// SPDX-License-Identifier: GPL-2.0

// Kernel headers and local C headers are intentionally omitted; their types,
// constants, macros, and external functions are supplied by other modules.

const BTRFS_DISCARD_DELAY: u64 = 120 * NSEC_PER_SEC;
const BTRFS_DISCARD_UNUSED_DELAY: u64 = 10 * NSEC_PER_SEC;
const BTRFS_DISCARD_MIN_DELAY_MSEC: usize = 1;
const BTRFS_DISCARD_MAX_DELAY_MSEC: usize = 1000;
const BTRFS_DISCARD_MAX_IOPS: u32 = 1000;

static mut discard_minlen: [i32; BTRFS_NR_DISCARD_LISTS] = [
    0,
    BTRFS_ASYNC_DISCARD_MAX_FILTER,
    BTRFS_ASYNC_DISCARD_MIN_FILTER,
];

unsafe fn get_discard_list(
    discard_ctl: *mut btrfs_discard_ctl,
    block_group: *const btrfs_block_group,
) -> *mut list_head {
    &mut (*discard_ctl).discard_list[(*block_group).discard_index as usize]
}

unsafe fn btrfs_run_discard_work(discard_ctl: *const btrfs_discard_ctl) -> bool {
    let fs_info = container_of!(discard_ctl, btrfs_fs_info, discard_ctl);
    ((*(*fs_info).sb).s_flags & SB_RDONLY) == 0
        && test_bit(BTRFS_FS_DISCARD_RUNNING, &(*fs_info).flags)
}

unsafe fn __add_to_discard_list(
    discard_ctl: *mut btrfs_discard_ctl,
    block_group: *mut btrfs_block_group,
) {
    lockdep_assert_held(&(*discard_ctl).lock);
    if list_empty(&(*block_group).discard_list)
        || (*block_group).discard_index == BTRFS_DISCARD_INDEX_UNUSED
    {
        if (*block_group).discard_index == BTRFS_DISCARD_INDEX_UNUSED {
            (*block_group).discard_index = BTRFS_DISCARD_INDEX_START;
        }
        (*block_group).discard_eligible_time = ktime_get_ns() + BTRFS_DISCARD_DELAY;
        (*block_group).discard_state = BTRFS_DISCARD_RESET_CURSOR;
    }
    if list_empty(&(*block_group).discard_list) {
        btrfs_get_block_group(block_group);
    }
    list_move_tail(&mut (*block_group).discard_list,
                   get_discard_list(discard_ctl, block_group));
}

unsafe fn add_to_discard_list(
    discard_ctl: *mut btrfs_discard_ctl,
    block_group: *mut btrfs_block_group,
) {
    if !btrfs_is_block_group_data_only(block_group) || !btrfs_run_discard_work(discard_ctl) {
        return;
    }
    spin_lock(&mut (*discard_ctl).lock);
    __add_to_discard_list(discard_ctl, block_group);
    spin_unlock(&mut (*discard_ctl).lock);
}

unsafe fn add_to_discard_unused_list(
    discard_ctl: *mut btrfs_discard_ctl,
    block_group: *mut btrfs_block_group,
) {
    spin_lock(&mut (*discard_ctl).lock);
    let queued = !list_empty(&(*block_group).discard_list);
    if !btrfs_run_discard_work(discard_ctl) {
        spin_unlock(&mut (*discard_ctl).lock);
        return;
    }
    list_del_init(&mut (*block_group).discard_list);
    (*block_group).discard_index = BTRFS_DISCARD_INDEX_UNUSED;
    (*block_group).discard_eligible_time = ktime_get_ns() + BTRFS_DISCARD_UNUSED_DELAY;
    (*block_group).discard_state = BTRFS_DISCARD_RESET_CURSOR;
    if !queued { btrfs_get_block_group(block_group); }
    list_add_tail(&mut (*block_group).discard_list,
                  &mut (*discard_ctl).discard_list[BTRFS_DISCARD_INDEX_UNUSED as usize]);
    spin_unlock(&mut (*discard_ctl).lock);
}

unsafe fn remove_from_discard_list(
    discard_ctl: *mut btrfs_discard_ctl,
    block_group: *mut btrfs_block_group,
) -> bool {
    let mut running = false;
    spin_lock(&mut (*discard_ctl).lock);
    if block_group == (*discard_ctl).block_group {
        running = true;
        (*discard_ctl).block_group = core::ptr::null_mut();
    }
    (*block_group).discard_eligible_time = 0;
    let queued = !list_empty(&(*block_group).discard_list);
    list_del_init(&mut (*block_group).discard_list);
    if queued { btrfs_put_block_group(block_group); }
    spin_unlock(&mut (*discard_ctl).lock);
    running
}

unsafe fn find_next_block_group(
    discard_ctl: *mut btrfs_discard_ctl,
    now: u64,
) -> *mut btrfs_block_group {
    let mut ret: *mut btrfs_block_group = core::ptr::null_mut();
    for i in 0..BTRFS_NR_DISCARD_LISTS {
        let list = &mut (*discard_ctl).discard_list[i];
        if !list_empty(list) {
            let bg = list_first_entry!(list, btrfs_block_group, discard_list);
            if ret.is_null() { ret = bg; }
            if (*ret).discard_eligible_time < now { break; }
            if (*ret).discard_eligible_time > (*bg).discard_eligible_time { ret = bg; }
        }
    }
    ret
}

unsafe fn block_group_is_empty(bg: *const btrfs_block_group) -> bool {
    if (*bg).flags & BTRFS_BLOCK_GROUP_REMAPPED != 0 {
        (*bg).identity_remap_count == 0
    } else {
        (*bg).used == 0 && (*bg).remap_bytes == 0
    }
}

unsafe fn peek_discard_list(
    discard_ctl: *mut btrfs_discard_ctl,
    discard_state: *mut btrfs_discard_state,
    discard_index: *mut i32,
    now: u64,
) -> *mut btrfs_block_group {
    spin_lock(&mut (*discard_ctl).lock);
    let mut block_group;
    'again: loop {
        block_group = find_next_block_group(discard_ctl, now);
        if !block_group.is_null() && now >= (*block_group).discard_eligible_time {
            let empty = block_group_is_empty(block_group);
            if (*block_group).discard_index == BTRFS_DISCARD_INDEX_UNUSED && !empty {
                if btrfs_is_block_group_data_only(block_group) {
                    __add_to_discard_list(discard_ctl, block_group);
                    ASSERT!((*block_group).discard_index != BTRFS_DISCARD_INDEX_UNUSED,
                            "discard_index=%d", (*block_group).discard_index);
                } else {
                    list_del_init(&mut (*block_group).discard_list);
                    btrfs_put_block_group(block_group);
                }
                continue 'again;
            }
            if (*block_group).discard_state == BTRFS_DISCARD_RESET_CURSOR {
                (*block_group).discard_cursor = (*block_group).start;
                if (*block_group).flags & BTRFS_BLOCK_GROUP_REMAPPED != 0 && empty {
                    (*block_group).discard_state = BTRFS_DISCARD_FULLY_REMAPPED;
                } else {
                    (*block_group).discard_state = BTRFS_DISCARD_EXTENTS;
                }
            }
        }
        break;
    }
    if !block_group.is_null() {
        btrfs_get_block_group(block_group);
        (*discard_ctl).block_group = block_group;
        *discard_state = (*block_group).discard_state;
        *discard_index = (*block_group).discard_index;
    }
    spin_unlock(&mut (*discard_ctl).lock);
    block_group
}

pub unsafe fn btrfs_discard_check_filter(block_group: *mut btrfs_block_group, bytes: u64) {
    if block_group.is_null() || !btrfs_test_opt((*block_group).fs_info, DISCARD_ASYNC) { return; }
    let discard_ctl = &mut (*(*block_group).fs_info).discard_ctl;
    if (*block_group).discard_index > BTRFS_DISCARD_INDEX_START
        && bytes >= discard_minlen[((*block_group).discard_index - 1) as usize]
    {
        remove_from_discard_list(discard_ctl, block_group);
        for i in BTRFS_DISCARD_INDEX_START..BTRFS_NR_DISCARD_LISTS {
            if bytes >= discard_minlen[i as usize] {
                (*block_group).discard_index = i;
                add_to_discard_list(discard_ctl, block_group);
                break;
            }
        }
    }
}

unsafe fn btrfs_update_discard_index(discard_ctl: *mut btrfs_discard_ctl, block_group: *mut btrfs_block_group) {
    (*block_group).discard_index += 1;
    if (*block_group).discard_index == BTRFS_NR_DISCARD_LISTS {
        (*block_group).discard_index = 1;
        return;
    }
    add_to_discard_list(discard_ctl, block_group);
}

pub unsafe fn btrfs_discard_cancel_work(discard_ctl: *mut btrfs_discard_ctl, block_group: *mut btrfs_block_group) {
    if remove_from_discard_list(discard_ctl, block_group) {
        cancel_delayed_work_sync(&mut (*discard_ctl).work);
        btrfs_discard_schedule_work(discard_ctl, true);
    }
}

pub unsafe fn btrfs_discard_queue_work(discard_ctl: *mut btrfs_discard_ctl, block_group: *mut btrfs_block_group) {
    if block_group.is_null() || !btrfs_test_opt((*block_group).fs_info, DISCARD_ASYNC) { return; }
    if block_group_is_empty(block_group) { add_to_discard_unused_list(discard_ctl, block_group); }
    else { add_to_discard_list(discard_ctl, block_group); }
    if !delayed_work_pending(&mut (*discard_ctl).work) { btrfs_discard_schedule_work(discard_ctl, false); }
}

unsafe fn __btrfs_discard_schedule_work(discard_ctl: *mut btrfs_discard_ctl, now: u64, override_: bool) {
    if !btrfs_run_discard_work(discard_ctl) || (!override_ && delayed_work_pending(&mut (*discard_ctl).work)) { return; }
    let block_group = find_next_block_group(discard_ctl, now);
    if !block_group.is_null() {
        let mut delay = (*discard_ctl).delay_ms * NSEC_PER_MSEC;
        let kbps_limit = READ_ONCE!((*discard_ctl).kbps_limit);
        if kbps_limit != 0 && (*discard_ctl).prev_discard != 0 {
            let bps_limit = (kbps_limit as u64) * SZ_1K;
            let bps_delay = div64_u64((*discard_ctl).prev_discard * NSEC_PER_SEC, bps_limit);
            delay = max(delay, bps_delay);
        }
        if now < (*block_group).discard_eligible_time {
            delay = max(delay, (*block_group).discard_eligible_time - now);
        }
        if override_ && (*discard_ctl).prev_discard != 0 {
            let elapsed = now - (*discard_ctl).prev_discard_time;
            delay = if delay > elapsed { delay - elapsed } else { 0 };
        }
        mod_delayed_work((*discard_ctl).discard_workers, &mut (*discard_ctl).work, nsecs_to_jiffies(delay));
    }
}

pub unsafe fn btrfs_discard_schedule_work(discard_ctl: *mut btrfs_discard_ctl, override_: bool) {
    let now = ktime_get_ns();
    spin_lock(&mut (*discard_ctl).lock);
    __btrfs_discard_schedule_work(discard_ctl, now, override_);
    spin_unlock(&mut (*discard_ctl).lock);
}

unsafe fn btrfs_finish_discard_pass(discard_ctl: *mut btrfs_discard_ctl, block_group: *mut btrfs_block_group) {
    remove_from_discard_list(discard_ctl, block_group);
    if block_group_is_empty(block_group) {
        if btrfs_is_free_space_trimmed(block_group) { btrfs_mark_bg_unused(block_group); }
        else { add_to_discard_unused_list(discard_ctl, block_group); }
    } else { btrfs_update_discard_index(discard_ctl, block_group); }
}

unsafe fn btrfs_discard_workfn(work: *mut work_struct) {
    let discard_ctl = container_of!(work, btrfs_discard_ctl, work.work);
    let mut discard_state = BTRFS_DISCARD_RESET_CURSOR;
    let mut discard_index = 0;
    let mut trimmed = 0u64;
    let now = ktime_get_ns();
    let block_group = peek_discard_list(discard_ctl, &mut discard_state, &mut discard_index, now);
    if block_group.is_null() { return; }
    if !btrfs_run_discard_work(discard_ctl) {
        spin_lock(&mut (*discard_ctl).lock); btrfs_put_block_group(block_group); (*discard_ctl).block_group = core::ptr::null_mut(); spin_unlock(&mut (*discard_ctl).lock); return;
    }
    if now < (*block_group).discard_eligible_time {
        spin_lock(&mut (*discard_ctl).lock); btrfs_put_block_group(block_group); (*discard_ctl).block_group = core::ptr::null_mut(); spin_unlock(&mut (*discard_ctl).lock);
        btrfs_discard_schedule_work(discard_ctl, false); return;
    }
    let minlen = discard_minlen[discard_index as usize];
    match discard_state {
        BTRFS_DISCARD_BITMAPS => {
            let maxlen = if discard_index != BTRFS_DISCARD_INDEX_UNUSED { discard_minlen[(discard_index - 1) as usize] } else { 0 };
            btrfs_trim_block_group_bitmaps(block_group, &mut trimmed, (*block_group).discard_cursor, btrfs_block_group_end(block_group), minlen, maxlen, true);
            (*discard_ctl).discard_bitmap_bytes += trimmed;
        }
        BTRFS_DISCARD_FULLY_REMAPPED => btrfs_trim_fully_remapped_block_group(block_group),
        _ => {
            btrfs_trim_block_group_extents(block_group, &mut trimmed, (*block_group).discard_cursor, btrfs_block_group_end(block_group), minlen, true);
            (*discard_ctl).discard_extent_bytes += trimmed;
        }
    }
    if (*block_group).discard_cursor >= btrfs_block_group_end(block_group) {
        if discard_state == BTRFS_DISCARD_BITMAPS || discard_state == BTRFS_DISCARD_FULLY_REMAPPED { btrfs_finish_discard_pass(discard_ctl, block_group); }
        else {
            (*block_group).discard_cursor = (*block_group).start;
            spin_lock(&mut (*discard_ctl).lock);
            if (*block_group).discard_state != BTRFS_DISCARD_RESET_CURSOR { (*block_group).discard_state = BTRFS_DISCARD_BITMAPS; }
            spin_unlock(&mut (*discard_ctl).lock);
        }
    }
    let now = ktime_get_ns();
    spin_lock(&mut (*discard_ctl).lock);
    (*discard_ctl).prev_discard = trimmed; (*discard_ctl).prev_discard_time = now;
    btrfs_put_block_group(block_group); (*discard_ctl).block_group = core::ptr::null_mut();
    __btrfs_discard_schedule_work(discard_ctl, now, false);
    spin_unlock(&mut (*discard_ctl).lock);
}

pub unsafe fn btrfs_discard_calc_delay(discard_ctl: *mut btrfs_discard_ctl) {
    let discardable_extents = atomic_read(&(*discard_ctl).discardable_extents);
    if discardable_extents == 0 { return; }
    spin_lock(&mut (*discard_ctl).lock);
    if discardable_extents < 0 { atomic_add(-discardable_extents, &mut (*discard_ctl).discardable_extents); }
    let discardable_bytes = atomic64_read(&(*discard_ctl).discardable_bytes);
    if discardable_bytes < 0 { atomic64_add(-discardable_bytes, &mut (*discard_ctl).discardable_bytes); }
    if discardable_extents <= 0 { spin_unlock(&mut (*discard_ctl).lock); return; }
    let iops_limit = READ_ONCE!((*discard_ctl).iops_limit);
    let (mut min_delay, delay) = if iops_limit != 0 { (BTRFS_DISCARD_MIN_DELAY_MSEC, MSEC_PER_SEC / iops_limit as usize) } else { (0, 0) };
    let delay = clamp(delay, min_delay, BTRFS_DISCARD_MAX_DELAY_MSEC);
    (*discard_ctl).delay_ms = delay;
    spin_unlock(&mut (*discard_ctl).lock);
}

pub unsafe fn btrfs_discard_update_discardable(block_group: *mut btrfs_block_group) {
    if block_group.is_null() || !btrfs_test_opt((*block_group).fs_info, DISCARD_ASYNC) || !btrfs_is_block_group_data_only(block_group) { return; }
    let ctl = (*block_group).free_space_ctl;
    let discard_ctl = &mut (*(*block_group).fs_info).discard_ctl;
    lockdep_assert_held(&(*ctl).tree_lock);
    let extents_delta = (*ctl).discardable_extents[BTRFS_STAT_CURR] - (*ctl).discardable_extents[BTRFS_STAT_PREV];
    if extents_delta != 0 { atomic_add(extents_delta, &mut (*discard_ctl).discardable_extents); (*ctl).discardable_extents[BTRFS_STAT_PREV] = (*ctl).discardable_extents[BTRFS_STAT_CURR]; }
    let bytes_delta = (*ctl).discardable_bytes[BTRFS_STAT_CURR] - (*ctl).discardable_bytes[BTRFS_STAT_PREV];
    if bytes_delta != 0 { atomic64_add(bytes_delta, &mut (*discard_ctl).discardable_bytes); (*ctl).discardable_bytes[BTRFS_STAT_PREV] = (*ctl).discardable_bytes[BTRFS_STAT_CURR]; }
}

pub unsafe fn btrfs_discard_punt_unused_bgs_list(fs_info: *mut btrfs_fs_info) {
    spin_lock(&mut (*fs_info).unused_bgs_lock);
    list_for_each_entry_safe!((*fs_info).unused_bgs, block_group, next, bg_list, {
        list_del_init(&mut (*block_group).bg_list);
        btrfs_discard_queue_work(&mut (*fs_info).discard_ctl, block_group);
        btrfs_put_block_group(block_group);
    });
    spin_unlock(&mut (*fs_info).unused_bgs_lock);
}

unsafe fn btrfs_discard_purge_list(discard_ctl: *mut btrfs_discard_ctl) {
    spin_lock(&mut (*discard_ctl).lock);
    for i in 0..BTRFS_NR_DISCARD_LISTS {
        list_for_each_entry_safe!((*discard_ctl).discard_list[i], block_group, next, discard_list, {
            list_del_init(&mut (*block_group).discard_list);
            spin_unlock(&mut (*discard_ctl).lock);
            if (*block_group).used == 0 { btrfs_mark_bg_unused(block_group); }
            spin_lock(&mut (*discard_ctl).lock);
            btrfs_put_block_group(block_group);
        });
    }
    spin_unlock(&mut (*discard_ctl).lock);
}

pub unsafe fn btrfs_discard_resume(fs_info: *mut btrfs_fs_info) {
    if !btrfs_test_opt(fs_info, DISCARD_ASYNC) { btrfs_discard_cleanup(fs_info); return; }
    btrfs_discard_punt_unused_bgs_list(fs_info);
    set_bit(BTRFS_FS_DISCARD_RUNNING, &mut (*fs_info).flags);
}

pub unsafe fn btrfs_discard_stop(fs_info: *mut btrfs_fs_info) { clear_bit(BTRFS_FS_DISCARD_RUNNING, &mut (*fs_info).flags); }

pub unsafe fn btrfs_discard_init(fs_info: *mut btrfs_fs_info) {
    let discard_ctl = &mut (*fs_info).discard_ctl;
    spin_lock_init(&mut discard_ctl.lock);
    INIT_DELAYED_WORK!(&mut discard_ctl.work, btrfs_discard_workfn);
    for i in 0..BTRFS_NR_DISCARD_LISTS { INIT_LIST_HEAD(&mut discard_ctl.discard_list[i]); }
    discard_ctl.prev_discard = 0; discard_ctl.prev_discard_time = 0;
    atomic_set(&mut discard_ctl.discardable_extents, 0); atomic64_set(&mut discard_ctl.discardable_bytes, 0);
    discard_ctl.max_discard_size = BTRFS_ASYNC_DISCARD_DEFAULT_MAX_SIZE;
    discard_ctl.delay_ms = BTRFS_DISCARD_MAX_DELAY_MSEC;
    discard_ctl.iops_limit = BTRFS_DISCARD_MAX_IOPS; discard_ctl.kbps_limit = 0;
    discard_ctl.discard_extent_bytes = 0; discard_ctl.discard_bitmap_bytes = 0;
    atomic64_set(&mut discard_ctl.discard_bytes_saved, 0);
}

pub unsafe fn btrfs_discard_cleanup(fs_info: *mut btrfs_fs_info) {
    btrfs_discard_stop(fs_info);
    cancel_delayed_work_sync(&mut (*fs_info).discard_ctl.work);
    btrfs_discard_purge_list(&mut (*fs_info).discard_ctl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
