// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Oracle.  All rights reserved.
 * Copyright (C) 2014 Fujitsu.  All rights reserved.
 */

// External kernel, Btrfs, and trace-event types and functions are supplied by
// the surrounding translation unit.

pub const WORK_DONE_BIT: usize = 0;
pub const WORK_ORDER_DONE_BIT: usize = 1;
pub const NO_THRESHOLD: i32 = -1;
pub const DEFAULT_THRESHOLD: i32 = 32;

#[repr(C)]
pub struct btrfs_workqueue {
    pub normal_wq: *mut workqueue_struct,
    pub fs_info: *mut btrfs_fs_info,
    pub ordered_list: list_head,
    pub list_lock: spinlock_t,
    pub pending: atomic_t,
    pub limit_active: i32,
    pub current_active: i32,
    pub thresh: i32,
    pub count: u32,
    pub thres_lock: spinlock_t,
}

extern "C" {
    pub type workqueue_struct;
    pub type btrfs_fs_info;
    pub type list_head;
    pub type spinlock_t;
    pub type atomic_t;
    pub type work_struct;
    pub type btrfs_work;
    pub type btrfs_func_t;
    pub type btrfs_ordered_func_t;
}

pub unsafe fn btrfs_workqueue_owner(wq: *const btrfs_workqueue) -> *mut btrfs_fs_info {
    (*wq).fs_info
}

pub unsafe fn btrfs_work_owner(work: *const btrfs_work) -> *mut btrfs_fs_info {
    (*(*work).wq).fs_info
}

pub unsafe fn btrfs_workqueue_normal_congested(wq: *const btrfs_workqueue) -> bool {
    if (*wq).thresh == NO_THRESHOLD {
        return false;
    }
    atomic_read(&(*wq).pending) > (*wq).thresh * 2
}

unsafe fn btrfs_init_workqueue(wq: *mut btrfs_workqueue, fs_info: *mut btrfs_fs_info) {
    (*wq).fs_info = fs_info;
    atomic_set(&mut (*wq).pending, 0);
    INIT_LIST_HEAD(&mut (*wq).ordered_list);
    spin_lock_init(&mut (*wq).list_lock);
    spin_lock_init(&mut (*wq).thres_lock);
}

pub unsafe fn btrfs_alloc_workqueue(
    fs_info: *mut btrfs_fs_info, name: *const i8, flags: u32,
    limit_active: i32, mut thresh: i32,
) -> *mut btrfs_workqueue {
    let ret = kzalloc_btrfs_workqueue();
    if ret.is_null() { return core::ptr::null_mut(); }
    btrfs_init_workqueue(ret, fs_info);
    (*ret).limit_active = limit_active;
    if thresh == 0 { thresh = DEFAULT_THRESHOLD; }
    if thresh < DEFAULT_THRESHOLD {
        (*ret).current_active = limit_active;
        (*ret).thresh = NO_THRESHOLD;
    } else {
        (*ret).current_active = 1;
        (*ret).thresh = thresh;
    }
    (*ret).normal_wq = alloc_workqueue(b"btrfs-%s\0".as_ptr() as *const i8, flags,
                                       (*ret).current_active, name);
    if (*ret).normal_wq.is_null() { kfree(ret as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    trace_btrfs_workqueue_alloc(ret, name);
    ret
}

pub unsafe fn btrfs_alloc_ordered_workqueue(
    fs_info: *mut btrfs_fs_info, name: *const i8, flags: u32,
) -> *mut btrfs_workqueue {
    let ret = kzalloc_btrfs_workqueue();
    if ret.is_null() { return core::ptr::null_mut(); }
    btrfs_init_workqueue(ret, fs_info);
    (*ret).limit_active = 1; (*ret).current_active = 1; (*ret).thresh = NO_THRESHOLD;
    (*ret).normal_wq = alloc_ordered_workqueue(b"btrfs-%s\0".as_ptr() as *const i8, flags, name);
    if (*ret).normal_wq.is_null() { kfree(ret as *mut core::ffi::c_void); return core::ptr::null_mut(); }
    trace_btrfs_workqueue_alloc(ret, name);
    ret
}

unsafe fn thresh_queue_hook(wq: *mut btrfs_workqueue) {
    if (*wq).thresh != NO_THRESHOLD { atomic_inc(&mut (*wq).pending); }
}

unsafe fn thresh_exec_hook(wq: *mut btrfs_workqueue) {
    if (*wq).thresh == NO_THRESHOLD { return; }
    atomic_dec(&mut (*wq).pending);
    spin_lock(&mut (*wq).thres_lock);
    (*wq).count += 1; (*wq).count %= ((*wq).thresh / 4) as u32;
    if (*wq).count != 0 {
        let pending = atomic_read(&(*wq).pending);
        let mut current = (*wq).current_active;
        if pending > (*wq).thresh { current += 1; }
        if pending < (*wq).thresh / 2 { current -= 1; }
        current = current.clamp(1, (*wq).limit_active);
        if current != (*wq).current_active {
            (*wq).current_active = current;
            spin_unlock(&mut (*wq).thres_lock);
            workqueue_set_max_active((*wq).normal_wq, current);
            return;
        }
    }
    spin_unlock(&mut (*wq).thres_lock);
}

unsafe fn run_ordered_work(wq: *mut btrfs_workqueue, self_work: *mut btrfs_work) {
    let list = &mut (*wq).ordered_list;
    let mut free_self = false;
    loop {
        let mut flags = 0u64;
        spin_lock_irqsave(&mut (*wq).list_lock, &mut flags);
        if list_empty(list) { spin_unlock_irqrestore(&mut (*wq).list_lock, flags); break; }
        let work = list_first_entry(list);
        if !test_bit(WORK_DONE_BIT, &(*work).flags) {
            spin_unlock_irqrestore(&mut (*wq).list_lock, flags); break;
        }
        smp_rmb();
        if test_and_set_bit(WORK_ORDER_DONE_BIT, &mut (*work).flags) {
            spin_unlock_irqrestore(&mut (*wq).list_lock, flags); break;
        }
        trace_btrfs_ordered_sched(work);
        spin_unlock_irqrestore(&mut (*wq).list_lock, flags);
        ((*work).ordered_func)(work, false);
        spin_lock_irqsave(&mut (*wq).list_lock, &mut flags);
        list_del(&mut (*work).ordered_list);
        spin_unlock_irqrestore(&mut (*wq).list_lock, flags);
        if work == self_work {
            free_self = true;
        } else {
            ((*work).ordered_func)(work, true);
            trace_btrfs_all_work_done((*wq).fs_info, work);
        }
    }
    if free_self {
        ((*self_work).ordered_func)(self_work, true);
        trace_btrfs_all_work_done((*wq).fs_info, self_work);
    }
}

unsafe extern "C" fn btrfs_work_helper(normal_work: *mut work_struct) {
    let work = container_of_work(normal_work);
    let wq = (*work).wq;
    let need_order = !(*work).ordered_func.is_none();
    trace_btrfs_work_sched(work);
    thresh_exec_hook(wq);
    ((*work).func)(work);
    if need_order {
        smp_mb__before_atomic();
        set_bit(WORK_DONE_BIT, &mut (*work).flags);
        run_ordered_work(wq, work);
    } else {
        trace_btrfs_all_work_done((*wq).fs_info, work);
    }
}

// The remaining workqueue operations mirror the C implementation and depend
// on the corresponding kernel/Btrfs declarations supplied by other files.
pub unsafe fn btrfs_init_work(work: *mut btrfs_work, func: btrfs_func_t, ordered_func: btrfs_ordered_func_t) {
    (*work).func = func; (*work).ordered_func = ordered_func;
    INIT_WORK(&mut (*work).normal_work, btrfs_work_helper);
    INIT_LIST_HEAD(&mut (*work).ordered_list);
    (*work).flags = 0;
}

pub unsafe fn btrfs_queue_work(wq: *mut btrfs_workqueue, work: *mut btrfs_work) {
    (*work).wq = wq;
    thresh_queue_hook(wq);
    if !(*work).ordered_func.is_none() {
        let mut flags = 0u64;
        spin_lock_irqsave(&mut (*wq).list_lock, &mut flags);
        list_add_tail(&mut (*work).ordered_list, &mut (*wq).ordered_list);
        spin_unlock_irqrestore(&mut (*wq).list_lock, flags);
    }
    trace_btrfs_work_queued(work);
    queue_work((*wq).normal_wq, &mut (*work).normal_work);
}

pub unsafe fn btrfs_destroy_workqueue(wq: *mut btrfs_workqueue) {
    if wq.is_null() { return; }
    destroy_workqueue((*wq).normal_wq); trace_btrfs_workqueue_destroy(wq);
    kfree(wq as *mut core::ffi::c_void);
}

pub unsafe fn btrfs_workqueue_set_max(wq: *mut btrfs_workqueue, limit_active: i32) {
    if !wq.is_null() { (*wq).limit_active = limit_active; }
}

pub unsafe fn btrfs_flush_workqueue(wq: *mut btrfs_workqueue) {
    flush_workqueue((*wq).normal_wq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
