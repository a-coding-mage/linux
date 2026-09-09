// SPDX-License-Identifier: GPL-2.0-only
/*
 * itmt.c: Support Intel Turbo Boost Max Technology 3.0
 *
 * (C) Copyright 2016 Intel Corporation
 * Author: Tim Chen <tim.c.chen@linux.intel.com>
 *
 * On platforms supporting Intel Turbo Boost Max Technology 3.0, (ITMT),
 * the maximum turbo frequencies of some cores in a CPU package may be
 * higher than for the other cores in the same package.  In that case,
 * better performance can be achieved by making the scheduler prefer
 * to run tasks on the CPUs with higher max turbo frequencies.
 *
 * This file provides functions and data structures for enabling the
 * scheduler to favor scheduling on cores can be boosted to a higher
 * frequency under ITMT.
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn()>,
    pub write: Option<unsafe extern "C" fn()>,
    pub open: Option<unsafe extern "C" fn()>,
    pub llseek: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut sysctl_sched_itmt_enabled: bool;
    static mut x86_topology_update: bool;
    static mut arch_debugfs_dir: *mut dentry;
    fn debugfs_write_file_bool(
        filp: *mut file,
        ubuf: *const c_char,
        cnt: usize,
        ppos: *mut c_long,
    ) -> isize;
    fn debugfs_read_file_bool();
    fn simple_open();
    fn default_llseek();
    fn rebuild_sched_domains();
    fn debugfs_create_file_unsafe(
        name: *const c_char,
        mode: c_ulong,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_ulong,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
    fn arch_asym_cpu_priority(cpu: c_int) -> c_int;
    fn seq_puts(s: *mut seq_file, text: *const c_char);
    fn seq_printf(s: *mut seq_file, fmt: *const c_char, ...);
    fn possible_cpu_count() -> c_int;
}

static mut itmt_update_mutex: mutex = mutex { _private: [] };
#[no_mangle]
pub static mut sched_core_priority: *mut c_int = core::ptr::null_mut();
static mut sched_itmt_capable: bool = false;
#[no_mangle]
pub static mut sysctl_sched_itmt_enabled: bool = false;

unsafe extern "C" fn sched_itmt_enabled_write(
    filp: *mut file,
    ubuf: *const c_char,
    cnt: usize,
    ppos: *mut c_long,
) -> isize {
    // guard(mutex)(&itmt_update_mutex);
    let orig = sysctl_sched_itmt_enabled;
    let result = debugfs_write_file_bool(filp, ubuf, cnt, ppos);

    if sysctl_sched_itmt_enabled != orig {
        x86_topology_update = true;
        rebuild_sched_domains();
    }

    result
}

unsafe extern "C" fn sched_core_priority_show(s: *mut seq_file, _unused: *mut c_void) -> c_int {
    seq_puts(s, b"CPU #\tPriority\0".as_ptr() as *const c_char);
    let mut cpu = 0;
    while cpu < possible_cpu_count() {
        let priority = arch_asym_cpu_priority(cpu);
        seq_printf(s, b"%d\t%d\n\0".as_ptr() as *const c_char, cpu, priority);
        cpu += 1;
    }
    0
}

static dfs_sched_itmt_fops: file_operations = file_operations {
    read: Some(debugfs_read_file_bool),
    write: Some(sched_itmt_enabled_write),
    open: Some(simple_open),
    llseek: Some(default_llseek),
};

static mut dfs_sched_itmt: *mut dentry = core::ptr::null_mut();
static mut dfs_sched_core_prio: *mut dentry = core::ptr::null_mut();

/// sched_set_itmt_support() - Indicate platform supports ITMT
///
/// This function is used by the OS to indicate to scheduler that the platform
/// is capable of supporting the ITMT feature.
///
/// Return: 0 on success
#[no_mangle]
pub unsafe extern "C" fn sched_set_itmt_support() -> c_int {
    // guard(mutex)(&itmt_update_mutex);
    if sched_itmt_capable {
        return 0;
    }

    dfs_sched_itmt = debugfs_create_file_unsafe(
        b"sched_itmt_enabled\0".as_ptr() as *const c_char,
        0o644,
        arch_debugfs_dir,
        &raw mut sysctl_sched_itmt_enabled as *mut c_void,
        &dfs_sched_itmt_fops,
    );
    if dfs_sched_itmt.is_null() {
        dfs_sched_itmt = core::ptr::null_mut();
        return -12;
    }

    dfs_sched_core_prio = debugfs_create_file(
        b"sched_core_priority\0".as_ptr() as *const c_char,
        0o644,
        arch_debugfs_dir,
        core::ptr::null_mut(),
        &dfs_sched_itmt_fops,
    );
    if dfs_sched_core_prio.is_null() {
        dfs_sched_core_prio = core::ptr::null_mut();
        return -12;
    }

    sched_itmt_capable = true;
    sysctl_sched_itmt_enabled = true;
    x86_topology_update = true;
    rebuild_sched_domains();
    0
}

/// sched_clear_itmt_support() - Revoke platform's support of ITMT
#[no_mangle]
pub unsafe extern "C" fn sched_clear_itmt_support() {
    // guard(mutex)(&itmt_update_mutex);
    if !sched_itmt_capable {
        return;
    }

    sched_itmt_capable = false;
    debugfs_remove(dfs_sched_itmt);
    dfs_sched_itmt = core::ptr::null_mut();
    debugfs_remove(dfs_sched_core_prio);
    dfs_sched_core_prio = core::ptr::null_mut();

    if sysctl_sched_itmt_enabled {
        /* disable sched_itmt if we are no longer ITMT capable */
        sysctl_sched_itmt_enabled = false;
        x86_topology_update = true;
        rebuild_sched_domains();
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_asym_cpu_priority(cpu: c_int) -> c_int {
    *sched_core_priority.add(cpu as usize)
}

/// sched_set_itmt_core_prio() - Set CPU priority based on ITMT
#[no_mangle]
pub unsafe extern "C" fn sched_set_itmt_core_prio(prio: c_int, cpu: c_int) {
    *sched_core_priority.add(cpu as usize) = prio;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
