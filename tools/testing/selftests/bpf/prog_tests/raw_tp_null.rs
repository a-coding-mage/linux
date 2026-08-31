// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* Dependencies from C source:
 * #include <test_progs.h>
 * #include "raw_tp_null.skel.h"
 * #include "raw_tp_null_fail.skel.h"
 */

pub unsafe fn test_raw_tp_null() {
    let mut skel: *mut raw_tp_null;

    RUN_TESTS!(raw_tp_null_fail);

    skel = raw_tp_null__open_and_load();
    if !ASSERT_OK_PTR(skel, c"raw_tp_null__open_and_load".as_ptr()) {
        return;
    }

    (*(*skel).bss).tid = sys_gettid();

    if !ASSERT_OK(
        raw_tp_null__attach(skel),
        c"raw_tp_null__attach".as_ptr(),
    ) {
        goto_end(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(2), c"trigger testmod read".as_ptr());
    ASSERT_EQ((*(*skel).bss).i, 3, c"invocations".as_ptr());

    goto_end(skel);
}

unsafe fn goto_end(skel: *mut raw_tp_null) {
    raw_tp_null__destroy(skel);
}
