// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

// C dependencies supplied by the kernel/live-update implementation are
// intentionally referenced here rather than reimplemented in this file.

use core::ffi::c_void;
use core::ptr;

const TEST_NFLBS: usize = 3;
const TEST_FLB_MAGIC_BASE: u64 = 0xFEED_F00D_CAFE_BEE0;

extern "C" {
    static THIS_MODULE: *mut c_void;

    fn liveupdate_flb_get_incoming(
        flb: *mut crate::liveupdate_flb,
        obj: *mut *mut c_void,
    ) -> i32;
    fn liveupdate_flb_put_incoming(flb: *mut crate::liveupdate_flb);
    fn liveupdate_register_flb(
        fh: *mut crate::liveupdate_file_handler,
        flb: *mut crate::liveupdate_flb,
    ) -> i32;
}

static mut TEST_FLB_OPS: crate::liveupdate_flb_ops = crate::liveupdate_flb_ops {
    preserve: Some(test_flb_preserve),
    unpreserve: Some(test_flb_unpreserve),
    retrieve: Some(test_flb_retrieve),
    finish: Some(test_flb_finish),
    owner: unsafe { &THIS_MODULE as *const *mut c_void as *mut c_void },
};

// LIVEUPDATE_TEST_FLB_COMPATIBLE(i) is supplied by the kernel headers.
static mut TEST_FLBS: [crate::liveupdate_flb; TEST_NFLBS] = [
    crate::liveupdate_flb {
        ops: unsafe { &TEST_FLB_OPS },
        compatible: crate::LIVEUPDATE_TEST_FLB_COMPATIBLE_0,
    },
    crate::liveupdate_flb {
        ops: unsafe { &TEST_FLB_OPS },
        compatible: crate::LIVEUPDATE_TEST_FLB_COMPATIBLE_1,
    },
    crate::liveupdate_flb {
        ops: unsafe { &TEST_FLB_OPS },
        compatible: crate::LIVEUPDATE_TEST_FLB_COMPATIBLE_2,
    },
];

unsafe extern "C" fn test_flb_preserve(argp: *mut crate::liveupdate_flb_op_args) -> i32 {
    let flb = (*argp).flb;
    let index = flb.offset_from(TEST_FLBS.as_ptr());

    pr_info!("%s: preserve was triggered\n", (*flb).compatible);
    (*argp).data = TEST_FLB_MAGIC_BASE.wrapping_add(index as u64);

    0
}

unsafe extern "C" fn test_flb_unpreserve(argp: *mut crate::liveupdate_flb_op_args) {
    let flb = (*argp).flb;
    pr_info!("%s: unpreserve was triggered\n", (*flb).compatible);
}

unsafe extern "C" fn test_flb_retrieve(argp: *mut crate::liveupdate_flb_op_args) -> i32 {
    let flb = (*argp).flb;
    let index = flb.offset_from(TEST_FLBS.as_ptr());
    let expected_data = TEST_FLB_MAGIC_BASE.wrapping_add(index as u64);

    if (*argp).data == expected_data {
        pr_info!("%s: found flb data from the previous boot\n", (*flb).compatible);
        (*argp).obj = (*argp).data as *mut c_void;
    } else {
        pr_err!(
            "%s: ERROR - incorrect data handle: %llx, expected %llx\n",
            (*flb).compatible,
            (*argp).data,
            expected_data
        );
        return -crate::EINVAL;
    }

    0
}

unsafe extern "C" fn test_flb_finish(argp: *mut crate::liveupdate_flb_op_args) {
    let flb = (*argp).flb;
    let index = flb.offset_from(TEST_FLBS.as_ptr());
    let expected_obj = TEST_FLB_MAGIC_BASE.wrapping_add(index as u64) as *mut c_void;

    if (*argp).obj == expected_obj {
        pr_info!("%s: finish was triggered\n", (*flb).compatible);
    } else {
        pr_err!("%s: ERROR - finish called with invalid object\n", (*flb).compatible);
    }
}

unsafe fn liveupdate_test_init() {
    static mut INITIALIZED: bool = false;

    // guard(mutex)(&init_lock); -- preserve the C mutex-guard intent.
    if INITIALIZED {
        return;
    }

    for i in 0..TEST_NFLBS {
        let flb = &mut TEST_FLBS[i] as *mut crate::liveupdate_flb;
        let mut obj: *mut c_void = ptr::null_mut();
        let err = liveupdate_flb_get_incoming(flb, &mut obj);

        if err != 0 && err != -crate::ENODATA && err != -crate::ENOENT {
            pr_err!("liveupdate_flb_get_incoming for %s failed: %pe\n", (*flb).compatible, err);
        }
        if err == 0 {
            liveupdate_flb_put_incoming(flb);
        }
    }
    INITIALIZED = true;
}

pub unsafe extern "C" fn liveupdate_test_register(fh: *mut crate::liveupdate_file_handler) {
    let mut err: i32;

    liveupdate_test_init();

    for i in 0..TEST_NFLBS {
        let flb = &mut TEST_FLBS[i] as *mut crate::liveupdate_flb;
        err = liveupdate_register_flb(fh, flb);
        if err != 0 {
            pr_err!("Failed to register %s %pe\n", (*flb).compatible, err);
        }
    }

    err = liveupdate_register_flb(fh, &mut TEST_FLBS[0]);
    if err == 0 || err != -crate::EEXIST {
        pr_err!(
            "Failed: %s should be already registered, but got err: %pe\n",
            TEST_FLBS[0].compatible,
            err
        );
    }

    pr_info!(
        "Registered %d FLBs with file handler: [%s]\n",
        TEST_NFLBS,
        (*fh).compatible
    );
}

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pasha Tatashin <pasha.tatashin@soleen.com>");
// MODULE_DESCRIPTION("In-kernel test for LUO mechanism");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
