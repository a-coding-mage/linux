// SPDX-License-Identifier: GPL-2.0
// C dependencies:
// #include <linux/kernel.h>
// #include <linux/rv.h>
// #include <rv/kunit.h>
/*
 * XXX: include required headers, e.g.,
 * #include <linux/sched.h>
 */
// #include "test_bak_kunit_kunit.h"

#[repr(C)]
pub struct kunit {
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rv_kunit_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_test_bak_kunit_ops_type {
    pub mon: rv_monitor,
}

unsafe extern "C" {
    pub static rv_test_bak_kunit_ops: rv_test_bak_kunit_ops_type;
    pub fn prepare_test(test: *mut kunit, mon: *const rv_monitor);
    pub fn rv_test_stub(test: *mut kunit);
}

// Original C condition:
// #if IS_REACHABLE(CONFIG_RV_MON_TEST_BAK_KUNIT)
#[cfg(CONFIG_RV_MON_TEST_BAK_KUNIT)]
unsafe extern "C" fn rv_test_test_bak_kunit(test: *mut kunit) {
    let ctx: *mut rv_kunit_ctx = unsafe { (*test).priv_ as *mut rv_kunit_ctx };
    let _ = ctx;
    /*
     * If you need to create task_structs with rv_kunit_alloc_mock_task()
     * do it BEFORE preparing the test.
     */

    unsafe {
        prepare_test(test, &rv_test_bak_kunit_ops.mon);
    }

    /*
     * XXX: write the test here
     * e.g.
     * RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
     *	rv_test_bak_kunit_ops.handle_event(args);
     */
}

// Original C fallback:
// #else
// #define rv_test_test_bak_kunit rv_test_stub
// #endif
#[cfg(not(CONFIG_RV_MON_TEST_BAK_KUNIT))]
pub use rv_test_stub as rv_test_test_bak_kunit;
