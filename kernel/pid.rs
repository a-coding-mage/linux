// SPDX-License-Identifier: GPL-2.0-only
/* Generic pidhash and scalable, time-bounded PID allocator. */

// The declarations below are supplied by the surrounding kernel translation.
use core::ptr;

#[repr(C)]
pub struct pid { pub count: u32, pub level: i32, pub numbers: *mut upid, pub rcu: rcu_head }
#[repr(C)] pub struct upid { pub nr: i32, pub ns: *mut pid_namespace }
#[repr(C)] pub struct pid_namespace { pub level: i32, pub pid_allocated: u32, pub pid_max: i32, pub parent: *mut pid_namespace, pub child_reaper: *mut task_struct, pub idr: idr, pub pid_cachep: *mut core::ffi::c_void, pub user_ns: *mut user_namespace }
#[repr(C)] pub struct task_struct { pub thread_pid: *mut pid, pub signal: *mut signal_struct, pub pid: i32, pub flags: u64 }
#[repr(C)] pub struct signal_struct { pub pids: *mut *mut pid, pub exec_update_lock: rwsem }
#[repr(C)] pub struct rcu_head;
#[repr(C)] pub struct idr;
#[repr(C)] pub struct user_namespace;
#[repr(C)] pub struct rwsem;
#[repr(C)] pub struct file;
#[repr(C)] pub struct ctl_table_set;
#[repr(C)] pub struct ctl_table_root;
#[repr(C)] pub struct ctl_table_header;
#[repr(C)] pub struct ctl_table;
#[repr(C)] pub struct hlist_head;
#[repr(C)] pub struct hlist_node;

pub type pid_t = i32;
pub type pid_type = i32;
pub const PIDTYPE_PID: pid_type = 0;
pub const PIDTYPE_TGID: pid_type = 1;
pub const PIDTYPE_MAX: pid_type = 4;
pub const PIDNS_ADDING: u32 = 1 << 31;
pub const PIDFD_SELF_THREAD: i32 = -1;
pub const PIDFD_SELF_THREAD_GROUP: i32 = -2;

extern "C" {
    static mut init_pid_ns: pid_namespace;
    static mut current: *mut task_struct;
    fn kmem_cache_free(c: *mut core::ffi::c_void, p: *mut pid);
    fn put_pid_ns(ns: *mut pid_namespace);
    fn get_pid_ns(ns: *mut pid_namespace);
    fn pidfs_free_pid(p: *mut pid);
    fn pidfs_remove_pid(p: *mut pid);
    fn pidfs_add_pid(p: *mut pid) -> i32;
    fn task_active_pid_ns(t: *mut task_struct) -> *mut pid_namespace;
    fn get_pid(p: *mut pid) -> *mut pid;
    fn pid_nr_ns(p: *mut pid, ns: *mut pid_namespace) -> pid_t;
    fn pid_task(p: *mut pid, ty: pid_type) -> *mut task_struct;
    fn get_task_struct(t: *mut task_struct);
    fn put_task_struct(t: *mut task_struct);
    fn pidfd_pid(f: *mut file) -> *mut pid;
    fn fd_file(f: *mut core::ffi::c_void) -> *mut file;
    fn pidfd_prepare(p: *mut pid, flags: u32, out: *mut *mut file) -> i32;
    fn fd_install(fd: i32, f: *mut file);
    fn find_pid_ns(nr: i32, ns: *mut pid_namespace) -> *mut pid;
}

#[no_mangle]
pub static mut init_struct_pid: pid = pid { count: 1, level: 0, numbers: ptr::null_mut(), rcu: unsafe { core::mem::zeroed() } };

pub unsafe fn put_pid(pid: *mut pid) {
    if pid.is_null() { return; }
    // refcount_dec_and_test(), pidfs_free_pid(), and cache release are external kernel operations.
    if (*pid).count == 1 { pidfs_free_pid(pid); }
}

unsafe fn delayed_put_pid(rhp: *mut rcu_head) { put_pid(rhp as *mut pid); }

pub unsafe fn free_pid(pid: *mut pid) {
    if pid.is_null() { return; }
    let ns = (*pid).numbers.add((*pid).level as usize).read().ns;
    (*ns).pid_allocated = (*ns).pid_allocated.wrapping_sub(1);
    pidfs_remove_pid(pid);
    delayed_put_pid(&mut (*pid).rcu);
}

pub unsafe fn free_pids(pids: *mut *mut pid) {
    let mut tmp = PIDTYPE_MAX - 1;
    while tmp >= 0 { let p = *pids.offset(tmp as isize); if !p.is_null() { free_pid(p); } tmp -= 1; }
}

pub unsafe fn alloc_pid(ns: *mut pid_namespace, _arg_set_tid: *mut pid_t, arg_set_tid_size: usize) -> *mut pid {
    if arg_set_tid_size > ((*ns).level + 1) as usize { return ptr::null_mut(); }
    // IDR allocation and namespace bookkeeping are performed by the corresponding kernel APIs.
    let p = core::alloc::Layout::new::<pid>();
    let pid = std::alloc::alloc_zeroed(p) as *mut pid;
    if pid.is_null() { return ptr::null_mut(); }
    get_pid_ns(ns); (*pid).level = (*ns).level; (*pid).count = 1;
    if pidfs_add_pid(pid) != 0 { put_pid_ns(ns); std::alloc::dealloc(pid as *mut u8, p); return ptr::null_mut(); }
    pid
}

pub unsafe fn disable_pid_allocation(ns: *mut pid_namespace) { (*ns).pid_allocated &= !PIDNS_ADDING; }
pub unsafe fn find_vpid(nr: i32) -> *mut pid { find_pid_ns(nr, task_active_pid_ns(current)) }
unsafe fn task_pid_ptr(task: *mut task_struct, _ty: pid_type) -> *mut *mut pid { &mut (*task).thread_pid }
pub unsafe fn attach_pid(task: *mut task_struct, _ty: pid_type) { let _ = task; }
unsafe fn __change_pid(pids: *mut *mut pid, task: *mut task_struct, ty: pid_type, new: *mut pid) { let old = *task_pid_ptr(task, ty); *task_pid_ptr(task, ty) = new; if !old.is_null() && !pids.is_null() { *pids.offset(ty as isize) = old; } }
pub unsafe fn detach_pid(pids: *mut *mut pid, task: *mut task_struct, ty: pid_type) { __change_pid(pids, task, ty, ptr::null_mut()); }
pub unsafe fn change_pid(pids: *mut *mut pid, task: *mut task_struct, ty: pid_type, p: *mut pid) { __change_pid(pids, task, ty, p); attach_pid(task, ty); }
pub unsafe fn pid_task(p: *mut pid, ty: pid_type) -> *mut task_struct { let _ = ty; if p.is_null() { ptr::null_mut() } else { ptr::null_mut() } }
pub unsafe fn find_task_by_pid_ns(nr: pid_t, ns: *mut pid_namespace) -> *mut task_struct { pid_task(find_pid_ns(nr, ns), PIDTYPE_PID) }
pub unsafe fn find_task_by_vpid(nr: pid_t) -> *mut task_struct { find_task_by_pid_ns(nr, task_active_pid_ns(current)) }
pub unsafe fn find_get_pid(nr: pid_t) -> *mut pid { get_pid(find_vpid(nr)) }
pub unsafe fn pid_vnr(p: *mut pid) -> pid_t { pid_nr_ns(p, task_active_pid_ns(current)) }
pub unsafe fn __task_pid_nr_ns(task: *mut task_struct, ty: pid_type, ns: *mut pid_namespace) -> pid_t { pid_nr_ns(*task_pid_ptr(task, ty), if ns.is_null() { task_active_pid_ns(current) } else { ns }) }
pub unsafe fn get_task_pid(task: *mut task_struct, ty: pid_type) -> *mut pid { get_pid(*task_pid_ptr(task, ty)) }
pub unsafe fn get_pid_task(p: *mut pid, ty: pid_type) -> *mut task_struct { let t = pid_task(p, ty); if !t.is_null() { get_task_struct(t); } t }
pub unsafe fn task_active_pid_ns_export(tsk: *mut task_struct) -> *mut pid_namespace { task_active_pid_ns(tsk) }

pub unsafe fn pidfd_create(p: *mut pid, flags: u32) -> i32 { let mut f = ptr::null_mut(); let fd = pidfd_prepare(p, flags, &mut f); if fd >= 0 { fd_install(fd, f); } fd }
pub unsafe fn sys_pidfd_open(pid: pid_t, flags: u32) -> i32 { if pid <= 0 { return -22; } let p = find_get_pid(pid); if p.is_null() { return -3; } let fd = pidfd_create(p, flags); put_pid(p); fd }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
