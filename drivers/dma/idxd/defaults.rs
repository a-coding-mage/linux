// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2023 Intel Corporation. All rights rsvd. */

// The declarations and constants used below are supplied by idxd.h and the
// kernel environment in the surrounding translation unit.

pub unsafe fn idxd_load_iaa_device_defaults(idxd: *mut idxd_device) -> i32 {
    let mut engine: *mut idxd_engine;
    let mut group: *mut idxd_group;
    let mut wq: *mut idxd_wq;
    let mut i: i32;

    if !test_bit(IDXD_FLAG_CONFIGURABLE, &(*idxd).flags) {
        return 0;
    }

    wq = *(*idxd).wqs.add(0);

    if (*wq).state != IDXD_WQ_DISABLED {
        return -(EPERM as i32);
    }

    /* set mode to "dedicated" */
    set_bit(WQ_FLAG_DEDICATED, &mut (*wq).flags);
    (*wq).threshold = 0;

    /* only setting up 1 wq, so give it all the wq space */
    (*wq).size = (*idxd).max_wq_size;

    /* set priority to 10 */
    (*wq).priority = 10;

    /* set type to "kernel" */
    (*wq).type_ = IDXD_WQT_KERNEL;

    /* set wq group to 0 */
    group = *(*idxd).groups.add(0);
    (*wq).group = group;
    (*group).num_wqs += 1;

    /* set name to "iaa_crypto" */
    strscpy_pad(&mut (*wq).name, "iaa_crypto");

    /* set driver_name to "crypto" */
    strscpy_pad(&mut (*wq).driver_name, "crypto");

    /* assign all engines to group 0 */
    i = 0;
    while i < (*idxd).max_engines {
        engine = *(*idxd).engines.add(i as usize);
        (*engine).group = group;
        (*group).num_engines += 1;
        i += 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
