/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const RESERVED_PIDS: ::core::ffi::c_int = 300;

pub enum pidfs_attr {}

#[repr(C)]
pub struct upid {
    pub nr: ::core::ffi::c_int,
    pub ns: *mut pid_namespace,
}

#[repr(C)]
pub struct pid {
    pub count: refcount_t,
    pub level: ::core::ffi::c_uint,
    pub lock: spinlock_t,
    pub ino: u64,
    pub pidfs_hash: rhash_head,
    pub stashed: *mut dentry,
    pub attr: *mut pidfs_attr,
    pub tasks: [hlist_head; PIDTYPE_MAX as usize],
    pub inodes: hlist_head,
    pub wait_pidfd: wait_queue_head_t,
    pub rcu: rcu_head,
    pub numbers: [upid; 0],
}

extern "C" {
    pub static mut init_struct_pid: pid;

    pub fn pidfd_pid(file: *const file) -> *mut pid;
    pub fn pidfd_get_pid(fd: ::core::ffi::c_uint, flags: *mut ::core::ffi::c_uint) -> *mut pid;
    pub fn pidfd_get_task(pidfd: ::core::ffi::c_int, flags: *mut ::core::ffi::c_uint) -> *mut task_struct;
    pub fn pidfd_prepare(pid: *mut pid, flags: ::core::ffi::c_uint, ret_file: *mut *mut file) -> ::core::ffi::c_int;
    pub fn do_notify_pidfd(task: *mut task_struct);

    pub fn put_pid(pid: *mut pid);
    pub fn pid_task(pid: *mut pid, ty: pid_type) -> *mut task_struct;
    pub fn get_pid_task(pid: *mut pid, ty: pid_type) -> *mut task_struct;
    pub fn get_task_pid(task: *mut task_struct, ty: pid_type) -> *mut pid;
    pub fn attach_pid(task: *mut task_struct, ty: pid_type);
    pub fn detach_pid(pids: *mut *mut pid, task: *mut task_struct, ty: pid_type);
    pub fn change_pid(pids: *mut *mut pid, task: *mut task_struct, ty: pid_type, pid: *mut pid);
    pub fn exchange_tids(task: *mut task_struct, old: *mut task_struct);
    pub fn transfer_pid(old: *mut task_struct, new: *mut task_struct, ty: pid_type);
    pub fn find_pid_ns(nr: ::core::ffi::c_int, ns: *mut pid_namespace) -> *mut pid;
    pub fn find_vpid(nr: ::core::ffi::c_int) -> *mut pid;
    pub fn find_get_pid(nr: ::core::ffi::c_int) -> *mut pid;
    pub fn find_ge_pid(nr: ::core::ffi::c_int, ns: *mut pid_namespace) -> *mut pid;
    pub fn alloc_pid(ns: *mut pid_namespace, set_tid: *mut pid_t, set_tid_size: usize) -> *mut pid;
    pub fn free_pid(pid: *mut pid);
    pub fn free_pids(pids: *mut *mut pid);
    pub fn disable_pid_allocation(ns: *mut pid_namespace);
    pub fn pid_nr_ns(pid: *mut pid, ns: *mut pid_namespace) -> pid_t;
    pub fn pid_vnr(pid: *mut pid) -> pid_t;
    pub fn __task_pid_nr_ns(task: *mut task_struct, ty: pid_type, ns: *mut pid_namespace) -> pid_t;
}

#[inline]
pub unsafe fn get_pid(p: *mut pid) -> *mut pid {
    if !p.is_null() { refcount_inc(&mut (*p).count); }
    p
}

#[inline]
pub unsafe fn pid_has_task(p: *mut pid, ty: pid_type) -> bool {
    !hlist_empty(&mut (*p).tasks[ty as usize])
}

#[inline]
pub unsafe fn ns_of_pid(p: *mut pid) -> *mut pid_namespace {
    if !p.is_null() { (*p).numbers[(*p).level as usize].ns } else { core::ptr::null_mut() }
}

#[inline]
pub unsafe fn is_child_reaper(p: *mut pid) -> bool {
    (*p).numbers[(*p).level as usize].nr == 1
}

#[inline]
pub unsafe fn pid_nr(p: *mut pid) -> pid_t {
    if !p.is_null() { (*p).numbers[0].nr } else { 0 }
}

#[macro_export]
macro_rules! do_each_pid_task { ($pid:expr, $ty:expr, $task:ident, $body:block) => {{ if !$pid.is_null() { hlist_for_each_entry_rcu!($task, &mut (*$pid).tasks[$ty as usize], pid_links[$ty as usize], $body); } }}; }
#[macro_export]
macro_rules! while_each_pid_task { ($pid:expr, $ty:expr, $task:ident) => {{ if $ty == PIDTYPE_PID { break; } }}; }
#[macro_export]
macro_rules! do_each_pid_thread { ($pid:expr, $ty:expr, $task:ident, $body:block) => {{ do_each_pid_task!($pid, $ty, $task, { let mut tg___ = $task; for_each_thread!(tg___, $task, $body); }); }}; }
#[macro_export]
macro_rules! while_each_pid_thread { ($pid:expr, $ty:expr, $task:ident) => {{ $task = tg___; while_each_pid_task!($pid, $ty, $task); }}; }

#[inline] pub unsafe fn task_pid(task: *mut task_struct) -> *mut pid { (*task).thread_pid }
#[inline] pub unsafe fn task_pid_nr(tsk: *mut task_struct) -> pid_t { (*tsk).pid }
#[inline] pub unsafe fn task_pid_nr_ns(tsk: *mut task_struct, ns: *mut pid_namespace) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_PID, ns) }
#[inline] pub unsafe fn task_pid_vnr(tsk: *mut task_struct) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_PID, core::ptr::null_mut()) }
#[inline] pub unsafe fn task_tgid_nr(tsk: *mut task_struct) -> pid_t { (*tsk).tgid }
#[inline] pub unsafe fn pid_alive(p: *const task_struct) -> ::core::ffi::c_int { (!(*p).thread_pid.is_null()) as ::core::ffi::c_int }
#[inline] pub unsafe fn task_pgrp_nr_ns(tsk: *mut task_struct, ns: *mut pid_namespace) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_PGID, ns) }
#[inline] pub unsafe fn task_pgrp_vnr(tsk: *mut task_struct) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_PGID, core::ptr::null_mut()) }
#[inline] pub unsafe fn task_session_nr_ns(tsk: *mut task_struct, ns: *mut pid_namespace) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_SID, ns) }
#[inline] pub unsafe fn task_session_vnr(tsk: *mut task_struct) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_SID, core::ptr::null_mut()) }
#[inline] pub unsafe fn task_tgid_nr_ns(tsk: *mut task_struct, ns: *mut pid_namespace) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_TGID, ns) }
#[inline] pub unsafe fn task_tgid_vnr(tsk: *mut task_struct) -> pid_t { __task_pid_nr_ns(tsk, PIDTYPE_TGID, core::ptr::null_mut()) }
#[inline] pub unsafe fn task_ppid_nr_ns(tsk: *const task_struct, ns: *mut pid_namespace) -> pid_t { let mut p = 0; rcu_read_lock(); if pid_alive(tsk) { p = task_tgid_nr_ns(rcu_dereference!((*tsk).real_parent), ns); } rcu_read_unlock(); p }
#[inline] pub unsafe fn task_ppid_vnr(tsk: *const task_struct) -> pid_t { task_ppid_nr_ns(tsk, core::ptr::null_mut()) }
#[inline] pub unsafe fn task_ppid_nr(tsk: *const task_struct) -> pid_t { task_ppid_nr_ns(tsk, &mut init_pid_ns) }
#[inline] pub unsafe fn task_pgrp_nr(tsk: *mut task_struct) -> pid_t { task_pgrp_nr_ns(tsk, &mut init_pid_ns) }
#[inline] pub unsafe fn is_global_init(tsk: *mut task_struct) -> ::core::ffi::c_int { (task_tgid_nr(tsk) == 1) as ::core::ffi::c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
