// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Free Electrons
 *
 * Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
 *
 * Allwinner A31 AR100 clock driver
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * sun6i_get_ar100_factors - Calculates factors p, m for AR100
 *
 * AR100 rate is calculated as follows
 * rate = (parent_rate >> p) / (m + 1);
 */
unsafe fn sun6i_get_ar100_factors(req: *mut factors_request) {
    let mut div: libc::c_ulong;
    let shift: libc::c_int;

    /* clock only divides */
    if (*req).rate > (*req).parent_rate {
        (*req).rate = (*req).parent_rate;
    }

    div = ((*req).parent_rate + (*req).rate - 1) / (*req).rate;

    if div < 32 {
        shift = 0;
    } else if div >> 1 < 32 {
        shift = 1;
    } else if div >> 2 < 32 {
        shift = 2;
    } else {
        shift = 3;
    }

    div >>= shift;

    if div > 32 {
        div = 32;
    }

    (*req).rate = ((*req).parent_rate >> shift) / div;
    (*req).m = div - 1;
    (*req).p = shift;
}

static sun6i_ar100_config: clk_factors_config = clk_factors_config {
    mwidth: 5,
    mshift: 8,
    pwidth: 2,
    pshift: 4,
};

static sun6i_ar100_data: factors_data = factors_data {
    mux: 16,
    muxmask: (1 << 2) - 1,
    table: &sun6i_ar100_config,
    getter: sun6i_get_ar100_factors,
};

static mut sun6i_ar100_lock: spinlock_t = spinlock_t::new();

unsafe fn sun6i_a31_ar100_clk_probe(pdev: *mut platform_device) -> libc::c_int {
    let np = (*(*pdev).dev.of_node);
    let mut reg: *mut core::ffi::c_void;
    let mut clk: *mut clk;

    reg = devm_platform_ioremap_resource(pdev, 0);
    if is_err(reg) {
        return ptr_err(reg);
    }

    clk = sunxi_factors_register(
        np,
        &sun6i_ar100_data,
        &mut sun6i_ar100_lock,
        reg,
    );
    if clk.is_null() {
        return -12;
    }

    platform_set_drvdata(pdev, clk);

    0
}

static sun6i_a31_ar100_clk_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"allwinner,sun6i-a31-ar100-clk", ..zeroed() },
    of_device_id { ..zeroed() },
];

static mut sun6i_a31_ar100_clk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"sun6i-a31-ar100-clk",
        of_match_table: sun6i_a31_ar100_clk_dt_ids.as_ptr(),
        suppress_bind_attrs: true,
        ..zeroed()
    },
    probe: Some(sun6i_a31_ar100_clk_probe),
    ..zeroed()
};

// builtin_platform_driver(sun6i_a31_ar100_clk_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
