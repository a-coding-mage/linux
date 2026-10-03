// SPDX-License-Identifier: GPL-2.0-only
// Runtime translation of kernel/pid.c, source aec86a8.
// Kernel layouts and config-dependent constants come only from C bindgen.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unused_mut,
    unused_macros,
    unused_unsafe,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/pid_generated.rs"));
}
use bindings::*;
use core::mem::zeroed;
use core::mem::{offset_of, size_of};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_long, c_uint};
include!("pid_layout.rs");
include!("pid_initializers.rs");

static mut pid_max_min: c_int = LUPOS_PID_RESERVED_PIDS + 1;
static mut pid_max_max: c_int = LUPOS_PID_MAX_LIMIT;

#[inline]
unsafe fn numbers(p: *mut pid) -> *mut upid {
    // The C flexible array starts at the bindgen-generated field offset.
    addr_of_mut!((*p).numbers).cast()
}
#[inline]
unsafe fn tasks(p: *mut pid, kind: pid_type) -> *mut hlist_head {
    addr_of_mut!((*p).tasks)
        .cast::<hlist_head>()
        .add(kind.0 as usize)
}
#[inline]
unsafe fn links(t: *mut task_struct, kind: pid_type) -> *mut hlist_node {
    addr_of_mut!((*t).pid_links)
        .cast::<hlist_node>()
        .add(kind.0 as usize)
}
#[inline]
unsafe fn error<T>(errno: c_int) -> *mut T {
    lupos_pid_err_ptr(errno as c_long).cast()
}

#[no_mangle]
pub unsafe extern "C" fn put_pid(p: *mut pid) {
    if p.is_null() {
        return;
    }
    let ns = (*numbers(p).add((*p).level as usize)).ns;
    if lupos_pid_ref_dec(addr_of_mut!((*p).count)) {
        pidfs_free_pid(p);
        kmem_cache_free((*ns).pid_cachep, p.cast());
        lupos_pid_ns_put(ns);
    }
}

#[no_mangle]
pub unsafe extern "C" fn lupos_pid_delayed_put_pid(head: *mut callback_head) {
    put_pid(head.cast::<u8>().sub(offset_of!(pid, rcu)).cast());
}

#[no_mangle]
pub unsafe extern "C" fn free_pid(p: *mut pid) {
    lupos_pid_assert_tasklist_unheld();
    let active_ns = (*numbers(p).add((*p).level as usize)).ns;
    lupos_pid_ns_active_put(active_ns);
    lupos_pid_map_lock();
    let mut i = 0;
    while i <= (*p).level {
        let up = numbers(p).add(i as usize);
        let ns = (*up).ns;
        (*ns).pid_allocated = (*ns).pid_allocated.wrapping_sub(1);
        match (*ns).pid_allocated {
            2 | 1 => {
                wake_up_process(lupos_pid_read_task(addr_of!((*ns).child_reaper)));
            }
            PIDNS_ADDING => {
                lupos_pid_warn_reaper(!lupos_pid_read_task(addr_of!((*ns).child_reaper)).is_null());
            }
            _ => {}
        }
        idr_remove(addr_of_mut!((*ns).idr), (*up).nr as _);
        i += 1;
    }
    lupos_pid_map_unlock();
    pidfs_remove_pid(p);
    lupos_pid_call_rcu(addr_of_mut!((*p).rcu));
}

#[no_mangle]
pub unsafe extern "C" fn free_pids(pids: *mut *mut pid) {
    let mut i = PIDTYPE_MAX.0 as c_int;
    loop {
        i -= 1;
        if i < 0 {
            break;
        }
        let p = *pids.add(i as usize);
        if !p.is_null() {
            free_pid(p);
        }
    }
}

// Corresponds to out_abort; retain the original namespace/cache release order.
unsafe fn alloc_abort(ns: *mut pid_namespace, p: *mut pid, result: c_int) -> *mut pid {
    lupos_pid_ns_put(ns);
    kmem_cache_free((*ns).pid_cachep, p.cast());
    error(result)
}

// Corresponds to out_free. i is the last unallocated namespace index, or the
// index below the just-allocated namespace when its init ordering check fails.
unsafe fn alloc_rollback(
    ns: *mut pid_namespace,
    p: *mut pid,
    mut i: c_int,
    result: c_int,
) -> *mut pid {
    i += 1;
    while i <= (*ns).level as c_int {
        let up = numbers(p).add(i as usize);
        idr_remove(addr_of_mut!((*(*up).ns).idr), (*up).nr as _);
        i += 1;
    }
    lupos_pid_map_unlock();
    lupos_pid_idr_preload_end();
    alloc_abort(ns, p, result)
}

#[no_mangle]
pub unsafe extern "C" fn alloc_pid(
    ns: *mut pid_namespace,
    arg_set_tid: *mut pid_t,
    mut arg_set_tid_size: usize,
) -> *mut pid {
    let mut set_tid = [0 as c_int; MAX_PID_NS_LEVEL as usize + 1];
    let mut pid_max = [0 as c_int; MAX_PID_NS_LEVEL as usize + 1];
    if arg_set_tid_size > (*ns).level as usize + 1 {
        return error(-(EINVAL as c_int));
    }
    let p: *mut pid = lupos_pid_cache_alloc((*ns).pid_cachep, LUPOS_PID_GFP_KERNEL).cast();
    if p.is_null() {
        return error(-(ENOMEM as c_int));
    }

    lupos_pid_ns_get(ns);
    (*p).level = (*ns).level;
    lupos_pid_ref_set(addr_of_mut!((*p).count), 1);
    lupos_pid_spin_init(addr_of_mut!((*p).lock));
    let mut kind = 0;
    while kind < PIDTYPE_MAX.0 {
        lupos_pid_hlist_init(tasks(p, pid_type(kind)));
        kind += 1;
    }
    lupos_pid_wait_init(addr_of_mut!((*p).wait_pidfd));
    lupos_pid_hlist_init(addr_of_mut!((*p).inodes));
    pidfs_prepare_pid(p);

    let mut tmp = ns;
    let mut i = (*ns).level as c_int;
    while i >= 0 {
        let index = (*ns).level as usize - i as usize;
        let limit = lupos_pid_read_int(addr_of!((*tmp).pid_max));
        *pid_max.as_mut_ptr().add(index) = limit;
        if arg_set_tid_size != 0 {
            let tid = *arg_set_tid.add(index);
            *set_tid.as_mut_ptr().add(index) = tid;
            if tid < 1 || tid >= limit {
                return alloc_abort(ns, p, -(EINVAL as c_int));
            }
            if !lupos_pid_checkpoint_capable((*tmp).user_ns) {
                return alloc_abort(ns, p, -(EPERM as c_int));
            }
            arg_set_tid_size -= 1;
        }
        tmp = (*tmp).parent;
        i -= 1;
    }

    let mut retried_preload = false;
    idr_preload(LUPOS_PID_GFP_KERNEL);
    lupos_pid_map_lock();
    if (*ns).pid_allocated == PIDNS_ADDING {
        lupos_pid_idr_set_cursor(addr_of_mut!((*ns).idr), 0);
    }
    tmp = ns;
    i = (*ns).level as c_int;
    while i >= 0 {
        let index = (*ns).level as usize - i as usize;
        let tid = *set_tid.as_ptr().add(index);
        let mut nr;
        if tid != 0 {
            nr = idr_alloc(
                addr_of_mut!((*tmp).idr),
                null_mut(),
                tid,
                tid + 1,
                LUPOS_PID_GFP_ATOMIC,
            );
            if nr == -(ENOSPC as c_int) {
                nr = -(EEXIST as c_int);
            }
        } else {
            let min =
                if lupos_pid_idr_cursor(addr_of!((*tmp).idr)) > LUPOS_PID_RESERVED_PIDS as c_uint {
                    LUPOS_PID_RESERVED_PIDS
                } else {
                    1
                };
            // Reserve a null slot: no partially initialized PID is published.
            nr = idr_alloc_cyclic(
                addr_of_mut!((*tmp).idr),
                null_mut(),
                min,
                *pid_max.as_ptr().add(index),
                LUPOS_PID_GFP_ATOMIC,
            );
            if nr == -(ENOSPC as c_int) {
                nr = -(EAGAIN as c_int);
            }
        }
        if nr < 0 {
            if nr == -(ENOMEM as c_int) && !retried_preload {
                lupos_pid_map_unlock();
                lupos_pid_idr_preload_end();
                retried_preload = true;
                idr_preload(LUPOS_PID_GFP_KERNEL);
                lupos_pid_map_lock();
                continue;
            }
            return alloc_rollback(ns, p, i, nr);
        }
        let up = numbers(p).add(i as usize);
        (*up).nr = nr;
        (*up).ns = tmp;
        i -= 1;
        retried_preload = false;
        if lupos_pid_read_task(addr_of!((*tmp).child_reaper)).is_null() && nr != 1 {
            return alloc_rollback(ns, p, i, -(EINVAL as c_int));
        }
        tmp = (*tmp).parent;
    }

    // This test must follow allocation to preserve the original errno priority.
    let mut j = (*ns).level as c_int;
    while j >= 0 {
        let up = numbers(p).add(j as usize);
        if (*(*up).ns).pid_allocated & PIDNS_ADDING == 0 {
            return alloc_rollback(ns, p, i, -(ENOMEM as c_int));
        }
        j -= 1;
    }
    j = (*ns).level as c_int;
    while j >= 0 {
        let up = numbers(p).add(j as usize);
        idr_replace(addr_of_mut!((*(*up).ns).idr), p.cast(), (*up).nr as _);
        (*(*up).ns).pid_allocated = (*(*up).ns).pid_allocated.wrapping_add(1);
        j -= 1;
    }
    lupos_pid_map_unlock();
    lupos_pid_idr_preload_end();
    lupos_pid_ns_active_get(ns);
    if pidfs_add_pid(p) != 0 {
        free_pid(p);
        return error(-(ENOMEM as c_int));
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn disable_pid_allocation(ns: *mut pid_namespace) {
    lupos_pid_map_lock();
    (*ns).pid_allocated &= !PIDNS_ADDING;
    lupos_pid_map_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn find_pid_ns(nr: c_int, ns: *mut pid_namespace) -> *mut pid {
    idr_find(addr_of!((*ns).idr), nr as _).cast()
}

#[no_mangle]
pub unsafe extern "C" fn find_vpid(nr: c_int) -> *mut pid {
    find_pid_ns(nr, task_active_pid_ns(lupos_pid_current()))
}

unsafe fn task_pid_ptr(task: *mut task_struct, kind: pid_type) -> *mut *mut pid {
    if kind == PIDTYPE_PID {
        addr_of_mut!((*task).thread_pid)
    } else {
        addr_of_mut!((*(*task).signal).pids)
            .cast::<*mut pid>()
            .add(kind.0 as usize)
    }
}

#[no_mangle]
pub unsafe extern "C" fn attach_pid(task: *mut task_struct, kind: pid_type) {
    lupos_pid_assert_tasklist_write();
    let p = *task_pid_ptr(task, kind);
    lupos_pid_hlist_add(links(task, kind), tasks(p, kind));
}

unsafe fn __change_pid(pids: *mut *mut pid, task: *mut task_struct, kind: pid_type, new: *mut pid) {
    lupos_pid_assert_tasklist_write();
    let pid_ptr = task_pid_ptr(task, kind);
    let p = *pid_ptr;
    lupos_pid_hlist_del(links(task, kind));
    *pid_ptr = new;
    let mut tmp = PIDTYPE_MAX.0 as c_int;
    loop {
        tmp -= 1;
        if tmp < 0 {
            break;
        }
        if !lupos_pid_hlist_empty(tasks(p, pid_type(tmp as _))) {
            return;
        }
    }
    lupos_pid_warn_pending(!(*pids.add(kind.0 as usize)).is_null());
    *pids.add(kind.0 as usize) = p;
}

#[no_mangle]
pub unsafe extern "C" fn detach_pid(pids: *mut *mut pid, task: *mut task_struct, kind: pid_type) {
    __change_pid(pids, task, kind, null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn change_pid(
    pids: *mut *mut pid,
    task: *mut task_struct,
    kind: pid_type,
    p: *mut pid,
) {
    __change_pid(pids, task, kind, p);
    attach_pid(task, kind);
}

#[no_mangle]
pub unsafe extern "C" fn exchange_tids(left: *mut task_struct, right: *mut task_struct) {
    let pid1 = (*left).thread_pid;
    let pid2 = (*right).thread_pid;
    let head1 = tasks(pid1, PIDTYPE_PID);
    let head2 = tasks(pid2, PIDTYPE_PID);
    lupos_pid_assert_tasklist_write();
    lupos_pid_hlist_swap(head1, head2);
    lupos_pid_rcu_store(addr_of_mut!((*left).thread_pid), pid2);
    lupos_pid_rcu_store(addr_of_mut!((*right).thread_pid), pid1);
    // pid_nr() uses global namespace slot zero, with its original null guard.
    lupos_pid_write_nr(
        addr_of_mut!((*left).pid),
        if pid2.is_null() {
            0
        } else {
            (*numbers(pid2)).nr
        },
    );
    lupos_pid_write_nr(
        addr_of_mut!((*right).pid),
        if pid1.is_null() {
            0
        } else {
            (*numbers(pid1)).nr
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn transfer_pid(
    old: *mut task_struct,
    new: *mut task_struct,
    kind: pid_type,
) {
    lupos_pid_warn_transfer(kind == PIDTYPE_PID);
    lupos_pid_assert_tasklist_write();
    lupos_pid_hlist_replace(links(old, kind), links(new, kind));
}

#[no_mangle]
pub unsafe extern "C" fn pid_task(p: *mut pid, kind: pid_type) -> *mut task_struct {
    if p.is_null() {
        return null_mut();
    }
    let first = lupos_pid_hlist_first(tasks(p, kind));
    if first.is_null() {
        return null_mut();
    }
    first
        .cast::<u8>()
        .sub(offset_of!(task_struct, pid_links) + kind.0 as usize * size_of::<hlist_node>())
        .cast()
}

#[no_mangle]
pub unsafe extern "C" fn find_task_by_pid_ns(
    nr: pid_t,
    ns: *mut pid_namespace,
) -> *mut task_struct {
    lupos_pid_warn_rcu();
    pid_task(find_pid_ns(nr, ns), PIDTYPE_PID)
}

#[no_mangle]
pub unsafe extern "C" fn find_task_by_vpid(nr: pid_t) -> *mut task_struct {
    find_task_by_pid_ns(nr, task_active_pid_ns(lupos_pid_current()))
}

#[no_mangle]
pub unsafe extern "C" fn find_get_task_by_vpid(nr: pid_t) -> *mut task_struct {
    lupos_pid_rcu_lock();
    let task = find_task_by_vpid(nr);
    if !task.is_null() {
        lupos_pid_get_task(task);
    }
    lupos_pid_rcu_unlock();
    task
}

#[no_mangle]
pub unsafe extern "C" fn get_task_pid(task: *mut task_struct, kind: pid_type) -> *mut pid {
    lupos_pid_rcu_lock();
    let p = lupos_pid_get(lupos_pid_rcu_load(task_pid_ptr(task, kind)));
    lupos_pid_rcu_unlock();
    p
}

#[no_mangle]
pub unsafe extern "C" fn get_pid_task(p: *mut pid, kind: pid_type) -> *mut task_struct {
    lupos_pid_rcu_lock();
    let task = pid_task(p, kind);
    if !task.is_null() {
        lupos_pid_get_task(task);
    }
    lupos_pid_rcu_unlock();
    task
}

#[no_mangle]
pub unsafe extern "C" fn find_get_pid(nr: pid_t) -> *mut pid {
    lupos_pid_rcu_lock();
    let p = lupos_pid_get(find_vpid(nr));
    lupos_pid_rcu_unlock();
    p
}

#[no_mangle]
pub unsafe extern "C" fn pid_nr_ns(p: *mut pid, ns: *mut pid_namespace) -> pid_t {
    if !p.is_null() && !ns.is_null() && (*ns).level <= (*p).level {
        let up = numbers(p).add((*ns).level as usize);
        if (*up).ns == ns {
            return (*up).nr;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pid_vnr(p: *mut pid) -> pid_t {
    pid_nr_ns(p, task_active_pid_ns(lupos_pid_current()))
}

#[no_mangle]
pub unsafe extern "C" fn __task_pid_nr_ns(
    task: *mut task_struct,
    kind: pid_type,
    mut ns: *mut pid_namespace,
) -> pid_t {
    lupos_pid_rcu_lock();
    if ns.is_null() {
        ns = task_active_pid_ns(lupos_pid_current());
    }
    let nr = pid_nr_ns(lupos_pid_rcu_load(task_pid_ptr(task, kind)), ns);
    lupos_pid_rcu_unlock();
    nr
}

#[no_mangle]
pub unsafe extern "C" fn task_active_pid_ns(task: *mut task_struct) -> *mut pid_namespace {
    let p = (*task).thread_pid;
    if p.is_null() {
        null_mut()
    } else {
        (*numbers(p).add((*p).level as usize)).ns
    }
}

#[no_mangle]
pub unsafe extern "C" fn find_ge_pid(mut nr: c_int, ns: *mut pid_namespace) -> *mut pid {
    idr_get_next(addr_of_mut!((*ns).idr), &mut nr).cast()
}

// Reproduce CLASS(fd, f) cleanup on every return without transferring ownership.
struct FdScope(fd);
impl Drop for FdScope {
    fn drop(&mut self) {
        unsafe {
            lupos_pid_fdput(self.0);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pidfd_get_pid(fd: c_uint, flags: *mut c_uint) -> *mut pid {
    let f = FdScope(fdget(fd));
    if lupos_pid_fd_empty(f.0) {
        return error(-(EBADF as c_int));
    }
    let file = lupos_pid_fd_file(f.0);
    let p = pidfd_pid(file);
    if !lupos_pid_is_err(p.cast()) {
        lupos_pid_get(p);
        *flags = (*file).f_flags;
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn pidfd_get_task(pidfd: c_int, flags: *mut c_uint) -> *mut task_struct {
    let mut f_flags = 0;
    let (p, kind) = if pidfd == PIDFD_SELF_THREAD as c_int {
        (get_task_pid(lupos_pid_current(), PIDTYPE_PID), PIDTYPE_PID)
    } else if pidfd == PIDFD_SELF_THREAD_GROUP as c_int {
        (
            get_task_pid(lupos_pid_current(), PIDTYPE_TGID),
            PIDTYPE_TGID,
        )
    } else {
        let p = pidfd_get_pid(pidfd as c_uint, &mut f_flags);
        if lupos_pid_is_err(p.cast()) {
            return p.cast();
        }
        (p, PIDTYPE_TGID)
    };
    let task = get_pid_task(p, kind);
    put_pid(p);
    if task.is_null() {
        return error(-(ESRCH as c_int));
    }
    *flags = f_flags;
    task
}

unsafe fn pidfd_create(p: *mut pid, flags: c_uint) -> c_int {
    let mut file = null_mut();
    let fd = pidfd_prepare(p, flags, &mut file);
    if fd < 0 {
        return fd;
    }
    fd_install(fd as c_uint, file);
    fd
}

#[no_mangle]
pub unsafe extern "C" fn lupos_pid_sys_pidfd_open(nr: pid_t, flags: c_uint) -> c_long {
    if flags & !((PIDFD_NONBLOCK | PIDFD_THREAD) as c_uint) != 0 {
        return -(EINVAL as c_long);
    }
    if nr <= 0 {
        return -(EINVAL as c_long);
    }
    let p = find_get_pid(nr);
    if p.is_null() {
        return -(ESRCH as c_long);
    }
    let fd = pidfd_create(p, flags);
    put_pid(p);
    fd as c_long
}

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
pub unsafe extern "C" fn lupos_pid_table_root_lookup(
    _root: *mut ctl_table_root,
) -> *mut ctl_table_set {
    addr_of_mut!((*task_active_pid_ns(lupos_pid_current())).set)
}

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
pub unsafe extern "C" fn lupos_pid_set_is_seen(set: *mut ctl_table_set) -> c_int {
    (addr_of_mut!((*task_active_pid_ns(lupos_pid_current())).set) == set) as c_int
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn table_pidns(head: *mut ctl_table_header) -> *mut pid_namespace {
    (*head)
        .set
        .cast::<u8>()
        .sub(offset_of!(pid_namespace, set))
        .cast()
}

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
pub unsafe extern "C" fn lupos_pid_table_root_permissions(
    head: *mut ctl_table_header,
    table: *const ctl_table,
) -> c_int {
    let ns = table_pidns(head);
    let mut mode = (*table).mode as c_int;
    if lupos_pid_ns_capable_noaudit((*ns).user_ns, CAP_SYS_ADMIN as c_int)
        || lupos_pid_uid_eq(
            lupos_pid_current_euid(),
            lupos_pid_make_kuid((*ns).user_ns, 0),
        )
    {
        mode = (mode & S_IRWXU as c_int) >> 6;
    } else if lupos_pid_in_egroup(lupos_pid_make_kgid((*ns).user_ns, 0)) != 0 {
        mode = (mode & S_IRWXG as c_int) >> 3;
    } else {
        mode &= S_IROTH as c_int;
    }
    (mode << 6) | (mode << 3) | mode
}

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
pub unsafe extern "C" fn lupos_pid_table_root_set_ownership(
    head: *mut ctl_table_header,
    uid: *mut kuid_t,
    gid: *mut kgid_t,
) {
    let ns = table_pidns(head);
    let ns_root_uid = lupos_pid_make_kuid((*ns).user_ns, 0);
    if lupos_pid_uid_valid(ns_root_uid) {
        *uid = ns_root_uid;
    }
    let ns_root_gid = lupos_pid_make_kgid((*ns).user_ns, 0);
    if lupos_pid_gid_valid(ns_root_gid) {
        *gid = ns_root_gid;
    }
}

#[cfg(CONFIG_SYSCTL)]
static mut pid_table_root: ctl_table_root = ctl_table_root {
    lookup: Some(lupos_pid_sysctl_lookup),
    permissions: Some(lupos_pid_sysctl_permissions),
    set_ownership: Some(lupos_pid_sysctl_ownership),
    ..unsafe { zeroed() }
};

#[cfg(CONFIG_SYSCTL)]
#[repr(transparent)]
struct PidTable([ctl_table; 1]);
#[cfg(CONFIG_SYSCTL)]
unsafe impl Sync for PidTable {}

#[cfg(CONFIG_SYSCTL)]
static pid_table: PidTable = PidTable([ctl_table {
    procname: c"pid_max".as_ptr().cast(),
    data: unsafe { addr_of_mut!(init_pid_ns.pid_max).cast() },
    maxlen: size_of::<c_int>() as c_int,
    mode: 0o644,
    proc_handler: Some(proc_dointvec_minmax),
    extra1: addr_of_mut!(pid_max_min).cast(),
    extra2: addr_of_mut!(pid_max_max).cast(),
    ..unsafe { zeroed() }
}]);

#[no_mangle]
pub unsafe extern "C" fn register_pidns_sysctls(pidns: *mut pid_namespace) -> c_int {
    #[cfg(CONFIG_SYSCTL)]
    {
        setup_sysctl_set(
            addr_of_mut!((*pidns).set),
            addr_of_mut!(pid_table_root),
            Some(lupos_pid_sysctl_seen),
        );
        let tbl: *mut ctl_table = lupos_pid_memdup(
            addr_of!(pid_table).cast(),
            size_of::<PidTable>() as _,
            LUPOS_PID_GFP_KERNEL,
        )
        .cast();
        // Deliberately no retire_sysctl_set here: preserve the original first
        // allocation-failure branch, rather than changing C behavior silently.
        if tbl.is_null() {
            return -(ENOMEM as c_int);
        }
        (*tbl).data = addr_of_mut!((*pidns).pid_max).cast();
        (*pidns).pid_max = core::cmp::min(
            pid_max_max,
            core::cmp::max(
                (*pidns).pid_max,
                LUPOS_PID_PIDS_PER_CPU_DEFAULT.wrapping_mul(lupos_pid_possible_cpus() as c_int),
            ),
        );
        (*pidns).sysctls =
            __register_sysctl_table(addr_of_mut!((*pidns).set), c"kernel".as_ptr().cast(), tbl, 1);
        if (*pidns).sysctls.is_null() {
            kfree(tbl.cast());
            retire_sysctl_set(addr_of_mut!((*pidns).set));
            return -(ENOMEM as c_int);
        }
    }
    #[cfg(not(CONFIG_SYSCTL))]
    let _ = pidns;
    0
}

#[no_mangle]
pub unsafe extern "C" fn unregister_pidns_sysctls(pidns: *mut pid_namespace) {
    #[cfg(CONFIG_SYSCTL)]
    {
        let tbl = (*(*pidns).sysctls).ctl_table_arg;
        unregister_sysctl_table((*pidns).sysctls);
        retire_sysctl_set(addr_of_mut!((*pidns).set));
        kfree(tbl.cast());
    }
    #[cfg(not(CONFIG_SYSCTL))]
    let _ = pidns;
}

#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn pid_idr_init() {
    // BUILD_BUG_ON is retained as the helper TU static_assert.
    init_pid_ns.pid_max = core::cmp::min(
        pid_max_max,
        core::cmp::max(
            init_pid_ns.pid_max,
            LUPOS_PID_PIDS_PER_CPU_DEFAULT.wrapping_mul(lupos_pid_possible_cpus() as c_int),
        ),
    );
    pid_max_min = core::cmp::max(
        pid_max_min,
        LUPOS_PID_PIDS_PER_CPU_MIN.wrapping_mul(lupos_pid_possible_cpus() as c_int),
    );
    lupos_pid_log_limits(init_pid_ns.pid_max as c_uint, pid_max_min as c_uint);
    lupos_pid_idr_init(addr_of_mut!(init_pid_ns.idr));
    init_pid_ns.pid_cachep = lupos_pid_cache_create(
        c"pid".as_ptr().cast(),
        LUPOS_PID_INITIAL_SIZE as c_uint,
        LUPOS_PID_ALIGN as c_uint,
        LUPOS_PID_SLAB_FLAGS,
    );
}

#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_pid_namespace_sysctl_init() -> c_int {
    #[cfg(CONFIG_SYSCTL)]
    lupos_pid_bug(register_pidns_sysctls(addr_of_mut!(init_pid_ns)) != 0);
    0
}

unsafe fn __pidfd_fget(task: *mut task_struct, fd: c_int) -> *mut file {
    let ret = down_read_killable(addr_of_mut!((*(*task).signal).exec_update_lock));
    if ret != 0 {
        return error(ret);
    }
    let mut f = if !ptrace_may_access(task, PTRACE_MODE_ATTACH_REALCREDS as c_uint) {
        error(-(EPERM as c_int))
    } else if (*task).flags & PF_EXITING as c_uint != 0 {
        error(-(ESRCH as c_int))
    } else {
        fget_task(task, fd as c_uint)
    };
    up_read(addr_of_mut!((*(*task).signal).exec_update_lock));
    if f.is_null() {
        // A disappearing files table after exit_signals is ESRCH, not EBADF.
        f = if (*task).flags & PF_EXITING as c_uint != 0 {
            error(-(ESRCH as c_int))
        } else {
            error(-(EBADF as c_int))
        };
    }
    f
}

unsafe fn pidfd_getfd(p: *mut pid, fd: c_int) -> c_int {
    let task = get_pid_task(p, PIDTYPE_PID);
    if task.is_null() {
        return -(ESRCH as c_int);
    }
    let f = __pidfd_fget(task, fd);
    lupos_pid_put_task(task);
    if lupos_pid_is_err(f.cast()) {
        return lupos_pid_ptr_err(f.cast()) as c_int;
    }
    let ret = receive_fd(f, null_mut(), O_CLOEXEC as c_uint);
    fput(f);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn lupos_pid_sys_pidfd_getfd(
    pidfd: c_int,
    fd: c_int,
    flags: c_uint,
) -> c_long {
    if flags != 0 {
        return -(EINVAL as c_long);
    }
    let f = FdScope(fdget(pidfd as c_uint));
    if lupos_pid_fd_empty(f.0) {
        return -(EBADF as c_long);
    }
    let p = pidfd_pid(lupos_pid_fd_file(f.0));
    if lupos_pid_is_err(p.cast()) {
        return lupos_pid_ptr_err(p.cast());
    }
    pidfd_getfd(p, fd) as c_long
}
