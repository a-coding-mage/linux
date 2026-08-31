// SPDX-License-Identifier: GPL-2.0
/*
 * ldt_gdt.c - Test cases for LDT and GDT access
 * Copyright (c) 2011-2015 Andrew Lutomirski
 */

use std::ffi::CStr;
use std::mem::{self, size_of, transmute};
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

use libc::{
    __errno_location, clockid_t, getauxval, sched_setaffinity, sscanf, strcmp, strerror,
    __suseconds_t, timespec, timezone, timeval, sysconf, syscall,
};

type __kernel_time_t = c_long;
type __kernel_old_time_t = c_long;

type vgettime_t = unsafe extern "C" fn(clockid_t, *mut timespec) -> c_int;
type vgettime64_t = unsafe extern "C" fn(clockid_t, *mut __kernel_timespec) -> c_int;
type vgtod_t = unsafe extern "C" fn(*mut timeval, *mut timezone) -> c_long;
type vtime_t = unsafe extern "C" fn(*mut __kernel_old_time_t) -> __kernel_time_t;
type getcpu_t = unsafe extern "C" fn(*mut c_uint, *mut c_uint, *mut c_void) -> c_long;

#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
#[repr(C)]
#[derive(Clone, Copy)]
struct CpuSet {
    bits: [c_ulong; 16],
}

macro_rules! cpu_zero {
    ($set:expr) => {{
        ($set).bits.iter_mut().for_each(|v| *v = 0);
    }};
}

macro_rules! cpu_set {
    ($cpu:expr, $set:expr) => {{
        let cpu: usize = $cpu as usize;
        let bits = size_of::<c_ulong>() * 8;
        let idx = cpu / bits;
        let off = cpu % bits;
        unsafe {
            let ptr = (*$set).bits.as_mut_ptr().add(idx);
            *ptr |= (1u64 << off) as c_ulong;
        }
    }};
}

macro_rules! VDSO_CALL {
    ($f:expr, $argc:expr, $($arg:expr),+) => {
        unsafe { ($f)($($arg),+) }
    };
}

/* max length of lines in /proc/self/maps - anything longer is skipped here */
const MAPS_LINE_LEN: usize = 128;

static mut VERSION: *const c_char = std::ptr::null();
static mut NAME: *const *const c_char = std::ptr::null();

#[cfg(not(feature = "clock_gettime64_def"))]
const __NR_clock_gettime64: c_long = 403;

/*
 * This translation keeps the fallback value from the C source when the platform
 * does not provide a dedicated clock_gettime64 syscall constant.
 */
#[allow(non_upper_case_globals)]
const __NR_time: c_long = libc::SYS_time;

// int nerrs = 0;
static mut NERRS: c_int = 0;

static mut VDSO_CLOCK_GETTIME: Option<vgettime_t> = None;
static mut VDSO_CLOCK_GETTIME64: Option<vgettime64_t> = None;
static mut VDSO_GETTIMEOFDAY: Option<vgtod_t> = None;
static mut VDSO_TIME: Option<vtime_t> = None;
static mut VGETCPU: Option<getcpu_t> = None;
static mut VDSO_GETCPU: Option<getcpu_t> = None;

extern "C" {
    fn vdso_init_from_sysinfo_ehdr(sysinfo_ehdr: c_ulong);
    fn vdso_sym(version: *const c_char, name: *const c_char) -> *const c_void;

    static versions: *const *const c_char;
    static names: *const *const c_char;
    static VDSO_VERSION: c_int;
    static VDSO_NAMES: c_int;

    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut libc::FILE;
    fn fclose(stream: *mut libc::FILE) -> c_int;
    fn fgets(s: *mut c_char, n: c_int, stream: *mut libc::FILE) -> *mut c_char;
}

fn get_errno() -> c_int {
    unsafe { *__errno_location() }
}

fn set_errno(value: c_int) {
    unsafe {
        *__errno_location() = value;
    }
}

fn cstr_to_str(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return "<null>".to_string();
    }
    unsafe {
        CStr::from_ptr(ptr)
            .to_string_lossy()
            .into_owned()
    }
}

fn ptr_from_sym_or_null<T>(sym: *const c_void) -> Option<T>
where
    T: Copy,
{
    if sym.is_null() {
        None
    } else {
        Some(unsafe { transmute::<*const c_void, T>(sym) })
    }
}

#[cfg(target_arch = "x86_64")]
fn vsyscall_getcpu() -> *mut c_void {
    use std::io::Write;

    let mut line = [0i8; MAPS_LINE_LEN];
    let mut found = false;
    let path = b"/proc/self/maps\0";
    let mode = b"r\0";

    let maps = unsafe { fopen(path.as_ptr() as *const c_char, mode.as_ptr() as *const c_char) };
    if maps.is_null() {
        // might still be present, but ignore it here, as we test vDSO not vsyscall
        return std::ptr::null_mut();
    }

    loop {
        let line_ptr = unsafe { fgets(line.as_mut_ptr(), MAPS_LINE_LEN as c_int, maps) };
        if line_ptr.is_null() {
            break;
        }

        let mut start: *mut c_void = std::ptr::null_mut();
        let mut end: *mut c_void = std::ptr::null_mut();
        let mut r: c_char = 0;
        let mut x: c_char = 0;
        let mut name = [0i8; MAPS_LINE_LEN];
        let n = unsafe {
            sscanf(
                line_ptr,
                b"%p-%p %c-%cp %*x %*x:%*x %*u %s\0".as_ptr() as *const c_char,
                &mut start as *mut _,
                &mut end as *mut _,
                &mut r as *mut _,
                &mut x as *mut _,
                name.as_mut_ptr(),
            )
        };
        if n != 5 {
            continue;
        }

        let is_vsyscall = unsafe { strcmp(name.as_ptr(), b"[vsyscall]\0".as_ptr() as *const c_char) } == 0;
        if !is_vsyscall {
            continue;
        }

        // assume entries are OK, as we test vDSO here not vsyscall
        found = true;
        let _ = unsafe { libc::memcpy(std::ptr::null_mut(), std::ptr::null_mut(), 0) };
        let _ = (start, end, r, x);
        break;
    }

    unsafe { fclose(maps) };

    if !found {
        println!("Warning: failed to find vsyscall getcpu");
        return std::ptr::null_mut();
    }

    0xffffffffff600800usize as *mut c_void
}

#[cfg(not(target_arch = "x86_64"))]
fn vsyscall_getcpu() -> *mut c_void {
    std::ptr::null_mut()
}

fn fill_function_pointers() {
    let sysinfo_ehdr = unsafe { getauxval(libc::AT_SYSINFO_EHDR) };

    if sysinfo_ehdr == 0 {
        println!("[WARN]\tfailed to find vDSO");
        return;
    }

    unsafe {
        vdso_init_from_sysinfo_ehdr(sysinfo_ehdr);

        // C: version = versions[VDSO_VERSION];
        VERSION = *versions.add(VDSO_VERSION as usize);
        // C: name = (const char **)&names[VDSO_NAMES];
        NAME = names.add(VDSO_NAMES as usize);

        let getcpu_sym = vdso_sym(VERSION, *NAME.add(4));
        VDSO_GETCPU = ptr_from_sym_or_null::<getcpu_t>(getcpu_sym);
        if VDSO_GETCPU.is_none() {
            println!("Warning: failed to find getcpu in vDSO");
        }

        VGETCPU = {
            let p = vsyscall_getcpu();
            if p.is_null() {
                None
            } else {
                Some(transmute::<*mut c_void, getcpu_t>(p as *const c_void))
            }
        };

        let clock_gettime_sym = vdso_sym(VERSION, *NAME.add(1));
        VDSO_CLOCK_GETTIME = ptr_from_sym_or_null::<vgettime_t>(clock_gettime_sym);
        if VDSO_CLOCK_GETTIME.is_none() {
            println!("Warning: failed to find clock_gettime in vDSO");
        }

        #[cfg(target_pointer_width = "32")]
        {
            let clock_gettime64_sym = vdso_sym(VERSION, *NAME.add(5));
            VDSO_CLOCK_GETTIME64 = ptr_from_sym_or_null::<vgettime64_t>(clock_gettime64_sym);
            if VDSO_CLOCK_GETTIME64.is_none() {
                println!("Warning: failed to find clock_gettime64 in vDSO");
            }
        }

        let gettimeofday_sym = vdso_sym(VERSION, *NAME.add(0));
        VDSO_GETTIMEOFDAY = ptr_from_sym_or_null::<vgtod_t>(gettimeofday_sym);
        if VDSO_GETTIMEOFDAY.is_none() {
            println!("Warning: failed to find gettimeofday in vDSO");
        }

        let time_sym = vdso_sym(VERSION, *NAME.add(2));
        VDSO_TIME = ptr_from_sym_or_null::<vtime_t>(time_sym);
        if VDSO_TIME.is_none() {
            println!("Warning: failed to find time in vDSO");
        }
    }
}

fn sys_getcpu(cpu: *mut c_uint, node: *mut c_uint, cache: *mut c_void) -> c_long {
    unsafe { syscall(libc::SYS_getcpu, cpu, node, cache) }
}

fn sys_clock_gettime(id: clockid_t, ts: *mut timespec) -> c_int {
    unsafe { syscall(libc::SYS_clock_gettime, id, ts) as c_int }
}

fn sys_clock_gettime64(id: clockid_t, ts: *mut __kernel_timespec) -> c_int {
    unsafe { syscall(__NR_clock_gettime64, id, ts) as c_int }
}

fn sys_gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int {
    unsafe { syscall(libc::SYS_gettimeofday, tv, tz) as c_int }
}

fn sys_time(tloc: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    unsafe { syscall(__NR_time, tloc) as __kernel_old_time_t }
}

fn test_getcpu() {
    println!("[RUN]\tTesting getcpu...");

    let mut cpu: c_int = 0;
    loop {
        let mut cpuset = CpuSet { bits: [0; 16] };
        cpu_zero!(&mut cpuset);
        cpu_set!(cpu, &mut cpuset);

        if unsafe {
            sched_setaffinity(
                0,
                size_of::<CpuSet>(),
                &cpuset as *const CpuSet as *mut libc::c_void,
            )
        } != 0
        {
            return;
        }

        let mut cpu_sys: c_uint = 0;
        let mut cpu_vdso: c_uint = 0;
        let mut cpu_vsys: c_uint = 0;
        let mut node_sys: c_uint = 0;
        let mut node_vdso: c_uint = 0;
        let mut node_vsys: c_uint = 0;
        let mut ret_sys: c_long = 1;
        let mut ret_vdso: c_long = 1;
        let mut ret_vsys: c_long = 1;
        let mut node: c_uint = 0;

        ret_sys = sys_getcpu(&mut cpu_sys, &mut node_sys, std::ptr::null_mut());
        unsafe {
            if let Some(vdso_getcpu) = VDSO_GETCPU {
                ret_vdso = VDSO_CALL!(vdso_getcpu, 3, &mut cpu_vdso, &mut node_vdso, std::ptr::null_mut());
            }

            if let Some(vsyscall_getcpu) = VGETCPU {
                ret_vsys = VDSO_CALL!(vsyscall_getcpu, 3, &mut cpu_vsys, &mut node_vsys, std::ptr::null_mut());
            }
        }

        if ret_sys == 0 {
            node = node_sys;
        } else if ret_vdso == 0 {
            node = node_vdso;
        } else if ret_vsys == 0 {
            node = node_vsys;
        }

        let mut ok = true;
        if ret_sys == 0 && (cpu_sys != cpu as c_uint || node_sys != node) {
            ok = false;
        }
        if ret_vdso == 0 && (cpu_vdso != cpu as c_uint || node_vdso != node) {
            ok = false;
        }
        if ret_vsys == 0 && (cpu_vsys != cpu as c_uint || node_vsys != node) {
            ok = false;
        }

        println!("[{}]\tCPU {}:", if ok { "OK" } else { "FAIL" }, cpu);
        if ret_sys == 0 {
            print!(" syscall: cpu {}, node {}", cpu_sys, node_sys);
        }
        if ret_vdso == 0 {
            print!(" vdso: cpu {}, node {}", cpu_vdso, node_vdso);
        }
        if ret_vsys == 0 {
            print!(" vsyscall: cpu {}, node {}", cpu_vsys, node_vsys);
        }
        println!("");

        if !ok {
            unsafe { NERRS += 1; }
        }

        cpu += 1;
    }
}

fn ts_leq(a: &timespec, b: &timespec) -> bool {
    if a.tv_sec != b.tv_sec {
        a.tv_sec < b.tv_sec
    } else {
        a.tv_nsec <= b.tv_nsec
    }
}

fn ts64_leq(a: &__kernel_timespec, b: &__kernel_timespec) -> bool {
    if a.tv_sec != b.tv_sec {
        a.tv_sec < b.tv_sec
    } else {
        a.tv_nsec <= b.tv_nsec
    }
}

fn tv_leq(a: &timeval, b: &timeval) -> bool {
    if a.tv_sec != b.tv_sec {
        a.tv_sec < b.tv_sec
    } else {
        a.tv_usec <= b.tv_usec
    }
}

static CLOCKNAMES: [&[u8]; 12] = [
    b"CLOCK_REALTIME\0",
    b"CLOCK_MONOTONIC\0",
    b"CLOCK_PROCESS_CPUTIME_ID\0",
    b"CLOCK_THREAD_CPUTIME_ID\0",
    b"CLOCK_MONOTONIC_RAW\0",
    b"CLOCK_REALTIME_COARSE\0",
    b"CLOCK_MONOTONIC_COARSE\0",
    b"CLOCK_BOOTTIME\0",
    b"CLOCK_REALTIME_ALARM\0",
    b"CLOCK_BOOTTIME_ALARM\0",
    b"CLOCK_SGI_CYCLE\0",
    b"CLOCK_TAI\0",
];

fn test_one_clock_gettime(clock: c_int, name: *const c_char) {
    let mut start = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut vdso = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let vdso_ret: c_int;
    let end_ret: c_int;

    println!(
        "[RUN]\tTesting clock_gettime for clock {} ({})...",
        cstr_to_str(name),
        clock
    );

    if sys_clock_gettime(clock, &mut start as *mut timespec) < 0 {
        if get_errno() == libc::EINVAL {
            let mut vdso_ret_tmp = 0;
            unsafe {
                if let Some(vdso_clock_gettime) = VDSO_CLOCK_GETTIME {
                    vdso_ret_tmp = VDSO_CALL!(vdso_clock_gettime, 2, clock, &mut vdso as *mut timespec);
                }
            }
            vdso_ret = vdso_ret_tmp;
            if vdso_ret == -libc::EINVAL {
                println!("[OK]\tNo such clock.");
            } else {
                println!(
                    "[FAIL]\tNo such clock, but __vdso_clock_gettime returned {}",
                    vdso_ret
                );
                unsafe { NERRS += 1; }
            }
        } else {
            println!(
                "[WARN]\t clock_gettime({}) syscall returned error {}",
                clock,
                get_errno()
            );
        }
        return;
    }

    unsafe {
        vdso_ret = if let Some(vdso_clock_gettime) = VDSO_CLOCK_GETTIME {
            VDSO_CALL!(vdso_clock_gettime, 2, clock, &mut vdso as *mut timespec)
        } else {
            1
        };
    }
    end_ret = sys_clock_gettime(clock, &mut end as *mut timespec);

    if vdso_ret != 0 || end_ret != 0 {
        println!(
            "[FAIL]\tvDSO returned {}, syscall errno={}",
            vdso_ret,
            get_errno()
        );
        unsafe { NERRS += 1; }
        return;
    }

    println!(
        "\t{}.{:09} {}.{:09} {}.{:09}",
        start.tv_sec,
        start.tv_nsec,
        vdso.tv_sec,
        vdso.tv_nsec,
        end.tv_sec,
        end.tv_nsec
    );

    if !ts_leq(&start, &vdso) || !ts_leq(&vdso, &end) {
        println!("[FAIL]\tTimes are out of sequence");
        unsafe { NERRS += 1; }
        return;
    }

    println!("[OK]\tTest Passed.");
}

fn test_clock_gettime() {
    if unsafe { VDSO_CLOCK_GETTIME.is_none() } {
        println!("[SKIP]\tNo vDSO, so skipping clock_gettime() tests");
        return;
    }

    for clock in 0..CLOCKNAMES.len() {
        test_one_clock_gettime(clock as c_int, CLOCKNAMES[clock].as_ptr() as *const c_char);
    }

    /* Also test some invalid clock ids */
    test_one_clock_gettime(-1, b"invalid\0".as_ptr() as *const c_char);
    test_one_clock_gettime(i32::MIN, b"invalid\0".as_ptr() as *const c_char);
    test_one_clock_gettime(i32::MAX, b"invalid\0".as_ptr() as *const c_char);
}

#[cfg(target_pointer_width = "32")]
fn test_one_clock_gettime64(clock: c_int, name: *const c_char) {
    let mut start = __kernel_timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut vdso = __kernel_timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end = __kernel_timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let vdso_ret: c_int;
    let end_ret: c_int;

    println!(
        "[RUN]\tTesting clock_gettime64 for clock {} ({})...",
        cstr_to_str(name),
        clock
    );

    if sys_clock_gettime64(clock, &mut start as *mut __kernel_timespec) < 0 {
        if get_errno() == libc::EINVAL {
            let mut vdso_ret_tmp = 0;
            unsafe {
                if let Some(vdso_clock_gettime64) = VDSO_CLOCK_GETTIME64 {
                    vdso_ret_tmp = VDSO_CALL!(vdso_clock_gettime64, 2, clock, &mut vdso as *mut __kernel_timespec);
                }
            }
            vdso_ret = vdso_ret_tmp;
            if vdso_ret == -libc::EINVAL {
                println!("[OK]\tNo such clock.");
            } else {
                println!(
                    "[FAIL]\tNo such clock, but __vdso_clock_gettime64 returned {}",
                    vdso_ret
                );
                unsafe { NERRS += 1; }
            }
        } else {
            println!(
                "[WARN]\t clock_gettime64({}) syscall returned error {}",
                clock,
                get_errno()
            );
        }
        return;
    }

    unsafe {
        vdso_ret = if let Some(vdso_clock_gettime64) = VDSO_CLOCK_GETTIME64 {
            VDSO_CALL!(vdso_clock_gettime64, 2, clock, &mut vdso as *mut __kernel_timespec)
        } else {
            1
        };
    }
    end_ret = sys_clock_gettime64(clock, &mut end as *mut __kernel_timespec);

    if vdso_ret != 0 || end_ret != 0 {
        println!(
            "[FAIL]\tvDSO returned {}, syscall errno={}",
            vdso_ret,
            get_errno()
        );
        unsafe { NERRS += 1; }
        return;
    }

    println!(
        "\t{}.{:09} {}.{:09} {}.{:09}",
        start.tv_sec,
        start.tv_nsec,
        vdso.tv_sec,
        vdso.tv_nsec,
        end.tv_sec,
        end.tv_nsec
    );

    if !ts64_leq(&start, &vdso) || !ts64_leq(&vdso, &end) {
        println!("[FAIL]\tTimes are out of sequence");
        unsafe { NERRS += 1; }
        return;
    }

    println!("[OK]\tTest Passed.");
}

#[cfg(target_pointer_width = "32")]
fn test_clock_gettime64() {
    if unsafe { VDSO_CLOCK_GETTIME64.is_none() } {
        println!("[SKIP]\tNo vDSO, so skipping clock_gettime64() tests");
        return;
    }

    for clock in 0..CLOCKNAMES.len() {
        test_one_clock_gettime64(clock as c_int, CLOCKNAMES[clock].as_ptr() as *const c_char);
    }

    /* Also test some invalid clock ids */
    test_one_clock_gettime64(-1, b"invalid\0".as_ptr() as *const c_char);
    test_one_clock_gettime64(i32::MIN, b"invalid\0".as_ptr() as *const c_char);
    test_one_clock_gettime64(i32::MAX, b"invalid\0".as_ptr() as *const c_char);
}

#[cfg(not(target_pointer_width = "32"))]
fn test_clock_gettime64() {
    // In the C source this section is compiled only when VDSO_32BIT is defined.
    println!("[SKIP]\tNo vDSO_32BIT target, so skipping clock_gettime64() tests");
}

fn test_gettimeofday() {
    let mut start = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut vdso = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut end = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut sys_tz = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let mut vdso_tz = timezone {
        tz_minuteswest: 0,
        tz_dsttime: 0,
    };
    let vdso_ret: c_int;
    let end_ret: c_int;

    if unsafe { VDSO_GETTIMEOFDAY.is_none() } {
        return;
    }

    println!("[RUN]\tTesting gettimeofday...");

    if sys_gettimeofday(&mut start as *mut timeval, &mut sys_tz as *mut timezone) < 0 {
        println!("[FAIL]\tsys_gettimeofday failed ({})", get_errno());
        unsafe { NERRS += 1; }
        return;
    }

    vdso_ret = unsafe {
        if let Some(vdso_gettimeofday) = VDSO_GETTIMEOFDAY {
            VDSO_CALL!(vdso_gettimeofday, 2, &mut vdso as *mut timeval, &mut vdso_tz as *mut timezone)
        } else {
            1
        }
    };
    end_ret = sys_gettimeofday(&mut end as *mut timeval, std::ptr::null_mut());

    if vdso_ret != 0 || end_ret != 0 {
        println!("[FAIL]\tvDSO returned {}, syscall errno={}", vdso_ret, get_errno());
        unsafe { NERRS += 1; }
        return;
    }

    println!(
        "\t{}.{:06} {}.{:06} {}.{:06}",
        start.tv_sec,
        start.tv_usec,
        vdso.tv_sec,
        vdso.tv_usec,
        end.tv_sec,
        end.tv_usec,
    );

    if !tv_leq(&start, &vdso) || !tv_leq(&vdso, &end) {
        println!("[FAIL]\tTimes are out of sequence");
        unsafe { NERRS += 1; }
    }

    if sys_tz.tz_minuteswest == vdso_tz.tz_minuteswest && sys_tz.tz_dsttime == vdso_tz.tz_dsttime {
        println!(
            "[OK]\ttimezones match: minuteswest={}, dsttime={}",
            sys_tz.tz_minuteswest,
            sys_tz.tz_dsttime
        );
    } else {
        println!("[FAIL]\ttimezones do not match");
        unsafe { NERRS += 1; }
    }

    // And make sure that passing NULL for tz doesn't crash.
    unsafe {
        if let Some(vdso_gettimeofday) = VDSO_GETTIMEOFDAY {
            let _ = VDSO_CALL!(vdso_gettimeofday, 2, &mut vdso as *mut timeval, std::ptr::null_mut());
        }
    }
}

fn test_time() {
    let mut start: __kernel_old_time_t = 0;
    let mut end: __kernel_old_time_t = 0;
    let vdso_ret: __kernel_old_time_t;
    let vdso_param: __kernel_old_time_t;

    if unsafe { VDSO_TIME.is_none() } {
        return;
    }

    println!("[RUN]\tTesting time...");

    if sys_time(&mut start as *mut __kernel_old_time_t) < 0 {
        if get_errno() == -libc::ENOSYS {
            println!("[SKIP]\tNo time() support");
        } else {
            println!("[FAIL]\tsys_time failed ({})", get_errno());
            unsafe { NERRS += 1; }
        }
        return;
    }

    let mut param = 0;
    vdso_ret = unsafe {
        if let Some(vdso_time) = VDSO_TIME {
            VDSO_CALL!(vdso_time, 1, &mut param as *mut __kernel_old_time_t)
        } else {
            -1
        }
    };
    let vdso_param = param;
    end = sys_time(std::ptr::null_mut());

    if vdso_ret < 0 || end < 0 {
        println!("[FAIL]\tvDSO returned {}, syscall errno={}", vdso_ret, get_errno());
        unsafe { NERRS += 1; }
        return;
    }

    println!("\t{} {} {}", start, vdso_ret, end);

    if vdso_ret != vdso_param {
        println!("[FAIL]\tinconsistent return values: {} {}", vdso_ret, vdso_param);
        unsafe { NERRS += 1; }
        return;
    }

    if !(start <= vdso_ret) || !(vdso_ret <= end) {
        println!("[FAIL]\tTimes are out of sequence");
        unsafe { NERRS += 1; }
    }
}

fn main() {
    unsafe {
        VERSION = *versions.add(VDSO_VERSION as usize);
        NAME = names.add(VDSO_NAMES as usize);
    }

    fill_function_pointers();

    test_clock_gettime();
    test_clock_gettime64();
    test_gettimeofday();
    test_time();

    /*
     * Test getcpu() last so that, if something goes wrong setting affinity,
     * we still run the other tests.
     */
    test_getcpu();

    unsafe { if NERRS != 0 { 1 } else { 0 } }
}
