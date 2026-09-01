/* SPDX-License-Identifier: GPL-2.0 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

type time_t = c_long;
type clockid_t = c_int;
type size_t = usize;
type pid_t = c_int;
type sig_atomic_t = c_int;

#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime: c_int,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    _private: [u8; 128],
}

#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: size_t,
}

#[cfg(target_arch = "x86_64")]
pub type greg_t = c_long;

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: [greg_t; 23],
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[cfg(target_arch = "x86_64")]
const TOTAL_TESTS: c_int = 13;
#[cfg(not(target_arch = "x86_64"))]
const TOTAL_TESTS: c_int = 8;

#[cfg(target_arch = "x86_64")]
const fn VSYS(x: usize) -> usize {
    x
}

#[cfg(not(target_arch = "x86_64"))]
const fn VSYS(_x: usize) -> usize {
    0
}

#[cfg(target_arch = "x86_64")]
const SYS_getcpu: c_long = 309;
#[cfg(not(target_arch = "x86_64"))]
const SYS_getcpu: c_long = 318;

const SYS_gettimeofday: c_long = 96;
const SYS_time: c_long = 201;

const RTLD_LAZY: c_int = 0x00001;
const RTLD_LOCAL: c_int = 0;
const RTLD_NOLOAD: c_int = 0x00004;
const SIGSEGV: c_int = 11;
const SIGTRAP: c_int = 5;

#[cfg(target_arch = "x86_64")]
const REG_RIP: usize = 16;
#[cfg(target_arch = "x86_64")]
const REG_ERR: usize = 19;
#[cfg(target_arch = "x86_64")]
const REG_TRAPNO: usize = 20;
#[cfg(target_arch = "x86_64")]
const X86_EFLAGS_TF: c_ulong = 0x00000100;

/* max length of lines in /proc/self/maps - anything longer is skipped here */
const MAPS_LINE_LEN: usize = 128;

/* vsyscalls and vDSO */
static mut vsyscall_map_r: bool = false;
static mut vsyscall_map_x: bool = false;

type gtod_t = unsafe extern "C" fn(tv: *mut timeval, tz: *mut timezone) -> c_long;
static vgtod: gtod_t = unsafe { core::mem::transmute::<usize, gtod_t>(VSYS(0xffffffffff600000usize)) };
static mut vdso_gtod: Option<gtod_t> = None;

type vgettime_t = unsafe extern "C" fn(clockid_t, *mut timespec) -> c_int;
static mut vdso_gettime: Option<vgettime_t> = None;

type time_func_t = unsafe extern "C" fn(t: *mut time_t) -> c_long;
static vtime: time_func_t = unsafe { core::mem::transmute::<usize, time_func_t>(VSYS(0xffffffffff600400usize)) };
static mut vdso_time: Option<time_func_t> = None;

type getcpu_t = unsafe extern "C" fn(*mut c_uint, *mut c_uint, *mut c_void) -> c_long;
static vgetcpu: getcpu_t = unsafe { core::mem::transmute::<usize, getcpu_t>(VSYS(0xffffffffff600800usize)) };
static mut vdso_getcpu: Option<getcpu_t> = None;

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn syscall(number: c_long, ...) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn getpid() -> pid_t;
    fn process_vm_readv(
        pid: pid_t,
        local_iov: *const iovec,
        liovcnt: c_ulong,
        remote_iov: *const iovec,
        riovcnt: c_ulong,
        flags: c_ulong,
    ) -> isize;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_finished() -> !;

    fn sethandler(sig: c_int, handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void), flags: c_int);
    fn get_eflags() -> c_ulong;
    fn set_eflags(eflags: c_ulong);
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    let set = &mut *set;
    let mut i = 0usize;
    while i < set.__bits.len() {
        set.__bits[i] = 0;
        i += 1;
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
    let set = &mut *set;
    set.__bits[cpu as usize / bits_per_word] |= 1c_ulong << (cpu as usize % bits_per_word);
}

unsafe fn init_vdso() {
    let mut vdso = dlopen(
        c"linux-vdso.so.1".as_ptr(),
        RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD,
    );
    if vdso.is_null() {
        vdso = dlopen(
            c"linux-gate.so.1".as_ptr(),
            RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD,
        );
    }
    if vdso.is_null() {
        ksft_print_msg(c"[WARN] failed to find vDSO\n".as_ptr());
        return;
    }

    let sym = dlsym(vdso, c"__vdso_gettimeofday".as_ptr());
    vdso_gtod = if sym.is_null() { None } else { Some(core::mem::transmute::<*mut c_void, gtod_t>(sym)) };
    if vdso_gtod.is_none() {
        ksft_print_msg(c"[WARN] failed to find gettimeofday in vDSO\n".as_ptr());
    }

    let sym = dlsym(vdso, c"__vdso_clock_gettime".as_ptr());
    vdso_gettime = if sym.is_null() { None } else { Some(core::mem::transmute::<*mut c_void, vgettime_t>(sym)) };
    if vdso_gettime.is_none() {
        ksft_print_msg(c"[WARN] failed to find clock_gettime in vDSO\n".as_ptr());
    }

    let sym = dlsym(vdso, c"__vdso_time".as_ptr());
    vdso_time = if sym.is_null() { None } else { Some(core::mem::transmute::<*mut c_void, time_func_t>(sym)) };
    if vdso_time.is_none() {
        ksft_print_msg(c"[WARN] failed to find time in vDSO\n".as_ptr());
    }

    let sym = dlsym(vdso, c"__vdso_getcpu".as_ptr());
    vdso_getcpu = if sym.is_null() { None } else { Some(core::mem::transmute::<*mut c_void, getcpu_t>(sym)) };
    if vdso_getcpu.is_none() {
        ksft_print_msg(c"[WARN] failed to find getcpu in vDSO\n".as_ptr());
    }
}

/* syscalls */
#[inline]
unsafe fn sys_gtod(tv: *mut timeval, tz: *mut timezone) -> c_long {
    syscall(SYS_gettimeofday, tv, tz)
}

#[inline]
unsafe fn sys_time(t: *mut time_t) -> c_long {
    syscall(SYS_time, t)
}

#[inline]
unsafe fn sys_getcpu(cpu: *mut c_uint, node: *mut c_uint, cache: *mut c_void) -> c_long {
    syscall(SYS_getcpu, cpu, node, cache)
}

unsafe fn tv_diff(a: *const timeval, b: *const timeval) -> c_double {
    ((*a).tv_sec - (*b).tv_sec) as c_double
        + (((*a).tv_usec as c_int - (*b).tv_usec as c_int) as c_double) * 1e-6
}

unsafe fn check_gtod(
    tv_sys1: *const timeval,
    tv_sys2: *const timeval,
    tz_sys: *const timezone,
    which: *const c_char,
    tv_other: *const timeval,
    tz_other: *const timezone,
) {
    let d1: c_double;
    let d2: c_double;

    if !tz_other.is_null()
        && ((*tz_sys).tz_minuteswest != (*tz_other).tz_minuteswest
            || (*tz_sys).tz_dsttime != (*tz_other).tz_dsttime)
    {
        ksft_print_msg(c"%s tz mismatch\n".as_ptr(), which);
    }

    d1 = tv_diff(tv_other, tv_sys1);
    d2 = tv_diff(tv_sys2, tv_other);

    ksft_print_msg(c"%s time offsets: %lf %lf\n".as_ptr(), which, d1, d2);

    ksft_test_result(!(d1 < 0.0 || d2 < 0.0), c"%s gettimeofday()'s timeval\n".as_ptr(), which);
}

unsafe fn test_gtod() {
    let mut tv_sys1: timeval = core::mem::zeroed();
    let mut tv_sys2: timeval = core::mem::zeroed();
    let mut tv_vdso: timeval = core::mem::zeroed();
    let mut tv_vsys: timeval = core::mem::zeroed();
    let mut tz_sys: timezone = core::mem::zeroed();
    let mut tz_vdso: timezone = core::mem::zeroed();
    let mut tz_vsys: timezone = core::mem::zeroed();
    let mut ret_vdso: c_long = -1;
    let mut ret_vsys: c_long = -1;

    ksft_print_msg(c"test gettimeofday()\n".as_ptr());

    if sys_gtod(&mut tv_sys1, &mut tz_sys) != 0 {
        ksft_exit_fail_msg(c"syscall gettimeofday: %s\n".as_ptr(), strerror(errno()));
    }
    if let Some(f) = vdso_gtod {
        ret_vdso = f(&mut tv_vdso, &mut tz_vdso);
    }
    if vsyscall_map_x {
        ret_vsys = vgtod(&mut tv_vsys, &mut tz_vsys);
    }
    if sys_gtod(&mut tv_sys2, &mut tz_sys) != 0 {
        ksft_exit_fail_msg(c"syscall gettimeofday: %s\n".as_ptr(), strerror(errno()));
    }

    if vdso_gtod.is_some() {
        if ret_vdso == 0 {
            check_gtod(&tv_sys1, &tv_sys2, &tz_sys, c"vDSO".as_ptr(), &tv_vdso, &tz_vdso);
        } else {
            ksft_test_result_fail(c"vDSO gettimeofday() failed: %ld\n".as_ptr(), ret_vdso);
        }
    } else {
        ksft_test_result_skip(c"vdso_gtod isn't set\n".as_ptr());
    }

    if vsyscall_map_x {
        if ret_vsys == 0 {
            check_gtod(&tv_sys1, &tv_sys2, &tz_sys, c"vsyscall".as_ptr(), &tv_vsys, &tz_vsys);
        } else {
            ksft_test_result_fail(c"vsys gettimeofday() failed: %ld\n".as_ptr(), ret_vsys);
        }
    } else {
        ksft_test_result_skip(c"vsyscall_map_x isn't set\n".as_ptr());
    }
}

unsafe fn test_time() {
    let t_sys1: c_long;
    let t_sys2: c_long;
    let mut t_vdso: c_long = 0;
    let mut t_vsys: c_long = 0;
    let mut t2_sys1: c_long = -1;
    let mut t2_sys2: c_long = -1;
    let mut t2_vdso: c_long = -1;
    let mut t2_vsys: c_long = -1;

    ksft_print_msg(c"test time()\n".as_ptr());
    t_sys1 = sys_time(&mut t2_sys1);
    if let Some(f) = vdso_time {
        t_vdso = f(&mut t2_vdso);
    }
    if vsyscall_map_x {
        t_vsys = vtime(&mut t2_vsys);
    }
    t_sys2 = sys_time(&mut t2_sys2);
    if t_sys1 < 0 || t_sys1 != t2_sys1 || t_sys2 < 0 || t_sys2 != t2_sys2 {
        ksft_print_msg(
            c"syscall failed (ret1:%ld output1:%ld ret2:%ld output2:%ld)\n".as_ptr(),
            t_sys1,
            t2_sys1,
            t_sys2,
            t2_sys2,
        );
        ksft_test_result_skip(c"vdso_time\n".as_ptr());
        ksft_test_result_skip(c"vdso_time\n".as_ptr());
        return;
    }

    if vdso_time.is_some() {
        if t_vdso < 0 || t_vdso != t2_vdso {
            ksft_test_result_fail(c"vDSO failed (ret:%ld output:%ld)\n".as_ptr(), t_vdso, t2_vdso);
        } else if t_vdso < t_sys1 || t_vdso > t_sys2 {
            ksft_test_result_fail(
                c"vDSO returned the wrong time (%ld %ld %ld)\n".as_ptr(),
                t_sys1,
                t_vdso,
                t_sys2,
            );
        } else {
            ksft_test_result_pass(c"vDSO time() is okay\n".as_ptr());
        }
    } else {
        ksft_test_result_skip(c"vdso_time isn't set\n".as_ptr());
    }

    if vsyscall_map_x {
        if t_vsys < 0 || t_vsys != t2_vsys {
            ksft_test_result_fail(c"vsyscall failed (ret:%ld output:%ld)\n".as_ptr(), t_vsys, t2_vsys);
        } else if t_vsys < t_sys1 || t_vsys > t_sys2 {
            ksft_test_result_fail(
                c"vsyscall returned the wrong time (%ld %ld %ld)\n".as_ptr(),
                t_sys1,
                t_vsys,
                t_sys2,
            );
        } else {
            ksft_test_result_pass(c"vsyscall time() is okay\n".as_ptr());
        }
    } else {
        ksft_test_result_skip(c"vsyscall_map_x isn't set\n".as_ptr());
    }
}

unsafe fn test_getcpu(cpu: c_int) {
    let mut cpu_sys: c_uint = 0;
    let mut cpu_vdso: c_uint = 0;
    let mut cpu_vsys: c_uint = 0;
    let mut node_sys: c_uint = 0;
    let mut node_vdso: c_uint = 0;
    let mut node_vsys: c_uint = 0;
    let ret_sys: c_long;
    let mut ret_vdso: c_long = -1;
    let mut ret_vsys: c_long = -1;
    let mut node: c_uint = 0;
    let mut have_node: bool = false;
    let mut cpuset: cpu_set_t = core::mem::zeroed();

    ksft_print_msg(c"getcpu() on CPU %d\n".as_ptr(), cpu);

    CPU_ZERO(&mut cpuset);
    CPU_SET(cpu, &mut cpuset);
    if sched_setaffinity(0, core::mem::size_of_val(&cpuset), &cpuset) != 0 {
        ksft_print_msg(c"failed to force CPU %d\n".as_ptr(), cpu);
        ksft_test_result_skip(c"vdso_getcpu\n".as_ptr());
        ksft_test_result_skip(c"vsyscall_map_x\n".as_ptr());

        return;
    }

    ret_sys = sys_getcpu(&mut cpu_sys, &mut node_sys, core::ptr::null_mut());
    if let Some(f) = vdso_getcpu {
        ret_vdso = f(&mut cpu_vdso, &mut node_vdso, core::ptr::null_mut());
    }
    if vsyscall_map_x {
        ret_vsys = vgetcpu(&mut cpu_vsys, &mut node_vsys, core::ptr::null_mut());
    }

    if ret_sys == 0 {
        if cpu_sys != cpu as c_uint {
            ksft_print_msg(c"syscall reported CPU %u but should be %d\n".as_ptr(), cpu_sys, cpu);
        }

        have_node = true;
        node = node_sys;
    }

    if vdso_getcpu.is_some() {
        if ret_vdso != 0 {
            ksft_test_result_fail(c"vDSO getcpu() failed\n".as_ptr());
        } else {
            if !have_node {
                have_node = true;
                node = node_vdso;
            }

            if cpu_vdso != cpu as c_uint || node_vdso != node {
                if cpu_vdso != cpu as c_uint {
                    ksft_print_msg(c"vDSO reported CPU %u but should be %d\n".as_ptr(), cpu_vdso, cpu);
                }
                if node_vdso != node {
                    ksft_print_msg(c"vDSO reported node %u but should be %u\n".as_ptr(), node_vdso, node);
                }
                ksft_test_result_fail(c"Wrong values\n".as_ptr());
            } else {
                ksft_test_result_pass(c"vDSO reported correct CPU and node\n".as_ptr());
            }
        }
    } else {
        ksft_test_result_skip(c"vdso_getcpu isn't set\n".as_ptr());
    }

    if vsyscall_map_x {
        if ret_vsys != 0 {
            ksft_test_result_fail(c"vsyscall getcpu() failed\n".as_ptr());
        } else {
            if !have_node {
                have_node = true;
                node = node_vsys;
            }

            if cpu_vsys != cpu as c_uint || node_vsys != node {
                if cpu_vsys != cpu as c_uint {
                    ksft_print_msg(c"vsyscall reported CPU %u but should be %d\n".as_ptr(), cpu_vsys, cpu);
                }
                if node_vsys != node {
                    ksft_print_msg(c"vsyscall reported node %u but should be %u\n".as_ptr(), node_vsys, node);
                }
                ksft_test_result_fail(c"Wrong values\n".as_ptr());
            } else {
                ksft_test_result_pass(c"vsyscall reported correct CPU and node\n".as_ptr());
            }
        }
    } else {
        ksft_test_result_skip(c"vsyscall_map_x isn't set\n".as_ptr());
    }
}

#[cfg(target_arch = "x86_64")]
type jmp_buf = [c_long; 8];

#[cfg(target_arch = "x86_64")]
static mut jmpbuf: jmp_buf = [0; 8];
#[cfg(target_arch = "x86_64")]
static mut segv_err: c_ulong = 0;
#[cfg(target_arch = "x86_64")]
static mut segv_trapno: c_ulong = 0;

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn sigsetjmp(env: *mut jmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut jmp_buf, val: c_int) -> !;
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn sigsegv(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void as *mut ucontext_t;

    segv_trapno = (*ctx).uc_mcontext.gregs[REG_TRAPNO] as c_ulong;
    segv_err = (*ctx).uc_mcontext.gregs[REG_ERR] as c_ulong;
    siglongjmp(&mut jmpbuf, 1);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_vsys_r() {
    ksft_print_msg(c"Checking read access to the vsyscall page\n".as_ptr());
    let can_read: bool;
    if sigsetjmp(&mut jmpbuf, 1) == 0 {
        core::ptr::read_volatile(0xffffffffff600000usize as *const c_int);
        can_read = true;
    } else {
        can_read = false;
    }

    if can_read && !vsyscall_map_r {
        ksft_test_result_fail(c"We have read access, but we shouldn't\n".as_ptr());
    } else if !can_read && vsyscall_map_r {
        ksft_test_result_fail(c"We don't have read access, but we should\n".as_ptr());
    } else if can_read {
        ksft_test_result_pass(c"We have read access\n".as_ptr());
    } else {
        ksft_test_result_pass(
            c"We do not have read access (trap=%ld, error=0x%lx)\n".as_ptr(),
            segv_trapno,
            segv_err,
        );
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_vsys_x() {
    if vsyscall_map_x {
        /* We already tested this adequately. */
        ksft_test_result_pass(c"vsyscall_map_x is true\n".as_ptr());
        return;
    }

    ksft_print_msg(c"Make sure that vsyscalls really cause a fault\n".as_ptr());

    let can_exec: bool;
    if sigsetjmp(&mut jmpbuf, 1) == 0 {
        vgtod(core::ptr::null_mut(), core::ptr::null_mut());
        can_exec = true;
    } else {
        can_exec = false;
    }

    if can_exec {
        ksft_test_result_fail(c"Executing the vsyscall did not fault\n".as_ptr());
    /* #GP or #PF (with X86_PF_INSTR) */
    } else if (segv_trapno == 13) || ((segv_trapno == 14) && (segv_err & (1 << 4)) != 0) {
        ksft_test_result_pass(
            c"Executing the vsyscall page failed (trap=%ld, error=0x%lx)\n".as_ptr(),
            segv_trapno,
            segv_err,
        );
    } else {
        ksft_test_result_fail(
            c"Execution failed with the wrong error (trap=%ld, error=0x%lx)\n".as_ptr(),
            segv_trapno,
            segv_err,
        );
    }
}

/*
 * Debuggers expect ptrace() to be able to peek at the vsyscall page.
 * Use process_vm_readv() as a proxy for ptrace() to test this.  We
 * want it to work in the vsyscall=emulate case and to fail in the
 * vsyscall=xonly case.
 *
 * It's worth noting that this ABI is a bit nutty.  write(2) can't
 * read from the vsyscall page on any kernel version or mode.  The
 * fact that ptrace() ever worked was a nice courtesy of old kernels,
 * but the code to support it is fairly gross.
 */
#[cfg(target_arch = "x86_64")]
unsafe fn test_process_vm_readv() {
    let mut buf = [0 as c_char; 4096];
    let mut local: iovec = core::mem::zeroed();
    let mut remote: iovec = core::mem::zeroed();
    let ret: c_int;

    ksft_print_msg(c"process_vm_readv() from vsyscall page\n".as_ptr());

    local.iov_base = buf.as_mut_ptr() as *mut c_void;
    local.iov_len = 4096;
    remote.iov_base = 0xffffffffff600000usize as *mut c_void;
    remote.iov_len = 4096;
    ret = process_vm_readv(getpid(), &local, 1, &remote, 1, 0) as c_int;
    if ret != 4096 {
        /*
         * We expect process_vm_readv() to work if and only if the
         * vsyscall page is readable.
         */
        ksft_test_result(
            !vsyscall_map_r,
            c"process_vm_readv() failed (ret = %d, errno = %d)\n".as_ptr(),
            ret,
            errno(),
        );
        return;
    }

    if vsyscall_map_r {
        ksft_test_result(
            memcmp(buf.as_ptr() as *const c_void, remote.iov_base, core::mem::size_of_val(&buf)) == 0,
            c"Read data\n".as_ptr(),
        );
    } else {
        ksft_test_result_fail(
            c"process_rm_readv() succeeded, but it should have failed in this configuration\n".as_ptr(),
        );
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn init_vsys() {
    let mut nerrs: c_int = 0;
    let maps: *mut FILE;
    let mut line = [0 as c_char; MAPS_LINE_LEN];
    let mut found: bool = false;

    maps = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if maps.is_null() {
        ksft_test_result_skip(c"Could not open /proc/self/maps -- assuming vsyscall is r-x\n".as_ptr());
        vsyscall_map_r = true;
        return;
    }

    while !fgets(line.as_mut_ptr(), MAPS_LINE_LEN as c_int, maps).is_null() {
        let mut r: c_char = 0;
        let mut x: c_char = 0;
        let mut start: *mut c_void = core::ptr::null_mut();
        let mut end: *mut c_void = core::ptr::null_mut();
        let mut name = [0 as c_char; MAPS_LINE_LEN];

        /* sscanf() is safe here as strlen(name) >= strlen(line) */
        if sscanf(
            line.as_ptr(),
            c"%p-%p %c-%cp %*x %*x:%*x %*u %s".as_ptr(),
            &mut start,
            &mut end,
            &mut r,
            &mut x,
            name.as_mut_ptr(),
        ) != 5
        {
            continue;
        }

        if strcmp(name.as_ptr(), c"[vsyscall]".as_ptr()) != 0 {
            continue;
        }

        ksft_print_msg(c"vsyscall map: %s".as_ptr(), line.as_ptr());

        if start != 0xffffffffff600000usize as *mut c_void
            || end != 0xffffffffff601000usize as *mut c_void
        {
            ksft_print_msg(c"address range is nonsense\n".as_ptr());
            nerrs += 1;
        }

        ksft_print_msg(c"vsyscall permissions are %c-%c\n".as_ptr(), r as c_int, x as c_int);
        vsyscall_map_r = r == b'r' as c_char;
        vsyscall_map_x = x == b'x' as c_char;

        found = true;
        break;
    }

    fclose(maps);

    if !found {
        ksft_print_msg(c"no vsyscall map in /proc/self/maps\n".as_ptr());
        vsyscall_map_r = false;
        vsyscall_map_x = false;
    }

    ksft_test_result(nerrs == 0, c"vsyscall map\n".as_ptr());
}

#[cfg(target_arch = "x86_64")]
static mut num_vsyscall_traps: sig_atomic_t = 0;

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn sigtrap(_sig: c_int, _info: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void as *mut ucontext_t;
    let ip: c_ulong = (*ctx).uc_mcontext.gregs[REG_RIP] as c_ulong;

    if ((ip ^ 0xffffffffff600000u64 as c_ulong) & !(0xfffu64 as c_ulong)) == 0 {
        num_vsyscall_traps += 1;
    }
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_emulation() {
    let mut tmp: time_t = 0;
    let is_native: bool;

    if !vsyscall_map_x {
        ksft_test_result_skip(c"vsyscall_map_x isn't set\n".as_ptr());
        return;
    }

    ksft_print_msg(c"checking that vsyscalls are emulated\n".as_ptr());
    sethandler(SIGTRAP, sigtrap, 0);
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    vtime(&mut tmp);
    set_eflags(get_eflags() & !X86_EFLAGS_TF);

    /*
     * If vsyscalls are emulated, we expect a single trap in the
     * vsyscall page -- the call instruction will trap with RIP
     * pointing to the entry point before emulation takes over.
     * In native mode, we expect two traps, since whatever code
     * the vsyscall page contains will be more than just a ret
     * instruction.
     */
    is_native = num_vsyscall_traps > 1;

    ksft_test_result(
        !is_native,
        c"vsyscalls are %s (%d instructions in vsyscall page)\n".as_ptr(),
        if is_native { c"native".as_ptr() } else { c"emulated".as_ptr() },
        num_vsyscall_traps as c_int,
    );
}

unsafe fn real_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let total_tests: c_int = TOTAL_TESTS;

    ksft_print_header();
    ksft_set_plan(total_tests);

    init_vdso();
    #[cfg(target_arch = "x86_64")]
    init_vsys();

    test_gtod();
    test_time();
    test_getcpu(0);
    test_getcpu(1);

    #[cfg(target_arch = "x86_64")]
    {
        sethandler(SIGSEGV, sigsegv, 0);
        test_vsys_r();
        test_vsys_x();
        test_process_vm_readv();
        test_emulation();
    }

    ksft_finished();
}

fn main() {
    let argv: *mut *mut c_char = core::ptr::null_mut();
    unsafe {
        real_main(0, argv);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
