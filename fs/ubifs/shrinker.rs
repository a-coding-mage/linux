// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2006-2008 Nokia Corporation.
 *
 * Authors: Artem Bityutskiy (Битюцкий Артём)
 *          Adrian Hunter
 */

/* Rust translation of the UBIFS TNC shrinker implementation. */

/* List of all UBIFS file-system instances */
pub static mut UBIFS_INFOS: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

/* Protects `ubifs_infos` list */
pub static mut UBIFS_INFOS_LOCK: spinlock_t = spinlock_t { _private: 0 };

/* Global clean znode counter (for all mounted UBIFS instances) */
pub static mut UBIFS_CLEAN_ZN_CNT: atomic_long_t = atomic_long_t { counter: 0 };

static mut SHRINKER_RUN_NO: u32 = 0;

unsafe fn shrink_tnc(c: *mut ubifs_info, nr: i32, age: i32, contention: *mut i32) -> i32 {
    let mut total_freed: i32 = 0;
    let mut zprev: *mut ubifs_znode = core::ptr::null_mut();
    let mut znode: *mut ubifs_znode;
    let time: time64_t = ktime_get_seconds();

    ubifs_assert(c, mutex_is_locked(&mut (*c).umount_mutex));
    ubifs_assert(c, mutex_is_locked(&mut (*c).tnc_mutex));

    if (*c).zroot.znode.is_null() || atomic_long_read(&(*c).clean_zn_cnt) == 0 {
        return 0;
    }

    znode = ubifs_tnc_levelorder_next(c, (*c).zroot.znode, core::ptr::null_mut());
    while !znode.is_null() && total_freed < nr && atomic_long_read(&(*c).clean_zn_cnt) > 0 {
        let mut freed: i32;

        if !(*znode).cnext.is_null() {
            *contention = 1;
        } else if !ubifs_zn_dirty(znode) && (time - (*znode).time).abs() >= age as i64 {
            if !(*znode).parent.is_null() {
                (*(*znode).parent).zbranch[(*znode).iip as usize].znode = core::ptr::null_mut();
            } else {
                (*c).zroot.znode = core::ptr::null_mut();
            }

            freed = ubifs_destroy_tnc_subtree(c, znode);
            atomic_long_sub(freed as i64, &mut UBIFS_CLEAN_ZN_CNT);
            atomic_long_sub(freed as i64, &mut (*c).clean_zn_cnt);
            total_freed += freed;
            znode = zprev;
        }

        if (*c).zroot.znode.is_null() {
            break;
        }

        zprev = znode;
        znode = ubifs_tnc_levelorder_next(c, (*c).zroot.znode, znode);
        cond_resched();
    }

    total_freed
}

unsafe fn shrink_tnc_trees(nr: i32, age: i32, contention: *mut i32) -> i32 {
    let mut c: *mut ubifs_info;
    let mut p: *mut list_head;
    let mut run_no: u32;
    let mut freed: i32 = 0;

    spin_lock(&mut UBIFS_INFOS_LOCK);
    loop {
        SHRINKER_RUN_NO = SHRINKER_RUN_NO.wrapping_add(1);
        run_no = SHRINKER_RUN_NO;
        if run_no != 0 { break; }
    }
    p = UBIFS_INFOS.next;
    while p != &mut UBIFS_INFOS as *mut list_head {
        c = list_entry(p, ubifs_info, infos_list);
        if (*c).shrinker_run_no == run_no { break; }
        if !mutex_trylock(&mut (*c).umount_mutex) {
            *contention = 1;
            p = (*p).next;
            continue;
        }
        if !mutex_trylock(&mut (*c).tnc_mutex) {
            mutex_unlock(&mut (*c).umount_mutex);
            *contention = 1;
            p = (*p).next;
            continue;
        }
        spin_unlock(&mut UBIFS_INFOS_LOCK);
        (*c).shrinker_run_no = run_no;
        freed += shrink_tnc(c, nr, age, contention);
        mutex_unlock(&mut (*c).tnc_mutex);
        spin_lock(&mut UBIFS_INFOS_LOCK);
        p = (*p).next;
        list_move_tail(&mut (*c).infos_list, &mut UBIFS_INFOS);
        mutex_unlock(&mut (*c).umount_mutex);
        if freed >= nr { break; }
    }
    spin_unlock(&mut UBIFS_INFOS_LOCK);
    freed
}

unsafe fn kick_a_thread() -> i32 {
    let mut c: *mut ubifs_info;
    spin_lock(&mut UBIFS_INFOS_LOCK);
    for i in 0..2 {
        list_for_each_entry!(c, &mut UBIFS_INFOS, infos_list) {
            let dirty_zn_cnt: i64;
            if !mutex_trylock(&mut (*c).umount_mutex) {
                spin_unlock(&mut UBIFS_INFOS_LOCK);
                return -1;
            }
            dirty_zn_cnt = atomic_long_read(&(*c).dirty_zn_cnt);
            if dirty_zn_cnt == 0 || (*c).cmt_state == COMMIT_BROKEN || (*c).ro_mount || (*c).ro_error {
                mutex_unlock(&mut (*c).umount_mutex);
                continue;
            }
            if (*c).cmt_state != COMMIT_RESTING {
                spin_unlock(&mut UBIFS_INFOS_LOCK);
                mutex_unlock(&mut (*c).umount_mutex);
                return -1;
            }
            if i == 1 {
                list_move_tail(&mut (*c).infos_list, &mut UBIFS_INFOS);
                spin_unlock(&mut UBIFS_INFOS_LOCK);
                ubifs_request_bg_commit(c);
                mutex_unlock(&mut (*c).umount_mutex);
                return -1;
            }
            mutex_unlock(&mut (*c).umount_mutex);
        }
    }
    spin_unlock(&mut UBIFS_INFOS_LOCK);
    0
}

pub unsafe fn ubifs_shrink_count(_shrink: *mut shrinker, _sc: *mut shrink_control) -> usize {
    let clean_zn_cnt = atomic_long_read(&UBIFS_CLEAN_ZN_CNT);
    if clean_zn_cnt >= 0 { clean_zn_cnt as usize } else { 1 }
}

pub unsafe fn ubifs_shrink_scan(_shrink: *mut shrinker, sc: *mut shrink_control) -> usize {
    let nr = (*sc).nr_to_scan;
    let mut contention: i32 = 0;
    let mut freed: i32;
    let clean_zn_cnt = atomic_long_read(&UBIFS_CLEAN_ZN_CNT);

    if clean_zn_cnt == 0 {
        dbg_tnc!("no clean znodes, kick a thread");
        return kick_a_thread() as usize;
    }
    freed = shrink_tnc_trees(nr as i32, OLD_ZNODE_AGE, &mut contention);
    if freed >= nr as i32 { return freed as usize; }
    dbg_tnc!("not enough old znodes, try to free young ones");
    freed += shrink_tnc_trees(nr as i32 - freed, YOUNG_ZNODE_AGE, &mut contention);
    if freed >= nr as i32 { return freed as usize; }
    dbg_tnc!("not enough young znodes, free all");
    freed += shrink_tnc_trees(nr as i32 - freed, 0, &mut contention);
    if freed == 0 && contention != 0 {
        dbg_tnc!("freed nothing, but contention");
        return SHRINK_STOP as usize;
    }
    dbg_tnc!("%lu znodes were freed, requested %lu", freed, nr);
    freed as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
