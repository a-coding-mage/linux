// SPDX-License-Identifier: GPL-2.0-only
/*
 * Process number limiting controller for cgroups.
 *
 * Rust translation of the C implementation. Kernel-provided types and
 * functions referenced below are supplied by other translation units.
 */

const PIDS_MAX: u64 = PID_MAX_LIMIT as u64 + 1;
const PIDS_MAX_STR: &str = "max";

#[repr(C)]
#[derive(Copy, Clone)]
enum PidcgEvent { PIDCG_MAX, PIDCG_FORKFAIL, NR_PIDCG_EVENTS }

#[repr(C)]
struct PidsCgroup {
    css: cgroup_subsys_state,
    counter: atomic64_t,
    limit: atomic64_t,
    watermark: i64,
    events_file: cgroup_file,
    events_local_file: cgroup_file,
    events: [atomic64_t; PidcgEvent::NR_PIDCG_EVENTS as usize],
    events_local: [atomic64_t; PidcgEvent::NR_PIDCG_EVENTS as usize],
}

unsafe fn css_pids(css: *mut cgroup_subsys_state) -> *mut PidsCgroup {
    container_of!(css, PidsCgroup, css)
}

unsafe fn parent_pids(pids: *mut PidsCgroup) -> *mut PidsCgroup {
    css_pids((*pids).css.parent)
}

unsafe fn pids_css_alloc(parent: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    let pids = kzalloc_obj!(PidsCgroup);
    if pids.is_null() { return ERR_PTR!(-ENOMEM); }
    atomic64_set(&mut (*pids).limit, PIDS_MAX as i64);
    &mut (*pids).css
}

unsafe fn pids_css_free(css: *mut cgroup_subsys_state) { kfree(css_pids(css)); }

unsafe fn pids_update_watermark(p: *mut PidsCgroup, nr_pids: i64) {
    /* This is racy, but perfectly accurate tallying is unnecessary here. */
    if nr_pids > READ_ONCE!((*p).watermark) { WRITE_ONCE!((*p).watermark, nr_pids); }
}

unsafe fn pids_cancel(pids: *mut PidsCgroup, num: i32) {
    WARN_ON_ONCE!(atomic64_add_negative(-(num as i64), &mut (*pids).counter));
}

unsafe fn pids_uncharge(pids: *mut PidsCgroup, num: i32) {
    let mut p = pids;
    while !parent_pids(p).is_null() { pids_cancel(p, num); p = parent_pids(p); }
}

unsafe fn pids_charge(pids: *mut PidsCgroup, num: i32) {
    let mut p = pids;
    while !parent_pids(p).is_null() {
        let new = atomic64_add_return(num as i64, &mut (*p).counter);
        pids_update_watermark(p, new);
        p = parent_pids(p);
    }
}

unsafe fn pids_try_charge(pids: *mut PidsCgroup, num: i32, fail: *mut *mut PidsCgroup) -> i32 {
    let mut p = pids;
    while !parent_pids(p).is_null() {
        let new = atomic64_add_return(num as i64, &mut (*p).counter);
        let limit = atomic64_read(&(*p).limit);
        if new > limit { *fail = p; break; }
        pids_update_watermark(p, new);
        p = parent_pids(p);
    }
    if p == pids || !parent_pids(p).is_null() && atomic64_read(&(*p).limit) < atomic64_read(&(*p).counter) {
        let mut q = pids;
        while q != p { pids_cancel(q, num); q = parent_pids(q); }
        pids_cancel(p, num);
        return -EAGAIN;
    }
    0
}

unsafe fn pids_can_attach(tset: *mut cgroup_taskset) -> i32 {
    cgroup_taskset_for_each!(task, dst_css, tset, {
        let pids = css_pids(dst_css);
        let old_pids = css_pids(task_css(task, pids_cgrp_id));
        pids_charge(pids, 1); pids_uncharge(old_pids, 1);
    });
    0
}

unsafe fn pids_cancel_attach(tset: *mut cgroup_taskset) {
    cgroup_taskset_for_each!(task, dst_css, tset, {
        let pids = css_pids(dst_css);
        let old_pids = css_pids(task_css(task, pids_cgrp_id));
        pids_charge(old_pids, 1); pids_uncharge(pids, 1);
    });
}

unsafe fn pids_event(pids_forking: *mut PidsCgroup, pids_over_limit: *mut PidsCgroup) {
    let p = pids_forking;
    if atomic64_inc_return(&mut (*p).events_local[PidcgEvent::PIDCG_FORKFAIL as usize]) == 1 {
        pr_info!("cgroup: fork rejected by pids controller in ");
        pr_cont_cgroup_path!((*p).css.cgroup); pr_cont!("\n");
    }
    if !cgroup_subsys_on_dfl!(pids_cgrp_subsys) || cgrp_dfl_root.flags & CGRP_ROOT_PIDS_LOCAL_EVENTS != 0 {
        cgroup_file_notify(&mut (*p).events_local_file); return;
    }
    atomic64_inc(&mut (*pids_over_limit).events_local[PidcgEvent::PIDCG_MAX as usize]);
    cgroup_file_notify(&mut (*pids_over_limit).events_local_file);
    let mut q = pids_over_limit;
    while !parent_pids(q).is_null() { atomic64_inc(&mut (*q).events[PidcgEvent::PIDCG_MAX as usize]); cgroup_file_notify(&mut (*q).events_file); q = parent_pids(q); }
}

unsafe fn pids_can_fork(_task: *mut task_struct, cset: *mut css_set) -> i32 {
    let pids = css_pids((*cset).subsys[pids_cgrp_id as usize]);
    let mut over = core::ptr::null_mut();
    let err = pids_try_charge(pids, 1, &mut over);
    if err != 0 { pids_event(pids, over); } err
}
unsafe fn pids_cancel_fork(_task: *mut task_struct, cset: *mut css_set) { pids_uncharge(css_pids((*cset).subsys[pids_cgrp_id as usize]), 1); }
unsafe fn pids_release(task: *mut task_struct) { pids_uncharge(css_pids(task_css(task, pids_cgrp_id)), 1); }

unsafe fn pids_max_write(of: *mut kernfs_open_file, buf: *mut i8, nbytes: usize, _off: loff_t) -> ssize_t {
    let pids = css_pids((*of).css);
    let s = strstrip(buf);
    let mut limit: i64;
    if strcmp(s, PIDS_MAX_STR.as_ptr() as *const i8) == 0 { limit = PIDS_MAX as i64; }
    else { let err = kstrtoll(s, 0, &mut limit); if err != 0 { return err as ssize_t; } if limit < 0 || limit >= PIDS_MAX as i64 { return -EINVAL as ssize_t; } }
    atomic64_set(&mut (*pids).limit, limit); nbytes as ssize_t
}
unsafe fn pids_max_show(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let limit = atomic64_read(&(*css_pids(seq_css(sf))).limit);
    if limit >= PIDS_MAX as i64 { seq_printf(sf, "%s\n", PIDS_MAX_STR.as_ptr()); }
    else { seq_printf(sf, "%lld\n", limit); } 0
}
unsafe fn pids_current_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> s64 { atomic64_read(&(*css_pids(css)).counter) }
unsafe fn pids_peak_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> s64 { READ_ONCE!((*css_pids(css)).watermark) }
unsafe fn __pids_events_show(sf: *mut seq_file, mut local: bool) -> i32 {
    let pids = css_pids(seq_css(sf)); let mut pe = PidcgEvent::PIDCG_MAX;
    if !cgroup_subsys_on_dfl!(pids_cgrp_subsys) || cgrp_dfl_root.flags & CGRP_ROOT_PIDS_LOCAL_EVENTS != 0 { pe = PidcgEvent::PIDCG_FORKFAIL; local = true; }
    let events = if local { (*pids).events_local.as_mut_ptr() } else { (*pids).events.as_mut_ptr() };
    seq_printf(sf, "max %lld\n", atomic64_read(&*events.add(pe as usize))); 0
}
unsafe fn pids_events_show(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { __pids_events_show(sf, false); 0 }
unsafe fn pids_events_local_show(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { __pids_events_show(sf, true); 0 }

static mut PIDS_FILES: [cftype; 6] = [
    cftype { name: "max", write: Some(pids_max_write), seq_show: Some(pids_max_show), flags: CFTYPE_NOT_ON_ROOT, ..cftype::ZERO },
    cftype { name: "current", read_s64: Some(pids_current_read), flags: CFTYPE_NOT_ON_ROOT, ..cftype::ZERO },
    cftype { name: "peak", read_s64: Some(pids_peak_read), flags: CFTYPE_NOT_ON_ROOT, ..cftype::ZERO },
    cftype { name: "events", seq_show: Some(pids_events_show), file_offset: offset_of!(PidsCgroup, events_file), flags: CFTYPE_NOT_ON_ROOT, ..cftype::ZERO },
    cftype { name: "events.local", seq_show: Some(pids_events_local_show), file_offset: offset_of!(PidsCgroup, events_local_file), flags: CFTYPE_NOT_ON_ROOT, ..cftype::ZERO },
    cftype::ZERO,
];
static mut PIDS_FILES_LEGACY: [cftype; 5] = [PIDS_FILES[0], PIDS_FILES[1], PIDS_FILES[2], PIDS_FILES[3], cftype::ZERO];

// The registration object retains the C ABI/data layout.
extern "C" {
    static mut pids_cgrp_subsys: cgroup_subsys;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
