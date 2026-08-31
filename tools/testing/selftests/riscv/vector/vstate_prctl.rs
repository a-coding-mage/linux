// SPDX-License-Identifier: GPL-2.0-only
// Translated from testing/selftests/riscv/vector/vstate_prctl.c.
// C dependencies: <sys/prctl.h>, <unistd.h>, <errno.h>, <sys/wait.h>,
// <sys/types.h>, <stdlib.h>, "kselftest_harness.h", "v_helpers.h".

use std::ffi::{c_char, c_int, c_long};

const NEXT_PROGRAM: &[u8] = b"./vstate_exec_nolibc\0";

const PR_RISCV_V_VSTATE_CTRL_CUR_SHIFT: c_long = 0;
const PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT: c_long = 2;

unsafe extern "C" {
    static mut errno: c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;

    fn launch_test(program: *const c_char, inherit: c_int, xtheadvector: c_int) -> c_int;
    fn is_vector_supported() -> c_int;
    fn is_xtheadvector_supported() -> c_int;
}

// Constants supplied by <sys/prctl.h> and the RISC-V uapi headers.
unsafe extern "C" {
    static PR_RISCV_V_SET_CONTROL: c_int;
    static PR_RISCV_V_GET_CONTROL: c_int;
    static PR_RISCV_V_VSTATE_CTRL_ON: c_long;
    static PR_RISCV_V_VSTATE_CTRL_OFF: c_long;
    static PR_RISCV_V_VSTATE_CTRL_INHERIT: c_long;
}

unsafe fn test_and_compare_child(
    provided: c_long,
    expected: c_long,
    inherit: c_int,
    xtheadvector: c_int,
) -> c_int {
    let mut rc: c_int;

    rc = prctl(PR_RISCV_V_SET_CONTROL, provided);
    if rc != 0 {
        printf(
            b"prctl with provided arg %lx failed with code %d\n\0".as_ptr() as *const c_char,
            provided,
            rc,
        );
        return -1;
    }
    rc = launch_test(
        NEXT_PROGRAM.as_ptr() as *const c_char,
        inherit,
        xtheadvector,
    );
    if rc as c_long != expected {
        printf(
            b"Test failed, check %d != %ld\n\0".as_ptr() as *const c_char,
            rc,
            expected,
        );
        return -2;
    }
    0
}

fn get_control_no_v() {
    unsafe {
        let rc: c_long;

        if is_vector_supported() != 0 || is_xtheadvector_supported() != 0 {
            SKIP!(return, "Test expects vector to be not supported");
        }

        rc = prctl(PR_RISCV_V_GET_CONTROL) as c_long;
        EXPECT_EQ!(-1, rc);
        TH_LOG!("GET_CONTROL should fail on kernel/hw without ZVE32X");
        EXPECT_EQ!(EINVAL, errno);
        TH_LOG!("GET_CONTROL should fail on kernel/hw without ZVE32X");
    }
}

fn set_control_no_v() {
    unsafe {
        let rc: c_long;

        if is_vector_supported() != 0 || is_xtheadvector_supported() != 0 {
            SKIP!(return, "Test expects vector to be not supported");
        }

        rc = prctl(PR_RISCV_V_SET_CONTROL, PR_RISCV_V_VSTATE_CTRL_ON) as c_long;
        EXPECT_EQ!(-1, rc);
        TH_LOG!("SET_CONTROL should fail on kernel/hw without ZVE32X");
        EXPECT_EQ!(EINVAL, errno);
        TH_LOG!("SET_CONTROL should fail on kernel/hw without ZVE32X");
    }
}

fn vstate_on_current() {
    unsafe {
        let flag: c_long;
        let rc: c_long;

        if is_vector_supported() == 0 && is_xtheadvector_supported() == 0 {
            SKIP!(return, "Vector not supported");
        }

        flag = PR_RISCV_V_VSTATE_CTRL_ON;
        rc = prctl(PR_RISCV_V_SET_CONTROL, flag) as c_long;
        EXPECT_EQ!(0, rc);
        TH_LOG!("Enabling V for current should always succeed");
    }
}

fn vstate_off_eperm() {
    unsafe {
        let flag: c_long;
        let rc: c_long;

        if is_vector_supported() == 0 && is_xtheadvector_supported() == 0 {
            SKIP!(return, "Vector not supported");
        }

        flag = PR_RISCV_V_VSTATE_CTRL_OFF;
        rc = prctl(PR_RISCV_V_SET_CONTROL, flag) as c_long;
        EXPECT_EQ!(EPERM, errno);
        TH_LOG!(
            "Disabling V in current thread with V enabled must fail with EPERM(%d)",
            errno
        );
        EXPECT_EQ!(-1, rc);
        TH_LOG!(
            "Disabling V in current thread with V enabled must fail with EPERM(%d)",
            errno
        );
    }
}

fn vstate_on_no_nesting() {
    unsafe {
        let flag: c_long;
        let mut xtheadvector: c_int = 0;

        if is_vector_supported() == 0 {
            if is_xtheadvector_supported() != 0 {
                xtheadvector = 1;
            } else {
                SKIP!(return, "Vector not supported");
            }
        }

        /* Turn on next's vector explicitly and test */
        flag = PR_RISCV_V_VSTATE_CTRL_ON << PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT;

        EXPECT_EQ!(
            0,
            test_and_compare_child(flag, PR_RISCV_V_VSTATE_CTRL_ON, 0, xtheadvector)
        );
    }
}

fn vstate_off_nesting() {
    unsafe {
        let flag: c_long;
        let mut xtheadvector: c_int = 0;

        if is_vector_supported() == 0 {
            if is_xtheadvector_supported() != 0 {
                xtheadvector = 1;
            } else {
                SKIP!(return, "Vector not supported");
            }
        }

        /* Turn off next's vector explicitly and test */
        flag = PR_RISCV_V_VSTATE_CTRL_OFF << PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT;

        EXPECT_EQ!(
            0,
            test_and_compare_child(flag, PR_RISCV_V_VSTATE_CTRL_OFF, 1, xtheadvector)
        );
    }
}

fn vstate_on_inherit_no_nesting() {
    unsafe {
        let mut flag: c_long;
        let expected: c_long;
        let mut xtheadvector: c_int = 0;

        if is_vector_supported() == 0 {
            if is_xtheadvector_supported() != 0 {
                xtheadvector = 1;
            } else {
                SKIP!(return, "Vector not supported");
            }
        }

        /* Turn on next's vector explicitly and test no inherit */
        flag = PR_RISCV_V_VSTATE_CTRL_ON << PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT;
        flag |= PR_RISCV_V_VSTATE_CTRL_INHERIT;
        expected = flag | PR_RISCV_V_VSTATE_CTRL_ON;

        EXPECT_EQ!(0, test_and_compare_child(flag, expected, 0, xtheadvector));
    }
}

fn vstate_on_inherit() {
    unsafe {
        let mut flag: c_long;
        let expected: c_long;
        let mut xtheadvector: c_int = 0;

        if is_vector_supported() == 0 {
            if is_xtheadvector_supported() != 0 {
                xtheadvector = 1;
            } else {
                SKIP!(return, "Vector not supported");
            }
        }

        /* Turn on next's vector explicitly and test inherit */
        flag = PR_RISCV_V_VSTATE_CTRL_ON << PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT;
        flag |= PR_RISCV_V_VSTATE_CTRL_INHERIT;
        expected = flag | PR_RISCV_V_VSTATE_CTRL_ON;

        EXPECT_EQ!(0, test_and_compare_child(flag, expected, 1, xtheadvector));
    }
}

fn vstate_off_inherit_no_nesting() {
    unsafe {
        let mut flag: c_long;
        let expected: c_long;
        let mut xtheadvector: c_int = 0;

        if is_vector_supported() == 0 {
            if is_xtheadvector_supported() != 0 {
                xtheadvector = 1;
            } else {
                SKIP!(return, "Vector not supported");
            }
        }
        /* Turn off next's vector explicitly and test no inherit */
        flag = PR_RISCV_V_VSTATE_CTRL_OFF << PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT;
        flag |= PR_RISCV_V_VSTATE_CTRL_INHERIT;
        expected = flag | PR_RISCV_V_VSTATE_CTRL_OFF;

        EXPECT_EQ!(0, test_and_compare_child(flag, expected, 0, xtheadvector));
    }
}

fn vstate_off_inherit() {
    unsafe {
        let mut flag: c_long;
        let expected: c_long;
        let mut xtheadvector: c_int = 0;

        if is_vector_supported() == 0 {
            if is_xtheadvector_supported() != 0 {
                xtheadvector = 1;
            } else {
                SKIP!(return, "Vector not supported");
            }
        }

        /* Turn off next's vector explicitly and test inherit */
        flag = PR_RISCV_V_VSTATE_CTRL_OFF << PR_RISCV_V_VSTATE_CTRL_NEXT_SHIFT;
        flag |= PR_RISCV_V_VSTATE_CTRL_INHERIT;
        expected = flag | PR_RISCV_V_VSTATE_CTRL_OFF;

        EXPECT_EQ!(0, test_and_compare_child(flag, expected, 1, xtheadvector));
    }
}

/* arguments should fail with EINVAL */
fn inval_set_control_1() {
    unsafe {
        let rc: c_int;

        if is_vector_supported() == 0 && is_xtheadvector_supported() == 0 {
            SKIP!(return, "Vector not supported");
        }

        rc = prctl(PR_RISCV_V_SET_CONTROL, 0xff0);
        EXPECT_EQ!(-1, rc);
        EXPECT_EQ!(EINVAL, errno);
    }
}

/* arguments should fail with EINVAL */
fn inval_set_control_2() {
    unsafe {
        let rc: c_int;

        if is_vector_supported() == 0 && is_xtheadvector_supported() == 0 {
            SKIP!(return, "Vector not supported");
        }

        rc = prctl(PR_RISCV_V_SET_CONTROL, 0x3);
        EXPECT_EQ!(-1, rc);
        EXPECT_EQ!(EINVAL, errno);
    }
}

/* arguments should fail with EINVAL */
fn inval_set_control_3() {
    unsafe {
        let rc: c_int;

        if is_vector_supported() == 0 && is_xtheadvector_supported() == 0 {
            SKIP!(return, "Vector not supported");
        }

        rc = prctl(PR_RISCV_V_SET_CONTROL, 0xc);
        EXPECT_EQ!(-1, rc);
        EXPECT_EQ!(EINVAL, errno);
    }
}

TEST_HARNESS_MAIN!();
