// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_ulong, c_ulonglong};
use std::ptr;

const LINE_LEN: usize = 10;
const EINVAL: c_int = 22;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const UINT_MAX: c_ulong = c_uint::MAX as c_ulong;
const NO_ARGUMENT: c_int = 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

#[repr(C)]
pub struct cpufreq_policy {
    pub min: c_ulong,
    pub max: c_ulong,
    pub governor: *mut c_char,
}

#[repr(C)]
pub struct cpufreq_available_frequencies {
    pub next: *mut cpufreq_available_frequencies,
    pub first: *mut cpufreq_available_frequencies,
    pub frequency: c_ulong,
}

#[repr(C)]
pub struct cpufreq_available_governors {
    pub next: *mut cpufreq_available_governors,
    pub first: *mut cpufreq_available_governors,
    pub governor: *mut c_char,
}

#[repr(C)]
pub struct cpufreq_affected_cpus {
    pub next: *mut cpufreq_affected_cpus,
    pub first: *mut cpufreq_affected_cpus,
    pub cpu: c_uint,
}

#[repr(C)]
pub struct cpufreq_stats {
    pub next: *mut cpufreq_stats,
    pub first: *mut cpufreq_stats,
    pub frequency: c_ulong,
    pub time_in_state: c_ulonglong,
}

#[repr(C)]
pub struct cpupower_cpu_info_t {
    pub vendor: c_uint,
    pub family: c_uint,
    pub caps: c_uint,
}

#[repr(C)]
pub struct bitmask {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static mut cpupower_cpu_info: cpupower_cpu_info_t;
    static mut cpus_chosen: *mut bitmask;
    static mut base_cpu: c_uint;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn gettext(msgid: *const c_char) -> *mut c_char;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    fn cpufreq_get_policy(cpu: c_uint) -> *mut cpufreq_policy;
    fn cpufreq_put_policy(policy: *mut cpufreq_policy);
    fn cpufreq_get_hardware_limits(cpu: c_uint, min: *mut c_ulong, max: *mut c_ulong) -> c_int;
    fn cpufreq_has_x86_boost_support(
        cpu: c_uint,
        support: *mut c_int,
        active: *mut c_int,
        b_states: *mut c_int,
    ) -> c_int;
    fn decode_pstates(
        cpu: c_uint,
        boost_states: c_int,
        pstates: *mut c_ulong,
        pstate_no: *mut c_int,
    ) -> c_int;
    fn msr_intel_get_turbo_ratio(cpu: c_uint) -> c_ulonglong;
    fn cpufreq_has_generic_boost_support(active: *mut bool) -> c_int;
    fn cpufreq_get_boost_frequencies(cpu: c_uint) -> *mut cpufreq_available_frequencies;
    fn cpufreq_put_available_frequencies(freqs: *mut cpufreq_available_frequencies);
    fn cpufreq_get_freq_kernel(cpu: c_uint) -> c_ulong;
    fn cpufreq_get_freq_hardware(cpu: c_uint) -> c_ulong;
    fn cpufreq_get_driver(cpu: c_uint) -> *mut c_char;
    fn cpufreq_put_driver(driver: *mut c_char);
    fn cpufreq_get_available_governors(cpu: c_uint) -> *mut cpufreq_available_governors;
    fn cpufreq_put_available_governors(governors: *mut cpufreq_available_governors);
    fn cpufreq_get_affected_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus;
    fn cpufreq_put_affected_cpus(cpus: *mut cpufreq_affected_cpus);
    fn cpufreq_get_related_cpus(cpu: c_uint) -> *mut cpufreq_affected_cpus;
    fn cpufreq_put_related_cpus(cpus: *mut cpufreq_affected_cpus);
    fn cpufreq_get_transitions(cpu: c_uint) -> c_ulong;
    fn cpufreq_get_stats(cpu: c_uint, total_time: *mut c_ulonglong) -> *mut cpufreq_stats;
    fn cpufreq_put_stats(stats: *mut cpufreq_stats);
    fn cpufreq_get_energy_performance_preference(cpu: c_uint) -> *mut c_char;
    fn cpufreq_put_energy_performance_preference(epp: *mut c_char);
    fn cpufreq_get_transition_latency(cpu: c_uint) -> c_ulong;
    fn amd_pstate_show_perf_and_freq(cpu: c_uint, no_rounding: c_int);
    fn cppc_show_perf_and_freq(cpu: c_uint, no_rounding: c_int);

    fn print_speed(speed: c_ulong, no_rounding: c_int);
    fn dprint(format: *const c_char, ...);
    fn sysfs_is_cpu_online(cpu: c_uint) -> c_int;
    fn bitmask_isallclear(mask: *mut bitmask) -> c_int;
    fn bitmask_setbit(mask: *mut bitmask, bit: c_uint) -> c_int;
    fn bitmask_first(mask: *mut bitmask) -> c_uint;
    fn bitmask_last(mask: *mut bitmask) -> c_uint;
    fn bitmask_isbitset(mask: *mut bitmask, bit: c_uint) -> c_int;
}

unsafe fn tr(msgid: *const c_char) -> *const c_char {
    gettext(msgid) as *const c_char
}

// Constants and build-time symbols supplied by cpupower headers.
const X86_VENDOR_AMD: c_uint = 2;
const X86_VENDOR_HYGON: c_uint = 4;
const X86_VENDOR_INTEL: c_uint = 1;
const CPUPOWER_CAP_AMD_PSTATE: c_uint = 1 << 0;
const CPUPOWER_CAP_HAS_TURBO_RATIO: c_uint = 1 << 1;
const CPUPOWER_CAP_IS_SNB: c_uint = 1 << 2;
const CPUPOWER_CAP_APERF: c_uint = 1 << 3;
const MAX_HW_PSTATES: usize = 32;

unsafe fn count_cpus() -> c_uint {
    let mut value = [0 as c_char; LINE_LEN];
    let mut ret: c_uint = 0;
    let mut cpunr: c_uint = 0;

    let fp = fopen(c"/proc/stat".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        printf(
            tr(c"Couldn't count the number of CPUs (%s: %s), assuming 1\n".as_ptr()),
            c"/proc/stat".as_ptr(),
            strerror(errno),
        );
        return 1;
    }

    while feof(fp) == 0 {
        if fgets(value.as_mut_ptr(), LINE_LEN as c_int, fp).is_null() {
            continue;
        }
        value[LINE_LEN - 1] = b'\0' as c_char;
        if strlen(value.as_ptr()) < LINE_LEN - 2 {
            continue;
        }
        if !strstr(value.as_ptr(), c"cpu ".as_ptr()).is_null() {
            continue;
        }
        if sscanf(value.as_ptr(), c"cpu%d ".as_ptr(), &mut cpunr) != 1 {
            continue;
        }
        if cpunr > ret {
            ret = cpunr;
        }
    }
    fclose(fp);

    /* cpu count starts from 0, on error return 1 (UP) */
    ret + 1
}

unsafe fn proc_cpufreq_output() {
    let mut cpu: c_uint;
    let nr_cpus: c_uint;
    let mut policy: *mut cpufreq_policy;
    let mut min_pctg: c_uint = 0;
    let mut max_pctg: c_uint = 0;
    let mut min: c_ulong = 0;
    let mut max: c_ulong = 0;

    printf(tr(c"          minimum CPU frequency  -  maximum CPU frequency  -  governor\n".as_ptr()));

    nr_cpus = count_cpus();
    cpu = 0;
    while cpu < nr_cpus {
        policy = cpufreq_get_policy(cpu);
        if policy.is_null() {
            cpu += 1;
            continue;
        }

        if cpufreq_get_hardware_limits(cpu, &mut min, &mut max) != 0 {
            max = 0;
        } else {
            min_pctg = ((*policy).min * 100) as c_uint / max as c_uint;
            max_pctg = ((*policy).max * 100) as c_uint / max as c_uint;
        }
        printf(
            c"CPU%3d    %9lu kHz (%3d %%)  -  %9lu kHz (%3d %%)  -  %s\n".as_ptr(),
            cpu,
            (*policy).min,
            if max != 0 { min_pctg } else { 0 },
            (*policy).max,
            if max != 0 { max_pctg } else { 0 },
            (*policy).governor,
        );

        cpufreq_put_policy(policy);
        cpu += 1;
    }
}

static mut no_rounding: c_int = 0;

unsafe fn print_duration(mut duration: c_ulong) {
    let mut tmp: c_ulong;

    if no_rounding != 0 {
        if duration > 1000000 {
            printf(
                c"%u.%06u ms".as_ptr(),
                (duration as c_uint) / 1000000,
                (duration as c_uint) % 1000000,
            );
        } else if duration > 100000 {
            printf(c"%u us".as_ptr(), (duration as c_uint) / 1000);
        } else if duration > 1000 {
            printf(
                c"%u.%03u us".as_ptr(),
                (duration as c_uint) / 1000,
                (duration as c_uint) % 1000,
            );
        } else {
            printf(c"%lu ns".as_ptr(), duration);
        }
    } else if duration > 1000000 {
        tmp = duration % 10000;
        if tmp >= 5000 {
            duration += 10000;
        }
        printf(
            c"%u.%02u ms".as_ptr(),
            (duration as c_uint) / 1000000,
            ((duration % 1000000) as c_uint) / 10000,
        );
    } else if duration > 100000 {
        tmp = duration % 1000;
        if tmp >= 500 {
            duration += 1000;
        }
        printf(c"%u us".as_ptr(), (duration as c_uint) / 1000);
    } else if duration > 1000 {
        tmp = duration % 100;
        if tmp >= 50 {
            duration += 100;
        }
        printf(
            c"%u.%01u us".as_ptr(),
            (duration as c_uint) / 1000,
            ((duration % 1000) as c_uint) / 100,
        );
    } else {
        printf(c"%lu ns".as_ptr(), duration);
    }
}

unsafe fn get_boost_mode_x86(cpu: c_uint) -> c_int {
    let mut support: c_int = 0;
    let mut active: c_int = 0;
    let mut b_states: c_int = 0;
    let mut ret: c_int;
    let mut pstate_no: c_int = 0;
    let mut i: c_int;
    /* ToDo: Make this more global */
    let mut pstates = [0 as c_ulong; MAX_HW_PSTATES];

    ret = cpufreq_has_x86_boost_support(cpu, &mut support, &mut active, &mut b_states);
    if ret != 0 {
        printf(
            tr(c"Error while evaluating Boost Capabilities on CPU %d -- are you root?\n".as_ptr()),
            cpu,
        );
        return ret;
    }
    /* P state changes via MSR are identified via cpuid 80000007
       on Intel and AMD, but we assume boost capable machines can do that
       if (cpuid_eax(0x80000000) >= 0x80000007
       && (cpuid_edx(0x80000007) & (1 << 7)))
    */

    printf(tr(c"  boost state support:\n".as_ptr()));

    printf(
        tr(c"    Supported: %s\n".as_ptr()),
        if support != 0 { tr(c"yes".as_ptr()) } else { tr(c"no".as_ptr()) },
    );
    printf(
        tr(c"    Active: %s\n".as_ptr()),
        if active != 0 { tr(c"yes".as_ptr()) } else { tr(c"no".as_ptr()) },
    );

    if cpupower_cpu_info.vendor == X86_VENDOR_AMD
        && (cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_PSTATE) != 0
    {
        return 0;
    } else if (cpupower_cpu_info.vendor == X86_VENDOR_AMD && cpupower_cpu_info.family >= 0x10)
        || cpupower_cpu_info.vendor == X86_VENDOR_HYGON
    {
        ret = decode_pstates(cpu, b_states, pstates.as_mut_ptr(), &mut pstate_no);
        if ret != 0 {
            return ret;
        }

        printf(tr(c"    Boost States: %d\n".as_ptr()), b_states);
        printf(tr(c"    Total States: %d\n".as_ptr()), pstate_no);
        i = 0;
        while i < pstate_no {
            if pstates[i as usize] == 0 {
                i += 1;
                continue;
            }
            if i < b_states {
                printf(
                    tr(c"    Pstate-Pb%d: %luMHz (boost state)\n".as_ptr()),
                    i,
                    pstates[i as usize],
                );
            } else {
                printf(
                    tr(c"    Pstate-P%d:  %luMHz\n".as_ptr()),
                    i - b_states,
                    pstates[i as usize],
                );
            }
            i += 1;
        }
    } else if (cpupower_cpu_info.caps & CPUPOWER_CAP_HAS_TURBO_RATIO) != 0 {
        let bclk: c_double;
        let mut intel_turbo_ratio: c_ulonglong = 0;
        let mut ratio: c_uint;

        /* Any way to autodetect this ? */
        if (cpupower_cpu_info.caps & CPUPOWER_CAP_IS_SNB) != 0 {
            bclk = 100.00;
        } else {
            bclk = 133.33;
        }
        intel_turbo_ratio = msr_intel_get_turbo_ratio(cpu);
        dprint(
            c"    Ratio: 0x%llx - bclk: %f\n".as_ptr(),
            intel_turbo_ratio,
            bclk,
        );

        ratio = ((intel_turbo_ratio >> 24) & 0xFF) as c_uint;
        if ratio != 0 {
            printf(tr(c"    %.0f MHz max turbo 4 active cores\n".as_ptr()), ratio as c_double * bclk);
        }

        ratio = ((intel_turbo_ratio >> 16) & 0xFF) as c_uint;
        if ratio != 0 {
            printf(tr(c"    %.0f MHz max turbo 3 active cores\n".as_ptr()), ratio as c_double * bclk);
        }

        ratio = ((intel_turbo_ratio >> 8) & 0xFF) as c_uint;
        if ratio != 0 {
            printf(tr(c"    %.0f MHz max turbo 2 active cores\n".as_ptr()), ratio as c_double * bclk);
        }

        ratio = ((intel_turbo_ratio >> 0) & 0xFF) as c_uint;
        if ratio != 0 {
            printf(tr(c"    %.0f MHz max turbo 1 active cores\n".as_ptr()), ratio as c_double * bclk);
        }
    }
    0
}

unsafe fn get_boost_mode_generic(_cpu: c_uint) -> c_int {
    let mut active = false;

    if cpufreq_has_generic_boost_support(&mut active) == 0 {
        printf(tr(c"  boost state support:\n".as_ptr()));
        printf(
            tr(c"    Active: %s\n".as_ptr()),
            if active { tr(c"yes".as_ptr()) } else { tr(c"no".as_ptr()) },
        );
    }

    0
}

/* --boost / -b */

unsafe fn get_boost_mode(cpu: c_uint) -> c_int {
    let mut freqs: *mut cpufreq_available_frequencies;
    let first_freqs: *mut cpufreq_available_frequencies;

    if cpupower_cpu_info.vendor == X86_VENDOR_AMD
        || cpupower_cpu_info.vendor == X86_VENDOR_HYGON
        || cpupower_cpu_info.vendor == X86_VENDOR_INTEL
    {
        return get_boost_mode_x86(cpu);
    } else {
        get_boost_mode_generic(cpu);
    }

    freqs = cpufreq_get_boost_frequencies(cpu);
    first_freqs = freqs;
    if !freqs.is_null() {
        printf(tr(c"  boost frequency steps: ".as_ptr()));
        while !(*freqs).next.is_null() {
            print_speed((*freqs).frequency, no_rounding);
            printf(c", ".as_ptr());
            freqs = (*freqs).next;
        }
        print_speed((*freqs).frequency, no_rounding);
        printf(c"\n".as_ptr());
        cpufreq_put_available_frequencies(first_freqs);
    }

    0
}

/* --freq / -f */

unsafe fn get_freq_kernel(cpu: c_uint, human: c_uint) -> c_int {
    let freq = cpufreq_get_freq_kernel(cpu);
    printf(tr(c"  current CPU frequency: ".as_ptr()));
    if freq == 0 {
        printf(tr(c" Unable to call to kernel\n".as_ptr()));
        return -EINVAL;
    }
    if human != 0 {
        print_speed(freq, no_rounding);
    } else {
        printf(c"%lu".as_ptr(), freq);
    }
    printf(tr(c" (asserted by call to kernel)\n".as_ptr()));
    0
}

/* --hwfreq / -w */

unsafe fn get_freq_hardware(cpu: c_uint, human: c_uint) -> c_int {
    let freq: c_ulong;

    freq = cpufreq_get_freq_hardware(cpu);
    if (cpupower_cpu_info.caps & CPUPOWER_CAP_APERF) == 0 && freq == 0 {
        return -EINVAL;
    }

    printf(tr(c"  current CPU frequency: ".as_ptr()));
    if freq == 0 {
        printf(c"Unable to call hardware\n".as_ptr());
        return -EINVAL;
    }
    if human != 0 {
        print_speed(freq, no_rounding);
    } else {
        printf(c"%lu".as_ptr(), freq);
    }
    printf(tr(c" (asserted by call to hardware)\n".as_ptr()));
    0
}

/* --hwlimits / -l */

unsafe fn get_hardware_limits(cpu: c_uint, human: c_uint) -> c_int {
    let mut min: c_ulong = 0;
    let mut max: c_ulong = 0;

    if cpufreq_get_hardware_limits(cpu, &mut min, &mut max) != 0 {
        printf(tr(c"Not Available\n".as_ptr()));
        return -EINVAL;
    }

    if human != 0 {
        printf(tr(c"  hardware limits: ".as_ptr()));
        print_speed(min, no_rounding);
        printf(c" - ".as_ptr());
        print_speed(max, no_rounding);
        printf(c"\n".as_ptr());
    } else {
        printf(c"%lu %lu\n".as_ptr(), min, max);
    }
    0
}

/* --driver / -d */

unsafe fn get_driver(cpu: c_uint) -> c_int {
    let driver = cpufreq_get_driver(cpu);
    if driver.is_null() {
        printf(tr(c"  no or unknown cpufreq driver is active on this CPU\n".as_ptr()));
        return -EINVAL;
    }
    printf(c"  driver: %s\n".as_ptr(), driver);
    cpufreq_put_driver(driver);
    0
}

/* --policy / -p */

unsafe fn get_policy(cpu: c_uint) -> c_int {
    let policy = cpufreq_get_policy(cpu);
    if policy.is_null() {
        printf(tr(c"  Unable to determine current policy\n".as_ptr()));
        return -EINVAL;
    }
    printf(tr(c"  current policy: frequency should be within ".as_ptr()));
    print_speed((*policy).min, no_rounding);
    printf(tr(c" and ".as_ptr()));
    print_speed((*policy).max, no_rounding);

    printf(c".\n                  ".as_ptr());
    printf(
        tr(c"The governor \"%s\" may decide which speed to use\n                  within this range.\n".as_ptr()),
        (*policy).governor,
    );
    cpufreq_put_policy(policy);
    0
}

/* --governors / -g */

unsafe fn get_available_governors(cpu: c_uint) -> c_int {
    let mut governors = cpufreq_get_available_governors(cpu);
    let first_governors = governors;

    printf(tr(c"  available cpufreq governors: ".as_ptr()));
    if governors.is_null() {
        printf(tr(c"Not Available\n".as_ptr()));
        return -EINVAL;
    }

    while !(*governors).next.is_null() {
        printf(c"%s ".as_ptr(), (*governors).governor);
        governors = (*governors).next;
    }
    printf(c"%s\n".as_ptr(), (*governors).governor);
    cpufreq_put_available_governors(first_governors);
    0
}

/* --affected-cpus  / -a */

unsafe fn get_affected_cpus(cpu: c_uint) -> c_int {
    let mut cpus = cpufreq_get_affected_cpus(cpu);
    let first_cpus = cpus;

    printf(tr(c"  CPUs which need to have their frequency coordinated by software: ".as_ptr()));
    if cpus.is_null() {
        printf(tr(c"Not Available\n".as_ptr()));
        return -EINVAL;
    }

    while !(*cpus).next.is_null() {
        printf(c"%d ".as_ptr(), (*cpus).cpu);
        cpus = (*cpus).next;
    }
    printf(c"%d\n".as_ptr(), (*cpus).cpu);
    cpufreq_put_affected_cpus(first_cpus);
    0
}

/* --related-cpus  / -r */

unsafe fn get_related_cpus(cpu: c_uint) -> c_int {
    let mut cpus = cpufreq_get_related_cpus(cpu);
    let first_cpus = cpus;

    printf(tr(c"  CPUs which run at the same hardware frequency: ".as_ptr()));
    if cpus.is_null() {
        printf(tr(c"Not Available\n".as_ptr()));
        return -EINVAL;
    }

    while !(*cpus).next.is_null() {
        printf(c"%d ".as_ptr(), (*cpus).cpu);
        cpus = (*cpus).next;
    }
    printf(c"%d\n".as_ptr(), (*cpus).cpu);
    cpufreq_put_related_cpus(first_cpus);
    0
}

/* --stats / -s */

unsafe fn get_freq_stats(cpu: c_uint, human: c_uint) -> c_int {
    let total_trans = cpufreq_get_transitions(cpu);
    let mut total_time: c_ulonglong = 0;
    let mut stats = cpufreq_get_stats(cpu, &mut total_time);
    let first_stats = stats;
    while !stats.is_null() {
        if human != 0 {
            print_speed((*stats).frequency, no_rounding);
            printf(
                c":%.2f%%".as_ptr(),
                (100.0 * (*stats).time_in_state as c_double) / total_time as c_double,
            );
        } else {
            printf(c"%lu:%llu".as_ptr(), (*stats).frequency, (*stats).time_in_state);
        }
        stats = (*stats).next;
        if !stats.is_null() {
            printf(c", ".as_ptr());
        }
    }
    cpufreq_put_stats(first_stats);
    if total_trans != 0 {
        printf(c"  (%lu)\n".as_ptr(), total_trans);
    }
    0
}

/* --epp / -z */

unsafe fn get_epp(cpu: c_uint, interactive: bool) -> c_int {
    let epp: *mut c_char;

    epp = cpufreq_get_energy_performance_preference(cpu);
    if epp.is_null() {
        return -EINVAL;
    }
    if interactive {
        printf(tr(c"  energy performance preference: %s\n".as_ptr()), epp);
    }

    cpufreq_put_energy_performance_preference(epp);

    0
}

/* --latency / -y */

unsafe fn get_latency(cpu: c_uint, human: c_uint) -> c_int {
    let latency = cpufreq_get_transition_latency(cpu);

    if get_epp(cpu, false) == 0 {
        return -EINVAL;
    }

    printf(tr(c"  maximum transition latency: ".as_ptr()));
    if latency == 0 || latency == UINT_MAX {
        printf(tr(c" Cannot determine or is not supported.\n".as_ptr()));
        return -EINVAL;
    }

    if human != 0 {
        print_duration(latency);
        printf(c"\n".as_ptr());
    } else {
        printf(c"%lu\n".as_ptr(), latency);
    }
    0
}

/* --performance / -c */
unsafe fn get_perf_cap(cpu: c_uint) -> c_int {
    if cpupower_cpu_info.vendor == X86_VENDOR_AMD
        && (cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_PSTATE) != 0
    {
        amd_pstate_show_perf_and_freq(cpu, no_rounding);
    } else {
        cppc_show_perf_and_freq(cpu, no_rounding);
    }

    0
}

unsafe fn debug_output_one(cpu: c_uint) {
    let mut freqs: *mut cpufreq_available_frequencies;
    let first_freqs: *mut cpufreq_available_frequencies;

    get_driver(cpu);
    get_related_cpus(cpu);
    get_affected_cpus(cpu);
    get_latency(cpu, 1);
    get_epp(cpu, true);
    get_hardware_limits(cpu, 1);

    freqs = cpufreq_get_available_frequencies(cpu);
    first_freqs = freqs;
    if !freqs.is_null() {
        printf(tr(c"  available frequency steps:  ".as_ptr()));
        while !(*freqs).next.is_null() {
            print_speed((*freqs).frequency, no_rounding);
            printf(c", ".as_ptr());
            freqs = (*freqs).next;
        }
        print_speed((*freqs).frequency, no_rounding);
        printf(c"\n".as_ptr());
        cpufreq_put_available_frequencies(first_freqs);
    }

    get_available_governors(cpu);
    get_policy(cpu);
    get_freq_hardware(cpu, 1);
    get_freq_kernel(cpu, 1);
    get_boost_mode(cpu);
    get_perf_cap(cpu);
}

unsafe extern "C" {
    fn cpufreq_get_available_frequencies(cpu: c_uint) -> *mut cpufreq_available_frequencies;
}

static mut INFO_OPTS: [option; 18] = [
    option { name: c"debug".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'e' as c_int },
    option { name: c"boost".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'b' as c_int },
    option { name: c"freq".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'f' as c_int },
    option { name: c"hwfreq".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'w' as c_int },
    option { name: c"hwlimits".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'l' as c_int },
    option { name: c"driver".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'd' as c_int },
    option { name: c"policy".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'p' as c_int },
    option { name: c"governors".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'g' as c_int },
    option { name: c"related-cpus".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'r' as c_int },
    option { name: c"affected-cpus".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'a' as c_int },
    option { name: c"stats".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b's' as c_int },
    option { name: c"latency".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'y' as c_int },
    option { name: c"proc".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'o' as c_int },
    option { name: c"human".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'm' as c_int },
    option { name: c"no-rounding".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'n' as c_int },
    option { name: c"performance".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'c' as c_int },
    option { name: c"epp".as_ptr(), has_arg: NO_ARGUMENT, flag: ptr::null_mut(), val: b'z' as c_int },
    option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_freq_info(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;
    let mut cont: c_int = 1;
    let mut cpu: c_uint;
    let mut human: c_uint = 0;
    let mut output_param: c_int = 0;

    while {
        ret = getopt_long(
            argc,
            argv,
            c"oefwldpgrasmybncz".as_ptr(),
            INFO_OPTS.as_ptr(),
            ptr::null_mut(),
        );
        match ret {
            x if x == b'?' as c_int => {
                output_param = b'?' as c_int;
                cont = 0;
            }
            -1 => {
                cont = 0;
            }
            x if x == b'b' as c_int
                || x == b'o' as c_int
                || x == b'a' as c_int
                || x == b'r' as c_int
                || x == b'g' as c_int
                || x == b'p' as c_int
                || x == b'd' as c_int
                || x == b'l' as c_int
                || x == b'w' as c_int
                || x == b'f' as c_int
                || x == b'e' as c_int
                || x == b's' as c_int
                || x == b'y' as c_int
                || x == b'c' as c_int
                || x == b'z' as c_int =>
            {
                if output_param != 0 {
                    output_param = -1;
                    cont = 0;
                } else {
                    output_param = ret;
                }
            }
            x if x == b'm' as c_int => {
                if human != 0 {
                    output_param = -1;
                    cont = 0;
                } else {
                    human = 1;
                }
            }
            x if x == b'n' as c_int => {
                no_rounding = 1;
            }
            _ => {
                fprintf(stderr, c"invalid or unknown argument\n".as_ptr());
                return EXIT_FAILURE;
            }
        }
        cont != 0
    } {}

    match output_param {
        x if x == b'o' as c_int => {
            if bitmask_isallclear(cpus_chosen) == 0 {
                printf(tr(c"The argument passed to this tool can't be combined with passing a --cpu argument\n".as_ptr()));
                return -EINVAL;
            }
        }
        0 => {
            output_param = b'e' as c_int;
        }
        _ => {}
    }

    ret = 0;

    /* Default is: show output of base_cpu only */
    if bitmask_isallclear(cpus_chosen) != 0 {
        bitmask_setbit(cpus_chosen, base_cpu);
    }

    match output_param {
        -1 => {
            printf(tr(c"You can't specify more than one --cpu parameter and/or\nmore than one output-specific argument\n".as_ptr()));
            return -EINVAL;
        }
        x if x == b'?' as c_int => {
            printf(tr(c"invalid or unknown argument\n".as_ptr()));
            return -EINVAL;
        }
        x if x == b'o' as c_int => {
            proc_cpufreq_output();
            return EXIT_SUCCESS;
        }
        _ => {}
    }

    cpu = bitmask_first(cpus_chosen);
    while cpu <= bitmask_last(cpus_chosen) {
        if bitmask_isbitset(cpus_chosen, cpu) == 0 {
            cpu += 1;
            continue;
        }

        printf(tr(c"analyzing CPU %d:\n".as_ptr()), cpu);

        if sysfs_is_cpu_online(cpu) != 1 {
            printf(tr(c" *is offline\n".as_ptr()));
            printf(c"\n".as_ptr());
            cpu += 1;
            continue;
        }

        match output_param {
            x if x == b'b' as c_int => {
                get_boost_mode(cpu);
            }
            x if x == b'e' as c_int => {
                debug_output_one(cpu);
            }
            x if x == b'a' as c_int => {
                ret = get_affected_cpus(cpu);
            }
            x if x == b'r' as c_int => {
                ret = get_related_cpus(cpu);
            }
            x if x == b'g' as c_int => {
                ret = get_available_governors(cpu);
            }
            x if x == b'p' as c_int => {
                ret = get_policy(cpu);
            }
            x if x == b'd' as c_int => {
                ret = get_driver(cpu);
            }
            x if x == b'l' as c_int => {
                ret = get_hardware_limits(cpu, human);
            }
            x if x == b'w' as c_int => {
                ret = get_freq_hardware(cpu, human);
            }
            x if x == b'f' as c_int => {
                ret = get_freq_kernel(cpu, human);
            }
            x if x == b's' as c_int => {
                ret = get_freq_stats(cpu, human);
            }
            x if x == b'y' as c_int => {
                ret = get_latency(cpu, human);
            }
            x if x == b'c' as c_int => {
                ret = get_perf_cap(cpu);
            }
            x if x == b'z' as c_int => {
                ret = get_epp(cpu, true);
            }
            _ => {}
        }
        if ret != 0 {
            return ret;
        }
        cpu += 1;
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
