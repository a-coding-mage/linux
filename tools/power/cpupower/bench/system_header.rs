/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  cpufreq-bench CPUFreq microbenchmark
 *
 *  Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
 */

// C header dependency: "parse.h" provides `struct config`.
use core::ffi::{c_char, c_int, c_longlong, c_uint};

use crate::config;

unsafe extern "C" {
    pub fn get_time() -> c_longlong;

    pub fn set_cpufreq_governor(governor: *mut c_char, cpu: c_uint) -> c_int;
    pub fn set_cpu_affinity(cpu: c_uint) -> c_int;
    pub fn set_process_priority(priority: c_int) -> c_int;

    pub fn prepare_user(config: *const config);
    pub fn prepare_system(config: *const config);
}
