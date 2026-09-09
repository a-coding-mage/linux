// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Andrei Vagin <avagin@openvz.org>
 * Author: Dmitry Safonov <dima@arista.com>
 */

// Kernel dependencies supplied by other translation units.

pub unsafe fn do_timens_ktime_to_host(
    clockid: clockid_t,
    mut tim: ktime_t,
    ns_offsets: *mut timens_offsets,
) -> ktime_t {
    let offset: ktime_t;

    match clockid {
        CLOCK_MONOTONIC => {
            offset = unsafe { timespec64_to_ktime((*ns_offsets).monotonic) };
        }
        CLOCK_BOOTTIME | CLOCK_BOOTTIME_ALARM => {
            offset = unsafe { timespec64_to_ktime((*ns_offsets).boottime) };
        }
        _ => return tim,
    }

    /* Check that @tim is in [offset, KTIME_MAX + offset] and subtract offset. */
    if tim < offset {
        /* User can specify @tim absolute value; lesser means already expired. */
        tim = 0;
    } else {
        tim = unsafe { ktime_sub(tim, offset) };
        if tim > KTIME_MAX {
            tim = KTIME_MAX;
        }
    }
    tim
}

unsafe extern "C" {
    pub fn inc_ucount(ns: *mut user_namespace, uid: kuid_t, ucount: u32) -> *mut ucounts;
    pub fn dec_ucount(ucounts: *mut ucounts, ucount: u32);
    pub fn current_euid() -> kuid_t;
    pub fn kzalloc_obj(size: usize, flags: u32) -> *mut time_namespace;
    pub fn kfree(ptr: *mut time_namespace);
    pub fn timens_vdso_alloc_vvar_page(ns: *mut time_namespace) -> i32;
    pub fn timens_vdso_free_vvar_page(ns: *mut time_namespace);
    pub fn ns_common_init(ns: *mut time_namespace) -> i32;
    pub fn ns_common_free(ns: *mut time_namespace);
    pub fn ns_tree_add(ns: *mut time_namespace);
    pub fn ns_tree_remove(ns: *mut time_namespace);
    pub fn get_user_ns(ns: *mut user_namespace) -> *mut user_namespace;
    pub fn put_user_ns(ns: *mut user_namespace);
    pub fn get_time_ns(ns: *mut time_namespace) -> *mut time_namespace;
    pub fn put_time_ns(ns: *mut time_namespace);
    pub fn kfree_rcu(ns: *mut time_namespace);
    pub fn to_time_ns(ns: *mut ns_common) -> *mut time_namespace;
    pub fn timens_commit(task: *mut task_struct, ns: *mut time_namespace);
    pub fn current_is_single_threaded() -> bool;
    pub fn ns_capable(ns: *mut user_namespace, cap: i32) -> bool;
    pub fn file_ns_capable(file: *mut file, ns: *mut user_namespace, cap: i32) -> bool;
    pub fn ktime_get_ts64(ts: *mut timespec64);
    pub fn ktime_get_boottime_ts64(ts: *mut timespec64);
    pub fn timespec64_add(a: timespec64, b: timespec64) -> timespec64;
    pub fn seq_printf(m: *mut seq_file, fmt: *const u8, ...);
}

unsafe fn inc_time_namespaces(ns: *mut user_namespace) -> *mut ucounts {
    unsafe { inc_ucount(ns, current_euid(), UCOUNT_TIME_NAMESPACES) }
}

unsafe fn dec_time_namespaces(ucounts: *mut ucounts) {
    unsafe { dec_ucount(ucounts, UCOUNT_TIME_NAMESPACES) }
}

unsafe fn clone_time_ns(user_ns: *mut user_namespace, old_ns: *mut time_namespace) -> *mut time_namespace {
    let ucounts = unsafe { inc_time_namespaces(user_ns) };
    if ucounts.is_null() { return unsafe { ERR_PTR(-ENOSPC) }; }

    let ns = unsafe { kzalloc_obj(core::mem::size_of::<time_namespace>(), GFP_KERNEL_ACCOUNT) };
    if ns.is_null() { unsafe { dec_time_namespaces(ucounts) }; return unsafe { ERR_PTR(-ENOMEM) }; }
    let mut err = unsafe { timens_vdso_alloc_vvar_page(ns) };
    if err != 0 { unsafe { kfree(ns); dec_time_namespaces(ucounts) }; return unsafe { ERR_PTR(err) }; }
    err = unsafe { ns_common_init(ns) };
    if err != 0 {
        unsafe { timens_vdso_free_vvar_page(ns); kfree(ns); dec_time_namespaces(ucounts) };
        return unsafe { ERR_PTR(err) };
    }
    unsafe {
        (*ns).ucounts = ucounts;
        (*ns).user_ns = get_user_ns(user_ns);
        (*ns).offsets = (*old_ns).offsets;
        (*ns).frozen_offsets = false;
        ns_tree_add(ns);
    }
    ns
}

pub unsafe fn copy_time_ns(flags: u64, user_ns: *mut user_namespace, old_ns: *mut time_namespace) -> *mut time_namespace {
    if flags & CLONE_NEWTIME == 0 { unsafe { get_time_ns(old_ns) } } else { unsafe { clone_time_ns(user_ns, old_ns) } }
}

pub static mut timens_offset_lock: mutex = mutex::new();

pub unsafe fn free_time_ns(ns: *mut time_namespace) {
    unsafe { ns_tree_remove(ns); dec_time_namespaces((*ns).ucounts); put_user_ns((*ns).user_ns); ns_common_free(ns); timens_vdso_free_vvar_page(ns); kfree_rcu(ns); }
}

unsafe fn timens_get(task: *mut task_struct) -> *mut ns_common {
    let nsproxy = unsafe { (*task).nsproxy };
    if nsproxy.is_null() { return core::ptr::null_mut(); }
    let ns = unsafe { (*nsproxy).time_ns };
    unsafe { get_time_ns(ns); &mut (*ns).ns }
}

unsafe fn timens_for_children_get(task: *mut task_struct) -> *mut ns_common {
    let nsproxy = unsafe { (*task).nsproxy };
    if nsproxy.is_null() { return core::ptr::null_mut(); }
    let ns = unsafe { (*nsproxy).time_ns_for_children };
    unsafe { get_time_ns(ns); &mut (*ns).ns }
}

unsafe fn timens_put(ns: *mut ns_common) { unsafe { put_time_ns(to_time_ns(ns)) } }

unsafe fn timens_install(nsset: *mut nsset, new: *mut ns_common) -> i32 {
    let nsproxy = unsafe { (*nsset).nsproxy };
    let ns = unsafe { to_time_ns(new) };
    if !unsafe { current_is_single_threaded() } { return -EUSERS; }
    if !unsafe { ns_capable((*ns).user_ns, CAP_SYS_ADMIN) } || !unsafe { ns_capable((*(*nsset).cred).user_ns, CAP_SYS_ADMIN) } { return -EPERM; }
    unsafe { get_time_ns(ns); put_time_ns((*nsproxy).time_ns); (*nsproxy).time_ns = ns; get_time_ns(ns); put_time_ns((*nsproxy).time_ns_for_children); (*nsproxy).time_ns_for_children = ns; }
    0
}

pub unsafe fn timens_on_fork(nsproxy: *mut nsproxy, tsk: *mut task_struct) {
    let ns = unsafe { (*nsproxy).time_ns_for_children };
    if unsafe { (*nsproxy).time_ns == ns } { return; }
    unsafe { get_time_ns(ns); put_time_ns((*nsproxy).time_ns); (*nsproxy).time_ns = ns; timens_commit(tsk, ns); }
}

unsafe fn timens_owner(ns: *mut ns_common) -> *mut user_namespace { unsafe { (*to_time_ns(ns)).user_ns } }

unsafe fn show_offset(m: *mut seq_file, clockid: i32, ts: *mut timespec64) {
    let clock = match clockid { CLOCK_BOOTTIME => b"boottime\0", CLOCK_MONOTONIC => b"monotonic\0", _ => b"unknown\0" };
    unsafe { seq_printf(m, b"%-10s %10lld %9ld\n\0".as_ptr(), clock.as_ptr(), (*ts).tv_sec, (*ts).tv_nsec); }
}

pub unsafe fn time_ns_init() { unsafe { ns_tree_add(&mut init_time_ns); } }

pub unsafe fn proc_timens_show_offsets(p: *mut task_struct, m: *mut seq_file) {
    let ns = unsafe { timens_for_children_get(p) };
    if ns.is_null() { return; }
    let time_ns = unsafe { to_time_ns(ns) };
    unsafe { show_offset(m, CLOCK_MONOTONIC, &mut (*time_ns).offsets.monotonic); show_offset(m, CLOCK_BOOTTIME, &mut (*time_ns).offsets.boottime); timens_put(ns); }
}

pub unsafe fn proc_timens_set_offset(file: *mut file, p: *mut task_struct, offsets: *mut proc_timens_offset, noffsets: i32) -> i32 {
    let ns = unsafe { timens_for_children_get(p) };
    if ns.is_null() { return -ESRCH; }
    let time_ns = unsafe { to_time_ns(ns) };
    if !unsafe { file_ns_capable(file, (*time_ns).user_ns, CAP_SYS_TIME) } { unsafe { timens_put(ns) }; return -EPERM; }
    let mut tp = timespec64::default();
    for i in 0..noffsets {
        let off = unsafe { &*offsets.add(i as usize) };
        match off.clockid { CLOCK_MONOTONIC => unsafe { ktime_get_ts64(&mut tp) }, CLOCK_BOOTTIME => unsafe { ktime_get_boottime_ts64(&mut tp) }, _ => { unsafe { timens_put(ns) }; return -EINVAL; } }
        if off.val.tv_sec > KTIME_SEC_MAX || off.val.tv_sec < -KTIME_SEC_MAX { unsafe { timens_put(ns) }; return -ERANGE; }
        if off.val.tv_nsec < 0 || off.val.tv_nsec >= NSEC_PER_SEC { unsafe { timens_put(ns) }; return -EINVAL; }
        tp = unsafe { timespec64_add(tp, off.val) };
        if tp.tv_sec < 0 || tp.tv_sec > KTIME_SEC_MAX / 2 { unsafe { timens_put(ns) }; return -ERANGE; }
    }
    if unsafe { (*time_ns).frozen_offsets } { unsafe { timens_put(ns) }; return -EACCES; }
    for i in 0..noffsets {
        let off = unsafe { &*offsets.add(i as usize) };
        unsafe { match off.clockid { CLOCK_MONOTONIC => (*time_ns).offsets.monotonic = off.val, CLOCK_BOOTTIME => (*time_ns).offsets.boottime = off.val, _ => {} } }
    }
    unsafe { timens_put(ns) };
    0
}

pub static mut init_time_ns: time_namespace = time_namespace { ns: NS_COMMON_INIT!(init_time_ns), user_ns: &mut init_user_ns, frozen_offsets: true, ..time_namespace::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
