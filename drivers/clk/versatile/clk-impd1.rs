// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clock driver for the ARM Integrator/IM-PD1 board
 * Copyright (C) 2012-2013 Linus Walleij
 */

// Dependencies are supplied by the surrounding kernel translation.

const IMPD1_OSC1: u32 = 0x00;
const IMPD1_OSC2: u32 = 0x04;
const IMPD1_LOCK: u32 = 0x08;

/* There are two VCO's on the IM-PD1 */

static IMPD1_VCO1_PARAMS: icst_params = icst_params {
    ref_: 24000000, // 24 MHz
    vco_max: ICST525_VCO_MAX_3V,
    vco_min: ICST525_VCO_MIN,
    vd_min: 12,
    vd_max: 519,
    rd_min: 3,
    rd_max: 120,
    s2div: icst525_s2div,
    idx2s: icst525_idx2s,
};

static IMPD1_ICST1_DESC: clk_icst_desc = clk_icst_desc {
    params: &IMPD1_VCO1_PARAMS,
    vco_offset: IMPD1_OSC1,
    lock_offset: IMPD1_LOCK,
};

static IMPD1_VCO2_PARAMS: icst_params = icst_params {
    ref_: 24000000, // 24 MHz
    vco_max: ICST525_VCO_MAX_3V,
    vco_min: ICST525_VCO_MIN,
    vd_min: 12,
    vd_max: 519,
    rd_min: 3,
    rd_max: 120,
    s2div: icst525_s2div,
    idx2s: icst525_idx2s,
};

static IMPD1_ICST2_DESC: clk_icst_desc = clk_icst_desc {
    params: &IMPD1_VCO2_PARAMS,
    vco_offset: IMPD1_OSC2,
    lock_offset: IMPD1_LOCK,
};

unsafe fn integrator_impd1_clk_spawn(
    dev: *mut device,
    parent: *mut device_node,
    np: *mut device_node,
) -> i32 {
    let map: *mut regmap;
    let mut clk: *mut clk = ERR_PTR(-EINVAL);
    let mut name: *const i8 = (*np).name;
    let parent_name: *const i8;
    let desc: *const clk_icst_desc;
    let ret: i32;

    map = syscon_node_to_regmap(parent);
    if IS_ERR(map) {
        pr_err!("no regmap for syscon IM-PD1 ICST clock parent\\n");
        return PTR_ERR(map);
    }

    if of_device_is_compatible(np, c"arm,impd1-vco1") {
        desc = &IMPD1_ICST1_DESC;
    } else if of_device_is_compatible(np, c"arm,impd1-vco2") {
        desc = &IMPD1_ICST2_DESC;
    } else {
        dev_err!(dev, "not a clock node %s\\n", name);
        return -ENODEV;
    }

    of_property_read_string(np, c"clock-output-names", &mut name);
    parent_name = of_clk_get_parent_name(np, 0);
    clk = icst_clk_setup(
        core::ptr::null_mut(), desc, name, parent_name, map,
        ICST_INTEGRATOR_IM_PD1,
    );
    if !IS_ERR(clk) {
        of_clk_add_provider(np, of_clk_src_simple_get, clk);
        ret = 0;
    } else {
        dev_err!(dev, "error setting up IM-PD1 ICST clock\\n");
        ret = PTR_ERR(clk);
    }

    ret
}

unsafe fn integrator_impd1_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let np: *mut device_node = (*dev).of_node;
    let mut ret: i32 = 0;

    // Equivalent of for_each_available_child_of_node_scoped(np, child).
    let mut child = of_get_next_available_child(np, core::ptr::null_mut());
    while !child.is_null() {
        ret = integrator_impd1_clk_spawn(dev, np, child);
        if ret != 0 {
            break;
        }
        child = of_get_next_available_child(np, child);
    }

    ret
}

static IMPD1_SYSCON_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"arm,im-pd1-syscon" },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, impd1_syscon_match);

static mut IMPD1_CLK_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: c"impd1-clk",
        of_match_table: IMPD1_SYSCON_MATCH.as_ptr(),
    },
    probe: Some(integrator_impd1_clk_probe),
};

// builtin_platform_driver(impd1_clk_driver);

// MODULE_AUTHOR("Linus Walleij <linusw@kernel.org>");
// MODULE_DESCRIPTION("Arm IM-PD1 module clock driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
