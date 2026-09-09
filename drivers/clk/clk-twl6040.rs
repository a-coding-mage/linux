// SPDX-License-Identifier: GPL-2.0-only
/*
 * TWL6040 clock module driver for OMAP4 McPDM functional clock
 *
 * Copyright (C) 2012 Texas Instruments Inc.
 * Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct twl6040_pdmclk {
    pub twl6040: *mut twl6040,
    pub dev: *mut device,
    pub pdmclk_hw: clk_hw,
    pub enabled: i32,
}

unsafe fn twl6040_pdmclk_is_prepared(hw: *mut clk_hw) -> i32 {
    let pdmclk = container_of!(hw, twl6040_pdmclk, pdmclk_hw);

    (*pdmclk).enabled
}

unsafe fn twl6040_pdmclk_reset_one_clock(
    pdmclk: *mut twl6040_pdmclk,
    reg: u32,
) -> i32 {
    const RESET_MASK: u8 = TWL6040_HPLLRST; // Same for HPPLL and LPPLL
    let mut ret: i32;

    ret = twl6040_set_bits((*pdmclk).twl6040, reg, RESET_MASK);
    if ret < 0 {
        return ret;
    }

    ret = twl6040_clear_bits((*pdmclk).twl6040, reg, RESET_MASK);
    if ret < 0 {
        return ret;
    }

    0
}

/*
 * TWL6040A2 Phoenix Audio IC erratum #6: "PDM Clock Generation Issue At
 * Cold Temperature". This affects cold boot and deeper idle states it
 * seems. The workaround consists of resetting HPPLL and LPPLL.
 */
unsafe fn twl6040_pdmclk_quirk_reset_clocks(pdmclk: *mut twl6040_pdmclk) -> i32 {
    let mut ret = twl6040_pdmclk_reset_one_clock(pdmclk, TWL6040_REG_HPPLLCTL);
    if ret != 0 {
        return ret;
    }

    ret = twl6040_pdmclk_reset_one_clock(pdmclk, TWL6040_REG_LPPLLCTL);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe fn twl6040_pdmclk_prepare(hw: *mut clk_hw) -> i32 {
    let pdmclk = container_of!(hw, twl6040_pdmclk, pdmclk_hw);
    let mut ret = twl6040_power((*pdmclk).twl6040, 1);
    if ret != 0 {
        return ret;
    }

    ret = twl6040_pdmclk_quirk_reset_clocks(pdmclk);
    if ret != 0 {
        dev_err!((*pdmclk).dev, "%s: error %i\n", __func__, ret);
        twl6040_power((*pdmclk).twl6040, 0);
        return ret;
    }

    (*pdmclk).enabled = 1;

    0
}

unsafe fn twl6040_pdmclk_unprepare(hw: *mut clk_hw) {
    let pdmclk = container_of!(hw, twl6040_pdmclk, pdmclk_hw);
    let ret = twl6040_power((*pdmclk).twl6040, 0);
    if ret == 0 {
        (*pdmclk).enabled = 0;
    }
}

unsafe fn twl6040_pdmclk_recalc_rate(
    hw: *mut clk_hw,
    _parent_rate: usize,
) -> usize {
    let pdmclk = container_of!(hw, twl6040_pdmclk, pdmclk_hw);

    twl6040_get_sysclk((*pdmclk).twl6040)
}

static twl6040_pdmclk_ops: clk_ops = clk_ops {
    is_prepared: Some(twl6040_pdmclk_is_prepared),
    prepare: Some(twl6040_pdmclk_prepare),
    unprepare: Some(twl6040_pdmclk_unprepare),
    recalc_rate: Some(twl6040_pdmclk_recalc_rate),
};

static twl6040_pdmclk_init: clk_init_data = clk_init_data {
    name: "pdmclk",
    ops: &twl6040_pdmclk_ops,
    flags: CLK_GET_RATE_NOCACHE,
};

unsafe fn twl6040_pdmclk_probe(pdev: *mut platform_device) -> i32 {
    let twl6040 = dev_get_drvdata((*pdev).dev.parent);
    let clkdata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<twl6040_pdmclk>(), GFP_KERNEL);
    if clkdata.is_null() {
        return -ENOMEM;
    }

    (*clkdata).dev = &mut (*pdev).dev;
    (*clkdata).twl6040 = twl6040;

    (*clkdata).pdmclk_hw.init = &twl6040_pdmclk_init;
    let ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*clkdata).pdmclk_hw);
    if ret != 0 {
        return ret;
    }

    platform_set_drvdata(pdev, clkdata);

    devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_simple_get, &mut (*clkdata).pdmclk_hw)
}

static mut twl6040_pdmclk_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "twl6040-pdmclk",
    },
    probe: Some(twl6040_pdmclk_probe),
};

module_platform_driver!(twl6040_pdmclk_driver);

module_description!("TWL6040 clock driver for McPDM functional clock");
module_author!("Peter Ujfalusi <peter.ujfalusi@ti.com>");
module_alias!("platform:twl6040-pdmclk");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
