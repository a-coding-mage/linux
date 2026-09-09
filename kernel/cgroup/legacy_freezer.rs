// SPDX-License-Identifier: LGPL-2.1
/*
 * cgroup_freezer.c -  control group freezer subsystem
 *
 * Copyright IBM Corporation, 2007
 *
 * Author : Cedric Le Goater <clg@fr.ibm.com>
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
struct freezer {
    css: cgroup_subsys_state,
    state: c_uint,
}

#[repr(C)]
struct cgroup_subsys_state {
    parent: *mut cgroup_subsys_state,
}

struct task_struct;
struct cgroup_taskset;
struct css_task_iter;
struct seq_file;
struct kernfs_open_file;
struct cftype;
struct cgroup_subsys;

type c_uint = u32;
type c_int = i32;
type c_ulong = u64;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

const CGROUP_FREEZER_ONLINE: c_uint = 1 << 0;
const CGROUP_FREEZING_SELF: c_uint = 1 << 1;
const CGROUP_FREEZING_PARENT: c_uint = 1 << 2;
const CGROUP_FROZEN: c_uint = 1 << 3;
const CGROUP_FREEZING: c_uint = CGROUP_FREEZING_SELF | CGROUP_FREEZING_PARENT;

static mut freezer_mutex: mutex = mutex {};

#[repr(C)]
struct mutex;

unsafe fn css_freezer(css: *mut cgroup_subsys_state) -> *mut freezer {
    if !css.is_null() {
        // Equivalent to container_of(css, struct freezer, css).
        css as *mut freezer
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn task_freezer(task: *mut task_struct) -> *mut freezer {
    css_freezer(task_css(task, freezer_cgrp_id))
}

unsafe fn parent_freezer(freezer: *mut freezer) -> *mut freezer {
    css_freezer((*freezer).css.parent)
}

unsafe fn cgroup1_freezing(task: *mut task_struct) -> bool {
    rcu_read_lock();
    let ret = (*task_freezer(task)).state & CGROUP_FREEZING != 0;
    rcu_read_unlock();
    ret
}

unsafe fn freezer_state_strs(state: c_uint) -> &'static str {
    if state & CGROUP_FROZEN != 0 { "FROZEN" }
    else if state & CGROUP_FREEZING != 0 { "FREEZING" }
    else { "THAWED" }
}

unsafe fn freezer_css_alloc(_parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    let freezer = kzalloc_obj::<freezer>();
    if freezer.is_null() { err_ptr(-12) } else { &mut (*freezer).css }
}

unsafe fn freezer_css_online(css: *mut cgroup_subsys_state) -> c_int {
    let freezer = css_freezer(css);
    let parent = parent_freezer(freezer);
    cpus_read_lock(); mutex_lock(&mut freezer_mutex);
    (*freezer).state |= CGROUP_FREEZER_ONLINE;
    if !parent.is_null() && (*parent).state & CGROUP_FREEZING != 0 {
        (*freezer).state |= CGROUP_FREEZING_PARENT | CGROUP_FROZEN;
        static_branch_inc_cpuslocked(&mut freezer_active);
    }
    mutex_unlock(&mut freezer_mutex); cpus_read_unlock(); 0
}

unsafe fn freezer_css_offline(css: *mut cgroup_subsys_state) {
    let freezer = css_freezer(css);
    cpus_read_lock(); mutex_lock(&mut freezer_mutex);
    if (*freezer).state & CGROUP_FREEZING != 0 { static_branch_dec_cpuslocked(&mut freezer_active); }
    (*freezer).state = 0;
    mutex_unlock(&mut freezer_mutex); cpus_read_unlock();
}

unsafe fn freezer_css_free(css: *mut cgroup_subsys_state) { kfree(css_freezer(css)); }

unsafe fn freezer_attach(tset: *mut cgroup_taskset) {
    let mut task: *mut task_struct = core::ptr::null_mut();
    let mut new_css: *mut cgroup_subsys_state = core::ptr::null_mut();
    mutex_lock(&mut freezer_mutex);
    cgroup_taskset_for_each!(task, new_css, tset, {
        let mut freezer = css_freezer(new_css);
        if (*freezer).state & CGROUP_FREEZING == 0 { __thaw_task(task); }
        else {
            while !freezer.is_null() && (*freezer).state & CGROUP_FROZEN != 0 {
                (*freezer).state &= !CGROUP_FROZEN;
                freezer = parent_freezer(freezer);
            }
            freeze_task(task);
        }
    });
    mutex_unlock(&mut freezer_mutex);
}

unsafe fn freezer_fork(task: *mut task_struct) {
    if task_css_is_root(task, freezer_cgrp_id) { return; }
    mutex_lock(&mut freezer_mutex); rcu_read_lock();
    let freezer = task_freezer(task);
    if (*freezer).state & CGROUP_FREEZING != 0 { freeze_task(task); }
    rcu_read_unlock(); mutex_unlock(&mut freezer_mutex);
}

unsafe fn update_if_frozen(css: *mut cgroup_subsys_state) {
    let freezer = css_freezer(css);
    let mut pos: *mut cgroup_subsys_state = core::ptr::null_mut();
    let mut it = core::mem::MaybeUninit::<css_task_iter>::uninit();
    let mut task: *mut task_struct;
    lockdep_assert_held(&freezer_mutex);
    if (*freezer).state & CGROUP_FREEZING == 0 || (*freezer).state & CGROUP_FROZEN != 0 { return; }
    rcu_read_lock();
    css_for_each_child!(pos, css, {
        let child = css_freezer(pos);
        if (*child).state & CGROUP_FREEZER_ONLINE != 0 && (*child).state & CGROUP_FROZEN == 0 { rcu_read_unlock(); return; }
    });
    rcu_read_unlock(); css_task_iter_start(css, 0, it.as_mut_ptr());
    while { task = css_task_iter_next(it.as_mut_ptr()); !task.is_null() } {
        if freezing(task) && !frozen(task) { break; }
    }
    if task.is_null() { (*freezer).state |= CGROUP_FROZEN; }
    css_task_iter_end(it.as_mut_ptr());
}

unsafe fn freezer_read(m: *mut seq_file, _v: *mut core::ffi::c_void) -> c_int {
    let css = seq_css(m); let mut pos: *mut cgroup_subsys_state = core::ptr::null_mut();
    mutex_lock(&mut freezer_mutex); rcu_read_lock();
    css_for_each_descendant_post!(pos, css, { if css_tryget_online(pos) { rcu_read_unlock(); update_if_frozen(pos); rcu_read_lock(); css_put(pos); } });
    rcu_read_unlock(); mutex_unlock(&mut freezer_mutex);
    seq_puts(m, freezer_state_strs((*css_freezer(css)).state)); seq_putc(m, b'\n' as c_int); 0
}

unsafe fn freeze_cgroup(freezer: *mut freezer) { let mut it = core::mem::MaybeUninit::<css_task_iter>::uninit(); css_task_iter_start(&mut (*freezer).css, 0, it.as_mut_ptr()); loop { let t = css_task_iter_next(it.as_mut_ptr()); if t.is_null() { break } freeze_task(t); } css_task_iter_end(it.as_mut_ptr()); }
unsafe fn unfreeze_cgroup(freezer: *mut freezer) { let mut it = core::mem::MaybeUninit::<css_task_iter>::uninit(); css_task_iter_start(&mut (*freezer).css, 0, it.as_mut_ptr()); loop { let t = css_task_iter_next(it.as_mut_ptr()); if t.is_null() { break } __thaw_task(t); } css_task_iter_end(it.as_mut_ptr()); }

unsafe fn freezer_apply_state(freezer: *mut freezer, freeze: bool, state: c_uint) {
    lockdep_assert_held(&freezer_mutex); if (*freezer).state & CGROUP_FREEZER_ONLINE == 0 { return; }
    if freeze { if (*freezer).state & CGROUP_FREEZING == 0 { static_branch_inc_cpuslocked(&mut freezer_active); } (*freezer).state |= state; freeze_cgroup(freezer); }
    else { let was_freezing = (*freezer).state & CGROUP_FREEZING != 0; (*freezer).state &= !state; if (*freezer).state & CGROUP_FREEZING == 0 { (*freezer).state &= !CGROUP_FROZEN; if was_freezing { static_branch_dec_cpuslocked(&mut freezer_active); } unfreeze_cgroup(freezer); } }
}

unsafe fn freezer_change_state(freezer: *mut freezer, freeze: bool) {
    let mut pos: *mut cgroup_subsys_state = core::ptr::null_mut(); cpus_read_lock(); mutex_lock(&mut freezer_mutex); rcu_read_lock();
    css_for_each_descendant_pre!(pos, &mut (*freezer).css, { let pos_f = css_freezer(pos); let parent = parent_freezer(pos_f); if css_tryget_online(pos) { rcu_read_unlock(); if pos_f == freezer { freezer_apply_state(pos_f, freeze, CGROUP_FREEZING_SELF); } else { freezer_apply_state(pos_f, !parent.is_null() && (*parent).state & CGROUP_FREEZING != 0, CGROUP_FREEZING_PARENT); } rcu_read_lock(); css_put(pos); } });
    rcu_read_unlock(); mutex_unlock(&mut freezer_mutex); cpus_read_unlock();
}

unsafe fn freezer_write(of: *mut kernfs_open_file, buf: *mut u8, nbytes: size_t, _off: loff_t) -> ssize_t {
    let buf = strstrip(buf); let freeze;
    if strcmp(buf, freezer_state_strs(0)) == 0 { freeze = false; }
    else if strcmp(buf, freezer_state_strs(CGROUP_FROZEN)) == 0 { pr_info_once!("Freezing with imperfect legacy cgroup freezer. See cgroup.freeze of cgroup v2\n"); freeze = true; }
    else { return -22; }
    freezer_change_state(css_freezer(of_css(of)), freeze); nbytes as ssize_t
}

unsafe fn freezer_self_freezing_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> c_ulong { ((*css_freezer(css)).state & CGROUP_FREEZING_SELF != 0) as c_ulong }
unsafe fn freezer_parent_freezing_read(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> c_ulong { ((*css_freezer(css)).state & CGROUP_FREEZING_PARENT != 0) as c_ulong }

static mut files: [cftype; 4] = [cftype {}, cftype {}, cftype {}, cftype {}];
static mut freezer_cgrp_subsys: cgroup_subsys = cgroup_subsys {};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
