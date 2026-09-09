// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 - 2014 Texas Instruments Incorporated - https://www.ti.com
 *
 * Authors:
 *    Jyri Sarha <jsarha@ti.com>
 *    Sergej Sawazki <ce3a@gmx.de>
 *
 * Gpio controlled clock implementation
 */

// Kernel dependencies supplied by other translation units.

/**
 * DOC: basic gpio gated clock which can be enabled and disabled
 *      with gpio output
 * Traits of this clock:
 * prepare - clk_(un)prepare are functional and control a gpio that can sleep
 * enable - clk_enable and clk_disable are functional & control
 *          non-sleeping gpio
 * rate - inherits rate from parent.  No clk_set_rate support
 * parent - fixed parent.  No clk_set_parent support
 */

/**
 * struct clk_gpio - gpio gated clock
 *
 * @hw:         handle between common and hardware-specific interfaces
 * @gpiod:      gpio descriptor
 *
 * Clock with a gpio control for enabling and disabling the parent clock
 * or switching between two parents by asserting or deasserting the gpio.
 *
 * Implements .enable, .disable and .is_enabled or
 * .get_parent, .set_parent and .determine_rate depending on which clk_ops
 * is used.
 */
#[repr(C)]
struct clk_gpio {
    hw: clk_hw,
    gpiod: *mut gpio_desc,
}

unsafe fn to_clk_gpio(hw: *mut clk_hw) -> *mut clk_gpio {
    ((hw as *mut u8).sub(core::mem::offset_of!(clk_gpio, hw)) as *mut clk_gpio)
}

unsafe extern "C" fn clk_gpio_gate_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_gpio(hw);
    gpiod_set_value((*clk).gpiod, 1);
    0
}

unsafe extern "C" fn clk_gpio_gate_disable(hw: *mut clk_hw) {
    let clk = to_clk_gpio(hw);
    gpiod_set_value((*clk).gpiod, 0);
}

unsafe extern "C" fn clk_gpio_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_gpio(hw);
    gpiod_get_value((*clk).gpiod)
}

static clk_gpio_gate_ops: clk_ops = clk_ops {
    enable: Some(clk_gpio_gate_enable),
    disable: Some(clk_gpio_gate_disable),
    is_enabled: Some(clk_gpio_gate_is_enabled),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn clk_sleeping_gpio_gate_prepare(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_gpio(hw);
    gpiod_set_value_cansleep((*clk).gpiod, 1);
    0
}

unsafe extern "C" fn clk_sleeping_gpio_gate_unprepare(hw: *mut clk_hw) {
    let clk = to_clk_gpio(hw);
    gpiod_set_value_cansleep((*clk).gpiod, 0);
}

unsafe extern "C" fn clk_sleeping_gpio_gate_is_prepared(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_gpio(hw);
    gpiod_get_value_cansleep((*clk).gpiod)
}

static clk_sleeping_gpio_gate_ops: clk_ops = clk_ops {
    prepare: Some(clk_sleeping_gpio_gate_prepare),
    unprepare: Some(clk_sleeping_gpio_gate_unprepare),
    is_prepared: Some(clk_sleeping_gpio_gate_is_prepared),
    ..unsafe { core::mem::zeroed() }
};

/** DOC: basic clock multiplexer controlled with a gpio output */
unsafe extern "C" fn clk_gpio_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = to_clk_gpio(hw);
    gpiod_get_value_cansleep((*clk).gpiod) as u8
}

unsafe extern "C" fn clk_gpio_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let clk = to_clk_gpio(hw);
    gpiod_set_value_cansleep((*clk).gpiod, index as i32);
    0
}

static clk_gpio_mux_ops: clk_ops = clk_ops {
    get_parent: Some(clk_gpio_mux_get_parent),
    set_parent: Some(clk_gpio_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn clk_register_gpio(
    dev: *mut device,
    num_parents: u8,
    gpiod: *mut gpio_desc,
    clk_gpio_ops: *const clk_ops,
) -> *mut clk_hw {
    let clk_gpio = devm_kzalloc(dev, core::mem::size_of::<clk_gpio>(), GFP_KERNEL)
        as *mut clk_gpio;
    if clk_gpio.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let gpio_parent_data = [
        clk_parent_data { index: 0 },
        clk_parent_data { index: 1 },
    ];
    let mut init: clk_init_data = core::mem::zeroed();
    (*init.name).write((*(*dev).of_node).name);
    init.ops = clk_gpio_ops;
    init.parent_data = gpio_parent_data.as_ptr();
    init.num_parents = num_parents;
    init.flags = CLK_SET_RATE_PARENT;

    (*clk_gpio).gpiod = gpiod;
    (*clk_gpio).hw.init = &init;

    let hw = &mut (*clk_gpio).hw as *mut clk_hw;
    let err = devm_clk_hw_register(dev, hw);
    if err != 0 {
        return ERR_PTR(err);
    }
    hw
}

unsafe fn clk_hw_register_gpio_gate(
    dev: *mut device,
    num_parents: i32,
    gpiod: *mut gpio_desc,
) -> *mut clk_hw {
    let ops = if gpiod_cansleep(gpiod) {
        &clk_sleeping_gpio_gate_ops
    } else {
        &clk_gpio_gate_ops
    };
    clk_register_gpio(dev, num_parents as u8, gpiod, ops)
}

unsafe fn clk_hw_register_gpio_mux(dev: *mut device, gpiod: *mut gpio_desc) -> *mut clk_hw {
    clk_register_gpio(dev, 2, gpiod, &clk_gpio_mux_ops)
}

unsafe extern "C" fn gpio_clk_driver_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let node = (*dev).of_node;
    let is_mux = of_device_is_compatible(node, c"gpio-mux-clock".as_ptr());
    let num_parents = of_clk_get_parent_count(node) as u32;
    if is_mux && num_parents != 2 {
        dev_err(dev, c"mux-clock must have 2 parents\n".as_ptr());
        return -EINVAL;
    }

    let gpio_name = if is_mux { c"select" } else { c"enable" };
    let gpiod = devm_gpiod_get(dev, gpio_name.as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR(gpiod) {
        return dev_err_probe(dev, PTR_ERR(gpiod), c"Can't get '%s' named GPIO property\n".as_ptr(), gpio_name.as_ptr());
    }

    let hw = if is_mux {
        clk_hw_register_gpio_mux(dev, gpiod)
    } else {
        clk_hw_register_gpio_gate(dev, num_parents as i32, gpiod)
    };
    if IS_ERR(hw) {
        return PTR_ERR(hw);
    }
    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, hw)
}

static gpio_clk_match_table: [of_device_id; 3] = [
    of_device_id { compatible: c"gpio-mux-clock".as_ptr() },
    of_device_id { compatible: c"gpio-gate-clock".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut gpio_clk_driver: platform_driver = platform_driver {
    probe: Some(gpio_clk_driver_probe),
    driver: device_driver {
        name: c"gpio-clk".as_ptr(),
        of_match_table: gpio_clk_match_table.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
};

builtin_platform_driver!(gpio_clk_driver);

/** DOC: gated fixed clock, controlled with a gpio output and a regulator */
#[repr(C)]
struct clk_gated_fixed {
    clk_gpio: clk_gpio,
    supply: *mut regulator,
    rate: c_ulong,
}

unsafe fn to_clk_gated_fixed(clk_gpio: *mut clk_gpio) -> *mut clk_gated_fixed {
    ((clk_gpio as *mut u8).sub(core::mem::offset_of!(clk_gated_fixed, clk_gpio)) as *mut clk_gated_fixed)
}

unsafe extern "C" fn clk_gated_fixed_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    (*to_clk_gated_fixed(to_clk_gpio(hw))).rate
}

unsafe extern "C" fn clk_gated_fixed_prepare(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_gated_fixed(to_clk_gpio(hw));
    if (*clk).supply.is_null() { return 0; }
    regulator_enable((*clk).supply)
}

unsafe extern "C" fn clk_gated_fixed_unprepare(hw: *mut clk_hw) {
    let clk = to_clk_gated_fixed(to_clk_gpio(hw));
    if !(*clk).supply.is_null() { regulator_disable((*clk).supply); }
}

unsafe extern "C" fn clk_gated_fixed_is_prepared(hw: *mut clk_hw) -> bool {
    let clk = to_clk_gated_fixed(to_clk_gpio(hw));
    if (*clk).supply.is_null() { return true; }
    regulator_is_enabled((*clk).supply) != 0
}

/* Fixed gated clock with non-sleeping gpio. */
static clk_gated_fixed_ops: clk_ops = clk_ops {
    prepare: Some(clk_gated_fixed_prepare),
    unprepare: Some(clk_gated_fixed_unprepare),
    is_prepared: Some(clk_gated_fixed_is_prepared),
    enable: Some(clk_gpio_gate_enable),
    disable: Some(clk_gpio_gate_disable),
    is_enabled: Some(clk_gpio_gate_is_enabled),
    recalc_rate: Some(clk_gated_fixed_recalc_rate),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn clk_sleeping_gated_fixed_prepare(hw: *mut clk_hw) -> i32 {
    let mut ret = clk_gated_fixed_prepare(hw);
    if ret != 0 { return ret; }
    ret = clk_sleeping_gpio_gate_prepare(hw);
    if ret != 0 { clk_gated_fixed_unprepare(hw); }
    ret
}

unsafe extern "C" fn clk_sleeping_gated_fixed_unprepare(hw: *mut clk_hw) {
    clk_gated_fixed_unprepare(hw);
    clk_sleeping_gpio_gate_unprepare(hw);
}

/* Fixed gated clock with sleeping gpio. */
static clk_sleeping_gated_fixed_ops: clk_ops = clk_ops {
    prepare: Some(clk_sleeping_gated_fixed_prepare),
    unprepare: Some(clk_sleeping_gated_fixed_unprepare),
    is_prepared: Some(clk_sleeping_gpio_gate_is_prepared),
    recalc_rate: Some(clk_gated_fixed_recalc_rate),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn clk_gated_fixed_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let clk = devm_kzalloc(dev, core::mem::size_of::<clk_gated_fixed>(), GFP_KERNEL)
        as *mut clk_gated_fixed;
    if clk.is_null() { return -ENOMEM; }

    let mut rate: u32 = 0;
    let mut ret = device_property_read_u32(dev, c"clock-frequency".as_ptr(), &mut rate);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to get clock-frequency\n".as_ptr()); }
    (*clk).rate = rate as c_ulong;

    let mut clk_name: *const c_char = core::ptr::null();
    ret = device_property_read_string(dev, c"clock-output-names".as_ptr(), &mut clk_name);
    if ret != 0 { clk_name = fwnode_get_name((*dev).fwnode); }

    (*clk).supply = devm_regulator_get_optional(dev, c"vdd".as_ptr());
    if IS_ERR((*clk).supply) {
        if PTR_ERR((*clk).supply) != -ENODEV {
            return dev_err_probe(dev, PTR_ERR((*clk).supply), c"Failed to get regulator\n".as_ptr());
        }
        (*clk).supply = core::ptr::null_mut();
    }

    (*clk).clk_gpio.gpiod = devm_gpiod_get_optional(dev, c"enable".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*clk).clk_gpio.gpiod) {
        return dev_err_probe(dev, PTR_ERR((*clk).clk_gpio.gpiod), c"Failed to get gpio\n".as_ptr());
    }

    let ops = if gpiod_cansleep((*clk).clk_gpio.gpiod) {
        &clk_sleeping_gated_fixed_ops
    } else {
        &clk_gated_fixed_ops
    };
    (*clk).clk_gpio.hw.init = CLK_HW_INIT_NO_PARENT(clk_name, ops, 0);

    ret = devm_clk_hw_register(dev, &mut (*clk).clk_gpio.hw);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to register clock\n".as_ptr()); }

    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut (*clk).clk_gpio.hw);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to register clock provider\n".as_ptr()); }
    0
}

static gated_fixed_clk_match_table: [of_device_id; 2] = [
    of_device_id { compatible: c"gated-fixed-clock".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut gated_fixed_clk_driver: platform_driver = platform_driver {
    probe: Some(clk_gated_fixed_probe),
    driver: device_driver {
        name: c"gated-fixed-clk".as_ptr(),
        of_match_table: gated_fixed_clk_match_table.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
};

builtin_platform_driver!(gated_fixed_clk_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
