// SPDX-License-Identifier: GPL-2.0
// Translated from perf/tests/subcmd-help.c.
// C includes: "tests.h", <linux/compiler.h>, <subcmd/help.h>

use core::ffi::{c_char, c_int};
use core::ptr;

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

#[repr(C)]
pub struct cmdnames {
    pub alloc: c_int,
    pub cnt: c_int,
    pub names: *mut *mut c_char,
}

unsafe extern "C" {
    fn add_cmdname(cmds: *mut cmdnames, name: *const c_char, len: usize);
    fn is_in_cmdlist(cmds: *mut cmdnames, name: *const c_char) -> c_int;
    fn clean_cmdnames(cmds: *mut cmdnames);
    fn uniq(cmds: *mut cmdnames);
    fn exclude_cmds(cmds: *mut cmdnames, excludes: *mut cmdnames);
}

const TEST_OK: c_int = 0;

macro_rules! TEST_ASSERT_VAL {
    ($msg:expr, $cond:expr) => {
        if !($cond) {
            return -1;
        }
    };
}

unsafe extern "C" fn test__load_cmdnames(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut cmds: cmdnames = cmdnames {
        alloc: 0,
        cnt: 0,
        names: ptr::null_mut(),
    };

    unsafe {
        add_cmdname(&mut cmds, c"aaa".as_ptr(), 3);
        add_cmdname(&mut cmds, c"foo".as_ptr(), 3);
        add_cmdname(&mut cmds, c"xyz".as_ptr(), 3);

        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds, c"aaa".as_ptr()) == 1);
        TEST_ASSERT_VAL!("wrong cmd", is_in_cmdlist(&mut cmds, c"bar".as_ptr()) == 0);
        TEST_ASSERT_VAL!("case sensitive", is_in_cmdlist(&mut cmds, c"XYZ".as_ptr()) == 0);

        clean_cmdnames(&mut cmds);
    }
    TEST_OK
}

unsafe extern "C" fn test__uniq_cmdnames(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut cmds: cmdnames = cmdnames {
        alloc: 0,
        cnt: 0,
        names: ptr::null_mut(),
    };

    unsafe {
        /* uniq() assumes it's sorted */
        add_cmdname(&mut cmds, c"aaa".as_ptr(), 3);
        add_cmdname(&mut cmds, c"aaa".as_ptr(), 3);
        add_cmdname(&mut cmds, c"bbb".as_ptr(), 3);

        TEST_ASSERT_VAL!("invalid original size", cmds.cnt == 3);
        /* uniquify command names (to remove second 'aaa') */
        uniq(&mut cmds);
        TEST_ASSERT_VAL!("invalid final size", cmds.cnt == 2);

        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds, c"aaa".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds, c"bbb".as_ptr()) == 1);
        TEST_ASSERT_VAL!("wrong cmd", is_in_cmdlist(&mut cmds, c"ccc".as_ptr()) == 0);

        clean_cmdnames(&mut cmds);
    }
    TEST_OK
}

unsafe extern "C" fn test__exclude_cmdnames(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut cmds1: cmdnames = cmdnames {
        alloc: 0,
        cnt: 0,
        names: ptr::null_mut(),
    };
    let mut cmds2: cmdnames = cmdnames {
        alloc: 0,
        cnt: 0,
        names: ptr::null_mut(),
    };

    unsafe {
        add_cmdname(&mut cmds1, c"aaa".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"bbb".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"ccc".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"ddd".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"eee".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"fff".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"ggg".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"hhh".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"iii".as_ptr(), 3);
        add_cmdname(&mut cmds1, c"jjj".as_ptr(), 3);

        add_cmdname(&mut cmds2, c"bbb".as_ptr(), 3);
        add_cmdname(&mut cmds2, c"eee".as_ptr(), 3);
        add_cmdname(&mut cmds2, c"jjj".as_ptr(), 3);

        TEST_ASSERT_VAL!("invalid original size", cmds1.cnt == 10);
        TEST_ASSERT_VAL!("invalid original size", cmds2.cnt == 3);

        /* remove duplicate command names in cmds1 */
        exclude_cmds(&mut cmds1, &mut cmds2);

        TEST_ASSERT_VAL!("invalid excluded size", cmds1.cnt == 7);
        TEST_ASSERT_VAL!("invalid excluded size", cmds2.cnt == 3);

        /* excluded commands should not belong to cmds1 */
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"aaa".as_ptr()) == 1);
        TEST_ASSERT_VAL!("wrong cmd", is_in_cmdlist(&mut cmds1, c"bbb".as_ptr()) == 0);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"ccc".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"ddd".as_ptr()) == 1);
        TEST_ASSERT_VAL!("wrong cmd", is_in_cmdlist(&mut cmds1, c"eee".as_ptr()) == 0);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"fff".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"ggg".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"hhh".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds1, c"iii".as_ptr()) == 1);
        TEST_ASSERT_VAL!("wrong cmd", is_in_cmdlist(&mut cmds1, c"jjj".as_ptr()) == 0);

        /* they should be only in cmds2 */
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds2, c"bbb".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds2, c"eee".as_ptr()) == 1);
        TEST_ASSERT_VAL!("cannot find cmd", is_in_cmdlist(&mut cmds2, c"jjj".as_ptr()) == 1);

        clean_cmdnames(&mut cmds1);
        clean_cmdnames(&mut cmds2);
    }
    TEST_OK
}

unsafe extern "C" fn test__exclude_cmdnames_no_overlap(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut cmds1: cmdnames = cmdnames {
        alloc: 0,
        cnt: 0,
        names: ptr::null_mut(),
    };
    let mut cmds2: cmdnames = cmdnames {
        alloc: 0,
        cnt: 0,
        names: ptr::null_mut(),
    };

    unsafe {
        add_cmdname(&mut cmds1, c"read-vdso32".as_ptr(), 11);
        add_cmdname(&mut cmds2, c"archive".as_ptr(), 7);

        TEST_ASSERT_VAL!("invalid original size", cmds1.cnt == 1);
        TEST_ASSERT_VAL!("invalid original size", cmds2.cnt == 1);

        exclude_cmds(&mut cmds1, &mut cmds2);

        TEST_ASSERT_VAL!("invalid excluded size", cmds1.cnt == 1);
        TEST_ASSERT_VAL!("invalid excluded size", cmds2.cnt == 1);

        TEST_ASSERT_VAL!(
            "cannot find cmd",
            is_in_cmdlist(&mut cmds1, c"read-vdso32".as_ptr()) == 1
        );
        TEST_ASSERT_VAL!("wrong cmd", is_in_cmdlist(&mut cmds1, c"archive".as_ptr()) == 0);

        clean_cmdnames(&mut cmds1);
        clean_cmdnames(&mut cmds2);
    }
    TEST_OK
}

#[unsafe(no_mangle)]
static mut tests__subcmd_help: [test_case; 5] = [
    test_case {
        name: c"Load subcmd names".as_ptr(),
        run_case: Some(test__load_cmdnames),
    },
    test_case {
        name: c"Uniquify subcmd names".as_ptr(),
        run_case: Some(test__uniq_cmdnames),
    },
    test_case {
        name: c"Exclude duplicate subcmd names".as_ptr(),
        run_case: Some(test__exclude_cmdnames),
    },
    test_case {
        name: c"Exclude disjoint subcmd names".as_ptr(),
        run_case: Some(test__exclude_cmdnames_no_overlap),
    },
    test_case {
        name: ptr::null(),
        run_case: None,
    },
];

#[unsafe(no_mangle)]
pub static mut suite__subcmd_help: test_suite = test_suite {
    desc: c"libsubcmd help tests".as_ptr(),
    test_cases: unsafe { tests__subcmd_help.as_mut_ptr() },
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
