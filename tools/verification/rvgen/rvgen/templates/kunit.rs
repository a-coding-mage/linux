// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// #include <linux/kernel.h>
// #include <linux/rv.h>
// #include <rv/kunit.h>
/*
 * XXX: include required headers, e.g.,
 * #include <linux/sched.h>
 */
// #include "%%MODEL_NAME%%_kunit.h"

// Original C conditional:
// #if IS_REACHABLE(CONFIG_RV_MON_%%MODEL_NAME_UP%%)

unsafe extern "C" {
    static mut %%STRUCT_NAME%%: %%STRUCT_NAME%%;

    fn prepare_test(test: *mut kunit, mon: *mut ::core::ffi::c_void);
}

#[repr(C)]
pub struct kunit {
    pub priv_: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct rv_kunit_ctx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct %%STRUCT_NAME%% {
    pub mon: ::core::ffi::c_void,
    pub handle_event: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" fn rv_test_%%MODEL_NAME%%(test: *mut kunit) {
    let ctx: *mut rv_kunit_ctx = unsafe { (*test).priv_ as *mut rv_kunit_ctx };
    /*
     * If you need to create task_structs with rv_kunit_alloc_mock_task()
     * do it BEFORE preparing the test.
     */

    unsafe {
        prepare_test(
            test,
            &mut %%STRUCT_NAME%%.mon as *mut _ as *mut ::core::ffi::c_void,
        );
    }

    /*
     * XXX: write the test here
     * e.g.
     * RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
     *	%%STRUCT_NAME%%.handle_event(args);
     */
    let _ = ctx;
}

// Original C alternative:
// #else
// #define rv_test_%%MODEL_NAME%% rv_test_stub
// #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
