// SPDX-License-Identifier: GPL-2.0

/* Based on Christian Brauner's clone3() example */

// C dependencies: errno.h, inttypes.h, linux/types.h, linux/sched.h,
// stdbool.h, stdint.h, stdio.h, stdlib.h, sys/syscall.h, sys/types.h,
// sys/un.h, sys/wait.h, unistd.h, sched.h, kselftest.h,
// clone3_selftests.h.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type u64_t = u64;
type __aligned_u64 = u64;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const E2BIG: c_int = 7;
const EINVAL: c_int = 22;
const F_OK: c_int = 0;
const SIGCHLD: c_int = 17;
const __WALL: c_int = 0x40000000;
const CLONE_NEWPID: u64_t = 0x20000000;
const CLONE_NEWTIME: u64_t = 0x00000080;
const CLONE_ARGS_SIZE_VER0: size_t = 64;

#[repr(C)]
#[derive(Copy, Clone)]
struct __clone_args {
    flags: __aligned_u64,
    pidfd: __aligned_u64,
    child_tid: __aligned_u64,
    parent_tid: __aligned_u64,
    exit_signal: __aligned_u64,
    stack: __aligned_u64,
    stack_size: __aligned_u64,
    tls: __aligned_u64,
    set_tid: __aligned_u64,
    set_tid_size: __aligned_u64,
    cgroup: __aligned_u64,
}

impl Default for __clone_args {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
enum test_mode {
    CLONE3_ARGS_NO_TEST,
    CLONE3_ARGS_ALL_0,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_BIG,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_NEG,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_CSIG,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_NSIG,
}

type filter_function = Option<unsafe extern "C" fn() -> bool>;
type size_function = Option<unsafe extern "C" fn() -> size_t>;

#[repr(C)]
struct test {
    name: *const c_char,
    flags: u64_t,
    size: size_t,
    size_function: size_function,
    expected: c_int,
    test_mode: test_mode,
    filter: filter_function,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn getpid() -> pid_t;
    fn getuid() -> u32;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn syscall(number: c_long, ...) -> c_long;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;

    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result(pass: bool, fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_finished() -> !;
    fn test_clone3_supported();
}

type c_uint = u32;

const SYS_clone3: c_long = 435;

unsafe fn sys_clone3(args: *mut __clone_args, size: size_t) -> pid_t {
    syscall(SYS_clone3, args, size) as pid_t
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn call_clone3(flags: u64_t, mut size: size_t, test_mode: test_mode) -> c_int {
    let mut args = __clone_args {
        flags,
        exit_signal: SIGCHLD as __aligned_u64,
        ..Default::default()
    };

    #[repr(C)]
    struct clone_args_extended {
        args: __clone_args,
        excess_space: [__aligned_u64; 2],
    }

    let mut args_ext: clone_args_extended = mem::zeroed();

    let mut pid: pid_t = -1;
    let mut status: c_int = 0;

    memset(
        &mut args_ext as *mut clone_args_extended as *mut c_void,
        0,
        mem::size_of::<clone_args_extended>(),
    );
    if size > mem::size_of::<__clone_args>() {
        args_ext.excess_space[1] = 1;
    }

    if size == 0 {
        size = mem::size_of::<__clone_args>();
    }

    match test_mode {
        test_mode::CLONE3_ARGS_NO_TEST => {
            /*
             * Uses default 'flags' and 'SIGCHLD'
             * assignment.
             */
        }
        test_mode::CLONE3_ARGS_ALL_0 => {
            args.flags = 0;
            args.exit_signal = 0;
        }
        test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_BIG => {
            args.exit_signal = 0xbadc0ded00000000_u64;
        }
        test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_NEG => {
            args.exit_signal = 0x0000000080000000_u64;
        }
        test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_CSIG => {
            args.exit_signal = 0x0000000000000100_u64;
        }
        test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_NSIG => {
            args.exit_signal = 0x00000000000000f0_u64;
        }
    }

    memcpy(
        &mut args_ext.args as *mut __clone_args as *mut c_void,
        &args as *const __clone_args as *const c_void,
        mem::size_of::<__clone_args>(),
    );

    pid = sys_clone3(&mut args_ext as *mut clone_args_extended as *mut __clone_args, size);
    if pid < 0 {
        ksft_print_msg(
            b"%s - Failed to create new process\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return -errno;
    }

    if pid == 0 {
        ksft_print_msg(
            b"I am the child, my PID is %d\n\0".as_ptr() as *const c_char,
            getpid(),
        );
        _exit(EXIT_SUCCESS);
    }

    ksft_print_msg(
        b"I am the parent (%d). My child's pid is %d\n\0".as_ptr() as *const c_char,
        getpid(),
        pid,
    );

    if waitpid(-1, &mut status as *mut c_int, __WALL) < 0 {
        ksft_print_msg(
            b"waitpid() returned %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return -errno;
    }
    if !WIFEXITED(status) {
        ksft_print_msg(
            b"Child did not exit normally, status 0x%x\n\0".as_ptr() as *const c_char,
            status,
        );
        return EXIT_FAILURE;
    }
    if WEXITSTATUS(status) != 0 {
        return WEXITSTATUS(status);
    }

    0
}

unsafe fn test_clone3(
    flags: u64_t,
    size: size_t,
    expected: c_int,
    test_mode: test_mode,
) -> bool {
    let ret: c_int;

    ksft_print_msg(
        b"[%d] Trying clone3() with flags %#lx (size %zu)\n\0".as_ptr() as *const c_char,
        getpid(),
        flags as c_ulong,
        size,
    );
    ret = call_clone3(flags, size, test_mode);
    ksft_print_msg(
        b"[%d] clone3() with flags says: %d expected %d\n\0".as_ptr() as *const c_char,
        getpid(),
        ret,
        expected,
    );
    if ret != expected {
        ksft_print_msg(
            b"[%d] Result (%d) is different than expected (%d)\n\0".as_ptr() as *const c_char,
            getpid(),
            ret,
            expected,
        );
        return false;
    }

    true
}

unsafe extern "C" fn not_root() -> bool {
    if getuid() != 0 {
        ksft_print_msg(b"Not running as root\n\0".as_ptr() as *const c_char);
        return true;
    }

    false
}

unsafe extern "C" fn no_timenamespace() -> bool {
    if not_root() {
        return true;
    }

    if access(
        b"/proc/self/ns/time\0".as_ptr() as *const c_char,
        F_OK,
    ) == 0
    {
        return false;
    }

    ksft_print_msg(b"Time namespaces are not supported\n\0".as_ptr() as *const c_char);
    true
}

unsafe extern "C" fn page_size_plus_8() -> size_t {
    (getpagesize() + 8) as size_t
}

static tests: [test; 20] = [
    test {
        name: b"simple clone3()\0".as_ptr() as *const c_char,
        flags: 0,
        size: 0,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"clone3() in a new PID_NS\0".as_ptr() as *const c_char,
        flags: CLONE_NEWPID,
        size: 0,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: Some(not_root),
    },
    test {
        name: b"CLONE_ARGS_SIZE_VER0\0".as_ptr() as *const c_char,
        flags: 0,
        size: CLONE_ARGS_SIZE_VER0,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"CLONE_ARGS_SIZE_VER0 - 8\0".as_ptr() as *const c_char,
        flags: 0,
        size: CLONE_ARGS_SIZE_VER0 - 8,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"sizeof(struct clone_args) + 8\0".as_ptr() as *const c_char,
        flags: 0,
        size: mem::size_of::<__clone_args>() + 8,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"exit_signal with highest 32 bits non-zero\0".as_ptr() as *const c_char,
        flags: 0,
        size: 0,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_BIG,
        filter: None,
    },
    test {
        name: b"negative 32-bit exit_signal\0".as_ptr() as *const c_char,
        flags: 0,
        size: 0,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_NEG,
        filter: None,
    },
    test {
        name: b"exit_signal not fitting into CSIGNAL mask\0".as_ptr() as *const c_char,
        flags: 0,
        size: 0,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_CSIG,
        filter: None,
    },
    test {
        name: b"NSIG < exit_signal < CSIG\0".as_ptr() as *const c_char,
        flags: 0,
        size: 0,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_INVAL_EXIT_SIGNAL_NSIG,
        filter: None,
    },
    test {
        name: b"Arguments sizeof(struct clone_args) + 8\0".as_ptr() as *const c_char,
        flags: 0,
        size: mem::size_of::<__clone_args>() + 8,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_ALL_0,
        filter: None,
    },
    test {
        name: b"Arguments sizeof(struct clone_args) + 16\0".as_ptr() as *const c_char,
        flags: 0,
        size: mem::size_of::<__clone_args>() + 16,
        size_function: None,
        expected: -E2BIG,
        test_mode: test_mode::CLONE3_ARGS_ALL_0,
        filter: None,
    },
    test {
        name: b"Arguments sizeof(struct clone_arg) * 2\0".as_ptr() as *const c_char,
        flags: 0,
        size: mem::size_of::<__clone_args>() + 16,
        size_function: None,
        expected: -E2BIG,
        test_mode: test_mode::CLONE3_ARGS_ALL_0,
        filter: None,
    },
    test {
        name: b"Arguments > page size\0".as_ptr() as *const c_char,
        flags: 0,
        size: 0,
        size_function: Some(page_size_plus_8),
        expected: -E2BIG,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"CLONE_ARGS_SIZE_VER0 in a new PID NS\0".as_ptr() as *const c_char,
        flags: CLONE_NEWPID,
        size: CLONE_ARGS_SIZE_VER0,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: Some(not_root),
    },
    test {
        name: b"CLONE_ARGS_SIZE_VER0 - 8 in a new PID NS\0".as_ptr() as *const c_char,
        flags: CLONE_NEWPID,
        size: CLONE_ARGS_SIZE_VER0 - 8,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"sizeof(struct clone_args) + 8 in a new PID NS\0".as_ptr() as *const c_char,
        flags: CLONE_NEWPID,
        size: mem::size_of::<__clone_args>() + 8,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: Some(not_root),
    },
    test {
        name: b"Arguments > page size in a new PID NS\0".as_ptr() as *const c_char,
        flags: CLONE_NEWPID,
        size: 0,
        size_function: Some(page_size_plus_8),
        expected: -E2BIG,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
    test {
        name: b"New time NS\0".as_ptr() as *const c_char,
        flags: CLONE_NEWTIME,
        size: 0,
        size_function: None,
        expected: 0,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: Some(no_timenamespace),
    },
    test {
        name: b"exit signal (SIGCHLD) in flags\0".as_ptr() as *const c_char,
        flags: SIGCHLD as u64_t,
        size: 0,
        size_function: None,
        expected: -EINVAL,
        test_mode: test_mode::CLONE3_ARGS_NO_TEST,
        filter: None,
    },
];

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut size: size_t;

    ksft_print_header();
    ksft_set_plan(ARRAY_SIZE(&tests) as c_uint);
    test_clone3_supported();

    let mut i: usize = 0;
    while i < ARRAY_SIZE(&tests) {
        if tests[i].filter.is_some() && tests[i].filter.unwrap()() {
            ksft_test_result_skip(b"%s\n\0".as_ptr() as *const c_char, tests[i].name);
            i += 1;
            continue;
        }

        if let Some(size_fn) = tests[i].size_function {
            size = size_fn();
        } else {
            size = tests[i].size;
        }

        ksft_print_msg(
            b"Running test '%s'\n\0".as_ptr() as *const c_char,
            tests[i].name,
        );

        ksft_test_result(
            test_clone3(
                tests[i].flags,
                size,
                tests[i].expected,
                tests[i].test_mode,
            ),
            b"%s\n\0".as_ptr() as *const c_char,
            tests[i].name,
        );

        i += 1;
    }

    ksft_finished();
}
