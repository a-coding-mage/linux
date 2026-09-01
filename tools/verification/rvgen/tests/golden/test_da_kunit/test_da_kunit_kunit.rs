// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/kernel.h>, <linux/rv.h>, <rv/kunit.h>
/*
 * XXX: include required headers, e.g.,
 * #include <linux/sched.h>
 */
// C dependency: "test_da_kunit_kunit.h"

// Original C condition:
// #if IS_REACHABLE(CONFIG_RV_MON_TEST_DA_KUNIT)

#[repr(C)]
pub struct kunit {
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rv_kunit_ctx {
    _private: [u8; 0],
}

unsafe extern "C" {
    static rv_test_da_kunit_ops: RvTestDaKunitOps;

    fn prepare_test(test: *mut kunit, mon: *const core::ffi::c_void);

    // Used by the original #else branch:
    // #define rv_test_test_da_kunit rv_test_stub
    fn rv_test_stub(test: *mut kunit);
}

#[repr(C)]
pub struct RvTestDaKunitOps {
    pub mon: core::ffi::c_void,
}

unsafe extern "C" fn rv_test_test_da_kunit(test: *mut kunit) {
    let ctx: *mut rv_kunit_ctx = unsafe { (*test).priv_ as *mut rv_kunit_ctx };
    /*
     * If you need to create task_structs with rv_kunit_alloc_mock_task()
     * do it BEFORE preparing the test.
     */

    unsafe {
        prepare_test(test, &rv_test_da_kunit_ops.mon as *const core::ffi::c_void);
    }

    /*
     * XXX: write the test here
     * e.g.
     * RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
     *	rv_test_da_kunit_ops.handle_event(args);
     */
    let _ = ctx;
}

// Original C fallback when !IS_REACHABLE(CONFIG_RV_MON_TEST_DA_KUNIT):
// #define rv_test_test_da_kunit rv_test_stub

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
