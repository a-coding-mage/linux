// SPDX-License-Identifier: GPL-2.0
/*
 * Clock driver for TPS68470 PMIC
 *
 * Copyright (c) 2021 Red Hat Inc.
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *\tHans de Goede <hdegoede@redhat.com>
 *\tZaikuo Wang <zaikuo.wang@intel.com>
 *\tTianshu Qiu <tian.shu.qiu@intel.com>
 *\tJian Xu Zheng <jian.xu.zheng@intel.com>
 *\tYuning Pu <yuning.pu@intel.com>
 *\tAntti Laakso <antti.laakso@intel.com>
 */

// External Linux kernel declarations and TPS68470 constants are supplied by dependencies.

const TPS68470_CLK_NAME: *const core::ffi::c_char = c"tps68470-clk".as_ptr();

#[repr(C)]
struct Tps68470ClkoutFreqs {
    freq: ::core::ffi::c_ulong,
    xtaldiv: u32,
    plldiv: u32,
    postdiv: u32,
    buckdiv: u32,
    boostdiv: u32,
}

static mut CLK_FREQS: [Tps68470ClkoutFreqs; 3] = [
    Tps68470ClkoutFreqs { freq: 19200000, xtaldiv: 170, plldiv: 32, postdiv: 1, buckdiv: 2, boostdiv: 3 },
    Tps68470ClkoutFreqs { freq: 20000000, xtaldiv: 170, plldiv: 40, postdiv: 1, buckdiv: 3, boostdiv: 4 },
    Tps68470ClkoutFreqs { freq: 24000000, xtaldiv: 170, plldiv: 80, postdiv: 1, buckdiv: 4, boostdiv: 8 },
];

#[repr(C)]
struct Tps68470Clkdata {
    clkout_hw: ClkHw,
    regmap: *mut Regmap,
    rate: ::core::ffi::c_ulong,
}

unsafe fn to_tps68470_clkdata(clkd: *mut ClkHw) -> *mut Tps68470Clkdata {
    (clkd as *mut u8).sub(core::mem::offset_of!(Tps68470Clkdata, clkout_hw)) as *mut Tps68470Clkdata
}

unsafe extern "C" fn tps68470_clk_is_prepared(hw: *mut ClkHw) -> i32 {
    let clkdata = &mut *to_tps68470_clkdata(hw);
    let mut val: i32 = 0;
    if regmap_read(clkdata.regmap, TPS68470_REG_PLLCTL, &mut val) != 0 { return 0; }
    val & TPS68470_PLL_EN_MASK
}

unsafe extern "C" fn tps68470_clk_prepare(hw: *mut ClkHw) -> i32 {
    let clkdata = &mut *to_tps68470_clkdata(hw);
    regmap_write(clkdata.regmap, TPS68470_REG_CLKCFG1,
        (TPS68470_PLL_OUTPUT_ENABLE << TPS68470_OUTPUT_A_SHIFT) |
        (TPS68470_PLL_OUTPUT_ENABLE << TPS68470_OUTPUT_B_SHIFT));
    regmap_update_bits(clkdata.regmap, TPS68470_REG_PLLCTL,
        TPS68470_PLL_EN_MASK, TPS68470_PLL_EN_MASK);
    // The PMIC lock bit is not a true lock indication; wait approximately 4 ms.
    usleep_range(4000, 5000);
    0
}

unsafe extern "C" fn tps68470_clk_unprepare(hw: *mut ClkHw) {
    let clkdata = &mut *to_tps68470_clkdata(hw);
    regmap_update_bits(clkdata.regmap, TPS68470_REG_PLLCTL, TPS68470_PLL_EN_MASK, 0);
    regmap_write(clkdata.regmap, TPS68470_REG_CLKCFG1, 0);
}

unsafe extern "C" fn tps68470_clk_recalc_rate(hw: *mut ClkHw, _parent_rate: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    (*to_tps68470_clkdata(hw)).rate
}

unsafe fn tps68470_clk_cfg_lookup(rate: ::core::ffi::c_ulong) -> u32 {
    let mut best_diff: i64 = i64::MAX;
    let mut best_idx: u32 = 0;
    let mut i = 0;
    while i < CLK_FREQS.len() {
        let diff = CLK_FREQS[i].freq as i64 - rate as i64;
        if diff == 0 { return i as u32; }
        let abs_diff = diff.abs();
        if abs_diff < best_diff { best_diff = abs_diff; best_idx = i as u32; }
        i += 1;
    }
    best_idx
}

unsafe extern "C" fn tps68470_clk_determine_rate(_hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let idx = tps68470_clk_cfg_lookup((*req).rate) as usize;
    (*req).rate = CLK_FREQS[idx].freq;
    0
}

unsafe extern "C" fn tps68470_clk_set_rate(hw: *mut ClkHw, rate: ::core::ffi::c_ulong, _parent_rate: ::core::ffi::c_ulong) -> i32 {
    let clkdata = &mut *to_tps68470_clkdata(hw);
    let idx = tps68470_clk_cfg_lookup(rate) as usize;
    if rate != CLK_FREQS[idx].freq { return -EINVAL; }
    regmap_write(clkdata.regmap, TPS68470_REG_BOOSTDIV, CLK_FREQS[idx].boostdiv);
    regmap_write(clkdata.regmap, TPS68470_REG_BUCKDIV, CLK_FREQS[idx].buckdiv);
    regmap_write(clkdata.regmap, TPS68470_REG_PLLSWR, TPS68470_PLLSWR_DEFAULT);
    regmap_write(clkdata.regmap, TPS68470_REG_XTALDIV, CLK_FREQS[idx].xtaldiv);
    regmap_write(clkdata.regmap, TPS68470_REG_PLLDIV, CLK_FREQS[idx].plldiv);
    regmap_write(clkdata.regmap, TPS68470_REG_POSTDIV, CLK_FREQS[idx].postdiv);
    regmap_write(clkdata.regmap, TPS68470_REG_POSTDIV2, CLK_FREQS[idx].postdiv);
    regmap_write(clkdata.regmap, TPS68470_REG_CLKCFG2, TPS68470_CLKCFG2_DRV_STR_2MA);
    regmap_write(clkdata.regmap, TPS68470_REG_PLLCTL,
        (TPS68470_OSC_EXT_CAP_DEFAULT << TPS68470_OSC_EXT_CAP_SHIFT) |
        (TPS68470_CLK_SRC_XTAL << TPS68470_CLK_SRC_SHIFT));
    clkdata.rate = rate;
    0
}

#[repr(C)]
struct ClkOps {
    is_prepared: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    prepare: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    unprepare: Option<unsafe extern "C" fn(*mut ClkHw)>,
    recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, ::core::ffi::c_ulong) -> ::core::ffi::c_ulong>,
    determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
    set_rate: Option<unsafe extern "C" fn(*mut ClkHw, ::core::ffi::c_ulong, ::core::ffi::c_ulong) -> i32>,
}

static TPS68470_CLK_OPS: ClkOps = ClkOps {
    is_prepared: Some(tps68470_clk_is_prepared), prepare: Some(tps68470_clk_prepare),
    unprepare: Some(tps68470_clk_unprepare), recalc_rate: Some(tps68470_clk_recalc_rate),
    determine_rate: Some(tps68470_clk_determine_rate), set_rate: Some(tps68470_clk_set_rate),
};

unsafe extern "C" fn tps68470_clk_probe(pdev: *mut PlatformDevice) -> i32 { /* source-level external-kernel integration */
    let pdata = (*(*pdev).dev.platform_data).cast::<Tps68470ClkPlatformData>();
    let initdata = ClkInitData { name: TPS68470_CLK_NAME, ops: &TPS68470_CLK_OPS, flags: CLK_SET_RATE_GATE };
    let clkdata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Tps68470Clkdata>(), GFP_KERNEL) as *mut Tps68470Clkdata;
    if clkdata.is_null() { return -ENOMEM; }
    (*clkdata).regmap = dev_get_drvdata((*pdev).dev.parent);
    (*clkdata).clkout_hw.init = &initdata;
    tps68470_clk_set_rate(&mut (*clkdata).clkout_hw, CLK_FREQS[0].freq, 0);
    let mut ret = devm_clk_hw_register(&mut (*pdev).dev, &mut (*clkdata).clkout_hw);
    if ret != 0 { return ret; }
    ret = devm_clk_hw_register_clkdev(&mut (*pdev).dev, &mut (*clkdata).clkout_hw, TPS68470_CLK_NAME, core::ptr::null());
    if ret != 0 { return ret; }
    if !pdata.is_null() {
        for i in 0..(*pdata).n_consumers {
            let consumer = &(*pdata).consumers.add(i);
            ret = devm_clk_hw_register_clkdev(&mut (*pdev).dev, &mut (*clkdata).clkout_hw,
                (*consumer).consumer_con_id, (*consumer).consumer_dev_name);
        }
    }
    ret
}

// subsys_initcall(tps68470_clk_init); module_exit(tps68470_clk_exit);
// MODULE_ALIAS("platform:tps68470-clk");
// MODULE_DESCRIPTION("clock driver for TPS68470 pmic");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
