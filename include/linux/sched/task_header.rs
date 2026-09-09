/* SPDX-License-Identifier: GPL-2.0 */

/* Interface between the scheduler and task lifetime functionality. */

/* C forward declarations and included dependencies are supplied externally. */

pub const CLONE_LEGACY_FLAGS: u64 = 0xffff_ffffu64;

#[repr(C)]
pub struct kernel_clone_args {
    pub flags: u64,
    pub pidfd: *mut i32,
    pub child_tid: *mut i32,
    pub parent_tid: *mut i32,
    pub name: *const i8,
    pub exit_signal: i32,
    /* C bitfields; represented as their underlying u32 storage. */
    pub kthread: u32,
    pub io_thread: u32,
    pub user_worker: u32,
    pub no_files: u32,
    pub umh: u32,
    pub stack: usize,
    pub stack_size: usize,
    pub tls: usize,
    pub set_tid: *mut i32,
    pub set_tid_size: usize,
    pub cgroup: i32,
    pub idle: i32,
    pub func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub fn_arg: *mut core::ffi::c_void,
    pub cgrp: *mut cgroup,
    pub cset: *mut css_set,
    pub kill_seq: u32,
}

extern "C" {
    pub static mut tasklist_lock: rwlock_t;
    pub static mut mmlist_lock: spinlock_t;
    pub static mut init_thread_union: thread_union;
    pub static mut init_task: task_struct;

    pub fn lockdep_tasklist_lock_is_held() -> i32;
    pub fn schedule_tail(prev: *mut task_struct);
    pub fn init_idle(idle: *mut task_struct, cpu: i32);
    pub fn sched_fork(clone_flags: u64, p: *mut task_struct) -> i32;
    pub fn sched_cgroup_fork(p: *mut task_struct, kargs: *mut kernel_clone_args) -> i32;
    pub fn sched_cancel_fork(p: *mut task_struct);
    pub fn sched_post_fork(p: *mut task_struct);
    pub fn sched_dead(p: *mut task_struct);
    pub fn do_task_dead() -> !;
    pub fn make_task_dead(signr: i32) -> !;
    pub fn mm_cache_init();
    pub fn proc_caches_init();
    pub fn fork_init();
    pub fn release_task(p: *mut task_struct);
    pub fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32;
    pub fn flush_thread();
    pub fn exit_thread(tsk: *mut task_struct);
    pub fn do_group_exit(code: i32) -> !;
    pub fn exit_files(p: *mut task_struct);
    pub fn exit_itimers(p: *mut task_struct);
    pub fn kernel_clone(kargs: *mut kernel_clone_args) -> i32;
    pub fn copy_process(pid: *mut pid, trace: i32, node: i32, args: *mut kernel_clone_args) -> *mut task_struct;
    pub fn create_io_thread(func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, arg: *mut core::ffi::c_void, node: i32) -> *mut task_struct;
    pub fn fork_idle(cpu: i32) -> *mut task_struct;
    pub fn kernel_thread(func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, arg: *mut core::ffi::c_void, name: *const i8, flags: usize) -> i32;
    pub fn user_mode_thread(func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, arg: *mut core::ffi::c_void, flags: usize) -> i32;
    pub fn kernel_wait4(pid: i32, stat: *mut i32, options: i32, ru: *mut rusage) -> isize;
    pub fn kernel_wait(pid: i32, stat: *mut i32) -> i32;
    pub fn free_task(tsk: *mut task_struct);
    pub fn sched_exec();
    pub fn __put_task_struct(t: *mut task_struct);
    pub fn __put_task_struct_rcu_cb(rhp: *mut rcu_head);
    pub fn put_task_struct_rcu_user(task: *mut task_struct);
    pub fn release_thread(dead_task: *mut task_struct);
}

pub unsafe fn get_task_struct(t: *mut task_struct) -> *mut task_struct {
    refcount_inc(&mut (*t).usage);
    t
}

pub unsafe fn tryget_task_struct(t: *mut task_struct) -> *mut task_struct {
    if refcount_inc_not_zero(&mut (*t).usage) { t } else { core::ptr::null_mut() }
}

pub unsafe fn put_task_struct(t: *mut task_struct) {
    if !refcount_dec_and_test(&mut (*t).usage) { return; }
    call_rcu(&mut (*t).rcu, __put_task_struct_rcu_cb);
}

pub unsafe fn put_task_struct_many(t: *mut task_struct, nr: i32) {
    if refcount_sub_and_test(nr, &mut (*t).usage) { __put_task_struct(t); }
}

/* DEFINE_FREE(put_task, ...): cleanup integration supplied by the Rust runtime. */

#[cfg(not(CONFIG_HAVE_ARCH_THREAD_STRUCT_WHITELIST))]
pub unsafe fn arch_thread_struct_whitelist(offset: *mut usize, size: *mut usize) {
    *offset = 0;
    *size = arch_task_struct_size() - core::mem::offset_of!(task_struct, thread);
}

#[cfg(CONFIG_VMAP_STACK)]
pub unsafe fn task_stack_vm_area(t: *const task_struct) -> *mut vm_struct { (*t).stack_vm_area }

#[cfg(not(CONFIG_VMAP_STACK))]
pub unsafe fn task_stack_vm_area(_t: *const task_struct) -> *mut vm_struct { core::ptr::null_mut() }

pub unsafe fn task_lock(p: *mut task_struct) { spin_lock(&mut (*p).alloc_lock); }
pub unsafe fn task_unlock(p: *mut task_struct) { spin_unlock(&mut (*p).alloc_lock); }

/* Lock-guard declarations and lock annotations are provided by the surrounding kernel bindings. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
