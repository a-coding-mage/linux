// SPDX-License-Identifier: GPL-2.0-only
/*
 * x86_energy_perf_policy -- set the energy versus performance
 * policy preference bias on recent X86 processors.
 */
/*
 * Copyright (c) 2010 - 2026 Intel Corporation.
 * Len Brown <len.brown@intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type FILE = c_void;
type cpu_set_t = c_void;

/* Includes in the C source provide libc, getopt, cpuid, errno, and MSR symbols. */
#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut errno: c_int;

    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn warn(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn rewind(stream: *mut FILE);
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strcspn(s: *const c_char, reject: *const c_char) -> size_t;
    fn atoi(nptr: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: c_long) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: c_long) -> ssize_t;
    fn getopt_long_only(argc: c_int, argv: *const *mut c_char, optstring: *const c_char,
                        longopts: *const option, longindex: *mut c_int) -> c_int;
    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, set: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, set: *mut cpu_set_t);
    fn CPU_ISSET_S(cpu: c_int, setsize: size_t, set: *const cpu_set_t) -> c_int;
    fn CPU_COUNT_S(setsize: size_t, set: *const cpu_set_t) -> c_int;
    fn sched_getcpu() -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn __get_cpuid(leaf: c_uint, eax: *mut c_uint, ebx: *mut c_uint,
                   ecx: *mut c_uint, edx: *mut c_uint) -> c_int;
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

const INT_MAX: c_int = 2147483647;
const INT_MIN: c_int = -2147483648;
const LONG_MAX: c_long = c_long::MAX;
const LONG_MIN: c_long = c_long::MIN;
const ERANGE: c_int = 34;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const R_OK: c_int = 4;
const W_OK: c_int = 2;
const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const SEEK_SET: c_int = 0;
const required_argument: c_int = 1;
const no_argument: c_int = 0;

const OPTARG_NORMAL: c_int = INT_MAX - 1;
const OPTARG_POWER: c_int = INT_MAX - 2;
const OPTARG_BALANCE_POWER: c_int = INT_MAX - 3;
const OPTARG_BALANCE_PERFORMANCE: c_int = INT_MAX - 4;
const OPTARG_PERFORMANCE: c_int = INT_MAX - 5;

/* Constants normally supplied by MSRHEADER. */
unsafe extern "C" {
    static ENERGY_PERF_BIAS_POWERSAVE: c_int;
    static ENERGY_PERF_BIAS_BALANCE_POWERSAVE: c_int;
    static ENERGY_PERF_BIAS_NORMAL: c_int;
    static ENERGY_PERF_BIAS_BALANCE_PERFORMANCE: c_int;
    static ENERGY_PERF_BIAS_PERFORMANCE: c_int;
    static HWP_EPP_POWERSAVE: c_int;
    static HWP_EPP_BALANCE_POWERSAVE: c_int;
    static HWP_EPP_BALANCE_PERFORMANCE: c_int;
    static HWP_EPP_PERFORMANCE: c_int;
    static MSR_HWP_REQUEST: c_int;
    static MSR_HWP_CAPABILITIES: c_int;
    static MSR_HWP_REQUEST_PKG: c_int;
    static MSR_HWP_INTERRUPT: c_int;
    static MSR_HWP_STATUS: c_int;
    static MSR_PM_ENABLE: c_int;
    static MSR_IA32_MISC_ENABLE: c_int;
    static MSR_IA32_MISC_ENABLE_TURBO_DISABLE: c_ulonglong;
    static MSR_TURBO_RATIO_LIMIT: c_int;
}

fn HWP_HIGHEST_PERF(msr: c_ulonglong) -> c_int { ((msr >> 0) & 0xff) as c_int }
fn HWP_GUARANTEED_PERF(msr: c_ulonglong) -> c_int { ((msr >> 8) & 0xff) as c_int }
fn HWP_MOSTEFFICIENT_PERF(msr: c_ulonglong) -> c_int { ((msr >> 16) & 0xff) as c_int }
fn HWP_LOWEST_PERF(msr: c_ulonglong) -> c_int { ((msr >> 24) & 0xff) as c_int }
fn HWP_MIN_PERF(x: c_int) -> c_ulonglong { ((x as c_ulonglong) & 0xff) << 0 }
fn HWP_MAX_PERF(x: c_int) -> c_ulonglong { ((x as c_ulonglong) & 0xff) << 8 }
fn HWP_DESIRED_PERF(x: c_int) -> c_ulonglong { ((x as c_ulonglong) & 0xff) << 16 }
fn HWP_ENERGY_PERF_PREFERENCE(x: u8) -> c_ulonglong { ((x as c_ulonglong) & 0xff) << 24 }
fn HWP_ACTIVITY_WINDOW(x: c_uint) -> c_ulonglong { ((x as c_ulonglong) & 0x3ff) << 32 }
fn HWP_PACKAGE_CONTROL(x: u8) -> c_ulonglong { ((x as c_ulonglong) & 0x1) << 42 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_hwp_cap {
    pub highest: u8,
    pub guaranteed: u8,
    pub efficient: u8,
    pub lowest: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_hwp_request {
    pub hwp_min: u8,
    pub hwp_max: u8,
    pub hwp_desired: u8,
    pub hwp_epp: u8,
    pub hwp_window: c_uint,
    pub hwp_use_pkg: u8,
}

static mut req_update: msr_hwp_request =
    msr_hwp_request { hwp_min: 0, hwp_max: 0, hwp_desired: 0, hwp_epp: 0, hwp_window: 0, hwp_use_pkg: 0 };

static mut debug: c_uint = 0;
static mut verbose: c_uint = 0;
static mut force: c_uint = 0;
static mut progname: *mut c_char = ptr::null_mut();
static mut base_cpu: c_int = 0;
static mut update_epb: u8 = 0;
static mut new_epb: c_ulonglong = 0;
static mut turbo_is_enabled: u8 = 0;
static mut update_turbo: u8 = 0;
static mut turbo_update_value: u8 = 0;
static mut update_hwp_epp: u8 = 0;
static mut update_hwp_min: u8 = 0;
static mut update_hwp_max: u8 = 0;
static mut hwp_limits_done_via_sysfs: u8 = 0;
static mut update_hwp_desired: u8 = 0;
static mut update_hwp_window: u8 = 0;
static mut update_hwp_use_pkg: u8 = 0;
static mut update_hwp_enable: u8 = 0;

unsafe fn hwp_update_enabled() -> u8 {
    update_hwp_enable | update_hwp_epp | update_hwp_max | update_hwp_min |
        update_hwp_desired | update_hwp_window | update_hwp_use_pkg
}

static mut max_cpu_num: c_int = 0;
static mut max_pkg_num: c_int = 0;
const MAX_PACKAGES: usize = 64;
static mut first_cpu_in_pkg: [c_uint; MAX_PACKAGES] = [0; MAX_PACKAGES];
static mut pkg_present_set: c_ulonglong = 0;
static mut pkg_selected_set: c_ulonglong = 0;
static mut cpu_present_set: *mut cpu_set_t = ptr::null_mut();
static mut cpu_selected_set: *mut cpu_set_t = ptr::null_mut();
static mut genuine_intel: c_int = 0;
static mut cpu_setsize: size_t = 0;
static mut proc_stat: *mut c_char = c"/proc/stat".as_ptr() as *mut c_char;

static mut has_epb: c_uint = 0;                 /* MSR_IA32_ENERGY_PERF_BIAS */
static mut has_hwp: c_uint = 0;                 /* IA32_PM_ENABLE, IA32_HWP_CAPABILITIES */
                                                /* IA32_HWP_REQUEST, IA32_HWP_STATUS */
static mut has_hwp_notify: c_uint = 0;          /* IA32_HWP_INTERRUPT */
static mut has_hwp_activity_window: c_uint = 0; /* IA32_HWP_REQUEST[bits 41:32] */
static mut has_hwp_epp: c_uint = 0;             /* IA32_HWP_REQUEST[bits 31:24] */
static mut has_hwp_request_pkg: c_uint = 0;     /* IA32_HWP_REQUEST_PKG */
static mut bdx_highest_ratio: c_uint = 0;

static mut update_soc_slider_balance: u8 = 0;
static mut update_soc_slider_offset: u8 = 0;
static mut update_platform_profile: u8 = 0;
static mut soc_slider_balance: c_int = 0;
static mut soc_slider_offset: c_int = 0;
static mut platform_profile: [c_char; 64] = [0; 64];

const PATH_TO_CPU: &str = "/sys/devices/system/cpu/";
const SYSFS_PATH_MAX: usize = 255;
const PATH_SOC_SLIDER_BALANCE: *const c_char =
    c"/sys/module/processor_thermal_soc_slider/parameters/slider_balance".as_ptr();
const PATH_SOC_SLIDER_OFFSET: *const c_char =
    c"/sys/module/processor_thermal_soc_slider/parameters/slider_offset".as_ptr();
const PATH_PLATFORM_PROFILE: *const c_char =
    c"/sys/class/platform-profile/platform-profile-0/profile".as_ptr();
const PATH_PLATFORM_PROFILE_NAME: *const c_char =
    c"/sys/class/platform-profile/platform-profile-0/name".as_ptr();
const POWER_SLIDER_NAME: *const c_char = c"SoC Power Slider".as_ptr();

static mut use_android_msr_path: c_int = 0;

/*
 * maintain compatibility with original implementation, but don't document it:
 */
pub unsafe fn usage() -> ! {
    fprintf(stderr, c"%s [options] [scope][field value]\n".as_ptr(), progname);
    fprintf(stderr, c"scope: --cpu cpu-list [--hwp-use-pkg #] | --pkg pkg-list\n".as_ptr());
    fprintf(stderr, c"field: --all | --epb | --hwp-epp | --hwp-min | --hwp-max | --hwp-desired\n".as_ptr());
    fprintf(stderr, c"other: --hwp-enable | --turbo-enable (0 | 1) | --help | --force\n".as_ptr());
    fprintf(stderr, c"soc-slider: --soc-slider-balance # | --soc-slider-offset # | --platform-profile <name>\n".as_ptr());
    fprintf(stderr, c"value: ( # | \"normal\" | \"performance\" | \"balance-performance\" | \"balance-power\"| \"power\")\n".as_ptr());
    fprintf(stderr, c"--hwp-window usec\n".as_ptr());
    fprintf(stderr, c"Specify only Energy Performance BIAS (legacy usage):\n".as_ptr());
    fprintf(stderr, c"%s: [-c cpu] [-v] (-r | policy-value )\n".as_ptr(), progname);
    exit(1);
}

/*
 * If bdx_highest_ratio is set,
 * then we must translate between MSR format and simple ratio
 * used on the cmdline.
 */
pub unsafe fn ratio_2_msr_perf(ratio: c_int) -> c_int {
    let msr_perf: c_int;
    if bdx_highest_ratio == 0 {
        return ratio;
    }
    msr_perf = ratio * 255 / bdx_highest_ratio as c_int;
    if debug != 0 {
        fprintf(stderr, c"%d = ratio_to_msr_perf(%d)\n".as_ptr(), msr_perf, ratio);
    }
    msr_perf
}

pub unsafe fn msr_perf_2_ratio(msr_perf: c_int) -> c_int {
    let ratio: c_int;
    let mut d: c_double;
    if bdx_highest_ratio == 0 {
        return msr_perf;
    }
    d = (msr_perf as c_double) * (bdx_highest_ratio as c_double) / 255.0;
    d = d + 0.5; /* round */
    ratio = d as c_int;
    if debug != 0 {
        fprintf(stderr, c"%d = msr_perf_ratio(%d) {%f}\n".as_ptr(), ratio, msr_perf, d);
    }
    ratio
}

pub unsafe fn parse_cmdline_epb(i: c_int) -> c_int {
    if has_epb == 0 {
        errx(1, c"EPB not enabled on this platform".as_ptr());
    }
    update_epb = 1;
    if i == OPTARG_POWER { return ENERGY_PERF_BIAS_POWERSAVE; }
    if i == OPTARG_BALANCE_POWER { return ENERGY_PERF_BIAS_BALANCE_POWERSAVE; }
    if i == OPTARG_NORMAL { return ENERGY_PERF_BIAS_NORMAL; }
    if i == OPTARG_BALANCE_PERFORMANCE { return ENERGY_PERF_BIAS_BALANCE_PERFORMANCE; }
    if i == OPTARG_PERFORMANCE { return ENERGY_PERF_BIAS_PERFORMANCE; }
    if i < 0 || i > ENERGY_PERF_BIAS_POWERSAVE {
        errx(1, c"--epb must be from 0 to 15".as_ptr());
    }
    i
}

const HWP_CAP_LOWEST: c_int = 0;
const HWP_CAP_HIGHEST: c_int = 255;

/*
 * "performance" changes hwp_min to cap.highest
 * All others leave it at cap.lowest
 */
pub unsafe fn parse_cmdline_hwp_min(i: c_int) -> c_int {
    update_hwp_min = 1;
    match i {
        OPTARG_POWER | OPTARG_BALANCE_POWER | OPTARG_NORMAL | OPTARG_BALANCE_PERFORMANCE => HWP_CAP_LOWEST,
        OPTARG_PERFORMANCE => HWP_CAP_HIGHEST,
        _ => i,
    }
}

/*
 * "power" changes hwp_max to cap.lowest
 * All others leave it at cap.highest
 */
pub unsafe fn parse_cmdline_hwp_max(i: c_int) -> c_int {
    update_hwp_max = 1;
    match i {
        OPTARG_POWER => HWP_CAP_LOWEST,
        OPTARG_NORMAL | OPTARG_BALANCE_POWER | OPTARG_BALANCE_PERFORMANCE | OPTARG_PERFORMANCE => HWP_CAP_HIGHEST,
        _ => i,
    }
}

/*
 * for --hwp-des, all strings leave it in autonomous mode
 * If you want to change it, you need to explicitly pick a value
 */
pub unsafe fn parse_cmdline_hwp_desired(i: c_int) -> c_int {
    update_hwp_desired = 1;
    match i {
        OPTARG_POWER | OPTARG_BALANCE_POWER | OPTARG_BALANCE_PERFORMANCE | OPTARG_NORMAL | OPTARG_PERFORMANCE => 0,
        _ => i,
    }
}

pub unsafe fn parse_cmdline_hwp_window(mut i: c_int) -> c_int {
    let mut exponent: c_uint;
    update_hwp_window = 1;
    match i {
        OPTARG_POWER | OPTARG_BALANCE_POWER | OPTARG_NORMAL | OPTARG_BALANCE_PERFORMANCE | OPTARG_PERFORMANCE => return 0,
        _ => {}
    }
    if i < 0 || i > 1270000000 {
        fprintf(stderr, c"--hwp-window: 0 for auto; 1 - 1270000000 usec for window duration\n".as_ptr());
        usage();
    }
    exponent = 0;
    loop {
        if debug != 0 {
            printf(c"%d 10^%d\n".as_ptr(), i, exponent);
        }
        if i <= 127 {
            break;
        }
        i = i / 10;
        exponent += 1;
    }
    if debug != 0 {
        fprintf(stderr, c"%d*10^%d: 0x%x\n".as_ptr(), i, exponent, (exponent << 7) | i as c_uint);
    }
    ((exponent << 7) | i as c_uint) as c_int
}

pub unsafe fn parse_cmdline_hwp_epp(i: c_int) -> c_int {
    update_hwp_epp = 1;
    if i == OPTARG_POWER { return HWP_EPP_POWERSAVE; }
    if i == OPTARG_BALANCE_POWER { return HWP_EPP_BALANCE_POWERSAVE; }
    if i == OPTARG_NORMAL || i == OPTARG_BALANCE_PERFORMANCE { return HWP_EPP_BALANCE_PERFORMANCE; }
    if i == OPTARG_PERFORMANCE { return HWP_EPP_PERFORMANCE; }
    if i < 0 || i > 0xff {
        fprintf(stderr, c"--hwp-epp must be from 0 to 0xff\n".as_ptr());
        usage();
    }
    i
}

pub unsafe fn parse_cmdline_turbo(i: c_int) -> c_int {
    update_turbo = 1;
    match i {
        OPTARG_POWER => 0,
        OPTARG_NORMAL | OPTARG_BALANCE_POWER | OPTARG_BALANCE_PERFORMANCE | OPTARG_PERFORMANCE => 1,
        _ => {
            if i < 0 || i > 1 {
                fprintf(stderr, c"--turbo-enable: 1 to enable, 0 to disable\n".as_ptr());
                usage();
            }
            i
        }
    }
}

pub unsafe fn parse_optarg_string(s: *mut c_char) -> c_int {
    let mut endptr: *mut c_char = ptr::null_mut();
    let i: c_long;
    if strncmp(s, c"default".as_ptr(), 7) == 0 { return OPTARG_NORMAL; }
    if strncmp(s, c"normal".as_ptr(), 6) == 0 { return OPTARG_NORMAL; }
    if strncmp(s, c"power".as_ptr(), 9) == 0 { return OPTARG_POWER; }
    if strncmp(s, c"balance-power".as_ptr(), 17) == 0 { return OPTARG_BALANCE_POWER; }
    if strncmp(s, c"balance-performance".as_ptr(), 19) == 0 { return OPTARG_BALANCE_PERFORMANCE; }
    if strncmp(s, c"performance".as_ptr(), 11) == 0 { return OPTARG_PERFORMANCE; }
    i = strtol(s, &mut endptr, 0);
    if s == endptr {
        fprintf(stderr, c"no digits in \"%s\"\n".as_ptr(), s);
        usage();
    }
    if i == LONG_MIN || i == LONG_MAX {
        errx(-1, c"%s".as_ptr(), s);
    }
    if i > 0xff {
        errx(-1, c"%d (0x%x) must be < 256".as_ptr(), i as c_int, i as c_int);
    }
    if i < 0 {
        errx(-1, c"%d (0x%x) must be >= 0".as_ptr(), i as c_int, i as c_int);
    }
    i as c_int
}

pub unsafe fn parse_cmdline_all(s: *mut c_char) {
    force += 1;
    update_hwp_enable = 1;
    req_update.hwp_min = parse_cmdline_hwp_min(parse_optarg_string(s)) as u8;
    req_update.hwp_max = parse_cmdline_hwp_max(parse_optarg_string(s)) as u8;
    req_update.hwp_epp = parse_cmdline_hwp_epp(parse_optarg_string(s)) as u8;
    if has_epb != 0 {
        new_epb = parse_cmdline_epb(parse_optarg_string(s)) as c_ulonglong;
    }
    turbo_update_value = parse_cmdline_turbo(parse_optarg_string(s)) as u8;
    req_update.hwp_desired = parse_cmdline_hwp_desired(parse_optarg_string(s)) as u8;
    req_update.hwp_window = parse_cmdline_hwp_window(parse_optarg_string(s)) as c_uint;
}

pub unsafe fn validate_cpu_selected_set() {
    let mut cpu: c_int;
    if CPU_COUNT_S(cpu_setsize, cpu_selected_set) == 0 {
        errx(0, c"no CPUs requested".as_ptr());
    }
    cpu = 0;
    while cpu <= max_cpu_num {
        if CPU_ISSET_S(cpu, cpu_setsize, cpu_selected_set) != 0 {
            if CPU_ISSET_S(cpu, cpu_setsize, cpu_present_set) == 0 {
                errx(1, c"Requested cpu%d is not present".as_ptr(), cpu);
            }
        }
        cpu += 1;
    }
}

pub unsafe fn parse_cmdline_cpu(s: *mut c_char) {
    let mut startp: *mut c_char;
    let mut endp: *mut c_char = ptr::null_mut();
    let mut cpu: c_int = 0;
    if pkg_selected_set != 0 {
        usage();
    }
    cpu_selected_set = CPU_ALLOC(max_cpu_num + 1);
    if cpu_selected_set.is_null() {
        err(1, c"cpu_selected_set".as_ptr());
    }
    CPU_ZERO_S(cpu_setsize, cpu_selected_set);
    startp = s;
    while !startp.is_null() && *startp != 0 {
        if *startp == b',' as c_char {
            startp = startp.add(1);
            continue;
        }
        if *startp == b'-' as c_char {
            let end_cpu: c_int;
            startp = startp.add(1);
            end_cpu = strtol(startp, &mut endp, 10) as c_int;
            if startp == endp { continue; }
            while cpu <= end_cpu {
                if cpu > max_cpu_num {
                    errx(1, c"Requested cpu%d exceeds max cpu%d".as_ptr(), cpu, max_cpu_num);
                }
                CPU_SET_S(cpu, cpu_setsize, cpu_selected_set);
                cpu += 1;
            }
            startp = endp;
            continue;
        }
        if strncmp(startp, c"all".as_ptr(), 3) == 0 {
            cpu = 0;
            while cpu <= max_cpu_num {
                if CPU_ISSET_S(cpu, cpu_setsize, cpu_present_set) != 0 {
                    CPU_SET_S(cpu, cpu_setsize, cpu_selected_set);
                }
                cpu += 1;
            }
            startp = startp.add(3);
            if *startp == 0 { break; }
        }
        /* "--cpu even" is not documented */
        if strncmp(startp, c"even".as_ptr(), 4) == 0 {
            cpu = 0;
            while cpu <= max_cpu_num {
                if CPU_ISSET_S(cpu, cpu_setsize, cpu_present_set) != 0 {
                    CPU_SET_S(cpu, cpu_setsize, cpu_selected_set);
                }
                cpu += 2;
            }
            startp = startp.add(4);
            if *startp == 0 { break; }
        }
        /* "--cpu odd" is not documented */
        if strncmp(startp, c"odd".as_ptr(), 3) == 0 {
            cpu = 1;
            while cpu <= max_cpu_num {
                if CPU_ISSET_S(cpu, cpu_setsize, cpu_present_set) != 0 {
                    CPU_SET_S(cpu, cpu_setsize, cpu_selected_set);
                }
                cpu += 2;
            }
            startp = startp.add(3);
            if *startp == 0 { break; }
        }
        cpu = strtol(startp, &mut endp, 10) as c_int;
        if startp == endp {
            errx(1, c"--cpu cpu-set: confused by '%s'".as_ptr(), startp);
        }
        if cpu > max_cpu_num {
            errx(1, c"Requested cpu%d exceeds max cpu%d".as_ptr(), cpu, max_cpu_num);
        }
        CPU_SET_S(cpu, cpu_setsize, cpu_selected_set);
        startp = endp;
    }
    validate_cpu_selected_set();
}

pub unsafe fn parse_cmdline_pkg(s: *mut c_char) {
    let mut startp: *mut c_char;
    let mut endp: *mut c_char = ptr::null_mut();
    let mut pkg: c_int = 0;
    if !cpu_selected_set.is_null() {
        usage();
    }
    pkg_selected_set = 0;
    startp = s;
    while !startp.is_null() && *startp != 0 {
        if *startp == b',' as c_char {
            startp = startp.add(1);
            continue;
        }
        if *startp == b'-' as c_char {
            let end_pkg: c_int;
            startp = startp.add(1);
            end_pkg = strtol(startp, &mut endp, 10) as c_int;
            if startp == endp { continue; }
            while pkg <= end_pkg {
                if pkg > max_pkg_num {
                    errx(1, c"Requested pkg%d exceeds max pkg%d".as_ptr(), pkg, max_pkg_num);
                }
                pkg_selected_set |= 1u64 << pkg;
                pkg += 1;
            }
            startp = endp;
            continue;
        }
        if strncmp(startp, c"all".as_ptr(), 3) == 0 {
            pkg_selected_set = pkg_present_set;
            return;
        }
        pkg = strtol(startp, &mut endp, 10) as c_int;
        if pkg > max_pkg_num {
            errx(1, c"Requested pkg%d Exceeds max pkg%d".as_ptr(), pkg, max_pkg_num);
        }
        pkg_selected_set |= 1u64 << pkg;
        startp = endp;
    }
}

pub unsafe fn for_packages(pkg_set: c_ulonglong, func: unsafe fn(c_int) -> c_int) {
    let mut pkg_num: c_int = 0;
    while pkg_num <= max_pkg_num {
        if (pkg_set & (1u64 << pkg_num)) != 0 {
            func(pkg_num);
        }
        pkg_num += 1;
    }
}

unsafe fn parse_cmdline_int(s: *const c_char, out: *mut c_int) -> c_int {
    let mut endp: *mut c_char = ptr::null_mut();
    let val: c_long = strtol(s, &mut endp, 0);
    if endp == s as *mut c_char || errno == ERANGE { return -1; }
    if *endp != 0 { return -1; }
    if val < INT_MIN as c_long || val > INT_MAX as c_long { return -1; }
    *out = val as c_int;
    0
}

pub unsafe fn print_version() {
    printf(c"x86_energy_perf_policy 2026.04.25 Len Brown <lenb@kernel.org>\n".as_ptr());
}

unsafe fn platform_profile_access(mode: c_int) -> c_int {
    if access(PATH_PLATFORM_PROFILE, mode) != 0 {
        if debug != 0 {
            fprintf(stderr, c"Can not access %s\n".as_ptr(), PATH_PLATFORM_PROFILE);
        }
        return 0;
    }
    1
}

unsafe fn platform_profile_name_is(name: *mut c_char) -> c_int {
    let mut buf: [c_char; 64] = [0; 64];
    if sysfs_read_string(PATH_PLATFORM_PROFILE_NAME, buf.as_mut_ptr(), size_of::<[c_char; 64]>()) != 0 {
        if debug != 0 {
            fprintf(stderr, c"Can not read %s\n".as_ptr(), PATH_PLATFORM_PROFILE_NAME);
        }
        return 0;
    }
    if strncmp(buf.as_ptr(), name, 16) != 0 {
        if debug != 0 {
            fprintf(stderr, c"%s does not match '%s'\n".as_ptr(), PATH_PLATFORM_PROFILE_NAME, name);
        }
        return 0;
    }
    1
}

unsafe fn soc_slider_access(mode: c_int) -> c_int {
    if platform_profile_access(R_OK) == 0 { return 0; }
    if platform_profile_name_is(POWER_SLIDER_NAME as *mut c_char) == 0 { return 0; }
    if access(PATH_SOC_SLIDER_BALANCE, mode) != 0 {
        if debug != 0 {
            fprintf(stderr, c"Can not access %s\n".as_ptr(), PATH_SOC_SLIDER_BALANCE);
        }
        return 0;
    }
    if access(PATH_SOC_SLIDER_OFFSET, mode) != 0 {
        if debug != 0 {
            fprintf(stderr, c"Can not access %s\n".as_ptr(), PATH_SOC_SLIDER_OFFSET);
        }
        return 0;
    }
    1
}

pub unsafe fn cmdline(argc: c_int, argv: *mut *mut c_char) {
    let mut opt: c_int;
    let mut option_index: c_int = 0;
    let long_options: [option; 21] = [
        option { name: c"all".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'a' as c_int },
        option { name: c"cpu".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'c' as c_int },
        option { name: c"pkg".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'p' as c_int },
        option { name: c"debug".as_ptr(), has_arg: no_argument, flag: ptr::null_mut(), val: b'd' as c_int },
        option { name: c"hwp-desired".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'D' as c_int },
        option { name: c"epb".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'B' as c_int },
        option { name: c"force".as_ptr(), has_arg: no_argument, flag: ptr::null_mut(), val: b'f' as c_int },
        option { name: c"hwp-enable".as_ptr(), has_arg: no_argument, flag: ptr::null_mut(), val: b'e' as c_int },
        option { name: c"help".as_ptr(), has_arg: no_argument, flag: ptr::null_mut(), val: b'h' as c_int },
        option { name: c"hwp-epp".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'P' as c_int },
        option { name: c"hwp-min".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'm' as c_int },
        option { name: c"hwp-max".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'M' as c_int },
        option { name: c"read".as_ptr(), has_arg: no_argument, flag: ptr::null_mut(), val: b'r' as c_int },
        option { name: c"turbo-enable".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b't' as c_int },
        option { name: c"hwp-use-pkg".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'u' as c_int },
        option { name: c"version".as_ptr(), has_arg: no_argument, flag: ptr::null_mut(), val: b'v' as c_int },
        option { name: c"hwp-window".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'w' as c_int },
        option { name: c"soc-slider-balance".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'S' as c_int },
        option { name: c"soc-slider-offset".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'O' as c_int },
        option { name: c"platform-profile".as_ptr(), has_arg: required_argument, flag: ptr::null_mut(), val: b'F' as c_int },
        option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
    ];
    progname = *argv;
    loop {
        opt = getopt_long_only(argc, argv, c"+a:c:dD:E:e:f:m:M:rt:u:vw::S:O:F:".as_ptr(), long_options.as_ptr(), &mut option_index);
        if opt == -1 { break; }
        match opt as u8 as char {
            'a' => parse_cmdline_all(optarg),
            'B' => new_epb = parse_cmdline_epb(parse_optarg_string(optarg)) as c_ulonglong,
            'c' => parse_cmdline_cpu(optarg),
            'e' => update_hwp_enable = 1,
            'h' => usage(),
            'd' => { debug += 1; verbose += 1; }
            'f' => force += 1,
            'D' => req_update.hwp_desired = parse_cmdline_hwp_desired(parse_optarg_string(optarg)) as u8,
            'F' => {
                if strlen(optarg) >= platform_profile.len() {
                    errx(1, c"--platform-profile: value too long".as_ptr());
                }
                if platform_profile_access(W_OK) == 0 {
                    errx(1, c"Can not update platform-profile in '%s'".as_ptr(), PATH_PLATFORM_PROFILE);
                }
                strcpy(platform_profile.as_mut_ptr(), optarg);
                update_platform_profile = 1;
            }
            'm' => req_update.hwp_min = parse_cmdline_hwp_min(parse_optarg_string(optarg)) as u8,
            'M' => req_update.hwp_max = parse_cmdline_hwp_max(parse_optarg_string(optarg)) as u8,
            'O' => {
                if parse_cmdline_int(optarg, &mut soc_slider_offset) != 0 {
                    errx(1, c"--soc-slider-offset: invalid value".as_ptr());
                }
                if soc_slider_access(W_OK) == 0 {
                    errx(1, c"Unable to write SOC Slider Offset".as_ptr());
                }
                update_soc_slider_offset = 1;
            }
            'p' => parse_cmdline_pkg(optarg),
            'P' => req_update.hwp_epp = parse_cmdline_hwp_epp(parse_optarg_string(optarg)) as u8,
            'r' => { /* v1 used -r to specify read-only mode, now the default */ }
            'S' => {
                if parse_cmdline_int(optarg, &mut soc_slider_balance) != 0 {
                    errx(1, c"--soc-slider-balance: invalid value".as_ptr());
                }
                if soc_slider_access(W_OK) == 0 {
                    errx(1, c"Unable to write SOC Slider-Balance in '%s'".as_ptr(), PATH_SOC_SLIDER_BALANCE);
                }
                update_soc_slider_balance = 1;
            }
            't' => turbo_update_value = parse_cmdline_turbo(parse_optarg_string(optarg)) as u8,
            'u' => {
                update_hwp_use_pkg += 1;
                req_update.hwp_use_pkg = if atoi(optarg) == 0 { 0 } else { 1 };
            }
            'v' => {
                print_version();
                exit(0);
            }
            'w' => req_update.hwp_window = parse_cmdline_hwp_window(parse_optarg_string(optarg)) as c_uint,
            _ => usage(),
        }
    }
    /*
     * v1 allowed "performance"|"normal"|"power" with no policy specifier
     * to update BIAS.  Continue to support that, even though no longer documented.
     */
    if argc == optind + 1 {
        new_epb = parse_cmdline_epb(parse_optarg_string(*argv.add(optind as usize))) as c_ulonglong;
    }
    if argc > optind + 1 {
        fprintf(stderr, c"stray parameter '%s'\n".as_ptr(), *argv.add((optind + 1) as usize));
        usage();
    }
}

/*
 * Open a file, and exit on failure
 */
pub unsafe fn fopen_or_die(path: *const c_char, mode: *const c_char) -> *mut FILE {
    let filep = fopen(path, mode);
    if filep.is_null() {
        err(1, c"%s: open failed".as_ptr(), path);
    }
    filep
}

pub unsafe fn err_on_hypervisor() {
    let cpuinfo: *mut FILE;
    let flags: *mut c_char;
    let hypervisor: *mut c_char;
    let buffer: *mut c_char;
    cpuinfo = fopen_or_die(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr());
    buffer = malloc(4096) as *mut c_char;
    if buffer.is_null() {
        fclose(cpuinfo);
        err(-ENOMEM, c"buffer malloc fail".as_ptr());
    }
    if fread(buffer as *mut c_void, 1024, 1, cpuinfo) == 0 {
        fclose(cpuinfo);
        free(buffer as *mut c_void);
        err(1, c"Reading /proc/cpuinfo failed".as_ptr());
    }
    flags = strstr(buffer, c"flags".as_ptr());
    if flags.is_null() {
        fclose(cpuinfo);
        free(buffer as *mut c_void);
        err(1, c"Failed to find 'flags' in /proc/cpuinfo".as_ptr());
    }
    rewind(cpuinfo);
    fseek(cpuinfo, flags.offset_from(buffer) as c_long, SEEK_SET);
    if fgets(buffer, 4096, cpuinfo).is_null() {
        fclose(cpuinfo);
        free(buffer as *mut c_void);
        err(1, c"Reading /proc/cpuinfo failed".as_ptr());
    }
    fclose(cpuinfo);
    hypervisor = strstr(buffer, c"hypervisor".as_ptr());
    free(buffer as *mut c_void);
    if !hypervisor.is_null() {
        err(-1, c"not supported on this virtual machine".as_ptr());
    }
}

pub unsafe fn get_msr(cpu: c_int, offset: c_int, msr: *mut c_ulonglong) -> c_int {
    let retval: c_int;
    let mut pathname: [c_char; 32] = [0; 32];
    let fd: c_int;
    sprintf(pathname.as_mut_ptr(), if use_android_msr_path != 0 { c"/dev/msr%d".as_ptr() } else { c"/dev/cpu/%d/msr".as_ptr() }, cpu);
    fd = open(pathname.as_ptr(), O_RDONLY);
    if fd < 0 {
        err(-1, c"%s open failed, try chown or chmod +r %s, or run as root".as_ptr(), pathname.as_ptr(),
            if use_android_msr_path != 0 { c"/dev/msr*".as_ptr() } else { c"/dev/cpu/*/msr".as_ptr() });
    }
    retval = pread(fd, msr as *mut c_void, size_of::<c_ulonglong>(), offset as c_long) as c_int;
    if retval as usize != size_of::<c_ulonglong>() {
        err_on_hypervisor();
        err(-1, c"%s offset 0x%llx read failed".as_ptr(), pathname.as_ptr(), offset as c_ulonglong);
    }
    if debug > 1 {
        fprintf(stderr, c"get_msr(cpu%d, 0x%X, 0x%llX)\n".as_ptr(), cpu, offset, *msr);
    }
    close(fd);
    0
}

pub unsafe fn put_msr(cpu: c_int, offset: c_int, new_msr: c_ulonglong) -> c_int {
    let mut pathname: [c_char; 32] = [0; 32];
    let retval: c_int;
    let fd: c_int;
    sprintf(pathname.as_mut_ptr(), if use_android_msr_path != 0 { c"/dev/msr%d".as_ptr() } else { c"/dev/cpu/%d/msr".as_ptr() }, cpu);
    fd = open(pathname.as_ptr(), O_RDWR);
    if fd < 0 {
        err(-1, c"%s open failed, try chown or chmod +r %s, or run as root".as_ptr(), pathname.as_ptr(),
            if use_android_msr_path != 0 { c"/dev/msr*".as_ptr() } else { c"/dev/cpu/*/msr".as_ptr() });
    }
    retval = pwrite(fd, &new_msr as *const _ as *const c_void, size_of::<c_ulonglong>(), offset as c_long) as c_int;
    if retval as usize != size_of::<c_ulonglong>() {
        err(-2, c"pwrite(cpu%d, offset 0x%x, 0x%llx) = %d".as_ptr(), cpu, offset, new_msr, retval);
    }
    close(fd);
    if debug > 1 {
        fprintf(stderr, c"put_msr(cpu%d, 0x%X, 0x%llX)\n".as_ptr(), cpu, offset, new_msr);
    }
    0
}

unsafe fn read_sysfs(path: *const c_char, buf: *mut c_char, buflen: size_t) -> c_uint {
    let numread: ssize_t;
    let fd: c_int = open(path, O_RDONLY);
    if fd == -1 { return 0; }
    numread = read(fd, buf as *mut c_void, buflen - 1);
    if numread < 1 {
        close(fd);
        return 0;
    }
    *buf.add(numread as usize) = 0;
    close(fd);
    numread as c_uint
}

unsafe fn write_sysfs(path: *const c_char, buf: *mut c_char, buflen: size_t) -> c_uint {
    let numwritten: ssize_t;
    let fd: c_int = open(path, O_WRONLY);
    if fd == -1 { return 0; }
    numwritten = write(fd, buf as *const c_void, buflen - 1);
    if numwritten < 1 {
        *buf.add(strcspn(buf, c"\n".as_ptr())) = 0;
        warn(c"Write '%s' to '%s' failed".as_ptr(), buf, path);
        close(fd);
        return -1i32 as c_uint;
    }
    close(fd);
    numwritten as c_uint
}

unsafe fn sysfs_read_string(path: *const c_char, buf: *mut c_char, buflen: size_t) -> c_int {
    let len: c_uint = read_sysfs(path, buf, buflen);
    let n: size_t;
    if len == 0 { return -1; }
    n = strcspn(buf, c"\n".as_ptr());
    *buf.add(n) = 0;
    0
}

unsafe fn sysfs_write_string(path: *const c_char, buf: *const c_char) -> c_int {
    let mut tmp: [c_char; 128] = [0; 128];
    let len: c_int = snprintf(tmp.as_mut_ptr(), tmp.len(), c"%s\n".as_ptr(), buf);
    if len < 0 || len >= tmp.len() as c_int { return -1; }
    if write_sysfs(path, tmp.as_mut_ptr(), len as size_t + 1) != 0 { 0 } else { -1 }
}

pub unsafe fn print_hwp_cap(cpu: c_int, cap: *mut msr_hwp_cap, _str: *mut c_char) {
    if cpu != -1 { printf(c"cpu%d: ".as_ptr(), cpu); }
    printf(c"HWP_CAP: low %d eff %d guar %d high %d\n".as_ptr(),
           (*cap).lowest as c_int, (*cap).efficient as c_int, (*cap).guaranteed as c_int, (*cap).highest as c_int);
}

pub unsafe fn read_hwp_cap(cpu: c_int, cap: *mut msr_hwp_cap, msr_offset: c_uint) {
    let mut msr: c_ulonglong = 0;
    get_msr(cpu, msr_offset as c_int, &mut msr);
    (*cap).highest = msr_perf_2_ratio(HWP_HIGHEST_PERF(msr)) as u8;
    (*cap).guaranteed = msr_perf_2_ratio(HWP_GUARANTEED_PERF(msr)) as u8;
    (*cap).efficient = msr_perf_2_ratio(HWP_MOSTEFFICIENT_PERF(msr)) as u8;
    (*cap).lowest = msr_perf_2_ratio(HWP_LOWEST_PERF(msr)) as u8;
}

pub unsafe fn print_hwp_request(cpu: c_int, h: *mut msr_hwp_request, str_: *mut c_char) {
    if cpu != -1 { printf(c"cpu%d: ".as_ptr(), cpu); }
    if !str_.is_null() { printf(c"%s".as_ptr(), str_); }
    printf(c"HWP_REQ: min %d max %d des %d epp %d window 0x%x (%d*10^%dus) use_pkg %d\n".as_ptr(),
           (*h).hwp_min as c_int, (*h).hwp_max as c_int, (*h).hwp_desired as c_int, (*h).hwp_epp as c_int,
           (*h).hwp_window, (*h).hwp_window & 0x7f, ((*h).hwp_window >> 7) & 0x7, (*h).hwp_use_pkg as c_int);
}

pub unsafe fn print_hwp_request_pkg(pkg: c_int, h: *mut msr_hwp_request, str_: *mut c_char) {
    printf(c"pkg%d: ".as_ptr(), pkg);
    if !str_.is_null() { printf(c"%s".as_ptr(), str_); }
    printf(c"HWP_REQ_PKG: min %d max %d des %d epp %d window 0x%x (%d*10^%dus)\n".as_ptr(),
           (*h).hwp_min as c_int, (*h).hwp_max as c_int, (*h).hwp_desired as c_int, (*h).hwp_epp as c_int,
           (*h).hwp_window, (*h).hwp_window & 0x7f, ((*h).hwp_window >> 7) & 0x7);
}

pub unsafe fn read_hwp_request_msr(cpu: c_int, hwp_req: *mut msr_hwp_request, msr_offset: c_uint) {
    let mut msr: c_ulonglong = 0;
    get_msr(cpu, msr_offset as c_int, &mut msr);
    (*hwp_req).hwp_min = msr_perf_2_ratio(((msr >> 0) & 0xff) as c_int) as u8;
    (*hwp_req).hwp_max = msr_perf_2_ratio(((msr >> 8) & 0xff) as c_int) as u8;
    (*hwp_req).hwp_desired = msr_perf_2_ratio(((msr >> 16) & 0xff) as c_int) as u8;
    (*hwp_req).hwp_epp = ((msr >> 24) & 0xff) as u8;
    (*hwp_req).hwp_window = ((msr >> 32) & 0x3ff) as c_uint;
    (*hwp_req).hwp_use_pkg = ((msr >> 42) & 0x1) as u8;
}

pub unsafe fn write_hwp_request_msr(cpu: c_int, hwp_req: *mut msr_hwp_request, msr_offset: c_uint) {
    let mut msr: c_ulonglong = 0;
    if debug > 1 {
        printf(c"cpu%d: requesting min %d max %d des %d epp %d window 0x%0x use_pkg %d\n".as_ptr(),
               cpu, (*hwp_req).hwp_min as c_int, (*hwp_req).hwp_max as c_int, (*hwp_req).hwp_desired as c_int,
               (*hwp_req).hwp_epp as c_int, (*hwp_req).hwp_window, (*hwp_req).hwp_use_pkg as c_int);
    }
    msr |= HWP_MIN_PERF(ratio_2_msr_perf((*hwp_req).hwp_min as c_int));
    msr |= HWP_MAX_PERF(ratio_2_msr_perf((*hwp_req).hwp_max as c_int));
    msr |= HWP_DESIRED_PERF(ratio_2_msr_perf((*hwp_req).hwp_desired as c_int));
    msr |= HWP_ENERGY_PERF_PREFERENCE((*hwp_req).hwp_epp);
    msr |= HWP_ACTIVITY_WINDOW((*hwp_req).hwp_window);
    msr |= HWP_PACKAGE_CONTROL((*hwp_req).hwp_use_pkg);
    put_msr(cpu, msr_offset as c_int, msr);
}

unsafe fn get_epb_sysfs(cpu: c_int) -> c_int {
    let mut path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
    let mut linebuf: [c_char; 3] = [0; 3];
    let mut endp: *mut c_char = ptr::null_mut();
    let val: c_long;
    if has_epb == 0 { return -1; }
    snprintf(path.as_mut_ptr(), path.len(), c"/sys/devices/system/cpu/cpu%u/power/energy_perf_bias".as_ptr(), cpu);
    if read_sysfs(path.as_ptr(), linebuf.as_mut_ptr(), 3) == 0 { return -1; }
    val = strtol(linebuf.as_ptr(), &mut endp, 0);
    if endp == linebuf.as_mut_ptr() || errno == ERANGE { return -1; }
    val as c_int
}

unsafe fn set_epb_sysfs(cpu: c_int, mut val: c_int) -> c_int {
    let mut path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
    let mut linebuf: [c_char; 3] = [0; 3];
    let mut endp: *mut c_char = ptr::null_mut();
    let ret: c_int;
    if has_epb == 0 { return -1; }
    snprintf(path.as_mut_ptr(), path.len(), c"/sys/devices/system/cpu/cpu%u/power/energy_perf_bias".as_ptr(), cpu);
    snprintf(linebuf.as_mut_ptr(), linebuf.len(), c"%d".as_ptr(), val);
    ret = write_sysfs(path.as_ptr(), linebuf.as_mut_ptr(), 3) as c_int;
    if ret <= 0 { return -1; }
    val = strtol(linebuf.as_ptr(), &mut endp, 0) as c_int;
    if endp == linebuf.as_mut_ptr() || errno == ERANGE { return -1; }
    val
}

unsafe fn print_soc_slider() {
    let mut buf: [c_char; 64] = [0; 64];
    if soc_slider_access(R_OK) == 0 { return; }
    if sysfs_read_string(PATH_SOC_SLIDER_BALANCE, buf.as_mut_ptr(), buf.len()) == 0 {
        printf(c"soc-slider-balance: %s\n".as_ptr(), buf.as_ptr());
    }
    if sysfs_read_string(PATH_SOC_SLIDER_OFFSET, buf.as_mut_ptr(), buf.len()) == 0 {
        printf(c"soc-slider-offset: %s\n".as_ptr(), buf.as_ptr());
    }
}

unsafe fn print_platform_profile() {
    let mut buf: [c_char; 64] = [0; 64];
    if platform_profile_access(R_OK) == 0 { return; }
    if sysfs_read_string(PATH_PLATFORM_PROFILE_NAME, buf.as_mut_ptr(), buf.len()) == 0 {
        printf(c"platform-profile-name: %s\n".as_ptr(), buf.as_ptr());
    }
    if sysfs_read_string(PATH_PLATFORM_PROFILE, buf.as_mut_ptr(), buf.len()) == 0 {
        printf(c"platform-profile: %s\n".as_ptr(), buf.as_ptr());
    }
}

unsafe fn update_soc_slider() -> c_int {
    let mut tmp: [c_char; 32] = [0; 32];
    if update_soc_slider_balance != 0 {
        snprintf(tmp.as_mut_ptr(), tmp.len(), c"%d".as_ptr(), soc_slider_balance);
        if sysfs_write_string(PATH_SOC_SLIDER_BALANCE, tmp.as_ptr()) != 0 {
            err(1, c"soc-slider-balance write failed".as_ptr());
        }
    }
    if update_soc_slider_offset != 0 {
        snprintf(tmp.as_mut_ptr(), tmp.len(), c"%d".as_ptr(), soc_slider_offset);
        if sysfs_write_string(PATH_SOC_SLIDER_OFFSET, tmp.as_ptr()) != 0 {
            err(1, c"soc-slider-offset write failed".as_ptr());
        }
    }
    if update_platform_profile != 0 {
        if sysfs_write_string(PATH_PLATFORM_PROFILE, platform_profile.as_ptr()) != 0 {
            err(1, c"platform-profile write failed".as_ptr());
        }
    }
    0
}

pub unsafe fn print_cpu_msrs(cpu: c_int) -> c_int {
    let mut req = msr_hwp_request { hwp_min: 0, hwp_max: 0, hwp_desired: 0, hwp_epp: 0, hwp_window: 0, hwp_use_pkg: 0 };
    let mut cap = msr_hwp_cap { highest: 0, guaranteed: 0, efficient: 0, lowest: 0 };
    let epb = get_epb_sysfs(cpu);
    if epb >= 0 {
        printf(c"cpu%d: EPB %u\n".as_ptr(), cpu, epb as c_uint);
    }
    if has_hwp == 0 { return 0; }
    read_hwp_request_msr(cpu, &mut req, MSR_HWP_REQUEST as c_uint);
    print_hwp_request(cpu, &mut req, c"".as_ptr() as *mut c_char);
    read_hwp_cap(cpu, &mut cap, MSR_HWP_CAPABILITIES as c_uint);
    print_hwp_cap(cpu, &mut cap, c"".as_ptr() as *mut c_char);
    0
}

pub unsafe fn print_pkg_msrs(pkg: c_int) -> c_int {
    let mut req = msr_hwp_request { hwp_min: 0, hwp_max: 0, hwp_desired: 0, hwp_epp: 0, hwp_window: 0, hwp_use_pkg: 0 };
    let mut msr: c_ulonglong = 0;
    if has_hwp == 0 { return 0; }
    read_hwp_request_msr(first_cpu_in_pkg[pkg as usize] as c_int, &mut req, MSR_HWP_REQUEST_PKG as c_uint);
    print_hwp_request_pkg(pkg, &mut req, c"".as_ptr() as *mut c_char);
    if has_hwp_notify != 0 {
        get_msr(first_cpu_in_pkg[pkg as usize] as c_int, MSR_HWP_INTERRUPT, &mut msr);
        fprintf(stderr, c"pkg%d: MSR_HWP_INTERRUPT: 0x%08llx (Excursion_Min-%sabled, Guaranteed_Perf_Change-%sabled)\n".as_ptr(),
                pkg, msr, if (msr & 0x2) != 0 { c"EN".as_ptr() } else { c"Dis".as_ptr() },
                if (msr & 0x1) != 0 { c"EN".as_ptr() } else { c"Dis".as_ptr() });
    }
    get_msr(first_cpu_in_pkg[pkg as usize] as c_int, MSR_HWP_STATUS, &mut msr);
    fprintf(stderr, c"pkg%d: MSR_HWP_STATUS: 0x%08llx (%sExcursion_Min, %sGuaranteed_Perf_Change)\n".as_ptr(),
            pkg, msr, if (msr & 0x4) != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
            if (msr & 0x1) != 0 { c"".as_ptr() } else { c"No-".as_ptr() });
    0
}

/*
 * Assumption: All HWP systems have 100 MHz bus clock
 */
pub unsafe fn ratio_2_sysfs_khz(ratio: c_int) -> c_int {
    let bclk_khz: c_int = 100 * 1000; /* 100,000 KHz = 100 MHz */
    ratio * bclk_khz
}

/*
 * If HWP is enabled and cpufreq sysfs attribtes are present,
 * then update via sysfs. The intel_pstate driver may modify (clip)
 * this request, say, when HWP_CAP is outside of PLATFORM_INFO limits,
 * and the driver-chosen value takes precidence.
 *
 * (intel_pstate's max_perf_pct and min_perf_pct will follow cpufreq,
 *  so we don't have to touch that.)
 */
pub unsafe fn update_cpufreq_scaling_freq(is_max: c_int, cpu: c_int, ratio: c_uint) {
    let mut pathname: [c_char; 64] = [0; 64];
    let fp: *mut FILE;
    let retval: c_int;
    let khz: c_int;
    sprintf(pathname.as_mut_ptr(), c"/sys/devices/system/cpu/cpu%d/cpufreq/scaling_%s_freq".as_ptr(),
            cpu, if is_max != 0 { c"max".as_ptr() } else { c"min".as_ptr() });
    fp = fopen(pathname.as_ptr(), c"w".as_ptr());
    if fp.is_null() {
        if debug != 0 { perror(pathname.as_ptr()); }
        return;
    }
    khz = ratio_2_sysfs_khz(ratio as c_int);
    retval = fprintf(fp, c"%d".as_ptr(), khz);
    if retval < 0 {
        if debug != 0 { perror(c"fprintf".as_ptr()); }
    }
    if debug != 0 {
        printf(c"echo %d > %s\n".as_ptr(), khz, pathname.as_ptr());
    }
    fclose(fp);
}

/*
 * We update all sysfs before updating any MSRs because of
 * bugs in cpufreq/intel_pstate where the sysfs writes
 * for a CPU may change the min/max values on other CPUS.
 */
pub unsafe fn update_sysfs(cpu: c_int) -> c_int {
    if has_hwp == 0 { return 0; }
    if hwp_update_enabled() == 0 { return 0; }
    if access(c"/sys/devices/system/cpu/cpu0/cpufreq".as_ptr(), F_OK) != 0 { return 0; }
    if update_hwp_min != 0 { update_cpufreq_scaling_freq(0, cpu, req_update.hwp_min as c_uint); }
    if update_hwp_max != 0 { update_cpufreq_scaling_freq(1, cpu, req_update.hwp_max as c_uint); }
    hwp_limits_done_via_sysfs = 1;
    0
}

pub unsafe fn verify_hwp_req_self_consistency(cpu: c_int, req: *mut msr_hwp_request) -> c_int {
    /* fail if min > max requested */
    if (*req).hwp_min > (*req).hwp_max {
        errx(1, c"cpu%d: requested hwp-min %d > hwp_max %d".as_ptr(), cpu, (*req).hwp_min as c_int, (*req).hwp_max as c_int);
    }
    /* fail if desired > max requestd */
    if (*req).hwp_desired != 0 && (*req).hwp_desired > (*req).hwp_max {
        errx(1, c"cpu%d: requested hwp-desired %d > hwp_max %d".as_ptr(), cpu, (*req).hwp_desired as c_int, (*req).hwp_max as c_int);
    }
    /* fail if desired < min requestd */
    if (*req).hwp_desired != 0 && (*req).hwp_desired < (*req).hwp_min {
        errx(1, c"cpu%d: requested hwp-desired %d < requested hwp_min %d".as_ptr(), cpu, (*req).hwp_desired as c_int, (*req).hwp_min as c_int);
    }
    0
}

pub unsafe fn check_hwp_request_v_hwp_capabilities(cpu: c_int, req: *mut msr_hwp_request, cap: *mut msr_hwp_cap) -> c_int {
    if update_hwp_max != 0 {
        if (*req).hwp_max > (*cap).highest { errx(1, c"cpu%d: requested max %d > capabilities highest %d, use --force?".as_ptr(), cpu, (*req).hwp_max as c_int, (*cap).highest as c_int); }
        if (*req).hwp_max < (*cap).lowest { errx(1, c"cpu%d: requested max %d < capabilities lowest %d, use --force?".as_ptr(), cpu, (*req).hwp_max as c_int, (*cap).lowest as c_int); }
    }
    if update_hwp_min != 0 {
        if (*req).hwp_min > (*cap).highest { errx(1, c"cpu%d: requested min %d > capabilities highest %d, use --force?".as_ptr(), cpu, (*req).hwp_min as c_int, (*cap).highest as c_int); }
        if (*req).hwp_min < (*cap).lowest { errx(1, c"cpu%d: requested min %d < capabilities lowest %d, use --force?".as_ptr(), cpu, (*req).hwp_min as c_int, (*cap).lowest as c_int); }
    }
    if update_hwp_min != 0 && update_hwp_max != 0 && (*req).hwp_min > (*req).hwp_max {
        errx(1, c"cpu%d: requested min %d > requested max %d".as_ptr(), cpu, (*req).hwp_min as c_int, (*req).hwp_max as c_int);
    }
    if update_hwp_desired != 0 && (*req).hwp_desired != 0 {
        if (*req).hwp_desired > (*req).hwp_max { errx(1, c"cpu%d: requested desired %d > requested max %d, use --force?".as_ptr(), cpu, (*req).hwp_desired as c_int, (*req).hwp_max as c_int); }
        if (*req).hwp_desired < (*req).hwp_min { errx(1, c"cpu%d: requested desired %d < requested min %d, use --force?".as_ptr(), cpu, (*req).hwp_desired as c_int, (*req).hwp_min as c_int); }
        if (*req).hwp_desired < (*cap).lowest { errx(1, c"cpu%d: requested desired %d < capabilities lowest %d, use --force?".as_ptr(), cpu, (*req).hwp_desired as c_int, (*cap).lowest as c_int); }
        if (*req).hwp_desired > (*cap).highest { errx(1, c"cpu%d: requested desired %d > capabilities highest %d, use --force?".as_ptr(), cpu, (*req).hwp_desired as c_int, (*cap).highest as c_int); }
    }
    0
}

pub unsafe fn update_hwp_request_msr(cpu: c_int) -> c_int {
    let mut req = msr_hwp_request { hwp_min: 0, hwp_max: 0, hwp_desired: 0, hwp_epp: 0, hwp_window: 0, hwp_use_pkg: 0 };
    let mut cap = msr_hwp_cap { highest: 0, guaranteed: 0, efficient: 0, lowest: 0 };
    let msr_offset: c_int = MSR_HWP_REQUEST;
    read_hwp_request_msr(cpu, &mut req, msr_offset as c_uint);
    if debug != 0 { print_hwp_request(cpu, &mut req, c"old: ".as_ptr() as *mut c_char); }
    if update_hwp_min != 0 && hwp_limits_done_via_sysfs == 0 { req.hwp_min = req_update.hwp_min; }
    if update_hwp_max != 0 && hwp_limits_done_via_sysfs == 0 { req.hwp_max = req_update.hwp_max; }
    if update_hwp_desired != 0 { req.hwp_desired = req_update.hwp_desired; }
    if update_hwp_window != 0 { req.hwp_window = req_update.hwp_window; }
    if update_hwp_epp != 0 { req.hwp_epp = req_update.hwp_epp; }
    req.hwp_use_pkg = req_update.hwp_use_pkg;
    read_hwp_cap(cpu, &mut cap, MSR_HWP_CAPABILITIES as c_uint);
    if debug != 0 { print_hwp_cap(cpu, &mut cap, c"".as_ptr() as *mut c_char); }
    if force == 0 { check_hwp_request_v_hwp_capabilities(cpu, &mut req, &mut cap); }
    verify_hwp_req_self_consistency(cpu, &mut req);
    write_hwp_request_msr(cpu, &mut req, msr_offset as c_uint);
    if debug != 0 {
        read_hwp_request_msr(cpu, &mut req, msr_offset as c_uint);
        print_hwp_request(cpu, &mut req, c"new: ".as_ptr() as *mut c_char);
    }
    0
}

pub unsafe fn update_hwp_request_pkg_msr(pkg: c_int) -> c_int {
    let mut req = msr_hwp_request { hwp_min: 0, hwp_max: 0, hwp_desired: 0, hwp_epp: 0, hwp_window: 0, hwp_use_pkg: 0 };
    let mut cap = msr_hwp_cap { highest: 0, guaranteed: 0, efficient: 0, lowest: 0 };
    let cpu: c_int = first_cpu_in_pkg[pkg as usize] as c_int;
    let msr_offset: c_int = MSR_HWP_REQUEST_PKG;
    read_hwp_request_msr(cpu, &mut req, msr_offset as c_uint);
    if debug != 0 { print_hwp_request_pkg(pkg, &mut req, c"old: ".as_ptr() as *mut c_char); }
    if update_hwp_min != 0 { req.hwp_min = req_update.hwp_min; }
    if update_hwp_max != 0 { req.hwp_max = req_update.hwp_max; }
    if update_hwp_desired != 0 { req.hwp_desired = req_update.hwp_desired; }
    if update_hwp_window != 0 { req.hwp_window = req_update.hwp_window; }
    if update_hwp_epp != 0 { req.hwp_epp = req_update.hwp_epp; }
    read_hwp_cap(cpu, &mut cap, MSR_HWP_CAPABILITIES as c_uint);
    if debug != 0 { print_hwp_cap(cpu, &mut cap, c"".as_ptr() as *mut c_char); }
    if force == 0 { check_hwp_request_v_hwp_capabilities(cpu, &mut req, &mut cap); }
    verify_hwp_req_self_consistency(cpu, &mut req);
    write_hwp_request_msr(cpu, &mut req, msr_offset as c_uint);
    if debug != 0 {
        read_hwp_request_msr(cpu, &mut req, msr_offset as c_uint);
        print_hwp_request_pkg(pkg, &mut req, c"new: ".as_ptr() as *mut c_char);
    }
    0
}

pub unsafe fn enable_hwp_on_cpu(cpu: c_int) -> c_int {
    let mut old_msr: c_ulonglong = 0;
    let new_msr: c_ulonglong;
    get_msr(cpu, MSR_PM_ENABLE, &mut old_msr);
    if (old_msr & 1) != 0 { return 0; } /* already enabled */
    new_msr = old_msr | 1;
    put_msr(cpu, MSR_PM_ENABLE, new_msr);
    if verbose != 0 {
        printf(c"cpu%d: MSR_PM_ENABLE old: %llX new: %llX\n".as_ptr(), cpu, old_msr, new_msr);
    }
    0
}

pub unsafe fn update_cpu_epb_sysfs(cpu: c_int) -> c_int {
    let epb = get_epb_sysfs(cpu);
    set_epb_sysfs(cpu, new_epb as c_int);
    if verbose != 0 {
        printf(c"cpu%d: ENERGY_PERF_BIAS old: %d new: %d\n".as_ptr(), cpu, epb, new_epb as c_uint);
    }
    0
}

pub unsafe fn update_cpu_msrs(cpu: c_int) -> c_int {
    let mut msr: c_ulonglong = 0;
    if update_turbo != 0 {
        let turbo_is_present_and_disabled: c_int;
        get_msr(cpu, MSR_IA32_MISC_ENABLE, &mut msr);
        turbo_is_present_and_disabled = ((msr & MSR_IA32_MISC_ENABLE_TURBO_DISABLE) != 0) as c_int;
        if turbo_update_value == 1 {
            if turbo_is_present_and_disabled != 0 {
                msr &= !MSR_IA32_MISC_ENABLE_TURBO_DISABLE;
                put_msr(cpu, MSR_IA32_MISC_ENABLE, msr);
                if verbose != 0 { printf(c"cpu%d: turbo ENABLE\n".as_ptr(), cpu); }
            }
        } else {
            /*
             * if "turbo_is_enabled" were known to be describe this cpu
             * then we could use it here to skip redundant disable requests.
             * but cpu may be in a different package, so we always write.
             */
            msr |= MSR_IA32_MISC_ENABLE_TURBO_DISABLE;
            put_msr(cpu, MSR_IA32_MISC_ENABLE, msr);
            if verbose != 0 { printf(c"cpu%d: turbo DISABLE\n".as_ptr(), cpu); }
        }
    }
    if has_hwp == 0 { return 0; }
    if hwp_update_enabled() == 0 { return 0; }
    update_hwp_request_msr(cpu);
    0
}

pub unsafe fn get_pkg_num(cpu: c_int) -> c_uint {
    let fp: *mut FILE;
    let mut pathname: [c_char; 128] = [0; 128];
    let mut pkg: c_uint = 0;
    let retval: c_int;
    sprintf(pathname.as_mut_ptr(), c"/sys/devices/system/cpu/cpu%d/topology/physical_package_id".as_ptr(), cpu);
    fp = fopen_or_die(pathname.as_ptr(), c"r".as_ptr());
    retval = fscanf(fp, c"%d\n".as_ptr(), &mut pkg);
    if retval != 1 {
        errx(1, c"%s: failed to parse".as_ptr(), pathname.as_ptr());
    }
    fclose(fp);
    pkg
}

pub unsafe fn set_max_cpu_pkg_num(cpu: c_int) -> c_int {
    let pkg: c_uint;
    if max_cpu_num < cpu { max_cpu_num = cpu; }
    pkg = get_pkg_num(cpu);
    if pkg as usize >= MAX_PACKAGES {
        errx(1, c"cpu%d: %d >= MAX_PACKAGES (%d)".as_ptr(), cpu, pkg, MAX_PACKAGES as c_int);
    }
    if pkg as c_int > max_pkg_num { max_pkg_num = pkg as c_int; }
    if (pkg_present_set & (1u64 << pkg)) == 0 {
        pkg_present_set |= 1u64 << pkg;
        first_cpu_in_pkg[pkg as usize] = cpu as c_uint;
    }
    0
}

pub unsafe fn mark_cpu_present(cpu: c_int) -> c_int {
    CPU_SET_S(cpu, cpu_setsize, cpu_present_set);
    0
}

/*
 * run func(cpu) on every cpu in /proc/stat
 * return max_cpu number
 */
pub unsafe fn for_all_proc_cpus(func: unsafe fn(c_int) -> c_int) -> c_int {
    let fp: *mut FILE;
    let mut cpu_num: c_int = 0;
    let mut retval: c_int;
    fp = fopen_or_die(proc_stat, c"r".as_ptr());
    retval = fscanf(fp, c"cpu %*d %*d %*d %*d %*d %*d %*d %*d %*d %*d\n".as_ptr());
    if retval != 0 {
        err(1, c"%s: failed to parse format".as_ptr(), proc_stat);
    }
    loop {
        retval = fscanf(fp, c"cpu%u %*d %*d %*d %*d %*d %*d %*d %*d %*d %*d\n".as_ptr(), &mut cpu_num);
        if retval != 1 { break; }
        retval = func(cpu_num);
        if retval != 0 {
            fclose(fp);
            return retval;
        }
    }
    fclose(fp);
    0
}

pub unsafe fn for_all_cpus_in_set(set_size: size_t, cpu_set: *mut cpu_set_t, func: unsafe fn(c_int) -> c_int) {
    let mut cpu_num: c_int = 0;
    while cpu_num <= max_cpu_num {
        if CPU_ISSET_S(cpu_num, set_size, cpu_set) != 0 {
            func(cpu_num);
        }
        cpu_num += 1;
    }
}

pub unsafe fn for_all_cpus_in_set_and(set_size: size_t, cpu_set: *mut cpu_set_t, func: unsafe fn(c_int) -> c_int) -> c_int {
    let mut cpu_num: c_int = 0;
    let mut retval: c_int = 1;
    while cpu_num <= max_cpu_num {
        if CPU_ISSET_S(cpu_num, set_size, cpu_set) != 0 {
            retval &= func(cpu_num);
        }
        cpu_num += 1;
    }
    retval
}

pub unsafe fn init_data_structures() {
    for_all_proc_cpus(set_max_cpu_pkg_num);
    cpu_setsize = CPU_ALLOC_SIZE(max_cpu_num + 1);
    cpu_present_set = CPU_ALLOC(max_cpu_num + 1);
    if cpu_present_set.is_null() {
        err(3, c"CPU_ALLOC".as_ptr());
    }
    CPU_ZERO_S(cpu_setsize, cpu_present_set);
    for_all_proc_cpus(mark_cpu_present);
}

pub unsafe fn is_hwp_enabled_on_cpu(cpu_num: c_int) -> c_int {
    let mut msr: c_ulonglong = 0;
    let retval: c_int;
    /* MSR_PM_ENABLE[1] == 1 if HWP is enabled and MSRs visible */
    get_msr(cpu_num, MSR_PM_ENABLE, &mut msr);
    retval = (msr & 1) as c_int;
    if verbose != 0 {
        fprintf(stderr, c"cpu%d: %sHWP\n".as_ptr(), cpu_num, if retval != 0 { c"".as_ptr() } else { c"No-".as_ptr() });
    }
    retval
}

/*
 * verify_hwp_is_enabled()
 *
 * Set (has_hwp=0) if no HWP feature or any of selected CPU set does not have HWP enabled
 */
pub unsafe fn verify_hwp_is_enabled() {
    let retval: c_int;
    if has_hwp == 0 { return; } /* set in early_cpuid() */
    retval = for_all_cpus_in_set_and(cpu_setsize, cpu_selected_set, is_hwp_enabled_on_cpu);
    if retval == 0 {
        fprintf(stderr, c"HWP can be enabled using '--hwp-enable'\n".as_ptr());
        has_hwp = 0;
    }
}

pub unsafe fn req_update_bounds_check() -> c_int {
    if hwp_update_enabled() == 0 { return 0; }
    /* fail if min > max requested */
    if (update_hwp_max != 0 && update_hwp_min != 0) && req_update.hwp_min > req_update.hwp_max {
        printf(c"hwp-min %d > hwp_max %d\n".as_ptr(), req_update.hwp_min as c_int, req_update.hwp_max as c_int);
        return -EINVAL;
    }
    /* fail if desired > max requestd */
    if req_update.hwp_desired != 0 && update_hwp_max != 0 && req_update.hwp_desired > req_update.hwp_max {
        printf(c"hwp-desired cannot be greater than hwp_max\n".as_ptr());
        return -EINVAL;
    }
    /* fail if desired < min requestd */
    if req_update.hwp_desired != 0 && update_hwp_min != 0 && req_update.hwp_desired < req_update.hwp_min {
        printf(c"hwp-desired cannot be less than hwp_min\n".as_ptr());
        return -EINVAL;
    }
    0
}

pub unsafe fn set_base_cpu() {
    base_cpu = sched_getcpu();
    if base_cpu < 0 {
        err(-ENODEV, c"No valid cpus found".as_ptr());
    }
}

unsafe fn probe_android_msr_path() {
    let mut sb = stat { _private: [] };
    let mut test_path: [c_char; 32] = [0; 32];
    sprintf(test_path.as_mut_ptr(), c"/dev/msr%d".as_ptr(), base_cpu);
    if stat(test_path.as_ptr(), &mut sb) == 0 {
        use_android_msr_path = 1;
    }
}

pub unsafe fn probe_dev_msr() {
    let mut sb = stat { _private: [] };
    let mut pathname: [c_char; 32] = [0; 32];
    probe_android_msr_path();
    sprintf(pathname.as_mut_ptr(), if use_android_msr_path != 0 { c"/dev/msr%d".as_ptr() } else { c"/dev/cpu/%d/msr".as_ptr() }, base_cpu);
    if stat(pathname.as_ptr(), &mut sb) != 0 {
        if system(c"/sbin/modprobe msr > /dev/null 2>&1".as_ptr()) != 0 {
            if use_android_msr_path != 0 {
                err(-5, c"no /dev/msr0, Try \"# modprobe msr\" ".as_ptr());
            } else {
                err(-5, c"no /dev/cpu/0/msr, Try \"# modprobe msr\" ".as_ptr());
            }
        }
    }
}

unsafe fn get_cpuid_or_exit(leaf: c_uint, eax: *mut c_uint, ebx: *mut c_uint, ecx: *mut c_uint, edx: *mut c_uint) {
    if __get_cpuid(leaf, eax, ebx, ecx, edx) == 0 {
        errx(1, c"Processor not supported\n".as_ptr());
    }
}

/*
 * early_cpuid()
 * initialize turbo_is_enabled, has_hwp, has_epb
 * before cmdline is parsed
 */
pub unsafe fn early_cpuid() {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;
    let mut fms: c_uint = 0;
    let mut family: c_uint;
    let mut model: c_uint;
    get_cpuid_or_exit(1, &mut fms, &mut ebx, &mut ecx, &mut edx);
    family = (fms >> 8) & 0xf;
    model = (fms >> 4) & 0xf;
    if family == 6 || family == 0xf {
        model += ((fms >> 16) & 0xf) << 4;
    }
    if model == 0x4f {
        let mut msr: c_ulonglong = 0;
        get_msr(base_cpu, MSR_TURBO_RATIO_LIMIT, &mut msr);
        bdx_highest_ratio = (msr & 0xff) as c_uint;
    }
    get_cpuid_or_exit(0x6, &mut eax, &mut ebx, &mut ecx, &mut edx);
    turbo_is_enabled = ((eax >> 1) & 1) as u8;
    has_hwp = (eax >> 7) & 1;
    has_epb = (ecx >> 3) & 1;
}

/*
 * parse_cpuid()
 * set
 * has_hwp, has_hwp_notify, has_hwp_activity_window, has_hwp_epp, has_hwp_request_pkg, has_epb
 */
pub unsafe fn parse_cpuid() {
    let mut eax: c_uint = 0;
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;
    let mut max_level: c_uint = 0;
    let mut fms: c_uint = 0;
    let mut family: c_uint;
    let mut model: c_uint;
    let stepping: c_uint;
    get_cpuid_or_exit(0, &mut max_level, &mut ebx, &mut ecx, &mut edx);
    if ebx == 0x756e6547 && edx == 0x49656e69 && ecx == 0x6c65746e {
        genuine_intel = 1;
    }
    if debug != 0 {
        fprintf(stderr, c"CPUID(0): %.4s%.4s%.4s ".as_ptr(),
                &ebx as *const _ as *const c_char, &edx as *const _ as *const c_char, &ecx as *const _ as *const c_char);
    }
    get_cpuid_or_exit(1, &mut fms, &mut ebx, &mut ecx, &mut edx);
    family = (fms >> 8) & 0xf;
    model = (fms >> 4) & 0xf;
    stepping = fms & 0xf;
    if family == 6 || family == 0xf {
        model += ((fms >> 16) & 0xf) << 4;
    }
    if debug != 0 {
        fprintf(stderr, c"%d CPUID levels; family:model:stepping 0x%x:%x:%x (%d:%d:%d)\n".as_ptr(),
                max_level, family, model, stepping, family, model, stepping);
        fprintf(stderr, c"CPUID(1): %s %s %s %s %s %s %s %s\n".as_ptr(),
                if (ecx & (1 << 0)) != 0 { c"SSE3".as_ptr() } else { c"-".as_ptr() },
                if (ecx & (1 << 3)) != 0 { c"MONITOR".as_ptr() } else { c"-".as_ptr() },
                if (ecx & (1 << 7)) != 0 { c"EIST".as_ptr() } else { c"-".as_ptr() },
                if (ecx & (1 << 8)) != 0 { c"TM2".as_ptr() } else { c"-".as_ptr() },
                if (edx & (1 << 4)) != 0 { c"TSC".as_ptr() } else { c"-".as_ptr() },
                if (edx & (1 << 5)) != 0 { c"MSR".as_ptr() } else { c"-".as_ptr() },
                if (edx & (1 << 22)) != 0 { c"ACPI-TM".as_ptr() } else { c"-".as_ptr() },
                if (edx & (1 << 29)) != 0 { c"TM".as_ptr() } else { c"-".as_ptr() });
    }
    if (edx & (1 << 5)) == 0 {
        errx(1, c"CPUID: no MSR".as_ptr());
    }
    get_cpuid_or_exit(0x6, &mut eax, &mut ebx, &mut ecx, &mut edx);
    /* turbo_is_enabled already set */
    /* has_hwp already set */
    has_hwp_notify = eax & (1 << 8);
    has_hwp_activity_window = eax & (1 << 9);
    has_hwp_epp = eax & (1 << 10);
    has_hwp_request_pkg = eax & (1 << 11);
    if has_hwp_request_pkg == 0 && update_hwp_use_pkg != 0 {
        errx(1, c"--hwp-use-pkg is not available on this hardware".as_ptr());
    }
    /* has_epb already set */
    if debug != 0 {
        fprintf(stderr, c"CPUID(6): %sTURBO, %sHWP, %sHWPnotify, %sHWPwindow, %sHWPepp, %sHWPpkg, %sEPB\n".as_ptr(),
                if turbo_is_enabled != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
                if has_hwp != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
                if has_hwp_notify != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
                if has_hwp_activity_window != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
                if has_hwp_epp != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
                if has_hwp_request_pkg != 0 { c"".as_ptr() } else { c"No-".as_ptr() },
                if has_epb != 0 { c"".as_ptr() } else { c"No-".as_ptr() });
    }
    return; /* success */
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    set_base_cpu();
    probe_dev_msr();
    init_data_structures();
    early_cpuid(); /* initial cpuid parse before cmdline */
    cmdline(argc, argv);
    if debug != 0 {
        print_version();
    }
    parse_cpuid();
    /* If CPU-set and PKG-set are not initialized, default to all CPUs */
    if cpu_selected_set.is_null() && pkg_selected_set == 0 {
        cpu_selected_set = cpu_present_set;
    }
    /*
     * If HWP is being enabled, do it now, so that subsequent operations
     * that access HWP registers can work.
     */
    if update_hwp_enable != 0 {
        for_all_cpus_in_set(cpu_setsize, cpu_selected_set, enable_hwp_on_cpu);
    }
    /* If HWP present, but disabled, warn and ignore from here forward */
    verify_hwp_is_enabled();
    if req_update_bounds_check() != 0 {
        return -EINVAL;
    }
    /* display information only, no updates to settings */
    if update_epb == 0 && update_turbo == 0 && hwp_update_enabled() == 0 &&
        update_soc_slider_balance == 0 && update_soc_slider_offset == 0 && update_platform_profile == 0 {
        if !cpu_selected_set.is_null() {
            for_all_cpus_in_set(cpu_setsize, cpu_selected_set, print_cpu_msrs);
        }
        print_soc_slider();
        print_platform_profile();
        if has_hwp_request_pkg != 0 {
            if pkg_selected_set == 0 {
                pkg_selected_set = pkg_present_set;
            }
            for_packages(pkg_selected_set, print_pkg_msrs);
        }
        return 0;
    }
    /* update CPU set */
    if !cpu_selected_set.is_null() {
        if update_epb != 0 {
            for_all_cpus_in_set(cpu_setsize, cpu_selected_set, update_cpu_epb_sysfs);
        }
        for_all_cpus_in_set(cpu_setsize, cpu_selected_set, update_sysfs);
        for_all_cpus_in_set(cpu_setsize, cpu_selected_set, update_cpu_msrs);
    } else if pkg_selected_set != 0 {
        for_packages(pkg_selected_set, update_hwp_request_pkg_msr);
    }
    if update_soc_slider_balance != 0 || update_soc_slider_offset != 0 || update_platform_profile != 0 {
        update_soc_slider();
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
