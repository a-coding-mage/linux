// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 ARM Limited.
 *
 * Tests for GCS mode locking.  These tests rely on both having GCS
 * unconfigured on entry and on the kselftest harness running each
 * test in a fork()ed process which will have it's own mode.
 */

// C includes removed:
// <limits.h>, <sys/auxv.h>, <sys/prctl.h>, <asm/hwcap.h>,
// "kselftest_harness.h", and "gcs-util.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static __NR_prctl: c_long;

    static PR_LOCK_SHADOW_STACK_STATUS: c_int;
    static PR_SET_SHADOW_STACK_STATUS: c_int;
    static PR_GET_SHADOW_STACK_STATUS: c_int;
    static PR_SHADOW_STACK_ENABLE: c_ulong;
    static PR_SHADOW_STACK_WRITE: c_ulong;
    static PR_SHADOW_STACK_PUSH: c_ulong;
    static PR_SHADOW_STACK_ALL_MODES: c_ulong;

    static AT_HWCAP: c_ulong;
    static HWCAP_GCS: c_ulong;

    static EBUSY: c_int;
    static EXIT_FAILURE: c_int;
    static KSFT_SKIP: c_int;

    fn prctl(option: c_int, ...) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn _exit(status: c_int) -> !;
}

type c_long = i64;

unsafe fn my_syscall2(num: c_long, arg1: c_long, arg2: c_long) -> c_long {
    let mut _num: c_long = num;
    let mut _arg1: c_long = arg1;
    let mut _arg2: c_long = arg2;
    let mut _arg3: c_long = 0;
    let mut _arg4: c_long = 0;
    let mut _arg5: c_long = 0;

    unsafe {
        asm!(
            "svc #0",
            inlateout("x0") _arg1,
            in("x1") _arg2,
            in("x2") _arg3,
            in("x3") _arg4,
            in("x4") _arg5,
            in("x8") _num,
            clobber_abi("C"),
        );
    }

    _arg1
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
    };
}

/* No mode bits are rejected for locking */
fn lock_all_modes() {
    let mut ret: c_int;

    unsafe {
        ret = prctl(PR_LOCK_SHADOW_STACK_STATUS, c_ulong::MAX, 0, 0, 0);
    }
    ASSERT_EQ!(ret, 0);
}

#[repr(C)]
struct valid_modes {}

#[repr(C)]
struct valid_modes_variant {
    mode: c_ulong,
}

static valid_modes_enable: valid_modes_variant = valid_modes_variant {
    mode: unsafe { PR_SHADOW_STACK_ENABLE },
};

static valid_modes_enable_write: valid_modes_variant = valid_modes_variant {
    mode: unsafe { PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE },
};

static valid_modes_enable_push: valid_modes_variant = valid_modes_variant {
    mode: unsafe { PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_PUSH },
};

static valid_modes_enable_write_push: valid_modes_variant = valid_modes_variant {
    mode: unsafe { PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE | PR_SHADOW_STACK_PUSH },
};

fn valid_modes_setup(_self: *mut valid_modes) {}

fn valid_modes_teardown(_self: *mut valid_modes) {}

/* We can set the mode at all */
unsafe fn valid_modes_set(variant: *const valid_modes_variant) {
    let mut ret: c_int;

    unsafe {
        ret = my_syscall2(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS as c_long,
            (*variant).mode as c_long,
        ) as c_int;
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        _exit(0);
    }
}

/* Enabling, locking then disabling is rejected */
unsafe fn valid_modes_enable_lock_disable(variant: *const valid_modes_variant) {
    let mut mode: c_ulong = 0;
    let mut ret: c_int;

    unsafe {
        ret = my_syscall2(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS as c_long,
            (*variant).mode as c_long,
        ) as c_int;
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);
    unsafe {
        ASSERT_EQ!(mode, (*variant).mode);
    }

    unsafe {
        ret = prctl(
            PR_LOCK_SHADOW_STACK_STATUS,
            (*variant).mode,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS as c_long, 0) as c_int;
    }
    unsafe {
        ASSERT_EQ!(ret, -EBUSY);
    }

    unsafe {
        _exit(0);
    }
}

/* Locking then enabling is rejected */
unsafe fn valid_modes_lock_enable(variant: *const valid_modes_variant) {
    let mut mode: c_ulong = 0;
    let mut ret: c_int;

    unsafe {
        ret = prctl(
            PR_LOCK_SHADOW_STACK_STATUS,
            (*variant).mode,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = my_syscall2(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS as c_long,
            (*variant).mode as c_long,
        ) as c_int;
    }
    unsafe {
        ASSERT_EQ!(ret, -EBUSY);
    }

    unsafe {
        ret = prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);
    ASSERT_EQ!(mode, 0);

    unsafe {
        _exit(0);
    }
}

/* Locking then changing other modes is fine */
unsafe fn valid_modes_lock_enable_disable_others(variant: *const valid_modes_variant) {
    let mut mode: c_ulong = 0;
    let mut ret: c_int;

    unsafe {
        ret = my_syscall2(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS as c_long,
            (*variant).mode as c_long,
        ) as c_int;
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);
    unsafe {
        ASSERT_EQ!(mode, (*variant).mode);
    }

    unsafe {
        ret = prctl(
            PR_LOCK_SHADOW_STACK_STATUS,
            (*variant).mode,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = my_syscall2(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS as c_long,
            PR_SHADOW_STACK_ALL_MODES as c_long,
        ) as c_int;
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);
    unsafe {
        ASSERT_EQ!(mode, PR_SHADOW_STACK_ALL_MODES);
    }

    unsafe {
        ret = my_syscall2(
            __NR_prctl,
            PR_SET_SHADOW_STACK_STATUS as c_long,
            (*variant).mode as c_long,
        ) as c_int;
    }
    ASSERT_EQ!(ret, 0);

    unsafe {
        ret = prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        );
    }
    ASSERT_EQ!(ret, 0);
    unsafe {
        ASSERT_EQ!(mode, (*variant).mode);
    }

    unsafe {
        _exit(0);
    }
}

fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut mode: c_ulong = 0;
    let mut ret: c_int;

    unsafe {
        if (getauxval(AT_HWCAP) & HWCAP_GCS) == 0 {
            ksft_exit_skip(c"SKIP GCS not supported\n".as_ptr());
        }
    }

    unsafe {
        ret = prctl(
            PR_GET_SHADOW_STACK_STATUS,
            &mut mode as *mut c_ulong,
            0,
            0,
            0,
        );
    }
    if ret != 0 {
        unsafe {
            ksft_print_msg(c"Failed to read GCS state: %d\n".as_ptr(), ret);
        }
        unsafe {
            return EXIT_FAILURE;
        }
    }

    unsafe {
        if (mode & PR_SHADOW_STACK_ENABLE) != 0 {
            ksft_print_msg(c"GCS was enabled, test unsupported\n".as_ptr());
            return KSFT_SKIP;
        }
    }

    unsafe { test_harness_run(argc, argv) }
}
