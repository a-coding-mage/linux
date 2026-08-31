// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 ARM Limited.
 */

/*
 * Translated from C. Header-provided constants, syscall numbers, helper
 * functions, and kselftest APIs are expected to be supplied by surrounding
 * bindings for the original includes:
 * <limits.h>, <stdbool.h>, <linux/prctl.h>, <sys/mman.h>, <asm/mman.h>,
 * <asm/hwcap.h>, <linux/sched.h>, "kselftest.h", and "gcs-util.h".
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type size_t = usize;
type pid_t = c_int;

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn fork() -> pid_t;
    fn vfork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn getauxval(type_: c_ulong) -> c_ulong;

    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result(result: bool, fmt: *const c_char, ...);
    fn ksft_finished();

    fn get_gcspr() -> *mut c_ulong;
    fn chkfeat_gcs() -> bool;

    static mut errno: c_int;
}

type c_uint = u32;
type uint64_t = u64;

const EXIT_FAILURE: c_int = 1;
const ULONG_MAX: c_ulong = c_ulong::MAX;
const MAP_FAILED: *mut uint64_t = !0usize as *mut uint64_t;

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/* nolibc doesn't have sysconf(), just hard code the maximum */
static mut page_size: size_t = 65536;

#[inline(never)]
unsafe fn valid_gcs_function() {
    /* Do something the compiler can't optimise out */
    unsafe {
        syscall(__NR_prctl, PR_SVE_GET_VL);
    }
}

#[inline]
unsafe fn gcs_set_status(mode: c_ulong) -> c_int {
    let enabling: bool = (mode & PR_SHADOW_STACK_ENABLE) != 0;
    let mut ret: c_int;
    let mut new_mode: c_ulong = 0;

    /*
     * The prctl takes 1 argument but we need to ensure that the
     * other 3 values passed in registers to the syscall are zero
     * since the kernel validates them.
     */
    ret = unsafe {
        syscall(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS,
            mode,
            0,
            0,
            0,
        ) as c_int
    };

    if ret == 0 {
        ret = unsafe {
            syscall(
                __NR_prctl,
                PR_GET_SHADOW_STACK_STATUS,
                &mut new_mode as *mut c_ulong,
                0,
                0,
                0,
            ) as c_int
        };
        if ret == 0 {
            if new_mode != mode {
                unsafe {
                    ksft_print_msg(
                        c"Mode set to %lx not %lx\n".as_ptr(),
                        new_mode,
                        mode,
                    );
                }
                ret = -EINVAL;
            }
        } else {
            unsafe {
                ksft_print_msg(c"Failed to validate mode: %d\n".as_ptr(), errno);
            }
        }

        if enabling != unsafe { chkfeat_gcs() } {
            unsafe {
                ksft_print_msg(
                    c"%senabled by prctl but %senabled in CHKFEAT\n".as_ptr(),
                    if enabling { c"".as_ptr() } else { c"not ".as_ptr() },
                    if chkfeat_gcs() { c"".as_ptr() } else { c"not ".as_ptr() },
                );
            }
            ret = -EINVAL;
        }
    }

    ret
}

/* Try to read the status */
unsafe fn read_status() -> bool {
    let mut state: c_ulong = 0;
    let mut ret: c_int;

    ret = unsafe {
        syscall(
            __NR_prctl,
            PR_GET_SHADOW_STACK_STATUS,
            &mut state as *mut c_ulong,
            0,
            0,
            0,
        ) as c_int
    };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"Failed to read state: %d\n".as_ptr(), errno);
        }
        return false;
    }

    (state & PR_SHADOW_STACK_ENABLE) != 0
}

/* Just a straight enable */
unsafe fn base_enable() -> bool {
    let mut ret: c_int;

    ret = unsafe { gcs_set_status(PR_SHADOW_STACK_ENABLE) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"PR_SHADOW_STACK_ENABLE failed %d\n".as_ptr(), ret);
        }
        return false;
    }

    true
}

/* Check we can read GCSPR_EL0 when GCS is enabled */
unsafe fn read_gcspr_el0() -> bool {
    let mut gcspr_el0: *mut c_ulong;

    unsafe {
        ksft_print_msg(c"GET GCSPR\n".as_ptr());
        gcspr_el0 = get_gcspr();
        ksft_print_msg(c"GCSPR_EL0 is %p\n".as_ptr(), gcspr_el0);
    }

    true
}

/* Also allow writes to stack */
unsafe fn enable_writeable() -> bool {
    let mut ret: c_int;

    ret = unsafe { gcs_set_status(PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(
                c"PR_SHADOW_STACK_ENABLE writeable failed: %d\n".as_ptr(),
                ret,
            );
        }
        return false;
    }

    ret = unsafe { gcs_set_status(PR_SHADOW_STACK_ENABLE) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"failed to restore plain enable %d\n".as_ptr(), ret);
        }
        return false;
    }

    true
}

/* Also allow writes to stack */
unsafe fn enable_push_pop() -> bool {
    let mut ret: c_int;

    ret = unsafe { gcs_set_status(PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_PUSH) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(
                c"PR_SHADOW_STACK_ENABLE with push failed: %d\n".as_ptr(),
                ret,
            );
        }
        return false;
    }

    ret = unsafe { gcs_set_status(PR_SHADOW_STACK_ENABLE) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"failed to restore plain enable %d\n".as_ptr(), ret);
        }
        return false;
    }

    true
}

/* Enable GCS and allow everything */
unsafe fn enable_all() -> bool {
    let mut ret: c_int;

    ret = unsafe {
        gcs_set_status(PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_PUSH | PR_SHADOW_STACK_WRITE)
    };
    if ret != 0 {
        unsafe {
            ksft_print_msg(
                c"PR_SHADOW_STACK_ENABLE with everything failed: %d\n".as_ptr(),
                ret,
            );
        }
        return false;
    }

    ret = unsafe { gcs_set_status(PR_SHADOW_STACK_ENABLE) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"failed to restore plain enable %d\n".as_ptr(), ret);
        }
        return false;
    }

    true
}

unsafe fn enable_invalid() -> bool {
    let ret: c_int = unsafe { gcs_set_status(ULONG_MAX) };
    if ret == 0 {
        unsafe {
            ksft_print_msg(c"GCS_SET_STATUS %lx succeeded\n".as_ptr(), ULONG_MAX);
        }
        return false;
    }

    true
}

/* Map a GCS */
unsafe fn map_guarded_stack() -> bool {
    let mut ret: c_int;
    let mut buf: *mut uint64_t;
    let mut expected_cap: uint64_t;
    let mut elem: c_int;
    let mut pass: bool = true;

    buf = unsafe {
        syscall(
            __NR_map_shadow_stack,
            0,
            page_size,
            SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN,
        ) as *mut uint64_t
    };
    if buf == MAP_FAILED {
        unsafe {
            ksft_print_msg(
                c"Failed to map %lu byte GCS: %d\n".as_ptr(),
                page_size,
                errno,
            );
        }
        return false;
    }
    unsafe {
        ksft_print_msg(
            c"Mapped GCS at %p-%p\n".as_ptr(),
            buf,
            (buf as uint64_t + page_size as uint64_t) as *mut c_void,
        );
    }

    /* The top of the newly allocated region should be 0 */
    elem = (unsafe { page_size } / core::mem::size_of::<uint64_t>()) as c_int - 1;
    if unsafe { *buf.offset(elem as isize) } != 0 {
        unsafe {
            ksft_print_msg(
                c"Last entry is 0x%llx not 0x0\n".as_ptr(),
                *buf.offset(elem as isize),
            );
        }
        pass = false;
    }

    /* Then a valid cap token */
    elem -= 1;
    expected_cap = (buf as uint64_t + unsafe { page_size } as uint64_t - 16) as uint64_t;
    expected_cap &= GCS_CAP_ADDR_MASK;
    expected_cap |= GCS_CAP_VALID_TOKEN;
    if unsafe { *buf.offset(elem as isize) } != expected_cap {
        unsafe {
            ksft_print_msg(
                c"Cap entry is 0x%llx not 0x%llx\n".as_ptr(),
                *buf.offset(elem as isize),
                expected_cap,
            );
        }
        pass = false;
    }
    unsafe {
        ksft_print_msg(c"cap token is 0x%llx\n".as_ptr(), *buf.offset(elem as isize));
    }

    /* The rest should be zeros */
    elem = 0;
    while elem < (unsafe { page_size } / core::mem::size_of::<uint64_t>()) as c_int - 2 {
        if unsafe { *buf.offset(elem as isize) } == 0 {
            elem += 1;
            continue;
        }
        unsafe {
            ksft_print_msg(
                c"GCS slot %d is 0x%llx not 0x0\n".as_ptr(),
                elem,
                *buf.offset(elem as isize),
            );
        }
        pass = false;
        elem += 1;
    }

    ret = unsafe { munmap(buf as *mut c_void, page_size) };
    if ret != 0 {
        unsafe {
            ksft_print_msg(
                c"Failed to unmap %ld byte GCS: %d\n".as_ptr(),
                page_size,
                errno,
            );
        }
        pass = false;
    }

    pass
}

/* A fork()ed process can run */
unsafe fn test_fork() -> bool {
    let mut child_mode: c_ulong = 0;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pass: bool = true;

    pid = unsafe { fork() };
    if pid == -1 {
        unsafe {
            ksft_print_msg(c"fork() failed: %d\n".as_ptr(), errno);
        }
        pass = false;
        return pass;
    }
    if pid == 0 {
        /* In child, make sure we can call a function, read
         * the GCS pointer and status and then exit */
        unsafe {
            valid_gcs_function();
            get_gcspr();
        }

        ret = unsafe {
            syscall(
                __NR_prctl,
                PR_GET_SHADOW_STACK_STATUS,
                &mut child_mode as *mut c_ulong,
                0,
                0,
                0,
            ) as c_int
        };
        if ret == 0 && (child_mode & PR_SHADOW_STACK_ENABLE) == 0 {
            unsafe {
                ksft_print_msg(c"GCS not enabled in child\n".as_ptr());
            }
            ret = -EINVAL;
        }

        unsafe {
            exit(ret);
        }
    }

    /*
     * In parent, check we can still do function calls then block
     * for the child.
     */
    unsafe {
        valid_gcs_function();
    }

    unsafe {
        ksft_print_msg(c"Waiting for child %d\n".as_ptr(), pid);
    }

    ret = unsafe { waitpid(pid, &mut status as *mut c_int, 0) };
    if ret == -1 {
        unsafe {
            ksft_print_msg(c"Failed to wait for child: %d\n".as_ptr(), errno);
        }
        return false;
    }

    if !wifexited(status) {
        unsafe {
            ksft_print_msg(c"Child exited due to signal %d\n".as_ptr(), wtermsig(status));
        }
        pass = false;
    } else {
        if wexitstatus(status) != 0 {
            unsafe {
                ksft_print_msg(
                    c"Child exited with status %d\n".as_ptr(),
                    wexitstatus(status),
                );
            }
            pass = false;
        }
    }

    pass
}

/* A vfork()ed process can run and exit */
unsafe fn test_vfork() -> bool {
    let mut child_mode: c_ulong = 0;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pass: bool = true;

    pid = unsafe { vfork() };
    if pid == -1 {
        unsafe {
            ksft_print_msg(c"vfork() failed: %d\n".as_ptr(), errno);
        }
        pass = false;
        return pass;
    }
    if pid == 0 {
        /*
         * In child, make sure we can call a function, read
         * the GCS pointer and status and then exit.
         */
        unsafe {
            valid_gcs_function();
            get_gcspr();
        }

        ret = unsafe {
            syscall(
                __NR_prctl,
                PR_GET_SHADOW_STACK_STATUS,
                &mut child_mode as *mut c_ulong,
                0,
                0,
                0,
            ) as c_int
        };
        if ret == 0 && (child_mode & PR_SHADOW_STACK_ENABLE) == 0 {
            unsafe {
                ksft_print_msg(c"GCS not enabled in child\n".as_ptr());
            }
            ret = EXIT_FAILURE;
        }

        unsafe {
            _exit(ret);
        }
    }

    /*
     * In parent, check we can still do function calls then check
     * on the child.
     */
    unsafe {
        valid_gcs_function();
    }

    unsafe {
        ksft_print_msg(c"Waiting for child %d\n".as_ptr(), pid);
    }

    ret = unsafe { waitpid(pid, &mut status as *mut c_int, 0) };
    if ret == -1 {
        unsafe {
            ksft_print_msg(c"Failed to wait for child: %d\n".as_ptr(), errno);
        }
        return false;
    }

    if !wifexited(status) {
        unsafe {
            ksft_print_msg(c"Child exited due to signal %d\n".as_ptr(), wtermsig(status));
        }
        pass = false;
    } else if wexitstatus(status) != 0 {
        unsafe {
            ksft_print_msg(
                c"Child exited with status %d\n".as_ptr(),
                wexitstatus(status),
            );
        }
        pass = false;
    }

    pass
}

type gcs_test = unsafe fn() -> bool;

#[repr(C)]
struct Test {
    name: *const c_char,
    test: gcs_test,
    needs_enable: bool,
}

static tests: [Test; 10] = [
    Test {
        name: c"read_status".as_ptr(),
        test: read_status,
        needs_enable: false,
    },
    Test {
        name: c"base_enable".as_ptr(),
        test: base_enable,
        needs_enable: true,
    },
    Test {
        name: c"read_gcspr_el0".as_ptr(),
        test: read_gcspr_el0,
        needs_enable: false,
    },
    Test {
        name: c"enable_writeable".as_ptr(),
        test: enable_writeable,
        needs_enable: true,
    },
    Test {
        name: c"enable_push_pop".as_ptr(),
        test: enable_push_pop,
        needs_enable: true,
    },
    Test {
        name: c"enable_all".as_ptr(),
        test: enable_all,
        needs_enable: true,
    },
    Test {
        name: c"enable_invalid".as_ptr(),
        test: enable_invalid,
        needs_enable: true,
    },
    Test {
        name: c"map_guarded_stack".as_ptr(),
        test: map_guarded_stack,
        needs_enable: false,
    },
    Test {
        name: c"fork".as_ptr(),
        test: test_fork,
        needs_enable: false,
    },
    Test {
        name: c"vfork".as_ptr(),
        test: test_vfork,
        needs_enable: false,
    },
];

fn main() {
    let mut i: c_int;
    let mut ret: c_int;
    let mut gcs_mode: c_ulong = 0;

    unsafe {
        ksft_print_header();
    }

    if unsafe { getauxval(AT_HWCAP) & HWCAP_GCS } == 0 {
        unsafe {
            ksft_exit_skip(c"SKIP GCS not supported\n".as_ptr());
        }
    }

    ret = unsafe {
        syscall(
            __NR_prctl,
            PR_GET_SHADOW_STACK_STATUS,
            &mut gcs_mode as *mut c_ulong,
            0,
            0,
            0,
        ) as c_int
    };
    if ret != 0 {
        unsafe {
            ksft_exit_fail_msg(c"Failed to read GCS state: %d\n".as_ptr(), errno);
        }
    }

    if (gcs_mode & PR_SHADOW_STACK_ENABLE) == 0 {
        gcs_mode = PR_SHADOW_STACK_ENABLE;
        ret = unsafe {
            syscall(
                __NR_prctl,
                PR_SET_SHADOW_STACK_STATUS,
                gcs_mode,
                0,
                0,
                0,
            ) as c_int
        };
        if ret != 0 {
            unsafe {
                ksft_exit_fail_msg(c"Failed to enable GCS: %d\n".as_ptr(), errno);
            }
        }
    }

    unsafe {
        ksft_set_plan(tests.len() as c_uint);
    }

    i = 0;
    while i < tests.len() as c_int {
        unsafe {
            ksft_test_result(
                (tests[i as usize].test)(),
                c"%s\n".as_ptr(),
                tests[i as usize].name,
            );
        }
        i += 1;
    }

    /* One last test: disable GCS, we can do this one time */
    ret = unsafe { syscall(__NR_prctl, PR_SET_SHADOW_STACK_STATUS, 0, 0, 0, 0) as c_int };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"Failed to disable GCS: %d\n".as_ptr(), errno);
        }
    }

    unsafe {
        ksft_finished();
    }
}
