/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/psi.h.
// The original declarations are conditional on CONFIG_PSI and CONFIG_CGROUPS.

#[allow(non_camel_case_types)]
pub type __poll_t = u32;

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct css_set {
    _private: [u8; 0],
}

#[repr(C)]
pub struct static_key_false {
    _private: [u8; 0],
}

#[repr(C)]
pub struct psi_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernfs_open_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct psi_trigger {
    _private: [u8; 0],
}

#[repr(C)]
pub enum psi_res {}

#[cfg(CONFIG_PSI)]
extern "C" {
    pub static mut psi_disabled: static_key_false;
    pub static mut psi_system: psi_group;

    pub fn psi_init();

    pub fn psi_memstall_enter(flags: *mut c_ulong);
    pub fn psi_memstall_leave(flags: *mut c_ulong);

    pub fn psi_show(s: *mut seq_file, group: *mut psi_group, res: psi_res) -> c_int;
    pub fn psi_trigger_create(
        group: *mut psi_group,
        buf: *mut c_char,
        res: psi_res,
        file: *mut file,
        of: *mut kernfs_open_file,
        need_rtpoll_worker: *mut bool,
    ) -> *mut psi_trigger;
    pub fn psi_trigger_create_rtpoll_worker(group: *mut psi_group) -> c_int;
    pub fn psi_trigger_destroy(t: *mut psi_trigger);

    pub fn psi_trigger_poll(
        trigger_ptr: *mut *mut core::ffi::c_void,
        file: *mut file,
        wait: *mut poll_table,
    ) -> __poll_t;
}

#[cfg(all(CONFIG_PSI, CONFIG_CGROUPS))]
#[inline]
pub unsafe fn cgroup_psi(cgrp: *mut cgroup) -> *mut psi_group {
    // cgroup_ino(cgrp) == 1 ? &psi_system : cgrp->psi
    if cgroup_ino(cgrp) == 1 {
        &raw mut psi_system
    } else {
        // The `psi` field is supplied by the included cgroup definitions.
        (*cgrp).psi
    }
}

#[cfg(all(CONFIG_PSI, CONFIG_CGROUPS))]
extern "C" {
    pub fn psi_cgroup_alloc(cgrp: *mut cgroup) -> c_int;
    pub fn psi_cgroup_free(cgrp: *mut cgroup);
    pub fn cgroup_move_task(p: *mut task_struct, to: *mut css_set);
    pub fn psi_cgroup_restart(group: *mut psi_group);
}

#[cfg(not(CONFIG_PSI))]
#[inline]
pub fn psi_init() {}

#[cfg(not(CONFIG_PSI))]
#[inline]
pub fn psi_memstall_enter(_flags: *mut c_ulong) {}

#[cfg(not(CONFIG_PSI))]
#[inline]
pub fn psi_memstall_leave(_flags: *mut c_ulong) {}

#[cfg(all(not(CONFIG_PSI), CONFIG_CGROUPS))]
#[inline]
pub fn psi_cgroup_alloc(_cgrp: *mut cgroup) -> c_int {
    0
}

#[cfg(all(not(CONFIG_PSI), CONFIG_CGROUPS))]
#[inline]
pub fn psi_cgroup_free(_cgrp: *mut cgroup) {}

#[cfg(all(not(CONFIG_PSI), CONFIG_CGROUPS))]
#[inline]
pub unsafe fn cgroup_move_task(p: *mut task_struct, to: *mut css_set) {
    // rcu_assign_pointer(p->cgroups, to)
    (*p).cgroups = to;
}

#[cfg(all(not(CONFIG_PSI), CONFIG_CGROUPS))]
#[inline]
pub fn psi_cgroup_restart(_group: *mut psi_group) {}

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    fn cgroup_ino(cgrp: *mut cgroup) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
