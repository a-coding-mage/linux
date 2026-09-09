// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/core/netclassid_cgroup.c\tClassid Cgroupfs Handling
 *
 * Authors:\tThomas Graf <tgraf@suug.ch>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/slab.h, linux/cgroup.h, linux/fdtable.h, linux/sched/task.h,
// net/cls_cgroup.h, and net/sock.h.

unsafe fn css_cls_state(
    css: *mut cgroup_subsys_state,
) -> *mut cgroup_cls_state {
    if !css.is_null() {
        container_of_css(css)
    } else {
        core::ptr::null_mut()
    }
}

pub unsafe fn task_cls_state(p: *mut task_struct) -> *mut cgroup_cls_state {
    css_cls_state(task_css_check(
        p,
        net_cls_cgrp_id,
        rcu_read_lock_held() || rcu_read_lock_bh_held() || rcu_read_lock_trace_held(),
    ))
}

// EXPORT_SYMBOL_GPL(task_cls_state);

unsafe fn cgrp_css_alloc(parent_css: *mut cgroup_subsys_state) -> *mut cgroup_subsys_state {
    let cs = kzalloc_obj_cgroup_cls_state();
    if cs.is_null() {
        return err_ptr(-ENOMEM);
    }
    let _ = parent_css;
    &mut (*cs).css
}

unsafe fn cgrp_css_online(css: *mut cgroup_subsys_state) -> i32 {
    let cs = css_cls_state(css);
    let parent = css_cls_state((*css).parent);

    if !parent.is_null() {
        (*cs).classid = (*parent).classid;
    }
    0
}

unsafe fn cgrp_css_free(css: *mut cgroup_subsys_state) {
    kfree(css_cls_state(css));
}

/*
 * To avoid freezing of sockets creation for tasks with big number of threads
 * and opened sockets lets release file_lock every 1000 iterated descriptors.
 * New sockets will already have been created with new classid.
 */

#[repr(C)]
struct update_classid_context {
    classid: u32,
    batch: u32,
}

const UPDATE_CLASSID_BATCH: u32 = 1000;

unsafe fn update_classid_sock(
    v: *const core::ffi::c_void,
    file: *mut file,
    n: u32,
) -> i32 {
    let ctx = v as *mut update_classid_context;
    let sock = sock_from_file(file);

    if !sock.is_null() {
        sock_cgroup_set_classid(&mut (*(*sock).sk).sk_cgrp_data, (*ctx).classid);
    }
    (*ctx).batch = (*ctx).batch.wrapping_sub(1);
    if (*ctx).batch == 0 {
        (*ctx).batch = UPDATE_CLASSID_BATCH;
        return n.wrapping_add(1) as i32;
    }
    0
}

unsafe fn update_classid_task(p: *mut task_struct, classid: u32) {
    let mut ctx = update_classid_context {
        classid,
        batch: UPDATE_CLASSID_BATCH,
    };
    let mut fd: u32 = 0;

    /* Only update the leader task, when many threads in this task,
     * so it can avoid the useless traversal.
     */
    if !thread_group_leader(p) {
        return;
    }

    loop {
        task_lock(p);
        fd = iterate_fd((*p).files, fd, update_classid_sock, &mut ctx);
        task_unlock(p);
        cond_resched();
        if fd == 0 {
            break;
        }
    }
}

unsafe fn cgrp_attach(tset: *mut cgroup_taskset) {
    let mut css: *mut cgroup_subsys_state = core::ptr::null_mut();
    let mut p: *mut task_struct = core::ptr::null_mut();

    cgroup_taskset_for_each(tset, &mut p, &mut css) {
        update_classid_task(p, (*css_cls_state(css)).classid);
    }
}

unsafe fn read_classid(css: *mut cgroup_subsys_state, _cft: *mut cftype) -> u64 {
    css_cls_state(css).as_ref().unwrap().classid as u64
}

unsafe fn write_classid(
    css: *mut cgroup_subsys_state,
    _cft: *mut cftype,
    value: u64,
) -> i32 {
    let cs = css_cls_state(css);
    let mut it: css_task_iter = core::mem::zeroed();
    let mut p: *mut task_struct;

    (*cs).classid = value as u32;

    css_task_iter_start(css, 0, &mut it);
    loop {
        p = css_task_iter_next(&mut it);
        if p.is_null() {
            break;
        }
        update_classid_task(p, (*cs).classid);
    }
    css_task_iter_end(&mut it);
    0
}

static mut ss_files: [cftype; 2] = [
    cftype {
        name: "classid",
        read_u64: Some(read_classid),
        write_u64: Some(write_classid),
    },
    cftype::TERMINATOR,
];

pub static mut net_cls_cgrp_subsys: cgroup_subsys = cgroup_subsys {
    css_alloc: Some(cgrp_css_alloc),
    css_online: Some(cgrp_css_online),
    css_free: Some(cgrp_css_free),
    attach: Some(cgrp_attach),
    legacy_cftypes: ss_files.as_mut_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
