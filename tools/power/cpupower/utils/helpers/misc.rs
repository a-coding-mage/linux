// SPDX-License-Identifier: GPL-2.0

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

// Dependencies originally provided by:
// helpers/helpers.h, helpers/sysfs.h, cpufreq.h, cpupower_intern.h
const MSR_AMD_HWCR: c_ulonglong = 0xc0010015;

extern "C" {
    static mut errno: c_int;

    static mut cpupower_cpu_info: cpupower_cpu_info;
    static mut online_cpus: *mut bitmask;
    static mut offline_cpus: *mut bitmask;
    static mut cpus_chosen: *mut bitmask;

    static CPUPOWER_CAP_AMD_CPB: c_ulonglong;
    static CPUPOWER_CAP_AMD_CPB_MSR: c_ulonglong;
    static CPUPOWER_CAP_AMD_PSTATE: c_ulonglong;
    static CPUPOWER_CAP_INTEL_IDA: c_ulonglong;
    static CPUPOWER_CAP_PERF_BIAS: c_ulonglong;
    static CPUPOWER_AMD_CPBDIS: c_ulonglong;
    static MAX_LINE_LEN: usize;
    static SYSFS_PATH_MAX: usize;
    static EACCES: c_int;
    static EINVAL: c_int;
    static ERANGE: c_int;

    fn read_msr(cpu: c_uint, msr: c_ulonglong, val: *mut c_ulonglong) -> c_int;
    fn amd_pci_get_num_boost_states(active: *mut c_int, states: *mut c_int) -> c_int;
    fn amd_pstate_boost_init(cpu: c_uint, support: *mut c_int, active: *mut c_int);
    fn is_valid_path(path: *const c_char) -> bool;
    fn cpupower_read_sysfs(path: *const c_char, buf: *mut c_char, len: usize) -> c_int;
    fn cpupower_write_sysfs(path: *const c_char, buf: *const c_char, len: usize) -> c_int;
    fn cpufreq_get_driver(cpu: c_uint) -> *mut c_char;
    fn cpufreq_put_driver(driver: *mut c_char);
    fn bitmask_clearall(bmp: *mut bitmask);
    fn bitmask_first(bmp: *mut bitmask) -> c_uint;
    fn bitmask_last(bmp: *mut bitmask) -> c_uint;
    fn cpupower_is_cpu_online(cpu: c_uint) -> c_int;
    fn bitmask_setbit(bmp: *mut bitmask, bit: c_uint) -> c_int;
    fn bitmask_isallclear(bmp: *const bitmask) -> c_int;
    fn bitmask_displaylist(buf: *mut c_char, len: c_int, bmp: *const bitmask) -> c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn gettext(msgid: *const c_char) -> *mut c_char;
}

type c_long = isize;

#[repr(C)]
pub struct cpupower_cpu_info {
    pub caps: c_ulonglong,
}

#[repr(C)]
pub struct bitmask {
    pub size: c_uint,
}

// PATH_TO_CPU is a C preprocessor string prefix supplied by included headers.
const PATH_TO_CPU: &[u8] = b"/sys/devices/system/cpu/\0";

unsafe fn snprintf_path1(path: *mut c_char, size: usize, suffix_fmt: *const c_char, arg: c_uint) {
    snprintf(path, size, b"%s\0".as_ptr() as *const c_char, PATH_TO_CPU.as_ptr());
    let mut len = 0usize;
    while len < size && *path.add(len) != 0 {
        len += 1;
    }
    snprintf(path.add(len), size.wrapping_sub(len), suffix_fmt, arg);
}

unsafe fn snprintf_path0(path: *mut c_char, size: usize, suffix: *const c_char) {
    snprintf(path, size, b"%s%s\0".as_ptr() as *const c_char, PATH_TO_CPU.as_ptr(), suffix);
}

// #if defined(__i386__) || defined(__x86_64__)

#[no_mangle]
pub unsafe extern "C" fn cpufreq_has_x86_boost_support(
    cpu: c_uint,
    support: *mut c_int,
    active: *mut c_int,
    states: *mut c_int,
) -> c_int {
    let mut ret: c_int;
    let mut val: c_ulonglong = 0;
    let mut linebuf = [0 as c_char; 4096];
    let mut path = [0 as c_char; 4096];
    let mut endp: *mut c_char = std::ptr::null_mut();

    *support = 0;
    *active = 0;
    *states = 0;

    if cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_CPB != 0 {
        *support = 1;

        /* AMD Family 0x17 does not utilize PCI D18F4 like prior
         * families and has no fixed discrete boost states but
         * has Hardware determined variable increments instead.
         */

        if cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_CPB_MSR != 0 {
            if read_msr(cpu, MSR_AMD_HWCR, &mut val) == 0 {
                if val & CPUPOWER_AMD_CPBDIS == 0 {
                    *active = 1;
                }
            }
        } else {
            ret = amd_pci_get_num_boost_states(active, states);
            if ret != 0 {
                return ret;
            }
        }
    } else if cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_PSTATE != 0 {
        amd_pstate_boost_init(cpu, support, active);
    } else if cpupower_cpu_info.caps & CPUPOWER_CAP_INTEL_IDA != 0 {
        *support = 1;
        *active = 1;

        snprintf_path0(
            path.as_mut_ptr(),
            path.len(),
            b"intel_pstate/no_turbo\0".as_ptr() as *const c_char,
        );

        if !is_valid_path(path.as_ptr()) {
            return 0;
        }

        if cpupower_read_sysfs(path.as_ptr(), linebuf.as_mut_ptr(), MAX_LINE_LEN) == 0 {
            return -1;
        }

        val = strtol(linebuf.as_ptr(), &mut endp, 0) as c_ulonglong;
        if endp == linebuf.as_mut_ptr() || errno == ERANGE {
            return -1;
        }

        *active = (val == 0) as c_int;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_set_intel_turbo_boost(turbo_boost: c_int) -> c_int {
    let mut path = [0 as c_char; 4096];
    let mut linebuf = [0 as c_char; 2];

    snprintf_path0(
        path.as_mut_ptr(),
        path.len(),
        b"intel_pstate/no_turbo\0".as_ptr() as *const c_char,
    );

    /* Fallback to generic solution when intel_pstate driver not running */
    if !is_valid_path(path.as_ptr()) {
        return cpupower_set_generic_turbo_boost(turbo_boost);
    }

    snprintf(
        linebuf.as_mut_ptr(),
        linebuf.len(),
        b"%d\0".as_ptr() as *const c_char,
        !turbo_boost,
    );

    if cpupower_write_sysfs(path.as_ptr(), linebuf.as_ptr(), 2) <= 0 {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_intel_get_perf_bias(cpu: c_uint) -> c_int {
    let mut linebuf = [0 as c_char; 4096];
    let mut path = [0 as c_char; 4096];
    let mut val: c_ulong;
    let mut endp: *mut c_char = std::ptr::null_mut();

    if cpupower_cpu_info.caps & CPUPOWER_CAP_PERF_BIAS == 0 {
        return -1;
    }

    snprintf_path1(
        path.as_mut_ptr(),
        path.len(),
        b"cpu%u/power/energy_perf_bias\0".as_ptr() as *const c_char,
        cpu,
    );

    if cpupower_read_sysfs(path.as_ptr(), linebuf.as_mut_ptr(), MAX_LINE_LEN) == 0 {
        return -1;
    }

    val = strtol(linebuf.as_ptr(), &mut endp, 0) as c_ulong;
    if endp == linebuf.as_mut_ptr() || errno == ERANGE {
        return -1;
    }

    val as c_int
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_intel_set_perf_bias(cpu: c_uint, val: c_uint) -> c_int {
    let mut path = [0 as c_char; 4096];
    let mut linebuf = [0 as c_char; 3];

    if cpupower_cpu_info.caps & CPUPOWER_CAP_PERF_BIAS == 0 {
        return -1;
    }

    snprintf_path1(
        path.as_mut_ptr(),
        path.len(),
        b"cpu%u/power/energy_perf_bias\0".as_ptr() as *const c_char,
        cpu,
    );
    snprintf(
        linebuf.as_mut_ptr(),
        linebuf.len(),
        b"%d\0".as_ptr() as *const c_char,
        val,
    );

    if cpupower_write_sysfs(path.as_ptr(), linebuf.as_ptr(), 3) <= 0 {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_set_epp(cpu: c_uint, epp: *mut c_char) -> c_int {
    let mut path = [0 as c_char; 4096];
    let mut linebuf = [0 as c_char; 30];

    snprintf_path1(
        path.as_mut_ptr(),
        path.len(),
        b"cpu%u/cpufreq/energy_performance_preference\0".as_ptr() as *const c_char,
        cpu,
    );

    if !is_valid_path(path.as_ptr()) {
        return -1;
    }

    snprintf(
        linebuf.as_mut_ptr(),
        linebuf.len(),
        b"%s\0".as_ptr() as *const c_char,
        epp,
    );

    if cpupower_write_sysfs(path.as_ptr(), linebuf.as_ptr(), 30) <= 0 {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_set_amd_pstate_mode(mode: *mut c_char) -> c_int {
    let mut path = [0 as c_char; 4096];
    let mut linebuf = [0 as c_char; 20];

    snprintf_path0(
        path.as_mut_ptr(),
        path.len(),
        b"amd_pstate/status\0".as_ptr() as *const c_char,
    );

    if !is_valid_path(path.as_ptr()) {
        return -1;
    }

    snprintf(
        linebuf.as_mut_ptr(),
        linebuf.len(),
        b"%s\n\0".as_ptr() as *const c_char,
        mode,
    );

    if cpupower_write_sysfs(path.as_ptr(), linebuf.as_ptr(), 20) <= 0 {
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_amd_pstate_enabled() -> bool {
    let driver = cpufreq_get_driver(0);
    let mut ret = false;

    if driver.is_null() {
        return ret;
    }

    if strncmp(driver, b"amd\0".as_ptr() as *const c_char, 3) == 0 {
        ret = true;
    }

    cpufreq_put_driver(driver);

    ret
}

// #endif /* #if defined(__i386__) || defined(__x86_64__) */

#[no_mangle]
pub unsafe extern "C" fn cpufreq_has_generic_boost_support(active: *mut bool) -> c_int {
    let mut path = [0 as c_char; 4096];
    let mut linebuf = [0 as c_char; 2];
    let mut val: c_ulong;
    let mut endp: *mut c_char = std::ptr::null_mut();

    snprintf_path0(
        path.as_mut_ptr(),
        path.len(),
        b"cpufreq/boost\0".as_ptr() as *const c_char,
    );

    if !is_valid_path(path.as_ptr()) {
        return -EACCES;
    }

    if cpupower_read_sysfs(path.as_ptr(), linebuf.as_mut_ptr(), 2) <= 0 {
        return -EINVAL;
    }

    val = strtoul(linebuf.as_ptr(), &mut endp, 0);
    if endp == linebuf.as_mut_ptr() || errno == ERANGE {
        return -EINVAL;
    }

    match val {
        0 => {
            *active = false;
        }
        1 => {
            *active = true;
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

/* get_cpustate
 *
 * Gather the information of all online CPUs into bitmask struct
 */
#[no_mangle]
pub unsafe extern "C" fn get_cpustate() {
    let mut cpu: c_uint;

    bitmask_clearall(online_cpus);
    bitmask_clearall(offline_cpus);

    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if cpupower_is_cpu_online(cpu) == 1 {
            bitmask_setbit(online_cpus, cpu);
        } else {
            bitmask_setbit(offline_cpus, cpu);
        }

        cpu = cpu.wrapping_add(1);
        continue;
    }
}

/* print_online_cpus
 *
 * Print the CPU numbers of all CPUs that are online currently
 */
#[no_mangle]
pub unsafe extern "C" fn print_online_cpus() {
    let mut str_len: c_int = 0;
    let mut online_cpus_str: *mut c_char = std::ptr::null_mut();

    str_len = (*online_cpus).size as c_int * 5;
    online_cpus_str = malloc(std::mem::size_of::<c_char>() * str_len as usize) as *mut c_char;

    if bitmask_isallclear(online_cpus) == 0 {
        bitmask_displaylist(online_cpus_str, str_len, online_cpus);
        printf(
            gettext(b"Following CPUs are online:\n%s\n\0".as_ptr() as *const c_char),
            online_cpus_str,
        );
    }
}

/* print_offline_cpus
 *
 * Print the CPU numbers of all CPUs that are offline currently
 */
#[no_mangle]
pub unsafe extern "C" fn print_offline_cpus() {
    let mut str_len: c_int = 0;
    let mut offline_cpus_str: *mut c_char = std::ptr::null_mut();

    str_len = (*offline_cpus).size as c_int * 5;
    offline_cpus_str = malloc(std::mem::size_of::<c_char>() * str_len as usize) as *mut c_char;

    if bitmask_isallclear(offline_cpus) == 0 {
        bitmask_displaylist(offline_cpus_str, str_len, offline_cpus);
        printf(
            gettext(b"Following CPUs are offline:\n%s\n\0".as_ptr() as *const c_char),
            offline_cpus_str,
        );
        printf(gettext(
            b"cpupower set operation was not performed on them\n\0".as_ptr() as *const c_char,
        ));
    }
}

/*
 * print_speed
 *
 * Print the exact CPU frequency with appropriate unit
 */
#[no_mangle]
pub unsafe extern "C" fn print_speed(mut speed: c_ulong, no_rounding: c_int) {
    let mut tmp: c_ulong;

    if no_rounding != 0 {
        if speed > 1000000 {
            printf(
                b"%u.%06u GHz\0".as_ptr() as *const c_char,
                (speed as c_uint / 1000000),
                (speed as c_uint % 1000000),
            );
        } else if speed > 1000 {
            printf(
                b"%u.%03u MHz\0".as_ptr() as *const c_char,
                (speed as c_uint / 1000),
                (speed as c_uint % 1000),
            );
        } else {
            printf(b"%lu kHz\0".as_ptr() as *const c_char, speed);
        }
    } else if speed > 1000000 {
        tmp = speed % 10000;
        if tmp >= 5000 {
            speed = speed.wrapping_add(10000);
        }
        printf(
            b"%u.%02u GHz\0".as_ptr() as *const c_char,
            (speed as c_uint / 1000000),
            ((speed as c_uint % 1000000) / 10000),
        );
    } else if speed > 100000 {
        tmp = speed % 1000;
        if tmp >= 500 {
            speed = speed.wrapping_add(1000);
        }
        printf(
            b"%u MHz\0".as_ptr() as *const c_char,
            (speed as c_uint / 1000),
        );
    } else if speed > 1000 {
        tmp = speed % 100;
        if tmp >= 50 {
            speed = speed.wrapping_add(100);
        }
        printf(
            b"%u.%01u MHz\0".as_ptr() as *const c_char,
            (speed as c_uint / 1000),
            ((speed as c_uint % 1000) / 100),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_set_generic_turbo_boost(turbo_boost: c_int) -> c_int {
    let mut path = [0 as c_char; 4096];
    let mut linebuf = [0 as c_char; 2];

    snprintf_path0(
        path.as_mut_ptr(),
        path.len(),
        b"cpufreq/boost\0".as_ptr() as *const c_char,
    );

    if !is_valid_path(path.as_ptr()) {
        return -1;
    }

    snprintf(
        linebuf.as_mut_ptr(),
        linebuf.len(),
        b"%d\0".as_ptr() as *const c_char,
        turbo_boost,
    );

    if cpupower_write_sysfs(path.as_ptr(), linebuf.as_ptr(), 2) <= 0 {
        return -1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
