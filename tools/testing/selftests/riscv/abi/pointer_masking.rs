// SPDX-License-Identifier: GPL-2.0-only

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type size_t = usize;
type ssize_t = isize;
type off_t = c_long;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

const PR_SET_TAGGED_ADDR_CTRL: c_int = 55;
const PR_GET_TAGGED_ADDR_CTRL: c_int = 56;

const PR_PMLEN_SHIFT: c_int = 24;
const PR_PMLEN_MASK: c_ulong = 0x7f_u64 << PR_PMLEN_SHIFT;

const ENODATA: c_int = 61;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;

const O_RDWR: c_int = 0o2;
const O_WRONLY: c_int = 0o1;

const SIGSEGV: c_int = 11;
const SIG_DFL: sighandler_t = None;

#[cfg(target_pointer_width = "64")]
const __riscv_xlen: c_int = 64;
#[cfg(target_pointer_width = "32")]
const __riscv_xlen: c_int = 32;

type c_ulong = u64;

#[repr(C)]
struct sigjmp_buf {
    __private: [c_long; 32],
}

#[repr(C)]
struct test_info {
    nr_tests: c_uint,
    test_fn: unsafe fn(),
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;

    fn prctl(option: c_int, ...) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> !;
    fn exit(status: c_int) -> !;
    fn fork() -> c_int;
    fn wait(wstatus: *mut c_int) -> c_int;
    fn execve(pathname: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result(condition: bool, fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_error(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_perror(msg: *const c_char);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_finished() -> !;
}

static mut dev_zero: c_int = 0;

static mut pipefd: [c_int; 2] = [0; 2];

static mut jmpbuf: sigjmp_buf = sigjmp_buf { __private: [0; 32] };

unsafe extern "C" fn sigsegv_handler(_sig: c_int) {
    unsafe {
        siglongjmp(core::ptr::addr_of_mut!(jmpbuf), 1);
    }
}

static mut min_pmlen: c_int = 0;
static mut max_pmlen: c_int = 0;

#[inline]
fn valid_pmlen(pmlen: c_int) -> bool {
    pmlen == 0 || pmlen == 7 || pmlen == 16
}

unsafe fn test_pmlen() {
    unsafe {
        ksft_print_msg(c"Testing available PMLEN values\n".as_ptr());

        for request in 0..=16 {
            let pmlen: c_int;
            let mut ret: c_int;

            ret = prctl(
                PR_SET_TAGGED_ADDR_CTRL,
                request << PR_PMLEN_SHIFT,
                0,
                0,
                0,
            );
            if ret != 0 {
                ksft_test_result_skip(
                    c"PMLEN=%d PR_GET_TAGGED_ADDR_CTRL\n".as_ptr(),
                    request,
                );
                ksft_test_result_skip(c"PMLEN=%d constraint\n".as_ptr(), request);
                ksft_test_result_skip(c"PMLEN=%d validity\n".as_ptr(), request);
                continue;
            }

            ret = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0);
            ksft_test_result(
                ret >= 0,
                c"PMLEN=%d PR_GET_TAGGED_ADDR_CTRL\n".as_ptr(),
                request,
            );
            if ret < 0 {
                ksft_test_result_skip(c"PMLEN=%d constraint\n".as_ptr(), request);
                ksft_test_result_skip(c"PMLEN=%d validity\n".as_ptr(), request);
                continue;
            }

            pmlen = ((ret as c_ulong & PR_PMLEN_MASK) >> PR_PMLEN_SHIFT) as c_int;
            ksft_test_result(pmlen >= request, c"PMLEN=%d constraint\n".as_ptr(), request);
            ksft_test_result(valid_pmlen(pmlen), c"PMLEN=%d validity\n".as_ptr(), request);

            if min_pmlen == 0 {
                min_pmlen = pmlen;
            }
            if max_pmlen < pmlen {
                max_pmlen = pmlen;
            }
        }

        if max_pmlen == 0 {
            ksft_exit_fail_msg(c"Failed to enable pointer masking\n".as_ptr());
        }
    }
}

unsafe fn set_tagged_addr_ctrl(pmlen: c_int, tagged_addr_abi: bool) -> c_int {
    unsafe {
        let arg: c_int = (pmlen << PR_PMLEN_SHIFT) | tagged_addr_abi as c_int;
        let mut ret: c_int = prctl(PR_SET_TAGGED_ADDR_CTRL, arg, 0, 0, 0);
        if ret == 0 {
            ret = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0);
            if ret == arg {
                return 0;
            }
        }

        if ret < 0 {
            -*__errno_location()
        } else {
            -ENODATA
        }
    }
}

unsafe fn test_dereference_pmlen(pmlen: c_int) {
    static mut i: c_int = 0;
    let mut p: *mut c_int;
    let ret: c_int;

    unsafe {
        ret = set_tagged_addr_ctrl(pmlen, false);
        if ret != 0 {
            ksft_test_result_error(c"PMLEN=%d setup (%d)\n".as_ptr(), pmlen, ret);
            return;
        }

        core::ptr::write_volatile(core::ptr::addr_of_mut!(i), pmlen);

        if pmlen != 0 {
            p = ((core::ptr::addr_of_mut!(i) as usize) | (1usize << (__riscv_xlen - pmlen)))
                as *mut c_int;

            /* These dereferences should succeed. */
            if sigsetjmp(core::ptr::addr_of_mut!(jmpbuf), 1) != 0 {
                ksft_test_result_fail(c"PMLEN=%d valid tag\n".as_ptr(), pmlen);
                return;
            }
            if core::ptr::read_volatile(p) != pmlen {
                ksft_test_result_fail(c"PMLEN=%d bad value\n".as_ptr(), pmlen);
                return;
            }
            core::ptr::write_volatile(p, core::ptr::read_volatile(p).wrapping_add(1));
        }

        p = ((core::ptr::addr_of_mut!(i) as usize) | (1usize << (__riscv_xlen - pmlen - 1)))
            as *mut c_int;

        /* These dereferences should raise SIGSEGV. */
        if sigsetjmp(core::ptr::addr_of_mut!(jmpbuf), 1) != 0 {
            ksft_test_result_pass(c"PMLEN=%d dereference\n".as_ptr(), pmlen);
            return;
        }
        core::ptr::write_volatile(p, core::ptr::read_volatile(p).wrapping_add(1));
        ksft_test_result_fail(c"PMLEN=%d invalid tag\n".as_ptr(), pmlen);
    }
}

unsafe fn test_dereference() {
    unsafe {
        ksft_print_msg(c"Testing userspace pointer dereference\n".as_ptr());

        signal(SIGSEGV, Some(sigsegv_handler));

        test_dereference_pmlen(0);
        test_dereference_pmlen(min_pmlen);
        test_dereference_pmlen(max_pmlen);

        signal(SIGSEGV, SIG_DFL);
    }
}

unsafe extern "C" fn execve_child_sigsegv_handler(_sig: c_int) {
    unsafe {
        exit(42);
    }
}

unsafe fn execve_child() -> c_int {
    static mut i: c_int = 0;
    let p: *mut c_int =
        ((core::ptr::addr_of_mut!(i) as usize) | (1usize << (__riscv_xlen - 7))) as *mut c_int;

    unsafe {
        signal(SIGSEGV, Some(execve_child_sigsegv_handler));

        /* This dereference should raise SIGSEGV. */
        core::ptr::read_volatile(p)
    }
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn test_fork_exec() {
    let mut ret: c_int;
    let mut status: c_int = 0;

    unsafe {
        ksft_print_msg(c"Testing fork/exec behavior\n".as_ptr());

        ret = set_tagged_addr_ctrl(min_pmlen, false);
        if ret != 0 {
            ksft_test_result_error(c"setup (%d)\n".as_ptr(), ret);
            return;
        }

        if fork() != 0 {
            wait(&mut status);
            ksft_test_result(
                wifexited(status) && wexitstatus(status) == 42,
                c"dereference after fork\n".as_ptr(),
            );
        } else {
            static mut i: c_int = 42;
            let p: *mut c_int;

            p = ((core::ptr::addr_of_mut!(i) as usize)
                | (1usize << (__riscv_xlen - min_pmlen))) as *mut c_int;

            /* This dereference should succeed. */
            exit(core::ptr::read_volatile(p));
        }

        if fork() != 0 {
            wait(&mut status);
            ksft_test_result(
                wifexited(status) && wexitstatus(status) == 42,
                c"dereference after fork+exec\n".as_ptr(),
            );
        } else {
            /* Will call execve_child(). */
            let mut argv0: [c_char; 1] = [0];
            let argv: [*mut c_char; 2] = [argv0.as_mut_ptr(), core::ptr::null_mut()];
            execve(c"/proc/self/exe".as_ptr(), argv.as_ptr(), core::ptr::null());
        }
    }
}

unsafe fn pwrite_wrapper(fd: c_int, buf: *mut c_void, count: size_t, msg: *const c_char) -> bool {
    unsafe {
        let ret: c_int = pwrite(fd, buf, count, 0) as c_int;

        if ret != count as c_int {
            ksft_perror(msg);
            return false;
        }
        true
    }
}

unsafe fn test_tagged_addr_abi_sysctl() {
    let err_pwrite_msg: *const c_char =
        c"failed to write to /proc/sys/abi/tagged_addr_disabled\n".as_ptr();
    let mut value: c_char;
    let fd: c_int;

    unsafe {
        ksft_print_msg(c"Testing tagged address ABI sysctl\n".as_ptr());

        fd = open(c"/proc/sys/abi/tagged_addr_disabled".as_ptr(), O_WRONLY);
        if fd < 0 {
            ksft_test_result_skip(c"failed to open sysctl file\n".as_ptr());
            ksft_test_result_skip(c"failed to open sysctl file\n".as_ptr());
            return;
        }

        value = b'1' as c_char;
        if !pwrite_wrapper(
            fd,
            (&mut value as *mut c_char).cast::<c_void>(),
            1,
            c"write '1'".as_ptr(),
        ) {
            ksft_test_result_fail(err_pwrite_msg);
        } else {
            ksft_test_result(
                set_tagged_addr_ctrl(min_pmlen, true) == -EINVAL,
                c"sysctl disabled\n".as_ptr(),
            );
        }

        value = b'0' as c_char;
        if !pwrite_wrapper(
            fd,
            (&mut value as *mut c_char).cast::<c_void>(),
            1,
            c"write '0'".as_ptr(),
        ) {
            ksft_test_result_fail(err_pwrite_msg);
        } else {
            ksft_test_result(
                set_tagged_addr_ctrl(min_pmlen, true) == 0,
                c"sysctl enabled\n".as_ptr(),
            );
        }

        set_tagged_addr_ctrl(0, false);

        close(fd);
    }
}

unsafe fn test_tagged_addr_abi_pmlen(pmlen: c_int) {
    let mut i: c_int;
    let mut p: *mut c_int;
    let mut ret: c_int;

    unsafe {
        i = !pmlen;

        if pmlen != 0 {
            p = ((&mut i as *mut c_int as usize) | (1usize << (__riscv_xlen - pmlen)))
                as *mut c_int;

            ret = set_tagged_addr_ctrl(pmlen, false);
            if ret != 0 {
                ksft_test_result_error(
                    c"PMLEN=%d ABI disabled setup (%d)\n".as_ptr(),
                    pmlen,
                    ret,
                );
                return;
            }

            ret = write(pipefd[1], p.cast::<c_void>(), core::mem::size_of_val(&*p)) as c_int;
            if ret >= 0 || *__errno_location() != EFAULT {
                ksft_test_result_fail(c"PMLEN=%d ABI disabled write\n".as_ptr(), pmlen);
                return;
            }

            ret = read(dev_zero, p.cast::<c_void>(), core::mem::size_of_val(&*p)) as c_int;
            if ret >= 0 || *__errno_location() != EFAULT {
                ksft_test_result_fail(c"PMLEN=%d ABI disabled read\n".as_ptr(), pmlen);
                return;
            }

            if i != !pmlen {
                ksft_test_result_fail(c"PMLEN=%d ABI disabled value\n".as_ptr(), pmlen);
                return;
            }

            ret = set_tagged_addr_ctrl(pmlen, true);
            if ret != 0 {
                ksft_test_result_error(
                    c"PMLEN=%d ABI enabled setup (%d)\n".as_ptr(),
                    pmlen,
                    ret,
                );
                return;
            }

            ret = write(pipefd[1], p.cast::<c_void>(), core::mem::size_of_val(&*p)) as c_int;
            if ret != core::mem::size_of_val(&*p) as c_int {
                ksft_test_result_fail(c"PMLEN=%d ABI enabled write\n".as_ptr(), pmlen);
                return;
            }

            ret = read(dev_zero, p.cast::<c_void>(), core::mem::size_of_val(&*p)) as c_int;
            if ret != core::mem::size_of_val(&*p) as c_int {
                ksft_test_result_fail(c"PMLEN=%d ABI enabled read\n".as_ptr(), pmlen);
                return;
            }

            if i != 0 {
                ksft_test_result_fail(c"PMLEN=%d ABI enabled value\n".as_ptr(), pmlen);
                return;
            }

            i = !pmlen;
        } else {
            /* The tagged address ABI cannot be enabled when PMLEN == 0. */
            ret = set_tagged_addr_ctrl(pmlen, true);
            if ret != -EINVAL {
                ksft_test_result_error(c"PMLEN=%d ABI setup (%d)\n".as_ptr(), pmlen, ret);
                return;
            }
        }

        p = ((&mut i as *mut c_int as usize) | (1usize << (__riscv_xlen - pmlen - 1)))
            as *mut c_int;

        ret = write(pipefd[1], p.cast::<c_void>(), core::mem::size_of_val(&*p)) as c_int;
        if ret >= 0 || *__errno_location() != EFAULT {
            ksft_test_result_fail(
                c"PMLEN=%d invalid tag write (%d)\n".as_ptr(),
                pmlen,
                *__errno_location(),
            );
            return;
        }

        ret = read(dev_zero, p.cast::<c_void>(), core::mem::size_of_val(&*p)) as c_int;
        if ret >= 0 || *__errno_location() != EFAULT {
            ksft_test_result_fail(c"PMLEN=%d invalid tag read\n".as_ptr(), pmlen);
            return;
        }

        if i != !pmlen {
            ksft_test_result_fail(c"PMLEN=%d invalid tag value\n".as_ptr(), pmlen);
            return;
        }

        ksft_test_result_pass(c"PMLEN=%d tagged address ABI\n".as_ptr(), pmlen);
    }
}

unsafe fn test_tagged_addr_abi() {
    unsafe {
        ksft_print_msg(c"Testing tagged address ABI\n".as_ptr());

        test_tagged_addr_abi_pmlen(0);
        test_tagged_addr_abi_pmlen(min_pmlen);
        test_tagged_addr_abi_pmlen(max_pmlen);
    }
}

static tests: [test_info; 5] = [
    test_info {
        nr_tests: 17 * 3,
        test_fn: test_pmlen,
    },
    test_info {
        nr_tests: 3,
        test_fn: test_dereference,
    },
    test_info {
        nr_tests: 2,
        test_fn: test_fork_exec,
    },
    test_info {
        nr_tests: 2,
        test_fn: test_tagged_addr_abi_sysctl,
    },
    test_info {
        nr_tests: 3,
        test_fn: test_tagged_addr_abi,
    },
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut plan: c_uint = 0;
    let ret: c_int;

    unsafe {
        /* Check if this is the child process after execve(). */
        if *(*argv) == 0 {
            return execve_child();
        }

        dev_zero = open(c"/dev/zero".as_ptr(), O_RDWR);
        if dev_zero < 0 {
            return 1;
        }

        /* Write to a pipe so the kernel must dereference the buffer pointer. */
        ret = pipe(pipefd.as_mut_ptr());
        if ret != 0 {
            return 1;
        }

        ksft_print_header();

        for test in tests.iter() {
            plan += test.nr_tests;
        }

        ksft_set_plan(plan);

        for test in tests.iter() {
            (test.test_fn)();
        }

        ksft_finished();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
