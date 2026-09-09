// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clock driver for Hi655x
 *
 * Copyright (c) 2017, Linaro Ltd.
 *
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org>
 */

// Linux kernel dependencies supplied by other translation units.

const HI655X_CLK_BASE: u32 = HI655X_BUS_ADDR(0x1c);
const HI655X_CLK_SET: u32 = BIT(6);

#[repr(C)]
pub struct hi655x_clk {
    pub hi655x: *mut hi655x_pmic,
    pub clk_hw: clk_hw,
}

unsafe extern "C" {
    fn regmap_update_bits(regmap: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_read(regmap: *mut regmap, reg: u32, val: *mut u32) -> i32;
}

unsafe fn hi655x_clk_recalc_rate(_hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    32768
}

unsafe fn hi655x_clk_enable(hw: *mut clk_hw, enable: bool) -> i32 {
    let hi655x_clk = container_of!(hw, hi655x_clk, clk_hw);
    let hi655x = (*hi655x_clk).hi655x;

    regmap_update_bits(
        (*hi655x).regmap,
        HI655X_CLK_BASE,
        HI655X_CLK_SET,
        if enable { HI655X_CLK_SET } else { 0 },
    )
}

unsafe fn hi655x_clk_prepare(hw: *mut clk_hw) -> i32 {
    hi655x_clk_enable(hw, true)
}

unsafe fn hi655x_clk_unprepare(hw: *mut clk_hw) {
    hi655x_clk_enable(hw, false);
}

unsafe fn hi655x_clk_is_prepared(hw: *mut clk_hw) -> i32 {
    let hi655x_clk = container_of!(hw, hi655x_clk, clk_hw);
    let hi655x = (*hi655x_clk).hi655x;
    let mut val: u32 = 0;

    let ret = regmap_read((*hi655x).regmap, HI655X_CLK_BASE, &mut val);
    if ret < 0 {
        return ret;
    }

    (val & HI655X_CLK_BASE) as i32
}

static hi655x_clk_ops: clk_ops = clk_ops {
    prepare: Some(hi655x_clk_prepare),
    unprepare: Some(hi655x_clk_unprepare),
    is_prepared: Some(hi655x_clk_is_prepared),
    recalc_rate: Some(hi655x_clk_recalc_rate),
};

unsafe fn hi655x_clk_probe(pdev: *mut platform_device) -> i32 {
    let parent = (*pdev).dev.parent;
    let hi655x = dev_get_drvdata(parent) as *mut hi655x_pmic;
    let mut hi655x_clk: *mut hi655x_clk;
    let mut clk_name: *const c_char = b"hi655x-clk\0".as_ptr() as *const c_char;
    let mut init = clk_init_data {
        name: clk_name,
        ops: &hi655x_clk_ops,
    };
    let ret: i32;

    hi655x_clk = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<hi655x_clk>(), GFP_KERNEL)
        as *mut hi655x_clk;
    if hi655x_clk.is_null() {
        return -ENOMEM;
    }

    of_property_read_string_index((*parent).of_node, b"clock-output-names\0".as_ptr() as *const c_char, 0, &mut clk_name);

    (*hi655x_clk).clk_hw.init = &mut init;
    (*hi655x_clk).hi655x = hi655x;

    platform_set_drvdata(pdev, hi655x_clk as *mut c_void);

    ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*hi655x_clk).clk_hw);
    if ret != 0 {
        return ret;
    }

    devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_simple_get, &mut (*hi655x_clk).clk_hw)
}

static mut hi655x_clk_driver: platform_driver = platform_driver {
    probe: Some(hi655x_clk_probe),
    driver: driver {
        name: b"hi655x-clk\0".as_ptr() as *const c_char,
    },
};

// module_platform_driver(hi655x_clk_driver);
// MODULE_DESCRIPTION("Clk driver for the hi655x series PMICs");
// MODULE_AUTHOR("Daniel Lezcano <daniel.lezcano@linaro.org>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:hi655x-clk");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
