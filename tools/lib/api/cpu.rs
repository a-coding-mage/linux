// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdio.h>, "cpu.h", "fs/fs.h"

use core::ffi::{c_char, c_int, c_ulonglong};

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn sysfs__read_int(entry: *const c_char, value: *mut c_int) -> c_int;
    fn sysfs__read_ull(entry: *const c_char, value: *mut c_ulonglong) -> c_int;
}

// From the C environment's PATH_MAX.
const PATH_MAX: usize = 4096;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__get_max_freq(freq: *mut c_ulonglong) -> c_int {
    let mut entry: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut cpu: c_int = 0;

    if unsafe { sysfs__read_int(c"devices/system/cpu/online".as_ptr(), &mut cpu) } < 0 {
        return -1;
    }

    unsafe {
        snprintf(
            entry.as_mut_ptr(),
            entry.len(),
            c"devices/system/cpu/cpu%d/cpufreq/cpuinfo_max_freq".as_ptr(),
            cpu,
        );
    }

    unsafe { sysfs__read_ull(entry.as_ptr(), freq) }
}
