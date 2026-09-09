// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM Integrator Logical Module bus driver
 * Copyright (C) 2020 Linaro Ltd.
 * Author: Linus Walleij <linus.walleij@linaro.org>
 *
 * See the device tree bindings for this block for more details on the
 * hardware.
 */

// Dependencies supplied by the surrounding kernel translation.

/* All information about the connected logic modules are in here */
const INTEGRATOR_SC_DEC_OFFSET: u32 = 0x10;

/* Base address for the expansion modules */
const INTEGRATOR_AP_EXP_BASE: u32 = 0xc0000000;
const INTEGRATOR_AP_EXP_STRIDE: u32 = 0x10000000;

unsafe fn integrator_lm_populate(num: i32, dev: *mut device) -> i32 {
    let np = (*dev).of_node;
    let mut child: *mut device_node;
    let base: u32;
    let mut ret: i32;

    base = INTEGRATOR_AP_EXP_BASE.wrapping_add(
        (num as u32).wrapping_mul(INTEGRATOR_AP_EXP_STRIDE),
    );

    /* Walk over the child nodes and see what chipselects we use */
    for_each_available_child_of_node!(np, child) {
        let mut res: resource = core::mem::zeroed();

        ret = of_address_to_resource(child, 0, &mut res);
        if ret != 0 {
            dev_info(dev, "no valid address on child\n");
            continue;
        }

        /* First populate the syscon then any devices */
        if res.start == base as u64 {
            dev_info!(dev, "populate module @0x%08x from DT\n", base);
            ret = of_platform_default_populate(child, core::ptr::null(), dev);
            if ret != 0 {
                dev_err!(dev, "failed to populate module\n");
                of_node_put(child);
                return ret;
            }
        }
    }

    0
}

static INTEGRATOR_AP_SYSCON_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "arm,integrator-ap-syscon" },
    of_device_id { compatible: "" },
];

unsafe fn integrator_ap_lm_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut syscon: *mut device_node;
    static mut MAP: *mut regmap = core::ptr::null_mut();
    let mut val: u32 = 0;
    let mut ret: i32;
    let mut i: i32;

    /* Look up the system controller */
    syscon = of_find_matching_node(core::ptr::null_mut(), INTEGRATOR_AP_SYSCON_MATCH.as_ptr());
    if syscon.is_null() {
        dev_err!(dev, "could not find Integrator/AP system controller\n");
        return -19;
    }
    MAP = syscon_node_to_regmap(syscon);
    of_node_put(syscon);
    if is_err(MAP) {
        dev_err!(dev, "could not find Integrator/AP system controller\n");
        return ptr_err(MAP);
    }

    ret = regmap_read(MAP, INTEGRATOR_SC_DEC_OFFSET, &mut val);
    if ret != 0 {
        dev_err!(dev, "could not read from Integrator/AP syscon\n");
        return ret;
    }

    /* Loop over the connected modules */
    i = 0;
    while i < 4 {
        if (val & (1u32 << (4 + i))) == 0 {
            i += 1;
            continue;
        }

        dev_info!(dev, "detected module in slot %d\n", i);
        ret = integrator_lm_populate(i, dev);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    0
}

static INTEGRATOR_AP_LM_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "arm,integrator-ap-lm" },
    of_device_id { compatible: "" },
];

static mut INTEGRATOR_AP_LM_DRIVER: platform_driver = platform_driver {
    probe: Some(integrator_ap_lm_probe),
    driver: driver {
        name: "integratorap-lm",
        of_match_table: INTEGRATOR_AP_LM_MATCH.as_ptr(),
    },
};

module_platform_driver!(INTEGRATOR_AP_LM_DRIVER);
module_author!("Linus Walleij <linus.walleij@linaro.org>");
module_description!("Integrator AP Logical Module driver");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
