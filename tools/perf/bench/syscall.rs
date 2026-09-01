/*
 *
 * syscall.c
 *
 * syscall: Benchmark for system call performance
 */

use core::ffi::{c_char, c_double, c_int, c_long, c_void};

const NULL: *mut c_void = core::ptr::null_mut();

// Includes in the original C file provide perf, option parsing, bench format,
// libc syscall numbers and POSIX routines.
#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

type pid_t = c_int;

const BENCH_FORMAT_DEFAULT: c_int = 0;
const BENCH_FORMAT_SIMPLE: c_int = 1;

// Fallback from the C preprocessor:
// #ifndef __NR_fork
// #define __NR_fork -1
// #endif
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
const __NR_fork: c_int = -1;

#[cfg(target_arch = "x86_64")]
const __NR_getppid: c_int = 110;
#[cfg(target_arch = "x86_64")]
const __NR_getpgid: c_int = 121;
#[cfg(target_arch = "x86_64")]
const __NR_fork: c_int = 57;
#[cfg(target_arch = "x86_64")]
const __NR_execve: c_int = 59;

#[cfg(target_arch = "x86")]
const __NR_getppid: c_int = 64;
#[cfg(target_arch = "x86")]
const __NR_getpgid: c_int = 132;
#[cfg(target_arch = "x86")]
const __NR_fork: c_int = 2;
#[cfg(target_arch = "x86")]
const __NR_execve: c_int = 11;

#[cfg(target_arch = "aarch64")]
const __NR_getppid: c_int = 173;
#[cfg(target_arch = "aarch64")]
const __NR_getpgid: c_int = 155;
#[cfg(target_arch = "aarch64")]
const __NR_fork: c_int = -1;
#[cfg(target_arch = "aarch64")]
const __NR_execve: c_int = 221;

extern "C" {
    static mut stderr: *mut c_void;
    static mut bench_format: c_int;

    static options: [option; 0];
    static bench_syscall_usage: [*const c_char; 0];

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;

    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn getppid() -> pid_t;
    fn getpgid(pid: pid_t) -> pid_t;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

static mut loops: c_int = 0;

// static const struct option options[] = {
//      OPT_INTEGER('l', "loop", &loops, "Specify number of loops"),
//      OPT_END()
// };

// static const char * const bench_syscall_usage[] = {
//      "perf bench syscall <options>",
//      NULL
// };

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1000000;
    }
}

unsafe fn test_fork() {
    let pid: pid_t = fork();

    if pid < 0 {
        fprintf(stderr, b"fork failed\n\0".as_ptr() as *const c_char);
        exit(1);
    } else if pid == 0 {
        exit(0);
    } else {
        if waitpid(pid, core::ptr::null_mut(), 0) < 0 {
            fprintf(stderr, b"waitpid failed\n\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}

unsafe fn test_execve() {
    let pathname: *const c_char = b"/bin/true\0".as_ptr() as *const c_char;
    let mut argv: [*mut c_char; 2] = [pathname as *mut c_char, core::ptr::null_mut()];
    let pid: pid_t = fork();

    if pid < 0 {
        fprintf(stderr, b"fork failed\n\0".as_ptr() as *const c_char);
        exit(1);
    } else if pid == 0 {
        execve(pathname, argv.as_mut_ptr(), core::ptr::null());
        fprintf(
            stderr,
            b"execve /bin/true failed\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    } else {
        if waitpid(pid, core::ptr::null_mut(), 0) < 0 {
            fprintf(stderr, b"waitpid failed\n\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}

unsafe fn bench_syscall_common(
    mut argc: c_int,
    argv: *const *const c_char,
    syscall: c_int,
) -> c_int {
    let mut start: timeval = core::mem::zeroed();
    let mut stop: timeval = core::mem::zeroed();
    let mut diff: timeval = core::mem::zeroed();
    let mut result_usec: u64 = 0;
    let mut name: *const c_char = core::ptr::null();
    let mut i: c_int;

    match syscall {
        __NR_fork | __NR_execve => {
            /* Limit default loop to 10000 times to save time */
            loops = 10000;
        }
        _ => {
            loops = 10000000;
        }
    }

    /* Options -l and --loops override default above */
    argc = parse_options(argc, argv, options.as_ptr(), bench_syscall_usage.as_ptr(), 0);
    let _ = argc;

    gettimeofday(&mut start, NULL);

    i = 0;
    while i < loops {
        match syscall {
            __NR_getppid => {
                getppid();
            }
            __NR_getpgid => {
                getpgid(0);
            }
            __NR_fork => {
                test_fork();
            }
            __NR_execve => {
                test_execve();
            }
            _ => {}
        }
        i += 1;
    }

    gettimeofday(&mut stop, NULL);
    timersub(&stop, &start, &mut diff);

    match syscall {
        __NR_getppid => {
            name = b"getppid()\0".as_ptr() as *const c_char;
        }
        __NR_getpgid => {
            name = b"getpgid()\0".as_ptr() as *const c_char;
        }
        __NR_fork => {
            name = b"fork()\0".as_ptr() as *const c_char;
        }
        __NR_execve => {
            name = b"execve()\0".as_ptr() as *const c_char;
        }
        _ => {}
    }

    match bench_format {
        BENCH_FORMAT_DEFAULT => {
            printf(
                b"# Executed %'d %s calls\n\0".as_ptr() as *const c_char,
                loops,
                name,
            );

            result_usec = (diff.tv_sec as u64).wrapping_mul(1000000);
            result_usec = result_usec.wrapping_add(diff.tv_usec as u64);

            printf(
                b" %14s: %lu.%03lu [sec]\n\n\0".as_ptr() as *const c_char,
                b"Total time\0".as_ptr() as *const c_char,
                diff.tv_sec as c_long,
                (diff.tv_usec / 1000) as c_long,
            );

            printf(
                b" %14lf usecs/op\n\0".as_ptr() as *const c_char,
                (result_usec as c_double) / (loops as c_double),
            );
            printf(
                b" %'14d ops/sec\n\0".as_ptr() as *const c_char,
                ((loops as c_double) / ((result_usec as c_double) / 1000000.0)) as c_int,
            );
        }

        BENCH_FORMAT_SIMPLE => {
            printf(
                b"%lu.%03lu\n\0".as_ptr() as *const c_char,
                diff.tv_sec as c_long,
                (diff.tv_usec / 1000) as c_long,
            );
        }

        _ => {
            /* reaching here is something disaster */
            fprintf(
                stderr,
                b"Unknown format:%d\n\0".as_ptr() as *const c_char,
                bench_format,
            );
            exit(1);
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn bench_syscall_basic(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_syscall_common(argc, argv, __NR_getppid)
}

#[no_mangle]
pub unsafe extern "C" fn bench_syscall_getpgid(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_syscall_common(argc, argv, __NR_getpgid)
}

#[no_mangle]
pub unsafe extern "C" fn bench_syscall_fork(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_syscall_common(argc, argv, __NR_fork)
}

#[no_mangle]
pub unsafe extern "C" fn bench_syscall_execve(argc: c_int, argv: *const *const c_char) -> c_int {
    bench_syscall_common(argc, argv, __NR_execve)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
