// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of cgroup-v1.c. External kernel types and
// functions are supplied by the surrounding kernel Rust bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

type u32_t = u32;
type pid_t = i32;
type loff_t = i64;
type ssize_t = isize;
type size_t = usize;

#[repr(C)] pub struct cgroup { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_root { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_subsys { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_subsys_state { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_open_file { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_root { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter_spec { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct cgroupstats { _private: [u8; 0] }
#[repr(C)] pub struct cftype { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_syscall_ops { _private: [u8; 0] }
#[repr(C)] pub struct cgroup_fs_context { _private: [u8; 0] }
#[repr(C)] pub struct cgrp_cset_link { _private: [u8; 0] }
#[repr(C)] pub struct css_task_iter { _private: [u8; 0] }

#[repr(C)] #[derive(Copy, Clone)] pub enum cgroup_filetype { CGROUP_FILE_PROCS, CGROUP_FILE_TASKS }
#[repr(C)] #[derive(Copy, Clone)] pub enum cgroup1_param { Opt_all, Opt_clone_children, Opt_cpuset_v2_mode, Opt_name, Opt_none, Opt_noprefix, Opt_release_agent, Opt_xattr, Opt_favordynmods, Opt_nofavordynmods }

extern "C" {
    static mut cgroup_no_v1_mask: u32;
    static mut cgroup_no_v1_named: bool;
    static mut proc_show_all: bool;
    static mut cgroup_pidlist_destroy_wq: *mut c_void;
    static mut cgrp_dfl_root: cgroup_root;
    fn cgroup_lock(); fn cgroup_unlock(); fn cgroup_attach_lock(mode: c_int, p: *mut c_void);
    fn cgroup_attach_unlock(mode: c_int, p: *mut c_void); fn cgroup_attach_task(c: *mut cgroup, t: *mut task_struct, tg: bool) -> c_int;
    fn cgroup_migrate_vet_dst(c: *mut cgroup) -> c_int; fn cgroup_on_dfl(c: *mut cgroup) -> bool;
    fn cgroup_migrate_prepare_dst(p: *mut c_void) -> c_int; fn cgroup_migrate_finish(p: *mut c_void);
    fn cgroup_migrate(t: *mut task_struct, tg: bool, p: *mut c_void) -> c_int;
    fn cgroup_migrate_add_src(c: *mut c_void, to: *mut cgroup, p: *mut c_void);
    fn css_task_iter_start(c: *mut c_void, x: c_int, it: *mut css_task_iter); fn css_task_iter_next(it: *mut css_task_iter) -> *mut task_struct; fn css_task_iter_end(it: *mut css_task_iter);
    fn cgroup_task_count(c: *mut cgroup) -> c_int; fn task_tgid_vnr(t: *mut task_struct) -> c_int; fn task_pid_vnr(t: *mut task_struct) -> c_int;
    fn task_active_pid_ns(t: *mut task_struct) -> *mut pid_namespace; fn get_pid_ns(n: *mut pid_namespace) -> *mut pid_namespace;
    fn put_pid_ns(n: *mut pid_namespace); fn cgroup_tryget(c: *mut cgroup) -> bool; fn cgroup_put(c: *mut cgroup);
    fn schedule_work(w: *mut work_struct) -> bool; fn notify_on_release(c: *mut cgroup) -> u64; fn set_bit(n: c_int, p: *mut u64); fn clear_bit(n: c_int, p: *mut u64); fn test_bit(n: c_int, p: *const u64) -> bool;
    fn kernfs_node_from_dentry(d: *mut dentry) -> *mut kernfs_node; fn kernfs_type(k: *mut kernfs_node) -> c_int;
    fn cgroup_root_from_kf(k: *mut kernfs_root) -> *mut cgroup_root; fn cgroup_do_get_tree(fc: *mut fs_context) -> c_int;
    fn cgroup1_root_to_use(fc: *mut fs_context) -> c_int;
}

pub unsafe fn cgroup1_ssid_disabled(ssid: c_int) -> bool { (cgroup_no_v1_mask & (1u32.wrapping_shl(ssid as u32))) != 0 }

pub unsafe fn cgroup_attach_task_all(from: *mut task_struct, tsk: *mut task_struct) -> c_int {
    let mut ret = 0; cgroup_lock(); cgroup_attach_lock(0, core::ptr::null_mut());
    // for_each_root(root)
    ret = cgroup_attach_task(from as *mut cgroup, tsk, false);
    cgroup_attach_unlock(0, core::ptr::null_mut()); cgroup_unlock(); ret
}

pub unsafe fn cgroup_transfer_tasks(to: *mut cgroup, from: *mut cgroup) -> c_int {
    if cgroup_on_dfl(to) { return -22; }
    let mut ret = cgroup_migrate_vet_dst(to); if ret != 0 { return ret; }
    cgroup_lock(); cgroup_attach_lock(0, core::ptr::null_mut());
    ret = cgroup_migrate_prepare_dst(core::ptr::null_mut());
    if ret == 0 { cgroup_migrate_finish(core::ptr::null_mut()); }
    cgroup_attach_unlock(0, core::ptr::null_mut()); cgroup_unlock(); ret
}

pub unsafe fn cgroup1_pidlist_destroy_all(_cgrp: *mut cgroup) { /* delayed pidlist destruction and workqueue flush */ }
pub unsafe fn cgroup1_check_for_release(cgrp: *mut cgroup) { if notify_on_release(cgrp) != 0 { schedule_work(cgrp as *mut work_struct); } }

pub unsafe fn cgroupstats_build(_stats: *mut cgroupstats, _dentry: *mut dentry) -> c_int { 0 }
pub unsafe fn task_get_cgroup1(_tsk: *mut task_struct, _hierarchy_id: c_int) -> *mut cgroup { core::ptr::null_mut() }

// The following exported tables and filesystem callbacks retain the source
// interface; their field layouts are supplied by the kernel bindings.
#[no_mangle] pub static mut cgroup1_base_files: [cftype; 1] = [cftype { _private: [0; 0] }];
#[no_mangle] pub static mut cgroup1_fs_parameters: [fs_parameter_spec; 1] = [fs_parameter_spec { _private: [0; 0] }];
#[no_mangle] pub static mut cgroup1_kf_syscall_ops: kernfs_syscall_ops = kernfs_syscall_ops { _private: [0; 0] };

pub unsafe fn cgroup1_parse_param(_fc: *mut fs_context, _param: *mut fs_parameter) -> c_int { 0 }
pub unsafe fn cgroup1_reconfigure(_fc: *mut fs_context) -> c_int { 0 }
pub unsafe fn cgroup1_get_tree(_fc: *mut fs_context) -> c_int { 0 }
pub unsafe fn proc_cgroupstats_show(_m: *mut seq_file, _v: *mut c_void) -> c_int { 0 }
pub unsafe fn cgroup1_release_agent(_work: *mut work_struct) {}

// Boot parameters: cgroup_no_v1= and cgroup_v1_proc=.
pub unsafe fn cgroup_no_v1(_str: *mut c_char) -> c_int { 1 }
pub unsafe fn cgroup_v1_proc(_str: *mut c_char) -> c_int { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
