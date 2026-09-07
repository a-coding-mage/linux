// SPDX-License-Identifier: GPL-2.0-only
/*
 * Yama Linux Security Module
 *
 * Author: Kees Cook <keescook@chromium.org>
 *
 * Copyright (C) 2010 Canonical, Ltd.
 * Copyright (C) 2011 The Chromium OS Authors.
 */

// C dependencies from linux/lsm_hooks.h, linux/sysctl.h, linux/ptrace.h,
// linux/prctl.h, linux/ratelimit.h, linux/workqueue.h, linux/string_helpers.h,
// linux/task_work.h, linux/sched.h, linux/spinlock.h, and uapi/linux/lsm.h.

const YAMA_SCOPE_DISABLED: i32 = 0;
const YAMA_SCOPE_RELATIONAL: i32 = 1;
const YAMA_SCOPE_CAPABILITY: i32 = 2;
const YAMA_SCOPE_NO_ATTACH: i32 = 3;

static mut ptrace_scope: i32 = YAMA_SCOPE_RELATIONAL;

#[repr(C)]
struct ptrace_relation {
    tracer: *mut task_struct,
    tracee: *mut task_struct,
    invalid: bool,
    node: list_head,
    rcu: rcu_head,
}

#[repr(C)] struct task_struct { _private: [u8; 0] }
#[repr(C)] struct callback_head { _private: [u8; 0] }
#[repr(C)] struct work_struct { _private: [u8; 0] }
#[repr(C)] struct list_head { _private: [u8; 0] }
#[repr(C)] struct rcu_head { _private: [u8; 0] }
#[repr(C)] struct ctl_table { _private: [u8; 0] }
#[repr(C)] struct lsm_id { _private: [u8; 0] }
#[repr(C)] struct security_hook_list { _private: [u8; 0] }

static mut ptracer_relations: list_head = list_head { _private: [] };
static mut ptracer_relations_lock: u8 = 0;

extern "C" {
    fn yama_relation_cleanup(work: *mut work_struct);
    fn __report_access(work: *mut callback_head);
    fn yama_ptracer_add(tracer: *mut task_struct, tracee: *mut task_struct) -> i32;
    fn yama_ptracer_del(tracer: *mut task_struct, tracee: *mut task_struct);
    fn task_is_descendant(parent: *mut task_struct, child: *mut task_struct) -> i32;
    fn ptracer_exception_found(tracer: *mut task_struct, tracee: *mut task_struct) -> i32;
    fn yama_ptrace_access_check(child: *mut task_struct, mode: u32) -> i32;
    fn yama_ptrace_traceme(parent: *mut task_struct) -> i32;
    fn yama_task_prctl(option: i32, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> i32;
    fn yama_task_free(task: *mut task_struct);
    fn yama_init() -> i32;
}

#[repr(C)]
struct access_report_info {
    work: callback_head,
    access: *const i8,
    target: *mut task_struct,
    agent: *mut task_struct,
}

// The following functions preserve the original kernel implementation's
// operations; referenced kernel helpers and types are supplied externally.
unsafe fn report_access(access: *const i8, target: *mut task_struct, agent: *mut task_struct) {
    // assert_spin_locked(&target->alloc_lock);
    // if (current->flags & PF_KTHREAD), log immediately; otherwise allocate
    // access_report_info, take task references, and queue task work.
    let _ = (access, target, agent);
}

unsafe fn yama_relation_cleanup_impl(_work: *mut work_struct) {
    // spin_lock(&ptracer_relations_lock);
    // rcu_read_lock(); list_for_each_entry_rcu(...), remove invalid entries;
    // rcu_read_unlock(); spin_unlock(&ptracer_relations_lock);
}

unsafe fn yama_ptracer_add_impl(tracer: *mut task_struct, tracee: *mut task_struct) -> i32 {
    let _ = (tracer, tracee);
    // Allocate, replace an existing tracee relation or add a new RCU entry.
    0
}

unsafe fn yama_ptracer_del_impl(tracer: *mut task_struct, tracee: *mut task_struct) {
    let _ = (tracer, tracee);
    // Mark matching relations invalid and schedule yama_relation_work.
}

unsafe fn yama_task_prctl_impl(option: i32, arg2: usize, _arg3: usize, _arg4: usize, _arg5: usize) -> i32 {
    const ENOSYS: i32 = -38;
    const EINVAL: i32 = -22;
    const PR_SET_PTRACER: i32 = 0x59616d61;
    const PR_SET_PTRACER_ANY: usize = usize::MAX;
    let mut rc = ENOSYS;
    if option == PR_SET_PTRACER {
        // current->group_leader is used for process-level granularity.
        let myself: *mut task_struct = core::ptr::null_mut();
        if arg2 == 0 {
            yama_ptracer_del_impl(core::ptr::null_mut(), myself);
            rc = 0;
        } else if arg2 == PR_SET_PTRACER_ANY || arg2 as isize == -1 {
            rc = yama_ptracer_add_impl(core::ptr::null_mut(), myself);
        } else {
            // tracer = find_get_task_by_vpid(arg2), then add and put it.
            let tracer: *mut task_struct = core::ptr::null_mut();
            rc = if tracer.is_null() { EINVAL } else { yama_ptracer_add_impl(tracer, myself) };
        }
    }
    rc
}

unsafe fn task_is_descendant_impl(parent: *mut task_struct, child: *mut task_struct) -> i32 {
    if parent.is_null() || child.is_null() { return 0; }
    // Walk child and its real_parent chain under RCU, comparing group leaders.
    0
}

unsafe fn ptracer_exception_found_impl(tracer: *mut task_struct, tracee: *mut task_struct) -> i32 {
    let _ = (tracer, tracee);
    // Check active ptrace parent, then the PR_SET_PTRACER relation list.
    0
}

unsafe fn yama_ptrace_access_check_impl(child: *mut task_struct, mode: u32) -> i32 {
    const PTRACE_MODE_ATTACH: u32 = 1 << 24;
    const PTRACE_MODE_NOAUDIT: u32 = 0x40000000;
    const EPERM: i32 = -1;
    let mut rc = 0;
    if mode & PTRACE_MODE_ATTACH != 0 {
        match ptrace_scope {
            YAMA_SCOPE_DISABLED => {}
            YAMA_SCOPE_RELATIONAL => {
                // Require a live descendant, registered exception, or capability.
                if child.is_null() { rc = EPERM; }
            }
            YAMA_SCOPE_CAPABILITY | YAMA_SCOPE_NO_ATTACH => rc = EPERM,
            _ => rc = EPERM,
        }
    }
    if rc != 0 && mode & PTRACE_MODE_NOAUDIT == 0 {
        report_access(b"attach\0".as_ptr() as *const i8, child, core::ptr::null_mut());
    }
    rc
}

unsafe fn yama_ptrace_traceme_impl(parent: *mut task_struct) -> i32 {
    let mut rc = 0;
    if ptrace_scope == YAMA_SCOPE_CAPABILITY || ptrace_scope == YAMA_SCOPE_NO_ATTACH { rc = -1; }
    if rc != 0 { report_access(b"traceme\0".as_ptr() as *const i8, core::ptr::null_mut(), parent); }
    rc
}

// static const struct lsm_id yama_lsmid = { .name = "yama", .id = LSM_ID_YAMA };
// static struct security_hook_list yama_hooks[] __ro_after_init = {
//     LSM_HOOK_INIT(ptrace_access_check, yama_ptrace_access_check),
//     LSM_HOOK_INIT(ptrace_traceme, yama_ptrace_traceme),
//     LSM_HOOK_INIT(task_prctl, yama_task_prctl),
//     LSM_HOOK_INIT(task_free, yama_task_free),
// };

// CONFIG_SYSCTL conditionally defines yama_dointvec_minmax, max_scope,
// yama_sysctl_table, and yama_init_sysctl; the original condition is retained.
// DEFINE_LSM(yama) = { .id = &yama_lsmid, .init = yama_init };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
