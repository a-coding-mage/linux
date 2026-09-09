/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of linux/cgroup.h. Includes and build-system conditions are
 * intentionally represented as comments; referenced kernel types/functions
 * are supplied by other translated headers. */

pub const CGROUP_WEIGHT_MIN: u32 = 1;
pub const CGROUP_WEIGHT_DFL: u32 = 100;
pub const CGROUP_WEIGHT_MAX: u32 = 10000;

#[cfg(feature = "CONFIG_CGROUPS")]
pub const CGROUP_HAS_SUBSYS_CONFIG: bool = cfg!(feature = "CGROUP_SUBSYS_COUNT_GT_0");

#[repr(u32)]
pub enum CssTaskIterFlags {
    CSS_TASK_ITER_PROCS = 1u32 << 0,
    CSS_TASK_ITER_THREADED = 1u32 << 1,
    CSS_TASK_ITER_WITH_DEAD = 1u32 << 2,
    CSS_TASK_ITER_SKIPPED = 1u32 << 16,
}

#[repr(C)]
pub struct CssTaskIter {
    pub ss: *mut CgroupSubsys,
    pub flags: u32,
    pub cset_pos: *mut ListHead,
    pub cset_head: *mut ListHead,
    pub tcset_pos: *mut ListHead,
    pub tcset_head: *mut ListHead,
    pub task_pos: *mut ListHead,
    pub cur_tasks_head: *mut ListHead,
    pub cur_cset: *mut CssSet,
    pub cur_dcset: *mut CssSet,
    pub cur_task: *mut TaskStruct,
    pub iters_node: ListHead,
}

#[repr(u32)]
pub enum CgroupLifetimeEvents { CGROUP_LIFETIME_ONLINE, CGROUP_LIFETIME_OFFLINE }
#[repr(u32)]
pub enum CgroupTaskEvents { CGROUP_TASK_MIGRATING, CGROUP_TASK_MIGRATED, CGROUP_TASK_MIGRATE_CANCELED }

#[repr(C)]
pub struct CgroupTaskMigrateCtx {
    pub task: *mut TaskStruct,
    pub src_dcgrp: *mut Cgroup,
    pub dst_dcgrp: *mut Cgroup,
}

pub enum FileSystemType {}
pub enum CgroupRoot {}
pub enum Mutex {}
pub enum Spinlock {}
pub enum BlockingNotifierHead {}
pub enum CgroupSubsys {}
pub enum ListHead {}
pub enum CssSet {}
pub enum TaskStruct {}
pub enum CgroupSubsysState {}
pub enum Dentry {}
pub enum Cftype {}
pub enum CgroupFile {}
pub enum CgroupStats {}
pub enum SeqFile {}
pub enum PidNamespace {}
pub enum Pid {}
pub enum KernelCloneArgs {}
pub enum KernfsOpenFile {}
pub enum SockCgroupData {}
pub enum CgroupOfPeak {}
pub type InoT = usize;
pub type S64 = i64;
pub type U64 = u64;
pub enum CpuUsageStat {}

extern "C" {
    pub static mut cgroup_fs_type: FileSystemType;
    pub static mut cgrp_dfl_root: CgroupRoot;
    pub static mut init_css_set: CssSet;
    pub static mut cgroup_mutex: Mutex;
    pub static mut css_set_lock: Spinlock;
    pub static mut cgroup_lifetime_notifier: BlockingNotifierHead;
    pub static mut cgroup_task_notifier: BlockingNotifierHead;

    pub fn cgroup_on_dfl(cgrp: *const Cgroup) -> bool;
    pub fn css_has_online_children(css: *mut CgroupSubsysState) -> bool;
    pub fn css_from_id(id: i32, ss: *mut CgroupSubsys) -> *mut CgroupSubsysState;
    pub fn cgroup_e_css(cgroup: *mut Cgroup, ss: *mut CgroupSubsys) -> *mut CgroupSubsysState;
    pub fn cgroup_get_e_css(cgroup: *mut Cgroup, ss: *mut CgroupSubsys) -> *mut CgroupSubsysState;
    pub fn css_tryget_online_from_dir(dentry: *mut Dentry, ss: *mut CgroupSubsys) -> *mut CgroupSubsysState;
    pub fn cgroup_get_from_path(path: *const i8) -> *mut Cgroup;
    pub fn cgroup_get_from_fd(fd: i32) -> *mut Cgroup;
    pub fn cgroup_v1v2_get_from_fd(fd: i32) -> *mut Cgroup;
    pub fn cgroup_attach_task_all(from: *mut TaskStruct, to: *mut TaskStruct) -> i32;
    pub fn cgroup_transfer_tasks(to: *mut Cgroup, from: *mut Cgroup) -> i32;
    pub fn cgroup_add_dfl_cftypes(ss: *mut CgroupSubsys, cfts: *mut Cftype) -> i32;
    pub fn cgroup_add_legacy_cftypes(ss: *mut CgroupSubsys, cfts: *mut Cftype) -> i32;
    pub fn cgroup_add_cftypes(ss: *mut CgroupSubsys, cfts: *mut Cftype) -> i32;
    pub fn cgroup_rm_cftypes(cfts: *mut Cftype) -> i32;
    pub fn cgroup_file_notify(cfile: *mut CgroupFile);
    pub fn cgroup_file_show(cfile: *mut CgroupFile, show: bool);
    pub fn cgroupstats_build(stats: *mut CgroupStats, dentry: *mut Dentry) -> i32;
    pub fn proc_cgroup_show(m: *mut SeqFile, ns: *mut PidNamespace, pid: *mut Pid, tsk: *mut TaskStruct) -> i32;
    pub fn cgroup_fork(p: *mut TaskStruct);
    pub fn cgroup_can_fork(p: *mut TaskStruct, kargs: *mut KernelCloneArgs) -> i32;
    pub fn cgroup_cancel_fork(p: *mut TaskStruct, kargs: *mut KernelCloneArgs);
    pub fn cgroup_post_fork(p: *mut TaskStruct, kargs: *mut KernelCloneArgs);
    pub fn cgroup_task_exit(p: *mut TaskStruct);
    pub fn cgroup_task_dead(p: *mut TaskStruct);
    pub fn cgroup_task_release(p: *mut TaskStruct);
    pub fn cgroup_task_free(p: *mut TaskStruct);
    pub fn cgroup_init_early() -> i32;
    pub fn cgroup_init() -> i32;
    pub fn cgroup_parse_float(input: *const i8, dec_shift: u32, v: *mut S64) -> i32;
    pub fn css_next_child(pos: *mut CgroupSubsysState, parent: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    pub fn css_next_descendant_pre(pos: *mut CgroupSubsysState, css: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    pub fn css_rightmost_descendant(pos: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    pub fn css_next_descendant_post(pos: *mut CgroupSubsysState, css: *mut CgroupSubsysState) -> *mut CgroupSubsysState;
    pub fn cgroup_taskset_first(tset: *mut CgroupTaskset, dst_cssp: *mut *mut CgroupSubsysState) -> *mut TaskStruct;
    pub fn cgroup_taskset_next(tset: *mut CgroupTaskset, dst_cssp: *mut *mut CgroupSubsysState) -> *mut TaskStruct;
    pub fn css_task_iter_start(css: *mut CgroupSubsysState, flags: u32, it: *mut CssTaskIter);
    pub fn css_task_iter_next(it: *mut CssTaskIter) -> *mut TaskStruct;
    pub fn css_task_iter_end(it: *mut CssTaskIter);
    pub fn css_get(css: *mut CgroupSubsysState);
    pub fn css_tryget(css: *mut CgroupSubsysState) -> bool;
    pub fn css_put(css: *mut CgroupSubsysState);
    pub fn of_css(of: *mut KernfsOpenFile) -> *mut CgroupSubsysState;
    pub fn cgroup_psi_enabled() -> bool;
    pub fn cgroup_path_from_kernfs_id(id: U64, buf: *mut i8, buflen: usize);
    pub fn __cgroup_get_from_id(id: U64) -> *mut Cgroup;
    pub fn cgroup_get_from_id(id: U64) -> *mut Cgroup;
    pub fn __css_rstat_updated(css: *mut CgroupSubsysState, cpu: i32);
    pub fn css_rstat_updated(css: *mut CgroupSubsysState, cpu: i32);
    pub fn css_rstat_flush(css: *mut CgroupSubsysState);
    pub fn cgroup_sk_alloc(skcd: *mut SockCgroupData);
    pub fn cgroup_sk_clone(skcd: *mut SockCgroupData);
    pub fn cgroup_sk_free(skcd: *mut SockCgroupData);
    pub fn cgroup_enter_frozen();
    pub fn cgroup_leave_frozen(always_leave: bool);
    pub fn cgroup_update_frozen(cgrp: *mut Cgroup);
    pub fn cgroup_freeze(cgrp: *mut Cgroup, freeze: bool);
    pub fn cgroup_freezer_migrate_task(task: *mut TaskStruct, src: *mut Cgroup, dst: *mut Cgroup);
    pub fn task_get_cgroup1(tsk: *mut TaskStruct, hierarchy_id: i32) -> *mut Cgroup;
    pub fn of_peak(of: *mut KernfsOpenFile) -> *mut CgroupOfPeak;
    pub fn cgroup_account_cputime(task: *mut TaskStruct, delta_exec: U64);
    pub fn cgroup_account_cputime_field(task: *mut TaskStruct, index: CpuUsageStat, delta_exec: U64);
}

#[repr(C)] pub struct Cgroup { _private: [u8; 0] }
#[repr(C)] pub struct CgroupTaskset { _private: [u8; 0] }

#[inline] pub unsafe fn cgroup_id(_cgrp: *const Cgroup) -> U64 { 0 }
#[inline] pub unsafe fn cgroup_lock() {}
#[inline] pub unsafe fn cgroup_unlock() {}
#[inline] pub unsafe fn cgroup_is_dead(_cgrp: *const Cgroup) -> bool { false }
#[inline] pub unsafe fn cgroup_parent(_cgrp: *mut Cgroup) -> *mut Cgroup { core::ptr::null_mut() }
#[inline] pub unsafe fn cgroup_psi_enabled_stub() -> bool { false }

#[inline] pub unsafe fn css_is_dying(_css: *mut CgroupSubsysState) -> bool { false }
#[inline] pub unsafe fn css_is_online(_css: *mut CgroupSubsysState) -> bool { false }
#[inline] pub unsafe fn css_is_self(_css: *mut CgroupSubsysState) -> bool { false }
#[inline] pub unsafe fn cgroup_get(cgrp: *mut Cgroup) { css_get(cgrp as *mut CgroupSubsysState); }
#[inline] pub unsafe fn cgroup_tryget(cgrp: *mut Cgroup) -> bool { css_tryget(cgrp as *mut CgroupSubsysState) }
#[inline] pub unsafe fn cgroup_put(cgrp: *mut Cgroup) { css_put(cgrp as *mut CgroupSubsysState); }
#[inline] pub unsafe fn cgroup_is_descendant(_cgrp: *mut Cgroup, _ancestor: *mut Cgroup) -> bool { false }
#[inline] pub unsafe fn cgroup_ancestor(_cgrp: *mut Cgroup, _ancestor_level: i32) -> *mut Cgroup { core::ptr::null_mut() }
#[inline] pub unsafe fn cgroup_common_ancestor(_a: *mut Cgroup, _b: *mut Cgroup) -> *mut Cgroup { core::ptr::null_mut() }
#[inline] pub unsafe fn cgroup_has_tasks(_cgrp: *mut Cgroup) -> bool { false }
#[inline] pub unsafe fn css_is_populated(_css: *mut CgroupSubsysState) -> bool { false }
#[inline] pub unsafe fn cgroup_is_populated(_cgrp: *mut Cgroup) -> bool { false }
#[inline] pub unsafe fn cgroup_ino(_cgrp: *mut Cgroup) -> InoT { 0 }
#[inline] pub unsafe fn of_cft(_of: *mut KernfsOpenFile) -> *mut Cftype { core::ptr::null_mut() }
#[inline] pub unsafe fn seq_cft(_seq: *mut SeqFile) -> *mut Cftype { core::ptr::null_mut() }
#[inline] pub unsafe fn seq_css(_seq: *mut SeqFile) -> *mut CgroupSubsysState { core::ptr::null_mut() }
#[inline] pub unsafe fn cgroup_name(_cgrp: *mut Cgroup, _buf: *mut i8, _buflen: usize) -> i32 { 0 }
#[inline] pub unsafe fn cgroup_path(_cgrp: *mut Cgroup, _buf: *mut i8, _buflen: usize) -> i32 { 0 }
#[inline] pub unsafe fn task_under_cgroup_hierarchy(_task: *mut TaskStruct, _ancestor: *mut Cgroup) -> bool { false }

/* C iteration macros are preserved as Rust macro forms; field-dependent
 * helpers are intentionally left to the corresponding translated headers. */
#[macro_export] macro_rules! css_for_each_child { ($pos:ident, $parent:expr, $body:block) => { for $pos in core::iter::successors(Some(unsafe { css_next_child(core::ptr::null_mut(), $parent) }), |p| { if p.is_null() { None } else { Some(unsafe { css_next_child(*p, $parent) }) } }) $body }; }
#[macro_export] macro_rules! css_for_each_descendant_pre { ($pos:ident, $css:expr, $body:block) => { for $pos in core::iter::successors(Some(unsafe { css_next_descendant_pre(core::ptr::null_mut(), $css) }), |p| { if p.is_null() { None } else { Some(unsafe { css_next_descendant_pre(*p, $css) }) } }) $body }; }
#[macro_export] macro_rules! css_for_each_descendant_post { ($pos:ident, $css:expr, $body:block) => { for $pos in core::iter::successors(Some(unsafe { css_next_descendant_post(core::ptr::null_mut(), $css) }), |p| { if p.is_null() { None } else { Some(unsafe { css_next_descendant_post(*p, $css) }) } }) $body }; }

#[cfg(not(feature = "CONFIG_CGROUPS"))]
#[inline] pub unsafe fn cgroup_id_disabled(_cgrp: *const Cgroup) -> U64 { 1 }
#[cfg(not(feature = "CONFIG_CGROUPS"))]
#[inline] pub unsafe fn task_under_cgroup_hierarchy(_task: *mut TaskStruct, _ancestor: *mut Cgroup) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
