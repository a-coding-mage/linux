// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) Sistina Software, Inc. 1997-2003. All rights reserved.
// Copyright (C) 2004-2010 Red Hat, Inc. All rights reserved.
//
// C dependencies: trace/events/dlm.h, dlm_internal.h, lvb_table.h, memory.h,
// lock.h, user.h, and ast.h.

unsafe fn dlm_run_callback(
    ls_id: u32,
    lkb_id: u32,
    mode: i8,
    flags: u32,
    sb_flags: u8,
    sb_status: i32,
    lksb: *mut dlm_lksb,
    astfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    bastfn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i32)>,
    astparam: *mut core::ffi::c_void,
    res_name: *const core::ffi::c_char,
    res_length: usize,
) {
    if flags & DLM_CB_BAST != 0 {
        trace_dlm_bast(ls_id, lkb_id, mode, res_name, res_length);
        bastfn.unwrap()(astparam, mode as i32);
    } else if flags & DLM_CB_CAST != 0 {
        trace_dlm_ast(ls_id, lkb_id, sb_flags, sb_status, res_name, res_length);
        (*lksb).sb_status = sb_status;
        (*lksb).sb_flags = sb_flags;
        astfn.unwrap()(astparam);
    }
}

unsafe fn dlm_do_callback(cb: *mut dlm_callback) {
    dlm_run_callback((*cb).ls_id, (*cb).lkb_id, (*cb).mode, (*cb).flags,
                     (*cb).sb_flags, (*cb).sb_status, (*cb).lkb_lksb,
                     (*cb).astfn, (*cb).bastfn, (*cb).astparam,
                     (*cb).res_name.as_ptr(), (*cb).res_length);
    dlm_free_cb(cb);
}

unsafe extern "C" fn dlm_callback_work(work: *mut work_struct) {
    let cb = container_of!(work, dlm_callback, work);
    dlm_do_callback(cb);
}

pub unsafe fn dlm_may_skip_callback(
    lkb: *mut dlm_lkb, flags: u32, mode: i32, status: i32,
    _sbflags: u32, copy_lvb: *mut i32,
) -> bool {
    let rsb = (*lkb).lkb_resource;
    let ls = (*rsb).res_ls;
    let mut prev_mode: i32;

    if !copy_lvb.is_null() { *copy_lvb = 0; }

    if flags & DLM_CB_BAST != 0 {
        if (*lkb).lkb_last_cast_cb_mode != -1 &&
            dlm_modes_compat(mode, (*lkb).lkb_last_cast_cb_mode) {
            log_debug(ls, "skip %x bast mode %d for cast mode %d", (*lkb).lkb_id, mode,
                      (*lkb).lkb_last_cast_cb_mode);
            return true;
        }
        if (*lkb).lkb_last_cb_mode != -1 && (*lkb).lkb_last_cb_flags & DLM_CB_BAST != 0 {
            prev_mode = (*lkb).lkb_last_cb_mode;
            if prev_mode == mode || (prev_mode > mode && prev_mode > DLM_LOCK_PR) {
                log_debug(ls, "skip %x add bast mode %d for bast mode %d", (*lkb).lkb_id, mode, prev_mode);
                return true;
            }
        }
        (*lkb).lkb_last_bast_time = ktime_get();
        (*lkb).lkb_last_bast_cb_mode = mode;
    } else if flags & DLM_CB_CAST != 0 {
        if test_bit(DLM_DFL_USER_BIT, &(*lkb).lkb_dflags) {
            prev_mode = (*lkb).lkb_last_cast_cb_mode;
            if status == 0 && !(*lkb).lkb_lksb.is_null() && (*(*lkb).lkb_lksb).sb_lvbptr != core::ptr::null_mut() &&
               dlm_lvb_operations[(prev_mode + 1) as usize][(mode + 1) as usize] != 0 {
                if !copy_lvb.is_null() { *copy_lvb = 1; }
            }
        }
        (*lkb).lkb_last_cast_cb_mode = mode;
        (*lkb).lkb_last_cast_time = ktime_get();
    }
    (*lkb).lkb_last_cb_mode = mode;
    (*lkb).lkb_last_cb_flags = flags;
    false
}

pub unsafe fn dlm_get_cb(lkb: *mut dlm_lkb, flags: u32, mode: i32, status: i32,
                         sbflags: u32, cb: *mut *mut dlm_callback) -> i32 {
    let rsb = (*lkb).lkb_resource;
    let ls = (*rsb).res_ls;
    *cb = dlm_allocate_cb();
    if (*cb).is_null() { return -12; }
    (**cb).lkb_id = (*lkb).lkb_id;
    (**cb).ls_id = (*ls).ls_global_id;
    core::ptr::copy_nonoverlapping((*rsb).res_name.as_ptr(), (**cb).res_name.as_mut_ptr(), (*rsb).res_length);
    (**cb).res_length = (*rsb).res_length;
    (**cb).flags = flags;
    (**cb).mode = mode;
    (**cb).sb_status = status;
    (**cb).sb_flags = sbflags & 0xff;
    (**cb).lkb_lksb = (*lkb).lkb_lksb;
    0
}

unsafe fn dlm_get_queue_cb(lkb: *mut dlm_lkb, flags: u32, mode: i32, status: i32,
                           sbflags: u32, cb: *mut *mut dlm_callback) -> i32 {
    let rv = dlm_get_cb(lkb, flags, mode, status, sbflags, cb);
    if rv != 0 { return rv; }
    (**cb).astfn = (*lkb).lkb_astfn;
    (**cb).bastfn = (*lkb).lkb_bastfn;
    (**cb).astparam = (*lkb).lkb_astparam;
    INIT_WORK!(&mut (**cb).work, dlm_callback_work);
    0
}

pub unsafe fn dlm_add_cb(lkb: *mut dlm_lkb, flags: u32, mode: i32, status: i32, sbflags: u32) {
    let rsb = (*lkb).lkb_resource;
    let ls = (*rsb).res_ls;
    let mut cb: *mut dlm_callback = core::ptr::null_mut();
    if test_bit(DLM_DFL_USER_BIT, &(*lkb).lkb_dflags) { dlm_user_add_ast(lkb, flags, mode, status, sbflags); return; }
    if dlm_may_skip_callback(lkb, flags, mode, status, sbflags, core::ptr::null_mut()) { return; }
    spin_lock_bh(&mut (*ls).ls_cb_lock);
    if test_bit(LSFL_CB_DELAY, &(*ls).ls_flags) {
        if dlm_get_queue_cb(lkb, flags, mode, status, sbflags, &mut cb) == 0 { list_add(&mut (*cb).list, &mut (*ls).ls_cb_delay); }
    } else if test_bit(LSFL_SOFTIRQ, &(*ls).ls_flags) {
        dlm_run_callback((*ls).ls_global_id, (*lkb).lkb_id, mode as i8, flags, sbflags as u8, status, (*lkb).lkb_lksb, (*lkb).lkb_astfn, (*lkb).lkb_bastfn, (*lkb).lkb_astparam, (*rsb).res_name.as_ptr(), (*rsb).res_length);
    } else if dlm_get_queue_cb(lkb, flags, mode, status, sbflags, &mut cb) == 0 { queue_work((*ls).ls_callback_wq, &mut (*cb).work); }
    spin_unlock_bh(&mut (*ls).ls_cb_lock);
}

pub unsafe fn dlm_callback_start(ls: *mut dlm_ls) -> i32 {
    if !test_bit(LSFL_FS, &(*ls).ls_flags) || test_bit(LSFL_SOFTIRQ, &(*ls).ls_flags) { return 0; }
    (*ls).ls_callback_wq = alloc_ordered_workqueue(c"dlm_callback", WQ_HIGHPRI | WQ_MEM_RECLAIM);
    if (*ls).ls_callback_wq.is_null() { log_print!("can't start dlm_callback workqueue"); return -12; }
    0
}

pub unsafe fn dlm_callback_stop(ls: *mut dlm_ls) { if !(*ls).ls_callback_wq.is_null() { destroy_workqueue((*ls).ls_callback_wq); } }

pub unsafe fn dlm_callback_suspend(ls: *mut dlm_ls) {
    if !test_bit(LSFL_FS, &(*ls).ls_flags) { return; }
    spin_lock_bh(&mut (*ls).ls_cb_lock); set_bit(LSFL_CB_DELAY, &mut (*ls).ls_flags); spin_unlock_bh(&mut (*ls).ls_cb_lock);
    if !(*ls).ls_callback_wq.is_null() { flush_workqueue((*ls).ls_callback_wq); }
}

const MAX_CB_QUEUE: i32 = 25;

pub unsafe fn dlm_callback_resume(ls: *mut dlm_ls) {
    let mut count = 0; let mut sum = 0;
    if !test_bit(LSFL_FS, &(*ls).ls_flags) { return; }
    'more: loop {
        spin_lock_bh(&mut (*ls).ls_cb_lock);
        let mut cb: *mut dlm_callback = core::ptr::null_mut();
        while !list_empty(&(*ls).ls_cb_delay) {
            list_for_each_entry_safe!(&mut cb, dlm_callback, &mut (*ls).ls_cb_delay, list);
            list_del(&mut (*cb).list);
            if test_bit(LSFL_SOFTIRQ, &(*ls).ls_flags) { dlm_do_callback(cb); } else { queue_work((*ls).ls_callback_wq, &mut (*cb).work); }
            count += 1; if count == MAX_CB_QUEUE { break; }
        }
        let empty = list_empty(&(*ls).ls_cb_delay);
        if empty { clear_bit(LSFL_CB_DELAY, &mut (*ls).ls_flags); }
        spin_unlock_bh(&mut (*ls).ls_cb_lock);
        sum += count;
        if !empty { count = 0; cond_resched(); continue 'more; }
        break;
    }
    if sum != 0 { log_rinfo(ls, c"%s %d", __func__, sum); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
