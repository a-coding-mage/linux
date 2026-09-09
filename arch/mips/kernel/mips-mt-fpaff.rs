// SPDX-License-Identifier: GPL-2.0
/*
 * General MIPS MT support routines, usable in AP/SP and SMVP.
 * Copyright (C) 2005 Mips Technologies, Inc
 */

// Linux and MIPS kernel dependencies supplied by the surrounding kernel.

/*
 * CPU mask used to set process affinity for MT VPEs/TCs with FPUs
 */
static mut mt_fpu_cpumask: cpumask_t = cpumask_t::default();

static mut fpaff_threshold: i32 = -1;
static mut mt_fpemul_threshold: ::core::ffi::c_ulong = 0;

/*
 * Replacement functions for the sys_sched_setaffinity() and
 * sys_sched_getaffinity() system calls, so that we can integrate
 * FPU affinity with the user's requested processor affinity.
 * This code is 98% identical with the sys_sched_setaffinity()
 * and sys_sched_getaffinity() system calls, and should be
 * updated when kernel/sched/core.c changes.
 */

/*
 * find_process_by_pid - find a process with a matching PID value.
 * used in sys_sched_set/getaffinity() in kernel/sched/core.c, so
 * cloned here.
 */
#[inline]
unsafe fn find_process_by_pid(pid: pid_t) -> *mut task_struct {
    if pid != 0 { find_task_by_vpid(pid) } else { current }
}

/*
 * check the target process has a UID that matches the current process's
 */
unsafe fn check_same_owner(p: *mut task_struct) -> bool {
    let cred = current_cred();
    let pcred: *const cred;
    let match_: bool;

    rcu_read_lock();
    pcred = __task_cred(p);
    match_ = uid_eq((*cred).euid, (*pcred).euid)
        || uid_eq((*cred).euid, (*pcred).uid);
    rcu_read_unlock();
    match_
}

/*
 * mipsmt_sys_sched_setaffinity - set the cpu affinity of a process
 */
unsafe fn mipsmt_sys_sched_setaffinity(
    pid: pid_t,
    mut len: ::core::ffi::c_uint,
    user_mask_ptr: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    let mut cpus_allowed: cpumask_var_t;
    let mut new_mask: cpumask_var_t;
    let mut effective_mask: cpumask_var_t;
    let mut ti: *mut thread_info;
    let p: *mut task_struct;
    let mut retval: i32;

    if !alloc_cpumask_var(&mut new_mask, GFP_KERNEL) { return -ENOMEM; }
    if len < cpumask_size() {
        cpumask_clear(new_mask);
    } else if len > cpumask_size() {
        len = cpumask_size();
    }
    if copy_from_user(new_mask, user_mask_ptr, len) != 0 {
        retval = -EFAULT;
        free_cpumask_var(new_mask);
        return retval as ::core::ffi::c_long;
    }

    cpus_read_lock();
    rcu_read_lock();
    p = find_process_by_pid(pid);
    if p.is_null() {
        rcu_read_unlock();
        cpus_read_unlock();
        free_cpumask_var(new_mask);
        return -ESRCH as ::core::ffi::c_long;
    }
    get_task_struct(p);
    rcu_read_unlock();

    if !alloc_cpumask_var(&mut cpus_allowed, GFP_KERNEL) {
        retval = -ENOMEM;
        put_task_struct(p); cpus_read_unlock(); free_cpumask_var(new_mask);
        return retval as ::core::ffi::c_long;
    }
    if !alloc_cpumask_var(&mut effective_mask, GFP_KERNEL) {
        retval = -ENOMEM;
        free_cpumask_var(cpus_allowed); put_task_struct(p); cpus_read_unlock();
        free_cpumask_var(new_mask);
        return retval as ::core::ffi::c_long;
    }
    if !check_same_owner(p) && !capable(CAP_SYS_NICE) { retval = -EPERM; }
    else { retval = security_task_setscheduler(p); }
    if retval != 0 {
        free_cpumask_var(effective_mask); free_cpumask_var(cpus_allowed);
        put_task_struct(p); cpus_read_unlock(); free_cpumask_var(new_mask);
        return retval as ::core::ffi::c_long;
    }

    cpumask_copy(&mut (*p).thread.user_cpus_allowed, new_mask);
    loop {
        ti = task_thread_info(p);
        if test_ti_thread_flag(ti, TIF_FPUBOUND)
            && cpumask_intersects(new_mask, &mt_fpu_cpumask) {
            cpumask_and(effective_mask, new_mask, &mt_fpu_cpumask);
            retval = set_cpus_allowed_ptr(p, effective_mask);
        } else {
            cpumask_copy(effective_mask, new_mask);
            clear_ti_thread_flag(ti, TIF_FPUBOUND);
            retval = set_cpus_allowed_ptr(p, new_mask);
        }
        if retval == 0 {
            cpuset_cpus_allowed(p, cpus_allowed);
            if !cpumask_subset(effective_mask, cpus_allowed) {
                cpumask_copy(new_mask, cpus_allowed);
                continue;
            }
        }
        break;
    }
    free_cpumask_var(effective_mask);
    free_cpumask_var(cpus_allowed);
    put_task_struct(p);
    cpus_read_unlock();
    free_cpumask_var(new_mask);
    retval as ::core::ffi::c_long
}

/*
 * mipsmt_sys_sched_getaffinity - get the cpu affinity of a process
 */
unsafe fn mipsmt_sys_sched_getaffinity(
    pid: pid_t, len: ::core::ffi::c_uint,
    user_mask_ptr: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_long {
    let real_len = core::mem::size_of::<cpumask_t>();
    let mut allowed = cpumask_t::default();
    let mut mask = cpumask_t::default();
    let mut retval: i32;
    let p: *mut task_struct;
    if (len as usize) < real_len { return -EINVAL as ::core::ffi::c_long; }
    cpus_read_lock(); rcu_read_lock();
    retval = -ESRCH;
    p = find_process_by_pid(pid);
    if !p.is_null() {
        retval = security_task_getscheduler(p);
        if retval == 0 {
            cpumask_or(&mut allowed, &(*p).thread.user_cpus_allowed, (*p).cpus_ptr);
            cpumask_and(&mut mask, &allowed, &cpu_active_mask);
        }
    }
    rcu_read_unlock(); cpus_read_unlock();
    if retval != 0 { return retval as ::core::ffi::c_long; }
    if copy_to_user(user_mask_ptr, &mask, real_len) != 0 { return -EFAULT as ::core::ffi::c_long; }
    real_len as ::core::ffi::c_long
}

unsafe fn fpaff_thresh(mut str_: *mut ::core::ffi::c_char) -> i32 {
    get_option(&mut str_, &mut fpaff_threshold);
    1
}

const FPUSEFACTOR: i32 = 2000;

unsafe fn mt_fp_affinity_init() -> i32 {
    if fpaff_threshold >= 0 {
        mt_fpemul_threshold = fpaff_threshold as ::core::ffi::c_ulong;
    } else {
        mt_fpemul_threshold = (FPUSEFACTOR as ::core::ffi::c_ulong
            * (loops_per_jiffy / (500000 / HZ))) / HZ;
    }
    printk!(KERN_DEBUG, "FPU Affinity set after {} emulations\n", mt_fpemul_threshold);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
