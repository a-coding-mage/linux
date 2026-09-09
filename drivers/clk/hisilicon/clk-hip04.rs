// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hisilicon HiP04 clock driver
 *
 * Copyright (c) 2013-2014 Hisilicon Limited.
 * Copyright (c) 2013-2014 Linaro Limited.
 *
 * Author: Haojian Zhuang <haojian.zhuang@linaro.org>
 */

// C dependencies supplied by the surrounding kernel clock framework:
// linux/kernel.h, linux/clk-provider.h, linux/io.h, linux/slab.h,
// dt-bindings/clock/hip04-clock.h, and clk.h.

/* fixed rate clocks */
static mut HIP04_FIXED_RATE_CLKS: [hisi_fixed_rate_clock; 3] = [
    hisi_fixed_rate_clock {
        id: HIP04_OSC50M,
        name: "osc50m",
        parent_name: core::ptr::null(),
        flags: 0,
        rate: 50000000,
    },
    hisi_fixed_rate_clock {
        id: HIP04_CLK_50M,
        name: "clk50m",
        parent_name: core::ptr::null(),
        flags: 0,
        rate: 50000000,
    },
    hisi_fixed_rate_clock {
        id: HIP04_CLK_168M,
        name: "clk168m",
        parent_name: core::ptr::null(),
        flags: 0,
        rate: 168750000,
    },
];

unsafe fn hip04_clk_init(np: *mut device_node) {
    let clk_data: *mut hisi_clock_data;

    clk_data = hisi_clk_init(np, HIP04_NR_CLKS);
    if clk_data.is_null() {
        return;
    }

    hisi_clk_register_fixed_rate(
        HIP04_FIXED_RATE_CLKS.as_mut_ptr(),
        HIP04_FIXED_RATE_CLKS.len(),
        clk_data,
    );
}

// Equivalent to CLK_OF_DECLARE(hip04_clk, "hisilicon,hip04-clock", hip04_clk_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
