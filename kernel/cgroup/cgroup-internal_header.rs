/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/cgroup.h, linux/kernfs.h, linux/workqueue.h, linux/list.h,
// linux/refcount.h, and linux/fs_parser.h.

pub const TRACE_CGROUP_PATH_LEN: usize = 1024;
extern "C" {
    pub static mut trace_cgroup_path_lock: spinlock_t;
    pub static mut trace_cgroup_path: [::core::ffi::c_char; TRACE_CGROUP_PATH_LEN];
    pub fn enable_debug_cgroup();
}

/*
 * cgroup_path() takes a spin lock. It is good practice not to take
 * spin locks within trace point handlers, as they are mostly hidden
 * from normal view. As cgroup_path() can take the kernfs_rename_lock
 * spin lock, it is best to not call that function from the trace event
 * handler.
 *
 * Note: trace_cgroup_##type##_enabled() is a static branch that will only
 *       be set when the trace event is enabled.
 */
// The trace event identifier is supplied by the caller's generated bindings.

/* The cgroup filesystem superblock creation/mount context. */
#[repr(C)]
pub struct cgroup_fs_context {
    pub kfc: kernfs_fs_context,
    pub root: *mut cgroup_root,
    pub ns: *mut cgroup_namespace,
    pub flags: ::core::ffi::c_uint,
    pub cpuset_clone_children: bool,
    pub none: bool,
    pub all_ss: bool,
    pub subsys_mask: u32,
    pub name: *mut ::core::ffi::c_char,
    pub release_agent: *mut ::core::ffi::c_char,
}

#[inline]
pub unsafe fn cgroup_fc2context(fc: *mut fs_context) -> *mut cgroup_fs_context {
    let kfc = (*fc).fs_private as *mut kernfs_fs_context;
    container_of!(kfc, cgroup_fs_context, kfc)
}

pub struct cgroup_pidlist;

#[repr(C)]
pub struct cgroup_file_ctx {
    pub ns: *mut cgroup_namespace,
    pub psi: cgroup_file_ctx_psi,
    pub procs: cgroup_file_ctx_procs,
    pub procs1: cgroup_file_ctx_procs1,
    pub peak: cgroup_of_peak,
}
#[repr(C)] pub struct cgroup_file_ctx_psi { pub trigger: *mut ::core::ffi::c_void }
#[repr(C)] pub struct cgroup_file_ctx_procs { pub started: bool, pub iter: css_task_iter }
#[repr(C)] pub struct cgroup_file_ctx_procs1 { pub pidlist: *mut cgroup_pidlist }

/*
 * A cgroup can be associated with multiple css_sets as different tasks may
 * belong to different cgroups on different hierarchies. In the other
 * direction, a css_set is naturally associated with multiple cgroups.
 * This M:N relationship is represented by the following link structure
 * which exists for each association and allows traversing the associations
 * from both sides.
 */
#[repr(C)]
pub struct cgrp_cset_link {
    pub cgrp: *mut cgroup,
    pub cset: *mut css_set,
    pub cset_link: list_head,
    pub cgrp_link: list_head,
}

/* used to track tasks and csets during migration */
#[repr(C)]
pub struct cgroup_taskset {
    pub src_csets: list_head,
    pub dst_csets: list_head,
    pub nr_tasks: ::core::ffi::c_int,
    pub ssid: ::core::ffi::c_int,
    pub csets: *mut list_head,
    pub cur_cset: *mut css_set,
    pub cur_task: *mut task_struct,
}

/* migration context also tracks preloading */
#[repr(C)]
pub struct cgroup_mgctx {
    pub preloaded_src_csets: list_head,
    pub preloaded_dst_csets: list_head,
    pub tset: cgroup_taskset,
    pub ss_mask: u32,
}

#[macro_export]
macro_rules! CGROUP_TASKSET_INIT { ($tset:expr) => { cgroup_taskset {
    src_csets: LIST_HEAD_INIT!($tset.src_csets), dst_csets: LIST_HEAD_INIT!($tset.dst_csets),
    nr_tasks: 0, ssid: 0, csets: &mut $tset.src_csets, cur_cset: ::core::ptr::null_mut(), cur_task: ::core::ptr::null_mut()
} }; }
#[macro_export]
macro_rules! CGROUP_MGCTX_INIT { ($name:expr) => { cgroup_mgctx {
    preloaded_src_csets: LIST_HEAD_INIT!($name.preloaded_src_csets), preloaded_dst_csets: LIST_HEAD_INIT!($name.preloaded_dst_csets),
    tset: CGROUP_TASKSET_INIT!($name.tset), ss_mask: 0
} }; }
#[macro_export]
macro_rules! DEFINE_CGROUP_MGCTX { ($name:ident) => { let mut $name: cgroup_mgctx = CGROUP_MGCTX_INIT!($name); }; }

extern "C" {
    pub static mut cgroup_subsys: *mut *mut cgroup_subsys;
    pub static mut cgroup_roots: list_head;
    pub static mut cgrp_dfl_visible: bool;
}

#[inline]
pub unsafe fn notify_on_release(cgrp: *const cgroup) -> bool {
    test_bit!(CGRP_NOTIFY_ON_RELEASE, &(*cgrp).flags)
}

extern "C" {
    pub fn put_css_set_locked(cset: *mut css_set);
}
#[inline]
pub unsafe fn put_css_set(cset: *mut css_set) {
    let mut flags: ::core::ffi::c_ulong = 0;
    if refcount_dec_not_one!(&mut (*cset).refcount) { return; }
    spin_lock_irqsave!(&mut css_set_lock, flags);
    put_css_set_locked(cset);
    spin_unlock_irqrestore!(&mut css_set_lock, flags);
}
#[inline]
pub unsafe fn get_css_set(cset: *mut css_set) { refcount_inc!(&mut (*cset).refcount); }

extern "C" {
    pub fn cgroup_ssid_enabled(ssid: ::core::ffi::c_int) -> bool;
    pub fn cgroup_root_from_kf(kf_root: *mut kernfs_root) -> *mut cgroup_root;
    pub fn task_cgroup_from_root(task: *mut task_struct, root: *mut cgroup_root) -> *mut cgroup;
    pub fn cgroup_kn_lock_live(kn: *mut kernfs_node, drain_offline: bool) -> *mut cgroup;
    pub fn cgroup_kn_unlock(kn: *mut kernfs_node);
    pub fn cgroup_path_ns_locked(cgrp: *mut cgroup, buf: *mut ::core::ffi::c_char, buflen: usize, ns: *mut cgroup_namespace) -> ::core::ffi::c_int;
    pub fn cgroup_favor_dynmods(root: *mut cgroup_root, favor: bool);
    pub fn cgroup_free_root(root: *mut cgroup_root);
    pub fn init_cgroup_root(ctx: *mut cgroup_fs_context);
    pub fn cgroup_setup_root(root: *mut cgroup_root, ss_mask: u32) -> ::core::ffi::c_int;
    pub fn rebind_subsystems(dst_root: *mut cgroup_root, ss_mask: u32) -> ::core::ffi::c_int;
    pub fn cgroup_do_get_tree(fc: *mut fs_context) -> ::core::ffi::c_int;
    pub fn cgroup_migrate_vet_dst(dst_cgrp: *mut cgroup) -> ::core::ffi::c_int;
    pub fn cgroup_migrate_finish(mgctx: *mut cgroup_mgctx);
    pub fn cgroup_migrate_add_src(src_cset: *mut css_set, dst_cgrp: *mut cgroup, mgctx: *mut cgroup_mgctx);
    pub fn cgroup_migrate_prepare_dst(mgctx: *mut cgroup_mgctx) -> ::core::ffi::c_int;
    pub fn cgroup_migrate(leader: *mut task_struct, threadgroup: bool, mgctx: *mut cgroup_mgctx) -> ::core::ffi::c_int;
    pub fn cgroup_attach_task(dst_cgrp: *mut cgroup, leader: *mut task_struct, threadgroup: bool) -> ::core::ffi::c_int;
    pub fn cgroup_attach_lock(lock_mode: cgroup_attach_lock_mode, tsk: *mut task_struct);
    pub fn cgroup_attach_unlock(lock_mode: cgroup_attach_lock_mode, tsk: *mut task_struct);
    pub fn cgroup_procs_write_start(buf: *mut ::core::ffi::c_char, threadgroup: bool, lock_mode: *mut cgroup_attach_lock_mode) -> *mut task_struct;
    pub fn cgroup_procs_write_finish(task: *mut task_struct, lock_mode: cgroup_attach_lock_mode);
    pub fn cgroup_lock_and_drain_offline(cgrp: *mut cgroup);
    pub fn cgroup_mkdir(parent_kn: *mut kernfs_node, name: *const ::core::ffi::c_char, mode: umode_t) -> ::core::ffi::c_int;
    pub fn cgroup_rmdir(kn: *mut kernfs_node) -> ::core::ffi::c_int;
    pub fn cgroup_show_path(sf: *mut seq_file, kf_node: *mut kernfs_node, kf_root: *mut kernfs_root) -> ::core::ffi::c_int;
    pub fn __cgroup_task_count(cgrp: *const cgroup) -> ::core::ffi::c_int;
    pub fn cgroup_task_count(cgrp: *const cgroup) -> ::core::ffi::c_int;
    pub fn css_rstat_init(css: *mut cgroup_subsys_state) -> ::core::ffi::c_int;
    pub fn css_rstat_exit(css: *mut cgroup_subsys_state);
    pub fn ss_rstat_init(ss: *mut cgroup_subsys) -> ::core::ffi::c_int;
    pub fn cgroup_base_stat_cputime_show(seq: *mut seq_file);
    pub static cgroupns_operations: proc_ns_operations;
    pub static mut cgroup1_base_files: cftype;
    pub static mut cgroup1_kf_syscall_ops: kernfs_syscall_ops;
    pub static cgroup1_fs_parameters: fs_parameter_spec;
    pub fn proc_cgroupstats_show(m: *mut seq_file, v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn cgroup1_ssid_disabled(ssid: ::core::ffi::c_int) -> bool;
    pub fn cgroup1_pidlist_destroy_all(cgrp: *mut cgroup);
    pub fn cgroup1_release_agent(work: *mut work_struct);
    pub fn cgroup1_check_for_release(cgrp: *mut cgroup);
    pub fn cgroup1_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> ::core::ffi::c_int;
    pub fn cgroup1_get_tree(fc: *mut fs_context) -> ::core::ffi::c_int;
    pub fn cgroup1_reconfigure(ctx: *mut fs_context) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
