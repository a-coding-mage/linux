// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 ARM Limited.
 */

// C dependencies preserved for the repository build environment:
// pthread.h, stdbool.h, sys/auxv.h, sys/mman.h, sys/prctl.h, sys/ptrace.h,
// sys/uio.h, asm/hwcap.h, asm/mman.h, asm/ptrace.h, linux/compiler.h,
// kselftest_harness.h, gcs-util.h.
// The kselftest TEST/FIXTURE macros are represented below as Rust macro calls
// and are expected to be supplied by the surrounding translated harness.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;
type pthread_t = c_ulong;
type size_t = usize;
type uint64_t = u64;

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub si_pid: pid_t,
}

#[repr(C)]
pub struct user_gcs {
    pub features_enabled: u64,
    pub features_locked: u64,
    pub gcspr_el0: u64,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn prctl(option: c_int, ...) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn fork() -> pid_t;
    fn ptrace(request: c_int, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn wait(status: *mut c_int) -> pid_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn process_vm_readv(
        pid: pid_t,
        local_iov: *const iovec,
        liovcnt: c_ulong,
        remote_iov: *const iovec,
        riovcnt: c_ulong,
        flags: c_ulong,
    ) -> isize;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn exit(status: c_int) -> !;

    fn get_gcspr() -> *mut c_ulong;
    fn gcsss1(gcspr: *mut c_ulong);
    fn gcsss2() -> *mut c_ulong;
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
}

unsafe fn my_syscall2(num: c_long, arg1: c_long, arg2: c_long) -> c_long {
    let mut _num: c_long = num;
    let mut _arg1: c_long = arg1;
    let _arg2: c_long = arg2;
    let _arg3: c_long = 0;
    let _arg4: c_long = 0;
    let _arg5: c_long = 0;

    unsafe {
        asm!(
            "svc #0",
            inout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x3") _arg4,
            in("x4") _arg5,
            in("x8") _num,
            options(nostack)
        );
    }
    _arg1
}

#[inline(never)]
unsafe fn gcs_recurse(depth: c_int) {
    if depth != 0 {
        unsafe {
            gcs_recurse(depth - 1);
        }
    }

    /* Prevent tail call optimization so we actually recurse */
    unsafe {
        asm!("dsb sy", options(nostack, preserves_flags));
    }
}

/* Smoke test that a function call and return works*/
TEST!(can_call_function, {
    unsafe {
        gcs_recurse(0);
    }
});

unsafe extern "C" fn gcs_test_thread(_arg: *mut c_void) -> *mut c_void {
    let mut ret: c_int;
    let mut mode: c_ulong = 0;

    /*
     * Some libcs don't seem to fill unused arguments with 0 but
     * the kernel validates this so we supply all 5 arguments.
     */
    ret = unsafe {
        prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        )
    };
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"PR_GET_SHADOW_STACK_STATUS failed: %d\n".as_ptr(), ret);
        }
        return ptr::null_mut();
    }

    if (mode & PR_SHADOW_STACK_ENABLE as c_ulong) == 0 {
        unsafe {
            ksft_print_msg(c"GCS not enabled in thread, mode is %lu\n".as_ptr(), mode);
        }
        return ptr::null_mut();
    }

    /* Just in case... */
    unsafe {
        gcs_recurse(0);
    }

    /* Use a non-NULL value to indicate a pass */
    gcs_test_thread as *mut c_void
}

/* Verify that if we start a new thread it has GCS enabled */
TEST!(gcs_enabled_thread, {
    unsafe {
        let mut thread: pthread_t = 0;
        let mut thread_ret: *mut c_void = ptr::null_mut();
        let mut ret: c_int;

        ret = pthread_create(
            &mut thread,
            ptr::null(),
            gcs_test_thread,
            ptr::null_mut(),
        );
        ASSERT_TRUE!(ret == 0);
        if ret != 0 {
            return;
        }

        ret = pthread_join(thread, &mut thread_ret);
        ASSERT_TRUE!(ret == 0);
        if ret != 0 {
            return;
        }

        ASSERT_TRUE!(!thread_ret.is_null());
    }
});

/* Read the GCS until we find the terminator */
TEST!(gcs_find_terminator, {
    unsafe {
        let gcs: *mut c_ulong = get_gcspr();
        let mut cur: *mut c_ulong = gcs;
        while *cur != 0 {
            cur = cur.add(1);
        }

        ksft_print_msg(c"GCS in use from %p-%p\n".as_ptr(), gcs, cur);

        /*
         * We should have at least whatever called into this test so
         * the two pointer should differ.
         */
        ASSERT_TRUE!(gcs != cur);
    }
});

/*
 * We can access a GCS via ptrace
 *
 * This could usefully have a fixture but note that each test is
 * fork()ed into a new child which causes issues.  Might be better to
 * lift at least some of this out into a separate, non-harness, test
 * program.
 */
TEST!(ptrace_read_write, {
    unsafe {
        let child: pid_t;
        let mut pid: pid_t;
        let mut ret: c_int;
        let mut status: c_int = 0;
        let mut si: siginfo_t = core::mem::zeroed();
        let mut val: uint64_t;
        let mut rval: uint64_t = 0;
        let gcspr: uint64_t;
        let mut child_gcs: user_gcs = core::mem::zeroed();
        let mut iov: iovec = core::mem::zeroed();
        let mut local_iov: iovec = core::mem::zeroed();
        let mut remote_iov: iovec = core::mem::zeroed();

        child = fork();
        if child == -1 {
            ksft_print_msg(
                c"fork() failed: %d (%s)\n".as_ptr(),
                errno,
                strerror(errno),
            );
            ASSERT_NE!(child, -1);
        }

        if child == 0 {
            /*
             * In child, make sure there's something on the stack and
             * ask to be traced.
             */
            gcs_recurse(0);
            if ptrace(PTRACE_TRACEME, -1, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
                ksft_exit_fail_msg(c"PTRACE_TRACEME %s".as_ptr(), strerror(errno));
            }

            if raise(SIGSTOP) != 0 {
                ksft_exit_fail_msg(c"raise(SIGSTOP) %s".as_ptr(), strerror(errno));
            }

            return;
        }

        ksft_print_msg(c"Child: %d\n".as_ptr(), child);

        /* Attach to the child */
        loop {
            let mut sig: c_int;

            pid = wait(&mut status);
            if pid == -1 {
                ksft_print_msg(c"wait() failed: %s".as_ptr(), strerror(errno));
                goto_error!();
            }

            /*
             * This should never happen but it's hard to flag in
             * the framework.
             */
            if pid != child {
                continue;
            }

            if WIFEXITED(status) || WIFSIGNALED(status) {
                ksft_exit_fail_msg(c"Child died unexpectedly\n".as_ptr());
            }

            if !WIFSTOPPED(status) {
                goto_error!();
            }

            sig = WSTOPSIG(status);

            if ptrace(
                PTRACE_GETSIGINFO,
                pid,
                ptr::null_mut::<c_void>(),
                &mut si as *mut siginfo_t,
            ) != 0
            {
                if errno == ESRCH {
                    ASSERT_NE!(errno, ESRCH);
                    return;
                }

                if errno == EINVAL {
                    sig = 0; /* bust group-stop */
                    if ptrace(PTRACE_CONT, pid, ptr::null_mut::<c_void>(), sig) != 0 {
                        if errno == ESRCH {
                            ASSERT_NE!(errno, ESRCH);
                            return;
                        }

                        ksft_print_msg(c"PTRACE_CONT: %s\n".as_ptr(), strerror(errno));
                        goto_error!();
                    }
                    continue;
                }

                ksft_print_msg(c"PTRACE_GETSIGINFO: %s\n".as_ptr(), strerror(errno));
                goto_error!();
            }

            if sig == SIGSTOP && si.si_code == SI_TKILL && si.si_pid == pid {
                break;
            }

            if ptrace(PTRACE_CONT, pid, ptr::null_mut::<c_void>(), sig) != 0 {
                if errno == ESRCH {
                    ASSERT_NE!(errno, ESRCH);
                    return;
                }

                ksft_print_msg(c"PTRACE_CONT: %s\n".as_ptr(), strerror(errno));
                goto_error!();
            }
        }

        /* Where is the child GCS? */
        iov.iov_base = &mut child_gcs as *mut user_gcs as *mut c_void;
        iov.iov_len = size_of::<user_gcs>();
        ret = ptrace(PTRACE_GETREGSET, child, NT_ARM_GCS, &mut iov as *mut iovec) as c_int;
        if ret != 0 {
            ksft_print_msg(
                c"Failed to read child GCS state: %s (%d)\n".as_ptr(),
                strerror(errno),
                errno,
            );
            goto_error!();
        }

        /* We should have inherited GCS over fork(), confirm */
        if (child_gcs.features_enabled & PR_SHADOW_STACK_ENABLE as u64) == 0 {
            ASSERT_TRUE!((child_gcs.features_enabled & PR_SHADOW_STACK_ENABLE as u64) != 0);
            goto_error!();
        }

        gcspr = child_gcs.gcspr_el0;
        ksft_print_msg(
            c"Child GCSPR 0x%lx, flags %llx, locked %llx\n".as_ptr(),
            gcspr,
            child_gcs.features_enabled,
            child_gcs.features_locked,
        );

        /* Ideally we'd cross check with the child memory map */

        errno = 0;
        val = ptrace(PTRACE_PEEKDATA, child, gcspr as *mut c_void, ptr::null_mut::<c_void>()) as uint64_t;
        ret = errno;
        if ret != 0 {
            ksft_print_msg(c"PTRACE_PEEKDATA failed: %s (%d)\n".as_ptr(), strerror(ret), ret);
        }
        EXPECT_EQ!(ret, 0);

        /* The child should be in a function, the GCSPR shouldn't be 0 */
        EXPECT_NE!(val, 0);

        /* Same thing via process_vm_readv() */
        local_iov.iov_base = &mut rval as *mut uint64_t as *mut c_void;
        local_iov.iov_len = size_of::<uint64_t>();
        remote_iov.iov_base = gcspr as *mut c_void;
        remote_iov.iov_len = size_of::<uint64_t>();
        ret = process_vm_readv(child, &local_iov, 1, &remote_iov, 1, 0) as c_int;
        if ret == -1 {
            ksft_print_msg(
                c"process_vm_readv() failed: %s (%d)\n".as_ptr(),
                strerror(errno),
                errno,
            );
        }
        EXPECT_EQ!(ret as usize, size_of::<uint64_t>());
        EXPECT_EQ!(val, rval);

        /* Write data via a peek */
        ret = ptrace(PTRACE_POKEDATA, child, gcspr as *mut c_void, ptr::null_mut::<c_void>()) as c_int;
        if ret == -1 {
            ksft_print_msg(c"PTRACE_POKEDATA failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }
        EXPECT_EQ!(ret, 0);
        EXPECT_EQ!(
            0,
            ptrace(PTRACE_PEEKDATA, child, gcspr as *mut c_void, ptr::null_mut::<c_void>())
        );

        /* Restore what we had before */
        ret = ptrace(PTRACE_POKEDATA, child, gcspr as *mut c_void, val) as c_int;
        if ret == -1 {
            ksft_print_msg(c"PTRACE_POKEDATA failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }
        EXPECT_EQ!(ret, 0);
        EXPECT_EQ!(
            val,
            ptrace(PTRACE_PEEKDATA, child, gcspr as *mut c_void, ptr::null_mut::<c_void>()) as uint64_t
        );

        /* That's all, folks */
        kill(child, SIGKILL);
        return;

        error: {
            kill(child, SIGKILL);
            ASSERT_FALSE!(true);
        }
    }
});

FIXTURE!(map_gcs, {
    stack: *mut c_ulong,
});

FIXTURE_VARIANT!(map_gcs, {
    stack_size: size_t,
    flags: c_ulong,
});

FIXTURE_VARIANT_ADD!(map_gcs, s2k_cap_marker, {
    stack_size: 2 * 1024,
    flags: SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s2k_cap, {
    stack_size: 2 * 1024,
    flags: SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s2k_marker, {
    stack_size: 2 * 1024,
    flags: SHADOW_STACK_SET_MARKER,
});

FIXTURE_VARIANT_ADD!(map_gcs, s2k, {
    stack_size: 2 * 1024,
    flags: 0,
});

FIXTURE_VARIANT_ADD!(map_gcs, s4k_cap_marker, {
    stack_size: 4 * 1024,
    flags: SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s4k_cap, {
    stack_size: 4 * 1024,
    flags: SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s3k_marker, {
    stack_size: 4 * 1024,
    flags: SHADOW_STACK_SET_MARKER,
});

FIXTURE_VARIANT_ADD!(map_gcs, s4k, {
    stack_size: 4 * 1024,
    flags: 0,
});

FIXTURE_VARIANT_ADD!(map_gcs, s16k_cap_marker, {
    stack_size: 16 * 1024,
    flags: SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s16k_cap, {
    stack_size: 16 * 1024,
    flags: SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s16k_marker, {
    stack_size: 16 * 1024,
    flags: SHADOW_STACK_SET_MARKER,
});

FIXTURE_VARIANT_ADD!(map_gcs, s16k, {
    stack_size: 16 * 1024,
    flags: 0,
});

FIXTURE_VARIANT_ADD!(map_gcs, s64k_cap_marker, {
    stack_size: 64 * 1024,
    flags: SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s64k_cap, {
    stack_size: 64 * 1024,
    flags: SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s64k_marker, {
    stack_size: 64 * 1024,
    flags: SHADOW_STACK_SET_MARKER,
});

FIXTURE_VARIANT_ADD!(map_gcs, s64k, {
    stack_size: 64 * 1024,
    flags: 0,
});

FIXTURE_VARIANT_ADD!(map_gcs, s128k_cap_marker, {
    stack_size: 128 * 1024,
    flags: SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s128k_cap, {
    stack_size: 128 * 1024,
    flags: SHADOW_STACK_SET_TOKEN,
});

FIXTURE_VARIANT_ADD!(map_gcs, s128k_marker, {
    stack_size: 128 * 1024,
    flags: SHADOW_STACK_SET_MARKER,
});

FIXTURE_VARIANT_ADD!(map_gcs, s128k, {
    stack_size: 128 * 1024,
    flags: 0,
});

FIXTURE_SETUP!(map_gcs, |self_, variant| {
    unsafe {
        self_.stack = syscall(
            __NR_map_shadow_stack,
            0,
            variant.stack_size,
            variant.flags,
        ) as *mut c_ulong;
        ASSERT_FALSE!(self_.stack == MAP_FAILED as *mut c_ulong);
        ksft_print_msg(
            c"Allocated stack from %p-%p\n".as_ptr(),
            self_.stack,
            self_.stack.add(variant.stack_size),
        );
    }
});

FIXTURE_TEARDOWN!(map_gcs, |self_, variant| {
    unsafe {
        let mut ret: c_int;

        if self_.stack != MAP_FAILED as *mut c_ulong {
            ret = munmap(self_.stack as *mut c_void, variant.stack_size);
            ASSERT_EQ!(ret, 0);
        }
    }
});

/* The stack has a cap token */
TEST_F!(map_gcs, stack_capped, |self_, variant| {
    unsafe {
        let stack: *mut c_ulong = self_.stack;
        let mut cap_index: size_t;

        cap_index = variant.stack_size / size_of::<c_ulong>();

        match variant.flags & (SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN) {
            x if x == (SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN) => {
                cap_index -= 2;
            }
            SHADOW_STACK_SET_TOKEN => {
                cap_index -= 1;
            }
            SHADOW_STACK_SET_MARKER | 0 => {
                /* No cap, no test */
                return;
            }
            _ => {}
        }

        ASSERT_EQ!(*stack.add(cap_index), GCS_CAP!(stack.add(cap_index)));
    }
});

/* The top of the stack is 0 */
TEST_F!(map_gcs, stack_terminated, |self_, variant| {
    unsafe {
        let stack: *mut c_ulong = self_.stack;
        let term_index: size_t;

        if (variant.flags & SHADOW_STACK_SET_MARKER) == 0 {
            return;
        }

        term_index = (variant.stack_size / size_of::<c_ulong>()) - 1;

        ASSERT_EQ!(*stack.add(term_index), 0);
    }
});

/* Writes should fault */
TEST_F_SIGNAL!(map_gcs, not_writeable, SIGSEGV, |self_, _variant| {
    unsafe {
        *self_.stack.add(0) = 0;
    }
});

/* Put it all together, we can safely switch to and from the stack */
TEST_F!(map_gcs, stack_switch, |self_, variant| {
    unsafe {
        let mut cap_index: size_t;
        cap_index = variant.stack_size / size_of::<c_ulong>();
        let mut orig_gcspr_el0: *mut c_ulong;
        let mut pivot_gcspr_el0: *mut c_ulong;

        /* Skip over the stack terminator and point at the cap */
        match variant.flags & (SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN) {
            x if x == (SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN) => {
                cap_index -= 2;
            }
            SHADOW_STACK_SET_TOKEN => {
                cap_index -= 1;
            }
            SHADOW_STACK_SET_MARKER | 0 => {
                /* No cap, no test */
                return;
            }
            _ => {}
        }
        pivot_gcspr_el0 = self_.stack.add(cap_index);

        /* Pivot to the new GCS */
        ksft_print_msg(
            c"Pivoting to %p from %p, target has value 0x%lx\n".as_ptr(),
            pivot_gcspr_el0,
            get_gcspr(),
            *pivot_gcspr_el0,
        );
        gcsss1(pivot_gcspr_el0);
        orig_gcspr_el0 = gcsss2();
        ksft_print_msg(
            c"Pivoted to %p from %p, target has value 0x%lx\n".as_ptr(),
            get_gcspr(),
            orig_gcspr_el0,
            *pivot_gcspr_el0,
        );

        ksft_print_msg(c"Pivoted, GCSPR_EL0 now %p\n".as_ptr(), get_gcspr());

        /* New GCS must be in the new buffer */
        ASSERT_TRUE!((get_gcspr() as c_ulong) > (self_.stack as c_ulong));
        ASSERT_TRUE!((get_gcspr() as c_ulong) <= (self_.stack as c_ulong) + variant.stack_size as c_ulong);

        /* We should be able to use all but 2 slots of the new stack */
        ksft_print_msg(c"Recursing %zu levels\n".as_ptr(), cap_index - 1);
        gcs_recurse((cap_index - 1) as c_int);

        /* Pivot back to the original GCS */
        gcsss1(orig_gcspr_el0);
        pivot_gcspr_el0 = gcsss2();

        gcs_recurse(0);
        ksft_print_msg(c"Pivoted back to GCSPR_EL0 0x%p\n".as_ptr(), get_gcspr());
    }
});

/* We fault if we try to go beyond the end of the stack */
TEST_F_SIGNAL!(map_gcs, stack_overflow, SIGSEGV, |self_, variant| {
    unsafe {
        let mut cap_index: size_t;
        cap_index = variant.stack_size / size_of::<c_ulong>();
        let mut orig_gcspr_el0: *mut c_ulong;
        let mut pivot_gcspr_el0: *mut c_ulong;

        /* Skip over the stack terminator and point at the cap */
        match variant.flags & (SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN) {
            x if x == (SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN) => {
                cap_index -= 2;
            }
            SHADOW_STACK_SET_TOKEN => {
                cap_index -= 1;
            }
            SHADOW_STACK_SET_MARKER | 0 => {
                /* No cap, no test but we need to SEGV to avoid a false fail */
                orig_gcspr_el0 = get_gcspr();
                *orig_gcspr_el0 = 0;
                return;
            }
            _ => {}
        }
        pivot_gcspr_el0 = self_.stack.add(cap_index);

        /* Pivot to the new GCS */
        ksft_print_msg(
            c"Pivoting to %p from %p, target has value 0x%lx\n".as_ptr(),
            pivot_gcspr_el0,
            get_gcspr(),
            *pivot_gcspr_el0,
        );
        gcsss1(pivot_gcspr_el0);
        orig_gcspr_el0 = gcsss2();
        ksft_print_msg(
            c"Pivoted to %p from %p, target has value 0x%lx\n".as_ptr(),
            pivot_gcspr_el0,
            orig_gcspr_el0,
            *pivot_gcspr_el0,
        );

        ksft_print_msg(c"Pivoted, GCSPR_EL0 now %p\n".as_ptr(), get_gcspr());

        /* New GCS must be in the new buffer */
        ASSERT_TRUE!((get_gcspr() as c_ulong) > (self_.stack as c_ulong));
        ASSERT_TRUE!((get_gcspr() as c_ulong) <= (self_.stack as c_ulong) + variant.stack_size as c_ulong);

        /* Now try to recurse, we should fault doing this. */
        ksft_print_msg(c"Recursing %zu levels...\n".as_ptr(), cap_index + 1);
        gcs_recurse((cap_index + 1) as c_int);
        ksft_print_msg(c"...done\n".as_ptr());

        /* Clean up properly to try to guard against spurious passes. */
        gcsss1(orig_gcspr_el0);
        pivot_gcspr_el0 = gcsss2();
        ksft_print_msg(c"Pivoted back to GCSPR_EL0 0x%p\n".as_ptr(), get_gcspr());
    }
});

FIXTURE!(map_invalid_gcs, {});

FIXTURE_VARIANT!(map_invalid_gcs, {
    stack_size: size_t,
});

FIXTURE_SETUP!(map_invalid_gcs, |_self_, _variant| {});

FIXTURE_TEARDOWN!(map_invalid_gcs, |_self_, _variant| {});

/* GCS must be larger than 16 bytes */
FIXTURE_VARIANT_ADD!(map_invalid_gcs, too_small, {
    stack_size: 8,
});

/* GCS size must be 16 byte aligned */
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_1, { stack_size: 1024 + 1 });
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_2, { stack_size: 1024 + 2 });
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_3, { stack_size: 1024 + 3 });
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_4, { stack_size: 1024 + 4 });
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_5, { stack_size: 1024 + 5 });
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_6, { stack_size: 1024 + 6 });
FIXTURE_VARIANT_ADD!(map_invalid_gcs, unligned_7, { stack_size: 1024 + 7 });

TEST_F!(map_invalid_gcs, do_map, |_self_, variant| {
    unsafe {
        let mut stack: *mut c_void;

        stack = syscall(__NR_map_shadow_stack, 0, variant.stack_size, 0) as *mut c_void;
        ASSERT_TRUE!(stack == MAP_FAILED);
        if stack != MAP_FAILED {
            munmap(stack, variant.stack_size);
        }
    }
});

FIXTURE!(invalid_mprotect, {
    stack: *mut c_ulong,
    stack_size: size_t,
});

FIXTURE_VARIANT!(invalid_mprotect, {
    flags: c_ulong,
});

FIXTURE_SETUP!(invalid_mprotect, |self_, _variant| {
    unsafe {
        self_.stack_size = sysconf(_SC_PAGE_SIZE) as size_t;
        self_.stack = syscall(__NR_map_shadow_stack, 0, self_.stack_size, 0) as *mut c_ulong;
        ASSERT_FALSE!(self_.stack == MAP_FAILED as *mut c_ulong);
        ksft_print_msg(
            c"Allocated stack from %p-%p\n".as_ptr(),
            self_.stack,
            self_.stack.add(self_.stack_size),
        );
    }
});

FIXTURE_TEARDOWN!(invalid_mprotect, |self_, _variant| {
    unsafe {
        let mut ret: c_int;

        if self_.stack != MAP_FAILED as *mut c_ulong {
            ret = munmap(self_.stack as *mut c_void, self_.stack_size);
            ASSERT_EQ!(ret, 0);
        }
    }
});

FIXTURE_VARIANT_ADD!(invalid_mprotect, exec, {
    flags: PROT_EXEC,
});

TEST_F!(invalid_mprotect, do_map, |self_, variant| {
    unsafe {
        let mut ret: c_int;

        ret = mprotect(self_.stack as *mut c_void, self_.stack_size, variant.flags as c_int);
        ASSERT_EQ!(ret, -1);
    }
});

TEST_F!(invalid_mprotect, do_map_read, |self_, variant| {
    unsafe {
        let mut ret: c_int;

        ret = mprotect(
            self_.stack as *mut c_void,
            self_.stack_size,
            (variant.flags | PROT_READ) as c_int,
        );
        ASSERT_EQ!(ret, -1);
    }
});

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut gcs_mode: c_ulong = 0;
    let mut ret: c_int;

    if (unsafe { getauxval(AT_HWCAP) } & HWCAP_GCS) == 0 {
        unsafe {
            ksft_exit_skip(c"SKIP GCS not supported\n".as_ptr());
        }
    }

    /*
     * Force shadow stacks on, our tests *should* be fine with or
     * without libc support and with or without this having ended
     * up tagged for GCS and enabled by the dynamic linker.  We
     * can't use the libc prctl() function since we can't return
     * from enabling the stack.
     */
    ret = unsafe {
        my_syscall2(
            __NR_prctl,
            PR_GET_SHADOW_STACK_STATUS as c_long,
            &mut gcs_mode as *mut c_ulong as c_long,
        )
    } as c_int;
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"Failed to read GCS state: %d\n".as_ptr(), ret);
        }
        return EXIT_FAILURE;
    }

    if (gcs_mode & PR_SHADOW_STACK_ENABLE as c_ulong) == 0 {
        gcs_mode = PR_SHADOW_STACK_ENABLE as c_ulong;
        ret = unsafe {
            my_syscall2(
                __NR_prctl,
                PR_SET_SHADOW_STACK_STATUS as c_long,
                gcs_mode as c_long,
            )
        } as c_int;
        if ret != 0 {
            unsafe {
                ksft_print_msg(c"Failed to configure GCS: %d\n".as_ptr(), ret);
            }
            return EXIT_FAILURE;
        }
    }

    /* Avoid returning in case libc doesn't understand GCS */
    unsafe {
        exit(test_harness_run(argc, argv));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
