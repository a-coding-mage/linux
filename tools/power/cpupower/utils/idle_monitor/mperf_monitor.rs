// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::arch::asm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_int = i32;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_uint = u32;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_ulong = u64;
#[cfg(all(target_arch = "x86"))]
type c_ulong_arch = u32;
#[cfg(target_arch = "x86_64")]
type c_ulong_arch = u64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_ulonglong = u64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_double = f64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type c_char = i8;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type size_t = usize;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_APERF: c_uint = 0xE8;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_MPERF: c_uint = 0xE7;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const RDPRU_ECX_MPERF: c_uint = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const RDPRU_ECX_APERF: c_uint = 1;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_TSC: c_uint = 0x10;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MSR_AMD_HWCR: c_uint = 0xc0010015;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const C0: c_uint = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const Cx: c_uint = 1;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const AVG_FREQ: c_uint = 2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MPERF_CSTATE_COUNT: usize = 3;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MAX_FREQ_SYSFS: c_int = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MAX_FREQ_TSC_REF: c_int = 1;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" {
    static mut base_cpu: c_int;
    static mut cpu_count: c_int;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;

    static CPUPOWER_CAP_AMD_RDPRU: c_uint;
    static CPUPOWER_CAP_INV_TSC: c_uint;
    static CPUPOWER_CAP_APERF: c_uint;
    static X86_VENDOR_AMD: c_uint;
    static X86_VENDOR_HYGON: c_uint;
    static X86_VENDOR_INTEL: c_uint;
    static RANGE_THREAD: c_uint;
    static CLOCK_REALTIME: c_int;

    fn read_msr(cpu: c_int, reg: c_uint, val: *mut c_ulonglong) -> c_int;
    fn bind_cpu(cpu: c_int) -> c_int;
    fn timespec_diff_us(start: timespec, end: timespec) -> c_ulonglong;
    fn cpufreq_get_hardware_limits(cpu: c_uint, min: *mut c_ulong, max: *mut c_ulong) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn dprint(fmt: *const c_char, ...);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
struct cpupower_cpu_info_t {
    caps: c_uint,
    vendor: c_uint,
    family: c_uint,
    model: c_uint,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
struct cpuidle_monitor_flags {
    needs_root: c_uint,
    per_cpu_schedule: c_uint,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
struct cstate_t {
    name: *const c_char,
    desc: *const c_char,
    id: c_uint,
    range: c_uint,
    get_count_percent: Option<unsafe extern "C" fn(c_uint, *mut c_double, c_uint) -> c_int>,
    get_count: Option<unsafe extern "C" fn(c_uint, *mut c_ulonglong, c_uint) -> c_int>,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
struct cpuidle_monitor {
    name: *const c_char,
    name_len: size_t,
    hw_states_num: usize,
    hw_states: *mut cstate_t,
    start: Option<unsafe extern "C" fn() -> c_int>,
    stop: Option<unsafe extern "C" fn() -> c_int>,
    do_register: Option<unsafe extern "C" fn() -> *mut cpuidle_monitor>,
    unregister: Option<unsafe extern "C" fn()>,
    flags: cpuidle_monitor_flags,
    overflow_s: c_ulong,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut time_start: *mut timespec = core::ptr::null_mut();
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut time_end: *mut timespec = core::ptr::null_mut();

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut mperf_cstates: [cstate_t; MPERF_CSTATE_COUNT] = [
    cstate_t {
        name: c"C0".as_ptr(),
        desc: c"Processor Core not idle".as_ptr(),
        id: C0,
        range: 0,
        get_count_percent: Some(mperf_get_count_percent),
        get_count: None,
    },
    cstate_t {
        name: c"Cx".as_ptr(),
        desc: c"Processor Core in an idle state".as_ptr(),
        id: Cx,
        range: 0,
        get_count_percent: Some(mperf_get_count_percent),
        get_count: None,
    },
    cstate_t {
        name: c"Freq".as_ptr(),
        desc: c"Average Frequency (including boost) in MHz".as_ptr(),
        id: AVG_FREQ,
        range: 0,
        get_count_percent: None,
        get_count: Some(mperf_get_count_freq),
    },
];

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut max_freq_mode: c_int = 0;
/*
 * The max frequency mperf is ticking at (in C0), either retrieved via:
 *   1) calculated after measurements if we know TSC ticks at mperf/P0 frequency
 *   2) cpufreq /sys/devices/.../cpu0/cpufreq/cpuinfo_max_freq at init time
 * 1. Is preferred as it also works without cpufreq subsystem (e.g. on Xen)
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut max_frequency: c_ulong = 0;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut tsc_at_measure_start: *mut c_ulonglong = core::ptr::null_mut();
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut tsc_at_measure_end: *mut c_ulonglong = core::ptr::null_mut();
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut mperf_previous_count: *mut c_ulonglong = core::ptr::null_mut();
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut aperf_previous_count: *mut c_ulonglong = core::ptr::null_mut();
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut mperf_current_count: *mut c_ulonglong = core::ptr::null_mut();
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut aperf_current_count: *mut c_ulonglong = core::ptr::null_mut();

/* valid flag for all CPUs. If a MSR read failed it will be zero */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
static mut is_valid: *mut c_int = core::ptr::null_mut();

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_get_tsc(tsc: *mut c_ulonglong) -> c_int {
    let ret: c_int;

    ret = read_msr(base_cpu, MSR_TSC, tsc);
    if ret != 0 {
        dprint(c"Reading TSC MSR failed, returning %llu\n".as_ptr(), *tsc);
    }
    ret
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn get_aperf_mperf(
    cpu: c_int,
    aval: *mut c_ulonglong,
    mval: *mut c_ulonglong,
) -> c_int {
    let mut low_a: c_ulong_arch = 0;
    let mut high_a: c_ulong_arch = 0;
    let mut low_m: c_ulong_arch = 0;
    let mut high_m: c_ulong_arch = 0;
    let mut ret: c_int;

    /*
     * Running on the cpu from which we read the registers will
     * prevent APERF/MPERF from going out of sync because of IPI
     * latency introduced by read_msr()s.
     */
    if mperf_monitor.flags.per_cpu_schedule != 0 {
        if bind_cpu(cpu) != 0 {
            return 1;
        }
    }

    if (cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_RDPRU) != 0 {
        asm!(
            ".byte 0x0f, 0x01, 0xfd",
            out("eax") low_a,
            out("edx") high_a,
            in("ecx") RDPRU_ECX_APERF,
            options(nostack, preserves_flags),
        );
        asm!(
            ".byte 0x0f, 0x01, 0xfd",
            out("eax") low_m,
            out("edx") high_m,
            in("ecx") RDPRU_ECX_MPERF,
            options(nostack, preserves_flags),
        );

        *aval = (low_a | (high_a << 32)) as c_ulonglong;
        *mval = (low_m | (high_m << 32)) as c_ulonglong;

        return 0;
    }

    ret = read_msr(cpu, MSR_APERF, aval);
    ret |= read_msr(cpu, MSR_MPERF, mval);

    ret
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_init_stats(cpu: c_uint) -> c_int {
    let mut aval: c_ulonglong = 0;
    let mut mval: c_ulonglong = 0;
    let ret: c_int;

    ret = get_aperf_mperf(cpu as c_int, &mut aval, &mut mval);
    *aperf_previous_count.add(cpu as usize) = aval;
    *mperf_previous_count.add(cpu as usize) = mval;
    *is_valid.add(cpu as usize) = (ret == 0) as c_int;

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_measure_stats(cpu: c_uint) -> c_int {
    let mut aval: c_ulonglong = 0;
    let mut mval: c_ulonglong = 0;
    let ret: c_int;

    ret = get_aperf_mperf(cpu as c_int, &mut aval, &mut mval);
    *aperf_current_count.add(cpu as usize) = aval;
    *mperf_current_count.add(cpu as usize) = mval;
    *is_valid.add(cpu as usize) |= (ret == 0) as c_int;

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_get_count_percent(
    id: c_uint,
    percent: *mut c_double,
    cpu: c_uint,
) -> c_int {
    let aperf_diff: c_ulonglong;
    let mperf_diff: c_ulonglong;
    let tsc_diff: c_ulonglong;
    let timediff: c_ulonglong;

    if *is_valid.add(cpu as usize) == 0 {
        return -1;
    }

    if id != C0 && id != Cx {
        return -1;
    }

    mperf_diff = (*mperf_current_count.add(cpu as usize))
        .wrapping_sub(*mperf_previous_count.add(cpu as usize));
    aperf_diff = (*aperf_current_count.add(cpu as usize))
        .wrapping_sub(*aperf_previous_count.add(cpu as usize));

    if max_freq_mode == MAX_FREQ_TSC_REF {
        tsc_diff = (*tsc_at_measure_end.add(cpu as usize))
            .wrapping_sub(*tsc_at_measure_start.add(cpu as usize));
        *percent = 100.0 * mperf_diff as c_double / tsc_diff as c_double;
        dprint(
            c"%s: TSC Ref - mperf_diff: %llu, tsc_diff: %llu\n".as_ptr(),
            mperf_cstates[id as usize].name,
            mperf_diff,
            tsc_diff,
        );
    } else if max_freq_mode == MAX_FREQ_SYSFS {
        timediff = max_frequency
            .wrapping_mul(timespec_diff_us(
                *time_start.add(cpu as usize),
                *time_end.add(cpu as usize),
            ) as c_ulong) as c_ulonglong;
        *percent = 100.0 * mperf_diff as c_double / timediff as c_double;
        dprint(
            c"%s: MAXFREQ - mperf_diff: %llu, time_diff: %llu\n".as_ptr(),
            mperf_cstates[id as usize].name,
            mperf_diff,
            timediff,
        );
    } else {
        return -1;
    }

    if id == Cx {
        *percent = 100.0 - *percent;
    }

    dprint(
        c"%s: previous: %llu - current: %llu - (%u)\n".as_ptr(),
        mperf_cstates[id as usize].name,
        mperf_diff,
        aperf_diff,
        cpu,
    );
    dprint(c"%s: %f\n".as_ptr(), mperf_cstates[id as usize].name, *percent);
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_get_count_freq(
    id: c_uint,
    count: *mut c_ulonglong,
    cpu: c_uint,
) -> c_int {
    let aperf_diff: c_ulonglong;
    let mperf_diff: c_ulonglong;
    let time_diff: c_ulonglong;
    let tsc_diff: c_ulonglong;

    if id != AVG_FREQ {
        return 1;
    }

    if *is_valid.add(cpu as usize) == 0 {
        return -1;
    }

    mperf_diff = (*mperf_current_count.add(cpu as usize))
        .wrapping_sub(*mperf_previous_count.add(cpu as usize));
    aperf_diff = (*aperf_current_count.add(cpu as usize))
        .wrapping_sub(*aperf_previous_count.add(cpu as usize));

    if max_freq_mode == MAX_FREQ_TSC_REF {
        /* Calculate max_freq from TSC count */
        tsc_diff = (*tsc_at_measure_end.add(cpu as usize))
            .wrapping_sub(*tsc_at_measure_start.add(cpu as usize));
        time_diff = timespec_diff_us(*time_start.add(cpu as usize), *time_end.add(cpu as usize));
        max_frequency = (tsc_diff / time_diff) as c_ulong;
    }

    *count = (max_frequency as c_double * (aperf_diff as c_double / mperf_diff as c_double))
        as c_ulonglong;
    dprint(
        c"%s: Average freq based on %s maximum frequency:\n".as_ptr(),
        mperf_cstates[id as usize].name,
        if max_freq_mode == MAX_FREQ_TSC_REF {
            c"TSC calculated".as_ptr()
        } else {
            c"sysfs read".as_ptr()
        },
    );
    dprint(c"max_frequency: %lu\n".as_ptr(), max_frequency);
    dprint(c"aperf_diff: %llu\n".as_ptr(), aperf_diff);
    dprint(c"mperf_diff: %llu\n".as_ptr(), mperf_diff);
    dprint(c"avg freq:   %llu\n".as_ptr(), *count);
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_start() -> c_int {
    let mut cpu: c_int;

    cpu = 0;
    while cpu < cpu_count {
        clock_gettime(CLOCK_REALTIME, time_start.add(cpu as usize));
        mperf_get_tsc(tsc_at_measure_start.add(cpu as usize));
        mperf_init_stats(cpu as c_uint);
        cpu += 1;
    }

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn mperf_stop() -> c_int {
    let mut cpu: c_int;

    cpu = 0;
    while cpu < cpu_count {
        clock_gettime(CLOCK_REALTIME, time_end.add(cpu as usize));
        mperf_get_tsc(tsc_at_measure_end.add(cpu as usize));
        mperf_measure_stats(cpu as c_uint);
        cpu += 1;
    }

    0
}

/*
 * Mperf register is defined to tick at P0 (maximum) frequency
 *
 * Instead of reading out P0 which can be tricky to read out from HW,
 * we use TSC counter if it reliably ticks at P0/mperf frequency.
 *
 * Still try to fall back to:
 * /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq
 * on older Intel HW without invariant TSC feature.
 * Or on AMD machines where TSC does not tick at P0 (do not exist yet, but
 * it's still double checked (MSR_AMD_HWCR)).
 *
 * On these machines the user would still get useful mperf
 * stats when acpi-cpufreq driver is loaded.
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe extern "C" fn init_maxfreq_mode() -> c_int {
    let ret: c_int;
    let mut hwcr: c_ulonglong = 0;
    let mut min: c_ulong = 0;

    if (cpupower_cpu_info.caps & CPUPOWER_CAP_INV_TSC) == 0 {
        return use_sysfs(&mut min);
    }

    if cpupower_cpu_info.vendor == X86_VENDOR_AMD
        || cpupower_cpu_info.vendor == X86_VENDOR_HYGON
    {
        /* MSR_AMD_HWCR tells us whether TSC runs at P0/mperf
         * freq.
         * A test whether hwcr is accessable/available would be:
         * (cpupower_cpu_info.family > 0x10 ||
         *   cpupower_cpu_info.family == 0x10 &&
         *   cpupower_cpu_info.model >= 0x2))
         * This should be the case for all aperf/mperf
         * capable AMD machines and is therefore safe to test here.
         * Compare with Linus kernel git commit: acf01734b1747b1ec4
         */
        ret = read_msr(0, MSR_AMD_HWCR, &mut hwcr);
        /*
         * If the MSR read failed, assume a Xen system that did
         * not explicitly provide access to it and assume TSC works
        */
        if ret != 0 {
            dprint(
                c"TSC read 0x%x failed - assume TSC working\n".as_ptr(),
                MSR_AMD_HWCR,
            );
            return 0;
        } else if (1 & (hwcr >> 24)) != 0 {
            max_freq_mode = MAX_FREQ_TSC_REF;
            return 0;
        } else {
            /* Use sysfs max frequency if available */
        }
    } else if cpupower_cpu_info.vendor == X86_VENDOR_INTEL {
        /*
         * On Intel we assume mperf (in C0) is ticking at same
         * rate than TSC
         */
        max_freq_mode = MAX_FREQ_TSC_REF;
        return 0;
    }

    use_sysfs(&mut min)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn use_sysfs(min: *mut c_ulong) -> c_int {
    if cpufreq_get_hardware_limits(0, min, &mut max_frequency) != 0 {
        dprint(
            c"Cannot retrieve max freq from cpufreq kernel subsystem\n".as_ptr(),
        );
        return -1;
    }
    max_freq_mode = MAX_FREQ_SYSFS;
    max_frequency /= 1000; /* Default automatically to MHz value */
    0
}

/*
 * This monitor provides:
 *
 * 1) Average frequency a CPU resided in
 *    This always works if the CPU has aperf/mperf capabilities
 *
 * 2) C0 and Cx (any sleep state) time a CPU resided in
 *    Works if mperf timer stops ticking in sleep states which
 *    seem to be the case on all current HW.
 * Both is directly retrieved from HW registers and is independent
 * from kernel statistics.
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mperf_register() -> *mut cpuidle_monitor {
    if (cpupower_cpu_info.caps & CPUPOWER_CAP_APERF) == 0 {
        return core::ptr::null_mut();
    }

    if init_maxfreq_mode() != 0 {
        return core::ptr::null_mut();
    }

    if cpupower_cpu_info.vendor == X86_VENDOR_AMD {
        mperf_monitor.flags.per_cpu_schedule = 1;
    }

    /* Free this at program termination */
    is_valid = calloc(cpu_count as size_t, core::mem::size_of::<c_int>()) as *mut c_int;
    mperf_previous_count =
        calloc(cpu_count as size_t, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
    aperf_previous_count =
        calloc(cpu_count as size_t, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
    mperf_current_count =
        calloc(cpu_count as size_t, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
    aperf_current_count =
        calloc(cpu_count as size_t, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
    tsc_at_measure_start =
        calloc(cpu_count as size_t, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
    tsc_at_measure_end =
        calloc(cpu_count as size_t, core::mem::size_of::<c_ulonglong>()) as *mut c_ulonglong;
    time_start = calloc(cpu_count as size_t, core::mem::size_of::<timespec>()) as *mut timespec;
    time_end = calloc(cpu_count as size_t, core::mem::size_of::<timespec>()) as *mut timespec;
    mperf_monitor.name_len = strlen(mperf_monitor.name);
    &mut mperf_monitor
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mperf_unregister() {
    free(mperf_previous_count as *mut core::ffi::c_void);
    free(aperf_previous_count as *mut core::ffi::c_void);
    free(mperf_current_count as *mut core::ffi::c_void);
    free(aperf_current_count as *mut core::ffi::c_void);
    free(tsc_at_measure_start as *mut core::ffi::c_void);
    free(tsc_at_measure_end as *mut core::ffi::c_void);
    free(time_start as *mut core::ffi::c_void);
    free(time_end as *mut core::ffi::c_void);
    free(is_valid as *mut core::ffi::c_void);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
pub static mut mperf_monitor: cpuidle_monitor = cpuidle_monitor {
    name: c"Mperf".as_ptr(),
    name_len: 0,
    hw_states_num: MPERF_CSTATE_COUNT,
    hw_states: unsafe { mperf_cstates.as_mut_ptr() },
    start: Some(mperf_start),
    stop: Some(mperf_stop),
    do_register: Some(mperf_register),
    unregister: Some(mperf_unregister),
    flags: cpuidle_monitor_flags {
        needs_root: 1,
        per_cpu_schedule: 0,
    },
    overflow_s: 922000000, /* 922337203 seconds TSC overflow
                            * at 20GHz */
};
