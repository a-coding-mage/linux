// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 */

use core::ptr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

type size_t = usize;
type ssize_t = isize;

// Constants and C library/syscall declarations supplied by headers in the
// original translation unit: stdio.h, errno.h, stdlib.h, string.h, fcntl.h,
// unistd.h, cpufreq.h, and cpupower_intern.h.
const SYSFS_PATH_MAX: usize = 255;
const MAX_LINE_LEN: usize = 255;
const PATH_TO_CPU: &[u8] = b"/sys/devices/system/cpu/";
const O_WRONLY: c_int = 1;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ERANGE: c_int = 34;

unsafe extern "C" {
    fn cpupower_read_sysfs(path: *const c_char, buf: *mut c_char, buflen: size_t) -> c_uint;

    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
}

unsafe fn errno_location() -> *mut c_int {
    unsafe extern "C" {
        fn __errno_location() -> *mut c_int;
    }
    unsafe { __errno_location() }
}

#[repr(C)]
pub struct cpufreq_policy {
    pub min: c_ulong,
    pub max: c_ulong,
    pub governor: *mut c_char,
}

#[repr(C)]
pub struct cpufreq_available_governors {
    pub governor: *mut c_char,
    pub first: *mut cpufreq_available_governors,
    pub next: *mut cpufreq_available_governors,
}

#[repr(C)]
pub struct cpufreq_available_frequencies {
    pub frequency: c_ulong,
    pub first: *mut cpufreq_available_frequencies,
    pub next: *mut cpufreq_available_frequencies,
}

#[repr(C)]
pub struct cpufreq_affected_cpus {
    pub cpu: c_uint,
    pub first: *mut cpufreq_affected_cpus,
    pub next: *mut cpufreq_affected_cpus,
}

#[repr(C)]
pub struct cpufreq_stats {
    pub frequency: c_ulong,
    pub time_in_state: c_ulonglong,
    pub first: *mut cpufreq_stats,
    pub next: *mut cpufreq_stats,
}

/* CPUFREQ sysfs access **************************************************/

/* helper function to read file from /sys into given buffer */
/* fname is a relative path under "cpuX/cpufreq" dir */
unsafe fn sysfs_cpufreq_read_file(
    cpu: c_uint,
    fname: *const c_char,
    buf: *mut c_char,
    buflen: size_t,
) -> c_uint {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"%scpu%u/cpufreq/%s\0".as_ptr() as *const c_char,
            PATH_TO_CPU.as_ptr() as *const c_char,
            cpu,
            fname,
        );
        cpupower_read_sysfs(path.as_ptr(), buf, buflen)
    }
}

/* helper function to write a new value to a /sys file */
/* fname is a relative path under "cpuX/cpufreq" dir */
unsafe fn sysfs_cpufreq_write_file(
    cpu: c_uint,
    fname: *const c_char,
    value: *const c_char,
    len: size_t,
) -> c_uint {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];
    let fd: c_int;
    let numwrite: ssize_t;

    unsafe {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"%scpu%u/cpufreq/%s\0".as_ptr() as *const c_char,
            PATH_TO_CPU.as_ptr() as *const c_char,
            cpu,
            fname,
        );

        fd = open(path.as_ptr(), O_WRONLY);
        if fd == -1 {
            return 0;
        }

        numwrite = write(fd, value as *const c_void, len);
        if numwrite < 1 {
            close(fd);
            return 0;
        }

        close(fd);
    }

    numwrite as c_uint
}

/* read access to files which contain one numeric value */

#[repr(usize)]
enum cpufreq_value {
    CPUINFO_CUR_FREQ,
    CPUINFO_MIN_FREQ,
    CPUINFO_MAX_FREQ,
    CPUINFO_LATENCY,
    SCALING_CUR_FREQ,
    SCALING_MIN_FREQ,
    SCALING_MAX_FREQ,
    STATS_NUM_TRANSITIONS,
    MAX_CPUFREQ_VALUE_READ_FILES,
}

static cpufreq_value_files: [*const c_char; cpufreq_value::MAX_CPUFREQ_VALUE_READ_FILES as usize] = [
    b"cpuinfo_cur_freq\0".as_ptr() as *const c_char,
    b"cpuinfo_min_freq\0".as_ptr() as *const c_char,
    b"cpuinfo_max_freq\0".as_ptr() as *const c_char,
    b"cpuinfo_transition_latency\0".as_ptr() as *const c_char,
    b"scaling_cur_freq\0".as_ptr() as *const c_char,
    b"scaling_min_freq\0".as_ptr() as *const c_char,
    b"scaling_max_freq\0".as_ptr() as *const c_char,
    b"stats/total_trans\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_sysfs_value_from_table(
    cpu: c_uint,
    table: *const *const c_char,
    index: c_uint,
    size: c_uint,
) -> c_ulong {
    let value: c_ulong;
    let len: c_uint;
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut endp: *mut c_char = ptr::null_mut();

    unsafe {
        if table.is_null() || index >= size || (*table.add(index as usize)).is_null() {
            return 0;
        }

        len = sysfs_cpufreq_read_file(
            cpu,
            *table.add(index as usize),
            linebuf.as_mut_ptr(),
            linebuf.len(),
        );

        if len == 0 {
            return 0;
        }

        if strcmp(linebuf.as_ptr(), b"enabled\n\0".as_ptr() as *const c_char) == 0 {
            return 1;
        }
        if strcmp(linebuf.as_ptr(), b"disabled\n\0".as_ptr() as *const c_char) == 0 {
            return 0;
        }
        value = strtoul(linebuf.as_ptr(), &mut endp, 0);

        if endp == linebuf.as_mut_ptr() || *errno_location() == ERANGE {
            return 0;
        }
    }

    value
}

unsafe fn sysfs_cpufreq_get_one_value(cpu: c_uint, which: cpufreq_value) -> c_ulong {
    unsafe {
        cpufreq_get_sysfs_value_from_table(
            cpu,
            cpufreq_value_files.as_ptr(),
            which as c_uint,
            cpufreq_value::MAX_CPUFREQ_VALUE_READ_FILES as c_uint,
        )
    }
}

/* read access to files which contain one string */

#[repr(usize)]
enum cpufreq_string {
    SCALING_DRIVER,
    SCALING_GOVERNOR,
    ENERGY_PERFORMANCE_PREFERENCE,
    MAX_CPUFREQ_STRING_FILES,
}

static cpufreq_string_files: [*const c_char; cpufreq_string::MAX_CPUFREQ_STRING_FILES as usize] = [
    b"scaling_driver\0".as_ptr() as *const c_char,
    b"scaling_governor\0".as_ptr() as *const c_char,
    b"energy_performance_preference\0".as_ptr() as *const c_char,
];

unsafe fn sysfs_cpufreq_get_one_string(cpu: c_uint, which: cpufreq_string) -> *mut c_char {
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let result: *mut c_char;
    let len: c_uint;

    if (which as usize) >= cpufreq_string::MAX_CPUFREQ_STRING_FILES as usize {
        return ptr::null_mut();
    }

    unsafe {
        len = sysfs_cpufreq_read_file(
            cpu,
            cpufreq_string_files[which as usize],
            linebuf.as_mut_ptr(),
            linebuf.len(),
        );
        if len == 0 {
            return ptr::null_mut();
        }

        result = strdup(linebuf.as_ptr());
        if result.is_null() {
            return ptr::null_mut();
        }

        if *result.add(strlen(result) - 1) == b'\n' as c_char {
            *result.add(strlen(result) - 1) = b'\0' as c_char;
        }
    }

    result
}

/* write access */

#[repr(usize)]
enum cpufreq_write {
    WRITE_SCALING_MIN_FREQ,
    WRITE_SCALING_MAX_FREQ,
    WRITE_SCALING_GOVERNOR,
    WRITE_SCALING_SET_SPEED,
    MAX_CPUFREQ_WRITE_FILES,
}

static cpufreq_write_files: [*const c_char; cpufreq_write::MAX_CPUFREQ_WRITE_FILES as usize] = [
    b"scaling_min_freq\0".as_ptr() as *const c_char,
    b"scaling_max_freq\0".as_ptr() as *const c_char,
    b"scaling_governor\0".as_ptr() as *const c_char,
    b"scaling_setspeed\0".as_ptr() as *const c_char,
];

unsafe fn sysfs_cpufreq_write_one_value(
    cpu: c_uint,
    which: cpufreq_write,
    new_value: *const c_char,
    len: size_t,
) -> c_int {
    if (which as usize) >= cpufreq_write::MAX_CPUFREQ_WRITE_FILES as usize {
        return 0;
    }

    unsafe {
        if sysfs_cpufreq_write_file(cpu, cpufreq_write_files[which as usize], new_value, len)
            != len as c_uint
        {
            return -ENODEV;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_freq_kernel(cpu: c_uint) -> c_ulong {
    unsafe { sysfs_cpufreq_get_one_value(cpu, cpufreq_value::SCALING_CUR_FREQ) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_freq_hardware(cpu: c_uint) -> c_ulong {
    unsafe { sysfs_cpufreq_get_one_value(cpu, cpufreq_value::CPUINFO_CUR_FREQ) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_transition_latency(cpu: c_uint) -> c_ulong {
    unsafe { sysfs_cpufreq_get_one_value(cpu, cpufreq_value::CPUINFO_LATENCY) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_energy_performance_preference(cpu: c_uint) -> *mut c_char {
    unsafe { sysfs_cpufreq_get_one_string(cpu, cpufreq_string::ENERGY_PERFORMANCE_PREFERENCE) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_energy_performance_preference(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        free(ptr as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_hardware_limits(
    cpu: c_uint,
    min: *mut c_ulong,
    max: *mut c_ulong,
) -> c_int {
    unsafe {
        if min.is_null() || max.is_null() {
            return -EINVAL;
        }

        *min = sysfs_cpufreq_get_one_value(cpu, cpufreq_value::CPUINFO_MIN_FREQ);
        if *min == 0 {
            return -ENODEV;
        }

        *max = sysfs_cpufreq_get_one_value(cpu, cpufreq_value::CPUINFO_MAX_FREQ);
        if *max == 0 {
            return -ENODEV;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_driver(cpu: c_uint) -> *mut c_char {
    unsafe { sysfs_cpufreq_get_one_string(cpu, cpufreq_string::SCALING_DRIVER) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_driver(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        free(ptr as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_policy(cpu: c_uint) -> *mut cpufreq_policy {
    let policy: *mut cpufreq_policy;

    unsafe {
        policy = malloc(core::mem::size_of::<cpufreq_policy>()) as *mut cpufreq_policy;
        if policy.is_null() {
            return ptr::null_mut();
        }

        (*policy).governor = sysfs_cpufreq_get_one_string(cpu, cpufreq_string::SCALING_GOVERNOR);
        if (*policy).governor.is_null() {
            free(policy as *mut c_void);
            return ptr::null_mut();
        }
        (*policy).min = sysfs_cpufreq_get_one_value(cpu, cpufreq_value::SCALING_MIN_FREQ);
        (*policy).max = sysfs_cpufreq_get_one_value(cpu, cpufreq_value::SCALING_MAX_FREQ);
        if (*policy).min == 0 || (*policy).max == 0 {
            free((*policy).governor as *mut c_void);
            free(policy as *mut c_void);
            return ptr::null_mut();
        }
    }

    policy
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_policy(policy: *mut cpufreq_policy) {
    unsafe {
        if policy.is_null() || (*policy).governor.is_null() {
            return;
        }

        free((*policy).governor as *mut c_void);
        (*policy).governor = ptr::null_mut();
        free(policy as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_available_governors(
    cpu: c_uint,
) -> *mut cpufreq_available_governors {
    let mut first: *mut cpufreq_available_governors = ptr::null_mut();
    let mut current: *mut cpufreq_available_governors = ptr::null_mut();
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut pos: c_uint;
    let mut i: c_uint;
    let len: c_uint;

    unsafe {
        len = sysfs_cpufreq_read_file(
            cpu,
            b"scaling_available_governors\0".as_ptr() as *const c_char,
            linebuf.as_mut_ptr(),
            linebuf.len(),
        );
        if len == 0 {
            return ptr::null_mut();
        }

        pos = 0;
        i = 0;
        while i < len {
            if linebuf[i as usize] == b' ' as c_char || linebuf[i as usize] == b'\n' as c_char {
                if i - pos < 2 {
                    i += 1;
                    continue;
                }
                if !current.is_null() {
                    (*current).next = malloc(core::mem::size_of::<cpufreq_available_governors>())
                        as *mut cpufreq_available_governors;
                    if (*current).next.is_null() {
                        goto_available_governors_error_out(first);
                        return ptr::null_mut();
                    }
                    current = (*current).next;
                } else {
                    first = malloc(core::mem::size_of::<cpufreq_available_governors>())
                        as *mut cpufreq_available_governors;
                    if first.is_null() {
                        return ptr::null_mut();
                    }
                    current = first;
                }
                (*current).first = first;
                (*current).next = ptr::null_mut();

                (*current).governor = malloc((i - pos + 1) as size_t) as *mut c_char;
                if (*current).governor.is_null() {
                    goto_available_governors_error_out(first);
                    return ptr::null_mut();
                }

                memcpy(
                    (*current).governor as *mut c_void,
                    linebuf.as_ptr().add(pos as usize) as *const c_void,
                    (i - pos) as size_t,
                );
                *(*current).governor.add((i - pos) as usize) = b'\0' as c_char;
                pos = i + 1;
            }
            i += 1;
        }
    }

    first
}

unsafe fn goto_available_governors_error_out(mut first: *mut cpufreq_available_governors) {
    let mut current: *mut cpufreq_available_governors;
    unsafe {
        while !first.is_null() {
            current = (*first).next;
            if !(*first).governor.is_null() {
                free((*first).governor as *mut c_void);
            }
            free(first as *mut c_void);
            first = current;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_available_governors(
    any: *mut cpufreq_available_governors,
) {
    let mut tmp: *mut cpufreq_available_governors;
    let mut next: *mut cpufreq_available_governors;

    if any.is_null() {
        return;
    }

    unsafe {
        tmp = (*any).first;
        while !tmp.is_null() {
            next = (*tmp).next;
            if !(*tmp).governor.is_null() {
                free((*tmp).governor as *mut c_void);
            }
            free(tmp as *mut c_void);
            tmp = next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_available_frequencies(
    cpu: c_uint,
) -> *mut cpufreq_available_frequencies {
    unsafe {
        cpufreq_get_available_frequencies_from_file(
            cpu,
            b"scaling_available_frequencies\0".as_ptr() as *const c_char,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_boost_frequencies(
    cpu: c_uint,
) -> *mut cpufreq_available_frequencies {
    unsafe {
        cpufreq_get_available_frequencies_from_file(
            cpu,
            b"scaling_boost_frequencies\0".as_ptr() as *const c_char,
        )
    }
}

unsafe fn cpufreq_get_available_frequencies_from_file(
    cpu: c_uint,
    file: *const c_char,
) -> *mut cpufreq_available_frequencies {
    let mut first: *mut cpufreq_available_frequencies = ptr::null_mut();
    let mut current: *mut cpufreq_available_frequencies = ptr::null_mut();
    let mut one_value = [0 as c_char; SYSFS_PATH_MAX];
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut pos: c_uint;
    let mut i: c_uint;
    let len: c_uint;

    unsafe {
        len = sysfs_cpufreq_read_file(cpu, file, linebuf.as_mut_ptr(), linebuf.len());
        if len == 0 {
            return ptr::null_mut();
        }

        pos = 0;
        i = 0;
        while i < len {
            if linebuf[i as usize] == b' ' as c_char || linebuf[i as usize] == b'\n' as c_char {
                if i - pos < 2 {
                    i += 1;
                    continue;
                }
                if i - pos >= SYSFS_PATH_MAX as c_uint {
                    goto_available_frequencies_error_out(first);
                    return ptr::null_mut();
                }
                if !current.is_null() {
                    (*current).next = malloc(core::mem::size_of::<cpufreq_available_frequencies>())
                        as *mut cpufreq_available_frequencies;
                    if (*current).next.is_null() {
                        goto_available_frequencies_error_out(first);
                        return ptr::null_mut();
                    }
                    current = (*current).next;
                } else {
                    first = malloc(core::mem::size_of::<cpufreq_available_frequencies>())
                        as *mut cpufreq_available_frequencies;
                    if first.is_null() {
                        return ptr::null_mut();
                    }
                    current = first;
                }
                (*current).first = first;
                (*current).next = ptr::null_mut();

                memcpy(
                    one_value.as_mut_ptr() as *mut c_void,
                    linebuf.as_ptr().add(pos as usize) as *const c_void,
                    (i - pos) as size_t,
                );
                one_value[(i - pos) as usize] = b'\0' as c_char;
                if sscanf(
                    one_value.as_ptr(),
                    b"%lu\0".as_ptr() as *const c_char,
                    &mut (*current).frequency,
                ) != 1
                {
                    goto_available_frequencies_error_out(first);
                    return ptr::null_mut();
                }

                pos = i + 1;
            }
            i += 1;
        }
    }

    first
}

unsafe fn goto_available_frequencies_error_out(mut first: *mut cpufreq_available_frequencies) {
    let mut current: *mut cpufreq_available_frequencies;
    unsafe {
        while !first.is_null() {
            current = (*first).next;
            free(first as *mut c_void);
            first = current;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_available_frequencies(
    any: *mut cpufreq_available_frequencies,
) {
    let mut tmp: *mut cpufreq_available_frequencies;
    let mut next: *mut cpufreq_available_frequencies;

    if any.is_null() {
        return;
    }

    unsafe {
        tmp = (*any).first;
        while !tmp.is_null() {
            next = (*tmp).next;
            free(tmp as *mut c_void);
            tmp = next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_boost_frequencies(any: *mut cpufreq_available_frequencies) {
    unsafe {
        cpufreq_put_available_frequencies(any);
    }
}

unsafe fn sysfs_get_cpu_list(cpu: c_uint, file: *const c_char) -> *mut cpufreq_affected_cpus {
    let mut first: *mut cpufreq_affected_cpus = ptr::null_mut();
    let mut current: *mut cpufreq_affected_cpus = ptr::null_mut();
    let mut one_value = [0 as c_char; SYSFS_PATH_MAX];
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut pos: c_uint;
    let mut i: c_uint;
    let len: c_uint;

    unsafe {
        len = sysfs_cpufreq_read_file(cpu, file, linebuf.as_mut_ptr(), linebuf.len());
        if len == 0 {
            return ptr::null_mut();
        }

        pos = 0;
        i = 0;
        while i < len {
            if i == len || linebuf[i as usize] == b' ' as c_char || linebuf[i as usize] == b'\n' as c_char {
                if i - pos < 1 {
                    i += 1;
                    continue;
                }
                if i - pos >= SYSFS_PATH_MAX as c_uint {
                    goto_affected_cpus_error_out(first);
                    return ptr::null_mut();
                }
                if !current.is_null() {
                    (*current).next = malloc(core::mem::size_of::<cpufreq_affected_cpus>())
                        as *mut cpufreq_affected_cpus;
                    if (*current).next.is_null() {
                        goto_affected_cpus_error_out(first);
                        return ptr::null_mut();
                    }
                    current = (*current).next;
                } else {
                    first = malloc(core::mem::size_of::<cpufreq_affected_cpus>())
                        as *mut cpufreq_affected_cpus;
                    if first.is_null() {
                        return ptr::null_mut();
                    }
                    current = first;
                }
                (*current).first = first;
                (*current).next = ptr::null_mut();

                memcpy(
                    one_value.as_mut_ptr() as *mut c_void,
                    linebuf.as_ptr().add(pos as usize) as *const c_void,
                    (i - pos) as size_t,
                );
                one_value[(i - pos) as usize] = b'\0' as c_char;

                if sscanf(
                    one_value.as_ptr(),
                    b"%u\0".as_ptr() as *const c_char,
                    &mut (*current).cpu,
                ) != 1
                {
                    goto_affected_cpus_error_out(first);
                    return ptr::null_mut();
                }

                pos = i + 1;
            }
            i += 1;
        }
    }

    first
}

unsafe fn goto_affected_cpus_error_out(mut first: *mut cpufreq_affected_cpus) {
    let mut current: *mut cpufreq_affected_cpus;
    unsafe {
        while !first.is_null() {
            current = (*first).next;
            free(first as *mut c_void);
            first = current;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_affected_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus {
    unsafe { sysfs_get_cpu_list(cpu, b"affected_cpus\0".as_ptr() as *const c_char) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_affected_cpus(any: *mut cpufreq_affected_cpus) {
    let mut tmp: *mut cpufreq_affected_cpus;
    let mut next: *mut cpufreq_affected_cpus;

    if any.is_null() {
        return;
    }

    unsafe {
        tmp = (*any).first;
        while !tmp.is_null() {
            next = (*tmp).next;
            free(tmp as *mut c_void);
            tmp = next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_related_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus {
    unsafe { sysfs_get_cpu_list(cpu, b"related_cpus\0".as_ptr() as *const c_char) }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_related_cpus(any: *mut cpufreq_affected_cpus) {
    unsafe {
        cpufreq_put_affected_cpus(any);
    }
}

unsafe fn verify_gov(new_gov: *mut c_char, passed_gov: *mut c_char) -> c_int {
    let mut i: c_uint;
    let mut j: c_uint = 0;

    unsafe {
        if passed_gov.is_null() || strlen(passed_gov) > 19 {
            return -EINVAL;
        }

        strncpy(new_gov, passed_gov, 20);
        i = 0;
        while i < 20 {
            if j != 0 {
                *new_gov.add(i as usize) = b'\0' as c_char;
                i += 1;
                continue;
            }
            if *new_gov.add(i as usize) >= b'a' as c_char
                && *new_gov.add(i as usize) <= b'z' as c_char
            {
                i += 1;
                continue;
            }

            if *new_gov.add(i as usize) >= b'A' as c_char
                && *new_gov.add(i as usize) <= b'Z' as c_char
            {
                i += 1;
                continue;
            }

            if *new_gov.add(i as usize) == b'-' as c_char {
                i += 1;
                continue;
            }

            if *new_gov.add(i as usize) == b'_' as c_char {
                i += 1;
                continue;
            }

            if *new_gov.add(i as usize) == b'\0' as c_char {
                j = 1;
                i += 1;
                continue;
            }
            return -EINVAL;
        }
        *new_gov.add(19) = b'\0' as c_char;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_set_policy(cpu: c_uint, policy: *mut cpufreq_policy) -> c_int {
    let mut min = [0 as c_char; SYSFS_PATH_MAX];
    let mut max = [0 as c_char; SYSFS_PATH_MAX];
    let mut gov = [0 as c_char; SYSFS_PATH_MAX];
    let mut ret: c_int;
    let old_min: c_ulong;
    let write_max_first: c_int;

    unsafe {
        if policy.is_null() || (*policy).governor.is_null() {
            return -EINVAL;
        }

        if (*policy).max < (*policy).min {
            return -EINVAL;
        }

        if verify_gov(gov.as_mut_ptr(), (*policy).governor) != 0 {
            return -EINVAL;
        }

        snprintf(
            min.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%lu\0".as_ptr() as *const c_char,
            (*policy).min,
        );
        snprintf(
            max.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%lu\0".as_ptr() as *const c_char,
            (*policy).max,
        );

        old_min = sysfs_cpufreq_get_one_value(cpu, cpufreq_value::SCALING_MIN_FREQ);
        write_max_first = if old_min != 0 && (*policy).max < old_min { 0 } else { 1 };

        if write_max_first != 0 {
            ret = sysfs_cpufreq_write_one_value(
                cpu,
                cpufreq_write::WRITE_SCALING_MAX_FREQ,
                max.as_ptr(),
                strlen(max.as_ptr()),
            );
            if ret != 0 {
                return ret;
            }
        }

        ret = sysfs_cpufreq_write_one_value(
            cpu,
            cpufreq_write::WRITE_SCALING_MIN_FREQ,
            min.as_ptr(),
            strlen(min.as_ptr()),
        );
        if ret != 0 {
            return ret;
        }

        if write_max_first == 0 {
            ret = sysfs_cpufreq_write_one_value(
                cpu,
                cpufreq_write::WRITE_SCALING_MAX_FREQ,
                max.as_ptr(),
                strlen(max.as_ptr()),
            );
            if ret != 0 {
                return ret;
            }
        }

        sysfs_cpufreq_write_one_value(
            cpu,
            cpufreq_write::WRITE_SCALING_GOVERNOR,
            gov.as_ptr(),
            strlen(gov.as_ptr()),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_modify_policy_min(cpu: c_uint, min_freq: c_ulong) -> c_int {
    let mut value = [0 as c_char; SYSFS_PATH_MAX];

    unsafe {
        snprintf(
            value.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%lu\0".as_ptr() as *const c_char,
            min_freq,
        );

        sysfs_cpufreq_write_one_value(
            cpu,
            cpufreq_write::WRITE_SCALING_MIN_FREQ,
            value.as_ptr(),
            strlen(value.as_ptr()),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_modify_policy_max(cpu: c_uint, max_freq: c_ulong) -> c_int {
    let mut value = [0 as c_char; SYSFS_PATH_MAX];

    unsafe {
        snprintf(
            value.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%lu\0".as_ptr() as *const c_char,
            max_freq,
        );

        sysfs_cpufreq_write_one_value(
            cpu,
            cpufreq_write::WRITE_SCALING_MAX_FREQ,
            value.as_ptr(),
            strlen(value.as_ptr()),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_modify_policy_governor(
    cpu: c_uint,
    governor: *mut c_char,
) -> c_int {
    let mut new_gov = [0 as c_char; SYSFS_PATH_MAX];

    unsafe {
        if governor.is_null() || strlen(governor) > 19 {
            return -EINVAL;
        }

        if verify_gov(new_gov.as_mut_ptr(), governor) != 0 {
            return -EINVAL;
        }

        sysfs_cpufreq_write_one_value(
            cpu,
            cpufreq_write::WRITE_SCALING_GOVERNOR,
            new_gov.as_ptr(),
            strlen(new_gov.as_ptr()),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_set_frequency(
    cpu: c_uint,
    target_frequency: c_ulong,
) -> c_int {
    let pol: *mut cpufreq_policy = unsafe { cpufreq_get_policy(cpu) };
    let userspace_gov = *b"userspace\0";
    let mut freq = [0 as c_char; SYSFS_PATH_MAX];
    let ret: c_int;

    unsafe {
        if pol.is_null() {
            return -ENODEV;
        }

        if strncmp((*pol).governor, userspace_gov.as_ptr() as *const c_char, 9) != 0 {
            ret = cpufreq_modify_policy_governor(cpu, userspace_gov.as_ptr() as *mut c_char);
            if ret != 0 {
                cpufreq_put_policy(pol);
                return ret;
            }
        }

        cpufreq_put_policy(pol);

        snprintf(
            freq.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%lu\0".as_ptr() as *const c_char,
            target_frequency,
        );

        sysfs_cpufreq_write_one_value(
            cpu,
            cpufreq_write::WRITE_SCALING_SET_SPEED,
            freq.as_ptr(),
            strlen(freq.as_ptr()),
        )
    }
}

unsafe extern "C" {
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_stats(
    cpu: c_uint,
    total_time: *mut c_ulonglong,
) -> *mut cpufreq_stats {
    let mut first: *mut cpufreq_stats = ptr::null_mut();
    let mut current: *mut cpufreq_stats = ptr::null_mut();
    let mut one_value = [0 as c_char; SYSFS_PATH_MAX];
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut pos: c_uint;
    let mut i: c_uint;
    let len: c_uint;

    unsafe {
        len = sysfs_cpufreq_read_file(
            cpu,
            b"stats/time_in_state\0".as_ptr() as *const c_char,
            linebuf.as_mut_ptr(),
            linebuf.len(),
        );
        if len == 0 {
            return ptr::null_mut();
        }

        *total_time = 0;
        pos = 0;
        i = 0;
        while i < len {
            if i == strlen(linebuf.as_ptr()) as c_uint || linebuf[i as usize] == b'\n' as c_char {
                if i - pos < 2 {
                    i += 1;
                    continue;
                }
                if i - pos >= SYSFS_PATH_MAX as c_uint {
                    goto_stats_error_out(first);
                    return ptr::null_mut();
                }
                if !current.is_null() {
                    (*current).next =
                        malloc(core::mem::size_of::<cpufreq_stats>()) as *mut cpufreq_stats;
                    if (*current).next.is_null() {
                        goto_stats_error_out(first);
                        return ptr::null_mut();
                    }
                    current = (*current).next;
                } else {
                    first = malloc(core::mem::size_of::<cpufreq_stats>()) as *mut cpufreq_stats;
                    if first.is_null() {
                        return ptr::null_mut();
                    }
                    current = first;
                }
                (*current).first = first;
                (*current).next = ptr::null_mut();

                memcpy(
                    one_value.as_mut_ptr() as *mut c_void,
                    linebuf.as_ptr().add(pos as usize) as *const c_void,
                    (i - pos) as size_t,
                );
                one_value[(i - pos) as usize] = b'\0' as c_char;
                if sscanf(
                    one_value.as_ptr(),
                    b"%lu %llu\0".as_ptr() as *const c_char,
                    &mut (*current).frequency,
                    &mut (*current).time_in_state,
                ) != 2
                {
                    goto_stats_error_out(first);
                    return ptr::null_mut();
                }

                *total_time = (*total_time).wrapping_add((*current).time_in_state);
                pos = i + 1;
            }
            i += 1;
        }
    }

    first
}

unsafe fn goto_stats_error_out(mut first: *mut cpufreq_stats) {
    let mut current: *mut cpufreq_stats;
    unsafe {
        while !first.is_null() {
            current = (*first).next;
            free(first as *mut c_void);
            first = current;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_put_stats(any: *mut cpufreq_stats) {
    let mut tmp: *mut cpufreq_stats;
    let mut next: *mut cpufreq_stats;

    if any.is_null() {
        return;
    }

    unsafe {
        tmp = (*any).first;
        while !tmp.is_null() {
            next = (*tmp).next;
            free(tmp as *mut c_void);
            tmp = next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpufreq_get_transitions(cpu: c_uint) -> c_ulong {
    unsafe { sysfs_cpufreq_get_one_value(cpu, cpufreq_value::STATS_NUM_TRANSITIONS) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
