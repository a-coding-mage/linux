// SPDX-License-Identifier: GPL-2.0
// C dependencies:
// #include <linux/kernel.h>
// #include <linux/rv.h>
// #include <rv/kunit.h>
/*
 * XXX: include required headers, e.g.,
 * #include <linux/sched.h>
 */
// #include "test_ha_kunit_kunit.h"

extern "C" {
    static mut rv_test_ha_kunit_ops: RvTestHaKunitOps;

    fn prepare_test(test: *mut kunit, mon: *mut ::core::ffi::c_void);
    fn rv_test_stub(test: *mut kunit);
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct rv_kunit_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RvTestHaKunitOps {
    pub mon: *mut ::core::ffi::c_void,
}

// Original C condition:
// #if IS_REACHABLE(CONFIG_RV_MON_TEST_HA_KUNIT)
#[cfg(CONFIG_RV_MON_TEST_HA_KUNIT)]
unsafe extern "C" fn rv_test_test_ha_kunit(test: *mut kunit) {
    let ctx: *mut rv_kunit_ctx = (*test).priv_ as *mut rv_kunit_ctx;
    /*
     * If you need to create task_structs with rv_kunit_alloc_mock_task()
     * do it BEFORE preparing the test.
     */

    prepare_test(test, &mut rv_test_ha_kunit_ops.mon as *mut _ as *mut ::core::ffi::c_void);

    /*
     * XXX: write the test here
     * e.g.
     * RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
     *	rv_test_ha_kunit_ops.handle_event(args);
     */
}

// Original C fallback:
// #else
// #define rv_test_test_ha_kunit rv_test_stub
#[cfg(not(CONFIG_RV_MON_TEST_HA_KUNIT))]
pub(crate) use rv_test_stub as rv_test_test_ha_kunit;
// #endif
