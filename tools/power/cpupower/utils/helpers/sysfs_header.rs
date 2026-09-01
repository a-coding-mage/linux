/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong};

pub const PATH_TO_CPU: &str = "/sys/devices/system/cpu/";
pub const MAX_LINE_LEN: usize = 255;
pub const SYSFS_PATH_MAX: usize = 255;

unsafe extern "C" {
    pub fn sysfs_read_file(path: *const c_char, buf: *mut c_char, buflen: usize) -> c_uint;

    pub fn sysfs_idlestate_file_exists(
        cpu: c_uint,
        idlestate: c_uint,
        fname: *const c_char,
    ) -> c_uint;

    pub fn sysfs_is_cpu_online(cpu: c_uint) -> c_int;

    pub fn sysfs_is_idlestate_disabled(cpu: c_uint, idlestate: c_uint) -> c_int;
    pub fn sysfs_idlestate_disable(cpu: c_uint, idlestate: c_uint, disable: c_uint) -> c_int;
    pub fn sysfs_get_idlestate_latency(cpu: c_uint, idlestate: c_uint) -> c_ulong;
    pub fn sysfs_get_idlestate_usage(cpu: c_uint, idlestate: c_uint) -> c_ulong;
    pub fn sysfs_get_idlestate_time(cpu: c_uint, idlestate: c_uint) -> c_ulonglong;
    pub fn sysfs_get_idlestate_name(cpu: c_uint, idlestate: c_uint) -> *mut c_char;
    pub fn sysfs_get_idlestate_desc(cpu: c_uint, idlestate: c_uint) -> *mut c_char;
    pub fn sysfs_get_idlestate_count(cpu: c_uint) -> c_uint;

    pub fn sysfs_get_cpuidle_governor() -> *mut c_char;
    pub fn sysfs_get_cpuidle_driver() -> *mut c_char;

    pub fn sysfs_get_sched(smt_mc: *const c_char) -> c_int;
    pub fn sysfs_set_sched(smt_mc: *const c_char, val: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
