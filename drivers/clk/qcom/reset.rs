// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding Linux/QCOM translation:
// linux/bitops.h, linux/export.h, linux/regmap.h,
// linux/reset-controller.h, linux/delay.h, and reset.h.

unsafe fn qcom_reset(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    let rst: *mut qcom_reset_controller = to_qcom_reset_controller(rcdev);

    ((*(*rcdev).ops).assert)(rcdev, id as ::core::ffi::c_ulong);
    fsleep(if (*rst).reset_map[id].udelay != 0 {
        (*rst).reset_map[id].udelay
    } else {
        1
    }); /* use 1 us as default */

    ((*(*rcdev).ops).deassert)(rcdev, id as ::core::ffi::c_ulong);
    0
}

unsafe fn qcom_reset_set_assert(
    rcdev: *mut reset_controller_dev,
    id: usize,
    assert_: bool,
) -> i32 {
    let rst: *mut qcom_reset_controller;
    let map: *const qcom_reset_map;
    let mut mask: u32;

    rst = to_qcom_reset_controller(rcdev);
    map = &(*rst).reset_map[id];
    mask = if (*map).bitmask != 0 {
        (*map).bitmask
    } else {
        1u32 << (*map).bit
    };

    regmap_update_bits(
        (*rst).regmap,
        (*map).reg,
        mask,
        if assert_ { mask } else { 0 },
    );

    /* Read back the register to ensure write completion, ignore the value */
    regmap_read((*rst).regmap, (*map).reg, &mut mask);

    0
}

unsafe fn qcom_reset_assert(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    qcom_reset_set_assert(rcdev, id, true)
}

unsafe fn qcom_reset_deassert(rcdev: *mut reset_controller_dev, id: usize) -> i32 {
    qcom_reset_set_assert(rcdev, id, false)
}

pub const qcom_reset_ops: reset_control_ops = reset_control_ops {
    reset: Some(qcom_reset),
    assert: Some(qcom_reset_assert),
    deassert: Some(qcom_reset_deassert),
};

// EXPORT_SYMBOL_GPL(qcom_reset_ops)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
